use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::info;

use harana_components_schedule::{
    DurableScheduler, Job, JobExecutor as ScheduleJobExecutor, Schedule, ScheduleStore,
    SchedulerConfig,
};
use harana_components_storage::Store;

use crate::{Task, TaskError, TaskExecutionHistory, TaskPriority, TaskResult, store as task_store};

// ============================================================================
// Scheduled Task Configuration
// ============================================================================

#[derive(Debug, Clone)]
pub struct ScheduledTaskConfig {
        pub name: String,
        pub description: String,
        pub task_type: String,
        pub queue: String,
        pub priority: TaskPriority,
        pub payload: HashMap<String, serde_json::Value>,
        pub timeout_secs: u64,
        pub tags: Vec<String>,
        pub owner_id: Option<String>,
}

impl ScheduledTaskConfig {
    pub fn new(
        name: impl Into<String>,
        task_type: impl Into<String>,
        queue: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            task_type: task_type.into(),
            queue: queue.into(),
            priority: TaskPriority::Normal,
            payload: HashMap::new(),
            timeout_secs: 300,
            tags: Vec::new(),
            owner_id: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_payload(mut self, payload: HashMap<String, serde_json::Value>) -> Self {
        self.payload = payload;
        self
    }

    pub fn with_payload_value(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.payload.insert(key.into(), value);
        self
    }

    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_owner(mut self, owner_id: impl Into<String>) -> Self {
        self.owner_id = Some(owner_id.into());
        self
    }
}

// ============================================================================
// Task Scheduler Bridge
// ============================================================================

pub struct TaskCreatingExecutor<S: Store<Task>> {
    task_store: Arc<S>,
    task_configs: RwLock<HashMap<String, ScheduledTaskConfig>>,
}

impl<S: Store<Task>> TaskCreatingExecutor<S> {
    pub fn new(task_store: Arc<S>) -> Self {
        Self {
            task_store,
            task_configs: RwLock::new(HashMap::new()),
        }
    }
    pub fn register_task_config(&self, schedule_id: impl Into<String>, config: ScheduledTaskConfig) {
        self.task_configs
            .write()
            .insert(schedule_id.into(), config);
    }

    /// Unregister a task configuration
    pub fn unregister_task_config(&self, schedule_id: &str) {
        self.task_configs.write().remove(schedule_id);
    }
    pub fn get_task_config(&self, schedule_id: &str) -> Option<ScheduledTaskConfig> {
        self.task_configs.read().get(schedule_id).cloned()
    }
}

#[async_trait::async_trait]
impl<S: Store<Task> + 'static> ScheduleJobExecutor for TaskCreatingExecutor<S> {
    async fn execute(&self, job: &Job) -> Result<Option<serde_json::Value>, String> {
        // Get the task configuration for this schedule
        let config = self
            .task_configs
            .read()
            .get(&job.schedule_id)
            .cloned()
            .ok_or_else(|| {
                format!(
                    "No task configuration found for schedule: {}",
                    job.schedule_id
                )
            })?;

        // Create the task
        let task = Task::new(&config.name, &config.task_type, &config.queue)
            .with_description(&config.description)
            .with_priority(config.priority)
            .with_payload(config.payload.clone())
            .with_timeout(config.timeout_secs)
            .with_tags(config.tags.clone())
            .with_schedule(&job.schedule_id)
            .with_correlation_id(&job.id);

        let task = if let Some(owner_id) = &config.owner_id {
            task.with_owner(owner_id)
        } else {
            task
        };

        // Store the task
        task_store::create_task(self.task_store.as_ref(), &task)
            .await
            .map_err(|e| format!("Failed to create task: {}", e))?;

        info!(
            task_id = %task.id,
            schedule_id = %job.schedule_id,
            job_id = %job.id,
            "Created task from schedule"
        );

        Ok(Some(serde_json::json!({
            "task_id": task.id,
            "task_name": task.name,
            "queue": task.queue,
        })))
    }

    fn can_handle(&self, action_type: &str) -> bool {
        action_type == "create_task" || action_type.starts_with("task:")
    }
}

// ============================================================================
// Task Scheduler Manager
// ============================================================================

#[derive(Debug, Clone)]
pub enum TaskSchedulerEvent {
    ScheduledTaskCreated {
        schedule_id: String,
        task_id: String,
    },
    ScheduleTriggered {
        schedule_id: String,
        job_id: String,
    },
    ScheduleEnabled { schedule_id: String },
    ScheduleDisabled { schedule_id: String },
}

pub struct TaskSchedulerManager<TS, THS, SS>
where
    TS: Store<Task> + 'static,
    THS: Store<TaskExecutionHistory> + 'static,
    SS: ScheduleStore + 'static,
{
    task_store: Arc<TS>,
    _history_store: Arc<THS>,
    scheduler: Arc<RwLock<DurableScheduler<SS>>>,
    executor: Arc<TaskCreatingExecutor<TS>>,
    event_sender: broadcast::Sender<TaskSchedulerEvent>,
}

impl<TS, THS, SS> TaskSchedulerManager<TS, THS, SS>
where
    TS: Store<Task> + 'static,
    THS: Store<TaskExecutionHistory> + 'static,
    SS: ScheduleStore + 'static,
{
    pub fn new(
        task_store: TS,
        history_store: THS,
        schedule_store: SS,
        scheduler_config: SchedulerConfig,
    ) -> Self {
        let task_store = Arc::new(task_store);
        let history_store = Arc::new(history_store);
        let executor = Arc::new(TaskCreatingExecutor::new(Arc::clone(&task_store)));
        
        let mut scheduler = DurableScheduler::new(schedule_store, scheduler_config);
        scheduler.register_executor(Arc::clone(&executor) as Arc<dyn ScheduleJobExecutor>);

        let (event_sender, _) = broadcast::channel(1000);

        Self {
            task_store,
            _history_store: history_store,
            scheduler: Arc::new(RwLock::new(scheduler)),
            executor,
            event_sender,
        }
    }
    pub fn subscribe(&self) -> broadcast::Receiver<TaskSchedulerEvent> {
        self.event_sender.subscribe()
    }

    /// Get the task store
    pub fn task_store(&self) -> &TS {
        &self.task_store
    }
    pub async fn create_cron_task(
        &self,
        schedule_id: impl Into<String>,
        schedule_name: impl Into<String>,
        cron_expression: impl Into<String>,
        task_config: ScheduledTaskConfig,
    ) -> TaskResult<Schedule> {
        let schedule_id = schedule_id.into();
        let schedule = Schedule::cron(&schedule_id, schedule_name, cron_expression)
            .with_action("create_task", HashMap::new());

        // Register the task config
        self.executor.register_task_config(&schedule_id, task_config);

        // Create the schedule
        let schedule = self
            .scheduler
            .write()
            .create_schedule(schedule)
            .await
            .map_err(|e| TaskError::ScheduleError(e.to_string()))?;

        Ok(schedule)
    }
    pub async fn create_interval_task(
        &self,
        schedule_id: impl Into<String>,
        schedule_name: impl Into<String>,
        interval_seconds: i64,
        task_config: ScheduledTaskConfig,
    ) -> TaskResult<Schedule> {
        let schedule_id = schedule_id.into();
        let schedule = Schedule::interval(&schedule_id, schedule_name, interval_seconds)
            .with_action("create_task", HashMap::new());

        // Register the task config
        self.executor.register_task_config(&schedule_id, task_config);

        // Create the schedule
        let schedule = self
            .scheduler
            .write()
            .create_schedule(schedule)
            .await
            .map_err(|e| TaskError::ScheduleError(e.to_string()))?;

        Ok(schedule)
    }
    pub async fn create_one_time_task(
        &self,
        schedule_id: impl Into<String>,
        schedule_name: impl Into<String>,
        run_at: DateTime<Utc>,
        task_config: ScheduledTaskConfig,
    ) -> TaskResult<Schedule> {
        let schedule_id = schedule_id.into();
        let schedule = Schedule::one_time(&schedule_id, schedule_name, run_at)
            .with_action("create_task", HashMap::new());

        // Register the task config
        self.executor.register_task_config(&schedule_id, task_config);

        // Create the schedule
        let schedule = self
            .scheduler
            .write()
            .create_schedule(schedule)
            .await
            .map_err(|e| TaskError::ScheduleError(e.to_string()))?;

        Ok(schedule)
    }
    pub async fn get_schedule(&self, schedule_id: &str) -> TaskResult<Option<Schedule>> {
        self.scheduler
            .read()
            .get_schedule(schedule_id)
            .await
            .map_err(|e| TaskError::ScheduleError(e.to_string()))
    }
    pub async fn enable_schedule(&self, schedule_id: &str) -> TaskResult<Schedule> {
        let schedule = self
            .scheduler
            .write()
            .enable_schedule(schedule_id)
            .await
            .map_err(|e| TaskError::ScheduleError(e.to_string()))?;

        let _ = self.event_sender.send(TaskSchedulerEvent::ScheduleEnabled {
            schedule_id: schedule_id.to_string(),
        });

        Ok(schedule)
    }
    pub async fn disable_schedule(&self, schedule_id: &str) -> TaskResult<Schedule> {
        let schedule = self
            .scheduler
            .write()
            .disable_schedule(schedule_id)
            .await
            .map_err(|e| TaskError::ScheduleError(e.to_string()))?;

        let _ = self.event_sender.send(TaskSchedulerEvent::ScheduleDisabled {
            schedule_id: schedule_id.to_string(),
        });

        Ok(schedule)
    }
    pub async fn delete_schedule(&self, schedule_id: &str) -> TaskResult<bool> {
        // Unregister the task config
        self.executor.unregister_task_config(schedule_id);

        // Delete the schedule
        self.scheduler
            .write()
            .delete_schedule(schedule_id)
            .await
            .map_err(|e| TaskError::ScheduleError(e.to_string()))
    }
    pub async fn trigger_now(&self, schedule_id: &str) -> TaskResult<Job> {
        let job = self
            .scheduler
            .write()
            .trigger_now(schedule_id)
            .await
            .map_err(|e| TaskError::ScheduleError(e.to_string()))?;

        let _ = self.event_sender.send(TaskSchedulerEvent::ScheduleTriggered {
            schedule_id: schedule_id.to_string(),
            job_id: job.id.clone(),
        });

        Ok(job)
    }
    pub async fn start(&self) -> TaskResult<()> {
        self.scheduler
            .write()
            .start()
            .await
            .map_err(|e| TaskError::ScheduleError(e.to_string()))
    }
    pub async fn stop(&self) -> TaskResult<()> {
        self.scheduler
            .read()
            .stop()
            .await
            .map_err(|e| TaskError::ScheduleError(e.to_string()))
    }
}
