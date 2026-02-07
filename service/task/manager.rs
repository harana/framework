// Harana Components - Task Manager
//
// Unified interface for task management, combining:
// - Task creation and querying
// - Distributed locking for safe execution
// - Scheduling for recurring tasks
// - Worker management for execution

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::info;

use harana_components_lock::{
    DistributedLock, DistributedLockManager, LockConfig, LockManager,
};
use harana_components_schedule::{Job, Schedule, ScheduleStore, SchedulerConfig};
use harana_components_storage::Store;

use crate::{
    QueueStats, ScheduledTaskConfig, Task, TaskError, TaskExecutionHistory,
    TaskExecutor, TaskPriority, TaskQuery, TaskResult, TaskSchedulerManager,
    TaskWorker, WorkerConfig, WorkerEvent,
    store as task_store,
};

// ============================================================================
// Task Manager Configuration
// ============================================================================

/// Configuration for the task manager
#[derive(Debug, Clone)]
pub struct TaskManagerConfig {
        pub worker_config: WorkerConfig,
        pub scheduler_config: SchedulerConfig,
        pub lock_config: LockConfig,
        pub enable_worker: bool,
        pub enable_scheduler: bool,
}

impl Default for TaskManagerConfig {
    fn default() -> Self {
        Self {
            worker_config: WorkerConfig::default(),
            scheduler_config: SchedulerConfig::default(),
            lock_config: LockConfig::default(),
            enable_worker: true,
            enable_scheduler: true,
        }
    }
}

impl TaskManagerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_worker_config(mut self, config: WorkerConfig) -> Self {
        self.worker_config = config;
        self
    }

    pub fn with_scheduler_config(mut self, config: SchedulerConfig) -> Self {
        self.scheduler_config = config;
        self
    }

    pub fn with_lock_config(mut self, config: LockConfig) -> Self {
        self.lock_config = config;
        self
    }

    pub fn with_worker(mut self, enabled: bool) -> Self {
        self.enable_worker = enabled;
        self
    }

    pub fn with_scheduler(mut self, enabled: bool) -> Self {
        self.enable_scheduler = enabled;
        self
    }
}

// ============================================================================
// Task Manager Events
// ============================================================================

/// Events from the task manager
#[derive(Debug, Clone)]
pub enum TaskManagerEvent {
    /// Task created
    TaskCreated { task_id: String, queue: String },
    /// Task started
    TaskStarted { task_id: String, queue: String },
    /// Task completed
    TaskCompleted { task_id: String, queue: String, duration_ms: i64 },
    /// Task failed
    TaskFailed { task_id: String, queue: String, error: String },
    /// Task cancelled
    TaskCancelled { task_id: String, queue: String },
    /// Schedule created
    ScheduleCreated { schedule_id: String },
    /// Schedule triggered
    ScheduleTriggered { schedule_id: String, task_id: String },
    /// Manager started
    Started,
    /// Manager stopped
    Stopped,
}

// ============================================================================
// Task Manager
// ============================================================================

/// Unified task management interface
pub struct TaskManager<TS, THS, SS, LS>
where
    TS: Store<Task> + 'static,
    THS: Store<TaskExecutionHistory> + 'static,
    SS: ScheduleStore + 'static,
    LS: Store<DistributedLock> + 'static,
{
    task_store: Arc<TS>,
    history_store: Arc<THS>,
    lock_manager: Arc<DistributedLockManager<LS>>,
    worker: Option<RwLock<TaskWorker<TS, THS, LS>>>,
    scheduler_manager: Option<TaskSchedulerManager<TS, THS, SS>>,
    config: TaskManagerConfig,
    event_sender: broadcast::Sender<TaskManagerEvent>,
    executors: RwLock<HashMap<String, Arc<dyn TaskExecutor>>>,
    default_executor: RwLock<Option<Arc<dyn TaskExecutor>>>,
}

impl<TS, THS, SS, LS> TaskManager<TS, THS, SS, LS>
where
    TS: Store<Task> + Clone + 'static,
    THS: Store<TaskExecutionHistory> + Clone + 'static,
    SS: ScheduleStore + 'static,
    LS: Store<DistributedLock> + Clone + Send + Sync + 'static,
{
    /// Create a new task manager
    pub fn new(
        task_store: TS,
        history_store: THS,
        schedule_store: SS,
        lock_store: LS,
        config: TaskManagerConfig,
    ) -> Self {
        let task_store = Arc::new(task_store);
        let history_store = Arc::new(history_store);
        let lock_manager = Arc::new(DistributedLockManager::new(
            lock_store.clone(),
            config.lock_config.clone(),
        ));

        let (event_sender, _) = broadcast::channel(1000);

        // Create worker if enabled
        let worker = if config.enable_worker {
            Some(RwLock::new(TaskWorker::new(
                (*task_store).clone(),
                (*history_store).clone(),
                lock_store,
                config.worker_config.clone(),
            )))
        } else {
            None
        };

        // Create scheduler manager if enabled
        let scheduler_manager = if config.enable_scheduler {
            Some(TaskSchedulerManager::new(
                (*task_store).clone(),
                (*history_store).clone(),
                schedule_store,
                config.scheduler_config.clone(),
            ))
        } else {
            None
        };

        Self {
            task_store,
            history_store,
            lock_manager,
            worker,
            scheduler_manager,
            config,
            event_sender,
            executors: RwLock::new(HashMap::new()),
            default_executor: RwLock::new(None),
        }
    }

    /// Subscribe to task manager events
    pub fn subscribe(&self) -> broadcast::Receiver<TaskManagerEvent> {
        self.event_sender.subscribe()
    }

    /// Subscribe to worker events
    pub fn subscribe_worker(&self) -> Option<broadcast::Receiver<WorkerEvent>> {
        self.worker.as_ref().map(|w| w.read().subscribe())
    }

    // ========================================================================
    // Executor Registration
    // ========================================================================

    /// Register an executor for specific task types
    pub fn register_executor(&self, task_types: Vec<String>, executor: Arc<dyn TaskExecutor>) {
        {
            let mut executors = self.executors.write();
            for task_type in task_types.clone() {
                executors.insert(task_type, Arc::clone(&executor));
            }
        }

        // Also register with worker if available
        if let Some(worker) = &self.worker {
            worker.read().register_executor(task_types, executor);
        }
    }

    /// Register a default executor for unhandled task types
    pub fn register_default_executor(&self, executor: Arc<dyn TaskExecutor>) {
        *self.default_executor.write() = Some(Arc::clone(&executor));

        if let Some(worker) = &self.worker {
            worker.read().register_default_executor(executor);
        }
    }

    // ========================================================================
    // Task Operations
    // ========================================================================

    /// Create a new task
    pub async fn create_task(&self, task: Task) -> TaskResult<Task> {
        task_store::create_task(self.task_store.as_ref(), &task).await?;

        let _ = self.event_sender.send(TaskManagerEvent::TaskCreated {
            task_id: task.id.clone(),
            queue: task.queue.clone(),
        });

        info!(
            task_id = %task.id,
            task_name = %task.name,
            queue = %task.queue,
            "Task created"
        );

        Ok(task)
    }

    /// Submit a task (create and make it runnable)
    pub async fn submit(
        &self,
        name: impl Into<String>,
        task_type: impl Into<String>,
        queue: impl Into<String>,
    ) -> TaskSubmitBuilder<'_, TS, THS, SS, LS> {
        TaskSubmitBuilder::new(self, name.into(), task_type.into(), queue.into())
    }

    /// Get a task by ID
    pub async fn get_task(&self, task_id: &str) -> TaskResult<Option<Task>> {
        task_store::get_task(self.task_store.as_ref(), task_id).await
    }

    /// Query tasks
    pub async fn query_tasks(&self, query: TaskQuery) -> TaskResult<Vec<Task>> {
        task_store::query_tasks(self.task_store.as_ref(), query).await
    }

    /// Count tasks matching query
    pub async fn count_tasks(&self, query: TaskQuery) -> TaskResult<u64> {
        task_store::count_tasks(self.task_store.as_ref(), query).await
    }

    /// Cancel a task
    pub async fn cancel_task(&self, task_id: &str) -> TaskResult<Task> {
        let mut task = task_store::get_task(self.task_store.as_ref(), task_id)
            .await?
            .ok_or_else(|| TaskError::TaskNotFound {
                task_id: task_id.to_string(),
            })?;

        if task.status.is_terminal() {
            return Err(TaskError::InvalidConfiguration {
                reason: format!("Task {} is already in terminal state: {:?}", task_id, task.status),
            });
        }

        task.cancel();
        task_store::update_task(self.task_store.as_ref(), &task).await?;

        // Record in history
        let history = TaskExecutionHistory::from_task(&task);
        task_store::record_history(self.history_store.as_ref(), &history).await?;

        let _ = self.event_sender.send(TaskManagerEvent::TaskCancelled {
            task_id: task.id.clone(),
            queue: task.queue.clone(),
        });

        info!(task_id = %task_id, "Task cancelled");

        Ok(task)
    }

    /// Delete a task
    pub async fn delete_task(&self, task_id: &str) -> TaskResult<bool> {
        task_store::delete_task(self.task_store.as_ref(), task_id).await
    }

    /// Get task execution history
    pub async fn get_task_history(
        &self,
        task_id: &str,
        limit: Option<usize>,
    ) -> TaskResult<Vec<TaskExecutionHistory>> {
        task_store::get_task_history(self.history_store.as_ref(), task_id, limit).await
    }

    /// Get queue statistics
    pub async fn get_queue_stats(&self, queue: &str) -> TaskResult<QueueStats> {
        task_store::get_queue_stats(self.task_store.as_ref(), queue).await
    }

    /// Get all queue names
    pub async fn get_queues(&self) -> TaskResult<Vec<String>> {
        task_store::get_queues(self.task_store.as_ref()).await
    }

    // ========================================================================
    // Scheduled Task Operations
    // ========================================================================

    /// Create a cron-based scheduled task
    pub async fn schedule_cron(
        &self,
        schedule_id: impl Into<String>,
        schedule_name: impl Into<String>,
        cron_expression: impl Into<String>,
        task_config: ScheduledTaskConfig,
    ) -> TaskResult<Schedule> {
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| {
            TaskError::InvalidConfiguration {
                reason: "Scheduler is not enabled".to_string(),
            }
        })?;

        let schedule = scheduler
            .create_cron_task(schedule_id, schedule_name, cron_expression, task_config)
            .await?;

        let _ = self.event_sender.send(TaskManagerEvent::ScheduleCreated {
            schedule_id: schedule.id.clone(),
        });

        Ok(schedule)
    }

    /// Create an interval-based scheduled task
    pub async fn schedule_interval(
        &self,
        schedule_id: impl Into<String>,
        schedule_name: impl Into<String>,
        interval_seconds: i64,
        task_config: ScheduledTaskConfig,
    ) -> TaskResult<Schedule> {
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| {
            TaskError::InvalidConfiguration {
                reason: "Scheduler is not enabled".to_string(),
            }
        })?;

        let schedule = scheduler
            .create_interval_task(schedule_id, schedule_name, interval_seconds, task_config)
            .await?;

        let _ = self.event_sender.send(TaskManagerEvent::ScheduleCreated {
            schedule_id: schedule.id.clone(),
        });

        Ok(schedule)
    }

    /// Create a one-time scheduled task
    pub async fn schedule_once(
        &self,
        schedule_id: impl Into<String>,
        schedule_name: impl Into<String>,
        run_at: DateTime<Utc>,
        task_config: ScheduledTaskConfig,
    ) -> TaskResult<Schedule> {
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| {
            TaskError::InvalidConfiguration {
                reason: "Scheduler is not enabled".to_string(),
            }
        })?;

        let schedule = scheduler
            .create_one_time_task(schedule_id, schedule_name, run_at, task_config)
            .await?;

        let _ = self.event_sender.send(TaskManagerEvent::ScheduleCreated {
            schedule_id: schedule.id.clone(),
        });

        Ok(schedule)
    }

    /// Get a schedule by ID
    pub async fn get_schedule(&self, schedule_id: &str) -> TaskResult<Option<Schedule>> {
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| {
            TaskError::InvalidConfiguration {
                reason: "Scheduler is not enabled".to_string(),
            }
        })?;

        scheduler.get_schedule(schedule_id).await
    }

    /// Enable a schedule
    pub async fn enable_schedule(&self, schedule_id: &str) -> TaskResult<Schedule> {
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| {
            TaskError::InvalidConfiguration {
                reason: "Scheduler is not enabled".to_string(),
            }
        })?;

        scheduler.enable_schedule(schedule_id).await
    }

    /// Disable a schedule
    pub async fn disable_schedule(&self, schedule_id: &str) -> TaskResult<Schedule> {
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| {
            TaskError::InvalidConfiguration {
                reason: "Scheduler is not enabled".to_string(),
            }
        })?;

        scheduler.disable_schedule(schedule_id).await
    }

    /// Delete a schedule
    pub async fn delete_schedule(&self, schedule_id: &str) -> TaskResult<bool> {
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| {
            TaskError::InvalidConfiguration {
                reason: "Scheduler is not enabled".to_string(),
            }
        })?;

        scheduler.delete_schedule(schedule_id).await
    }

    /// Trigger a schedule immediately
    pub async fn trigger_schedule(&self, schedule_id: &str) -> TaskResult<Job> {
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| {
            TaskError::InvalidConfiguration {
                reason: "Scheduler is not enabled".to_string(),
            }
        })?;

        scheduler.trigger_now(schedule_id).await
    }

    // ========================================================================
    // Lock Operations
    // ========================================================================

    /// Acquire a lock on a resource
    pub async fn acquire_lock(
        &self,
        resource_id: &str,
        owner_id: &str,
        ttl_seconds: Option<u64>,
    ) -> TaskResult<u64> {
        self.lock_manager
            .acquire(resource_id, owner_id, ttl_seconds)
            .await
            .map_err(|e| TaskError::LockFailed {
                resource: e.to_string(),
            })
    }

    /// Try to acquire a lock without waiting
    pub async fn try_acquire_lock(
        &self,
        resource_id: &str,
        owner_id: &str,
        ttl_seconds: Option<u64>,
    ) -> TaskResult<Option<u64>> {
        self.lock_manager
            .try_acquire(resource_id, owner_id, ttl_seconds)
            .await
            .map_err(|e| TaskError::LockFailed {
                resource: e.to_string(),
            })
    }

    /// Release a lock
    pub async fn release_lock(&self, resource_id: &str, owner_id: &str) -> TaskResult<bool> {
        self.lock_manager
            .release_lock(resource_id, owner_id)
            .await
            .map_err(|e| TaskError::LockFailed {
                resource: e.to_string(),
            })
    }

    /// Extend a lock's TTL
    pub async fn extend_lock(
        &self,
        resource_id: &str,
        owner_id: &str,
        extension_seconds: u64,
    ) -> TaskResult<bool> {
        self.lock_manager
            .extend_lock(resource_id, owner_id, extension_seconds)
            .await
            .map_err(|e| TaskError::LockFailed {
                resource: e.to_string(),
            })
    }

    /// Check if a resource is locked
    pub async fn is_locked(&self, resource_id: &str) -> TaskResult<bool> {
        self.lock_manager
            .is_locked(resource_id)
            .await
            .map_err(|e| TaskError::LockFailed {
                resource: e.to_string(),
            })
    }

    /// Get lock information
    pub async fn get_lock_info(&self, resource_id: &str) -> TaskResult<Option<DistributedLock>> {
        self.lock_manager
            .get_lock_info(resource_id)
            .await
            .map_err(|e| TaskError::LockFailed {
                resource: e.to_string(),
            })
    }

    // ========================================================================
    // Lifecycle
    // ========================================================================

    /// Start the task manager (worker and scheduler)
    pub async fn start(&mut self) -> TaskResult<()> {
        // Initialize lock manager
        self.lock_manager.initialize().await?;

        // Start worker if enabled
        if let Some(worker) = &mut self.worker {
            worker.get_mut().start().await?;
        }

        // Start scheduler if enabled
        if let Some(scheduler) = &self.scheduler_manager {
            scheduler.start().await?;
        }

        let _ = self.event_sender.send(TaskManagerEvent::Started);

        info!("Task manager started");

        Ok(())
    }

    /// Stop the task manager
    pub async fn stop(&self) -> TaskResult<()> {
        // Stop worker if enabled
        if let Some(worker) = &self.worker {
            worker.read().stop().await?;
        }

        // Stop scheduler if enabled
        if let Some(scheduler) = &self.scheduler_manager {
            scheduler.stop().await?;
        }

        let _ = self.event_sender.send(TaskManagerEvent::Stopped);

        info!("Task manager stopped");

        Ok(())
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.worker
            .as_ref()
            .map(|w| w.read().is_running())
            .unwrap_or(false)
    }
}

// ============================================================================
// Task Submit Builder
// ============================================================================

/// Builder for submitting tasks with a fluent API
pub struct TaskSubmitBuilder<'a, TS, THS, SS, LS>
where
    TS: Store<Task> + Clone + 'static,
    THS: Store<TaskExecutionHistory> + Clone + 'static,
    SS: ScheduleStore + 'static,
    LS: Store<DistributedLock> + Clone + Send + Sync + 'static,
{
    manager: &'a TaskManager<TS, THS, SS, LS>,
    task: Task,
}

impl<'a, TS, THS, SS, LS> TaskSubmitBuilder<'a, TS, THS, SS, LS>
where
    TS: Store<Task> + Clone + 'static,
    THS: Store<TaskExecutionHistory> + Clone + 'static,
    SS: ScheduleStore + 'static,
    LS: Store<DistributedLock> + Clone + Send + Sync + 'static,
{
    fn new(
        manager: &'a TaskManager<TS, THS, SS, LS>,
        name: String,
        task_type: String,
        queue: String,
    ) -> Self {
        Self {
            manager,
            task: Task::new(name, task_type, queue),
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.task.id = id.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.task.description = description.into();
        self
    }

    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.task.priority = priority;
        self
    }

    pub fn with_payload(mut self, payload: HashMap<String, serde_json::Value>) -> Self {
        self.task.payload = payload;
        self
    }

    pub fn with_payload_value(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.task.payload.insert(key.into(), value);
        self
    }

    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.task.timeout_secs = secs;
        self
    }

    pub fn with_delay(mut self, delay_secs: i64) -> Self {
        self.task = self.task.with_delay(delay_secs);
        self
    }

    pub fn with_scheduled_at(mut self, scheduled_at: DateTime<Utc>) -> Self {
        self.task = self.task.with_scheduled_at(scheduled_at);
        self
    }

    pub fn with_retry(mut self, config: crate::RetryConfig) -> Self {
        self.task.retry_config = config;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.task.tags = tags;
        self
    }

    pub fn with_owner(mut self, owner_id: impl Into<String>) -> Self {
        self.task.owner_id = Some(owner_id.into());
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.task.correlation_id = Some(correlation_id.into());
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.task.metadata.insert(key.into(), value);
        self
    }

    /// Submit the task
    pub async fn execute(self) -> TaskResult<Task> {
        self.manager.create_task(self.task).await
    }
}
