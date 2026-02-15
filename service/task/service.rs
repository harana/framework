use async_trait::async_trait;
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Semaphore, broadcast, mpsc};
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

use harana_components_cache::CacheService;
use harana_components_lock::{DistributedLock, DistributedLockManager, LockConfig, LockManager};
use harana_components_schedule::{
    DurableScheduler, Job, JobExecutor as ScheduleJobExecutor, Schedule, ScheduleService,
    SchedulerConfig,
};
use harana_components_storage::Store;

use crate::{
    QueueStats, ScheduledTaskConfig, Task, TaskError, TaskExecutionHistory, TaskManagerConfig,
    TaskManagerEvent, TaskPriority, TaskQuery, TaskResult, TaskSchedulerEvent, WorkerConfig,
    WorkerEvent, store as task_store, task_lock_resource,
};

// ============================================================================
// Task Executor Trait
// ============================================================================

#[async_trait]
pub trait TaskExecutor: Send + Sync {
    async fn execute(&self, task: &Task) -> Result<Option<Value>, String>;
    fn can_handle(&self, task_type: &str) -> bool;
    fn task_types(&self) -> Vec<&str> {
        vec![]
    }
}

pub struct LoggingExecutor;

#[async_trait]
impl TaskExecutor for LoggingExecutor {
    async fn execute(&self, task: &Task) -> Result<Option<Value>, String> {
        tracing::info!(
            task_id = %task.id,
            task_name = %task.name,
            task_type = %task.task_type,
            queue = %task.queue,
            "Executing task"
        );
        Ok(Some(serde_json::json!({ "status": "logged" })))
    }

    fn can_handle(&self, _task_type: &str) -> bool {
        true
    }
}

pub struct FnExecutor<F>
where
    F: Fn(&Task) -> Result<Option<Value>, String> + Send + Sync,
{
    task_types: Vec<String>,
    handler: F,
}

impl<F> FnExecutor<F>
where
    F: Fn(&Task) -> Result<Option<Value>, String> + Send + Sync,
{
    pub fn new(task_types: Vec<String>, handler: F) -> Self {
        Self { task_types, handler }
    }

    pub fn for_all_types(handler: F) -> Self {
        Self {
            task_types: vec![],
            handler,
        }
    }
}

#[async_trait]
impl<F> TaskExecutor for FnExecutor<F>
where
    F: Fn(&Task) -> Result<Option<Value>, String> + Send + Sync,
{
    async fn execute(&self, task: &Task) -> Result<Option<Value>, String> {
        (self.handler)(task)
    }

    fn can_handle(&self, task_type: &str) -> bool {
        self.task_types.is_empty() || self.task_types.iter().any(|t| t == task_type)
    }

    fn task_types(&self) -> Vec<&str> {
        self.task_types.iter().map(|s| s.as_str()).collect()
    }
}

pub struct AsyncFnExecutor<F, Fut>
where
    F: Fn(Task) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<Option<Value>, String>> + Send,
{
    task_types: Vec<String>,
    handler: F,
}

impl<F, Fut> AsyncFnExecutor<F, Fut>
where
    F: Fn(Task) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<Option<Value>, String>> + Send,
{
    pub fn new(task_types: Vec<String>, handler: F) -> Self {
        Self { task_types, handler }
    }

    pub fn for_all_types(handler: F) -> Self {
        Self {
            task_types: vec![],
            handler,
        }
    }
}

#[async_trait]
impl<F, Fut> TaskExecutor for AsyncFnExecutor<F, Fut>
where
    F: Fn(Task) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<Option<Value>, String>> + Send,
{
    async fn execute(&self, task: &Task) -> Result<Option<Value>, String> {
        (self.handler)(task.clone()).await
    }

    fn can_handle(&self, task_type: &str) -> bool {
        self.task_types.is_empty() || self.task_types.iter().any(|t| t == task_type)
    }

    fn task_types(&self) -> Vec<&str> {
        self.task_types.iter().map(|s| s.as_str()).collect()
    }
}

pub struct CompositeExecutor {
    executors: Vec<Box<dyn TaskExecutor>>,
}

impl CompositeExecutor {
    pub fn new() -> Self {
        Self { executors: vec![] }
    }

    pub fn add<E: TaskExecutor + 'static>(mut self, executor: E) -> Self {
        self.executors.push(Box::new(executor));
        self
    }

    pub fn register<E: TaskExecutor + 'static>(&mut self, executor: E) {
        self.executors.push(Box::new(executor));
    }
}

impl Default for CompositeExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TaskExecutor for CompositeExecutor {
    async fn execute(&self, task: &Task) -> Result<Option<Value>, String> {
        for executor in &self.executors {
            if executor.can_handle(&task.task_type) {
                return executor.execute(task).await;
            }
        }
        Err(format!("No executor found for task type: {}", task.task_type))
    }

    fn can_handle(&self, task_type: &str) -> bool {
        self.executors.iter().any(|e| e.can_handle(task_type))
    }

    fn task_types(&self) -> Vec<&str> {
        self.executors.iter().flat_map(|e| e.task_types()).collect()
    }
}

// ============================================================================
// Task Worker
// ============================================================================

struct WorkerState {
    running: bool,
    active_tasks: usize,
}

pub struct TaskWorker<S, H, L>
where
    S: Store<Task> + 'static,
    H: Store<TaskExecutionHistory> + 'static,
    L: CacheService + 'static,
{
    store: Arc<S>,
    history_store: Arc<H>,
    lock_manager: Arc<DistributedLockManager<L>>,
    config: WorkerConfig,
    executors: RwLock<HashMap<String, Arc<dyn TaskExecutor>>>,
    default_executor: RwLock<Option<Arc<dyn TaskExecutor>>>,
    state: RwLock<WorkerState>,
    event_sender: broadcast::Sender<WorkerEvent>,
    shutdown_sender: Option<mpsc::Sender<()>>,
    tasks: RwLock<Vec<JoinHandle<()>>>,
    semaphore: Arc<Semaphore>,
}

impl<S, H, L> TaskWorker<S, H, L>
where
    S: Store<Task> + 'static,
    H: Store<TaskExecutionHistory> + 'static,
    L: CacheService + Send + Sync + 'static,
{
    pub fn new(store: S, history_store: H, lock_store: L, config: WorkerConfig) -> Self {
        let (event_sender, _) = broadcast::channel(1000);
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent_tasks));
        let lock_config = LockConfig::new().with_ttl(config.lock_duration_secs);

        Self {
            store: Arc::new(store),
            history_store: Arc::new(history_store),
            lock_manager: Arc::new(DistributedLockManager::new(lock_store, lock_config)),
            config,
            executors: RwLock::new(HashMap::new()),
            default_executor: RwLock::new(None),
            state: RwLock::new(WorkerState {
                running: false,
                active_tasks: 0,
            }),
            event_sender,
            shutdown_sender: None,
            tasks: RwLock::new(Vec::new()),
            semaphore,
        }
    }
    pub fn register_executor(&self, task_types: Vec<String>, executor: Arc<dyn TaskExecutor>) {
        let mut executors = self.executors.write();
        for task_type in task_types {
            executors.insert(task_type, Arc::clone(&executor));
        }
    }
    pub fn register_default_executor(&self, executor: Arc<dyn TaskExecutor>) {
        *self.default_executor.write() = Some(executor);
    }
    /// Get an executor for a task type
    fn get_executor(&self, task_type: &str) -> Option<Arc<dyn TaskExecutor>> {
        self.executors
            .read()
            .get(task_type)
            .cloned()
            .or_else(|| self.default_executor.read().clone())
    }
    pub fn subscribe(&self) -> broadcast::Receiver<WorkerEvent> {
        self.event_sender.subscribe()
    }
    pub fn store(&self) -> &S {
        &self.store
    }

    /// Get the lock manager
    pub fn lock_manager(&self) -> &DistributedLockManager<L> {
        &self.lock_manager
    }
    pub fn is_running(&self) -> bool {
        self.state.read().running
    }

    /// Get the number of active tasks
    pub fn active_task_count(&self) -> usize {
        self.state.read().active_tasks
    }
    fn emit(&self, event: WorkerEvent) {
        let _ = self.event_sender.send(event);
    }
    pub async fn start(&mut self) -> TaskResult<()> {
        {
            let mut state = self.state.write();
            if state.running {
                return Ok(());
            }
            state.running = true;
        }

        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        self.shutdown_sender = Some(shutdown_tx);

        // Initialize lock manager
        self.lock_manager.initialize().await?;

        // Start the main worker loop
        let worker = self.clone_for_task();
        let poll_task = tokio::spawn(async move {
            worker.worker_loop(shutdown_rx).await;
        });

        // Start the stale task recovery loop
        let worker = self.clone_for_task();
        let (_, stale_shutdown_rx) = mpsc::channel(1);
        let stale_task = tokio::spawn(async move {
            worker.stale_recovery_loop(stale_shutdown_rx).await;
        });

        // Start the cleanup loop
        let worker = self.clone_for_task();
        let (_, cleanup_shutdown_rx) = mpsc::channel(1);
        let cleanup_task = tokio::spawn(async move {
            worker.cleanup_loop(cleanup_shutdown_rx).await;
        });

        {
            let mut tasks = self.tasks.write();
            tasks.push(poll_task);
            tasks.push(stale_task);
            tasks.push(cleanup_task);
        }

        self.emit(WorkerEvent::Started {
            worker_id: self.config.worker_id.clone(),
        });

        info!(worker_id = %self.config.worker_id, "Task worker started");

        Ok(())
    }
    pub async fn stop(&self) -> TaskResult<()> {
        {
            let mut state = self.state.write();
            if !state.running {
                return Ok(());
            }
            state.running = false;
        }

        self.emit(WorkerEvent::Stopped {
            worker_id: self.config.worker_id.clone(),
        });

        info!(worker_id = %self.config.worker_id, "Task worker stopped");

        Ok(())
    }
    fn clone_for_task(&self) -> WorkerTaskHandle<S, H, L> {
        WorkerTaskHandle {
            store: Arc::clone(&self.store),
            history_store: Arc::clone(&self.history_store),
            lock_manager: Arc::clone(&self.lock_manager),
            config: self.config.clone(),
            executors: self.executors.read().clone(),
            default_executor: self.default_executor.read().clone(),
            event_sender: self.event_sender.clone(),
            state: Arc::new(RwLock::new(true)),
            semaphore: Arc::clone(&self.semaphore),
        }
    }
}

struct WorkerTaskHandle<S: Store<Task>, H: Store<TaskExecutionHistory>, L: CacheService> {
    store: Arc<S>,
    history_store: Arc<H>,
    lock_manager: Arc<DistributedLockManager<L>>,
    config: WorkerConfig,
    executors: HashMap<String, Arc<dyn TaskExecutor>>,
    default_executor: Option<Arc<dyn TaskExecutor>>,
    event_sender: broadcast::Sender<WorkerEvent>,
    state: Arc<RwLock<bool>>,
    semaphore: Arc<Semaphore>,
}

impl<
        S: Store<Task> + 'static,
        H: Store<TaskExecutionHistory> + 'static,
        L: CacheService + Send + Sync + 'static,
    > WorkerTaskHandle<S, H, L>
{
    fn emit(&self, event: WorkerEvent) {
        let _ = self.event_sender.send(event);
    }

    fn is_running(&self) -> bool {
        *self.state.read()
    }

    fn get_executor(&self, task_type: &str) -> Option<Arc<dyn TaskExecutor>> {
        self.executors
            .get(task_type)
            .cloned()
            .or_else(|| self.default_executor.clone())
    }

    async fn worker_loop(&self, mut shutdown: mpsc::Receiver<()>) {
        let poll_interval = Duration::from_millis(self.config.poll_interval_ms);

        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("Worker loop shutting down");
                    break;
                }
                _ = tokio::time::sleep(poll_interval) => {
                    if !self.is_running() {
                        break;
                    }

                    if let Err(e) = self.process_tasks().await {
                        error!(error = %e, "Error processing tasks");
                    }
                }
            }
        }
    }

    async fn process_tasks(&self) -> TaskResult<()> {
        // Get the queues to process
        let queues = if self.config.queues.is_empty() {
            task_store::get_queues(self.store.as_ref()).await?
        } else {
            self.config.queues.clone()
        };

        for queue in queues {
            // Get runnable tasks for this queue
            let tasks =
                task_store::get_runnable_tasks(self.store.as_ref(), &queue, self.config.batch_size).await?;

            for task in tasks {
                // Try to acquire a semaphore permit
                let permit = match self.semaphore.clone().try_acquire_owned() {
                    Ok(p) => p,
                    Err(_) => {
                        // Max concurrent tasks reached
                        debug!("Max concurrent tasks reached, skipping");
                        break;
                    }
                };

                // Try to lock and execute the task
                let store = Arc::clone(&self.store);
                let history_store = Arc::clone(&self.history_store);
                let lock_manager = Arc::clone(&self.lock_manager);
                let executor = self.get_executor(&task.task_type);
                let config = self.config.clone();
                let event_sender = self.event_sender.clone();

                tokio::spawn(async move {
                    let _permit = permit; // Hold permit until task completes

                    if let Err(e) = Self::execute_task(
                        store,
                        history_store,
                        lock_manager,
                        executor,
                        task,
                        &config,
                        &event_sender,
                    )
                    .await
                    {
                        error!(error = %e, "Error executing task");
                    }
                });
            }
        }

        Ok(())
    }

    async fn execute_task(
        store: Arc<S>,
        history_store: Arc<H>,
        lock_manager: Arc<DistributedLockManager<L>>,
        executor: Option<Arc<dyn TaskExecutor>>,
        mut task: Task,
        config: &WorkerConfig,
        event_sender: &broadcast::Sender<WorkerEvent>,
    ) -> TaskResult<()> {
        let task_id = task.id.clone();
        let queue = task.queue.clone();
        let task_type = task.task_type.clone();

        // Try to acquire lock
        let lock_resource = task_lock_resource(&task_id);
        let fencing_token = match lock_manager
            .try_acquire(&lock_resource, &config.worker_id, Some(config.lock_duration_secs))
            .await?
        {
            Some(token) => token,
            None => {
                debug!(task_id = %task_id, "Failed to acquire lock, task may be running elsewhere");
                return Ok(());
            }
        };

        let _ = event_sender.send(WorkerEvent::TaskPickedUp {
            task_id: task_id.clone(),
            queue: queue.clone(),
        });

        // Try to lock in store as well
        let lock_token = match task_store::try_lock_task(
            store.as_ref(),
            &task_id,
            &config.worker_id,
            config.lock_duration_secs,
        )
        .await?
        {
            Some(token) => token,
            None => {
                // Release distributed lock
                let _ = lock_manager.release_lock(&lock_resource, &config.worker_id).await;
                return Ok(());
            }
        };

        let _ = event_sender.send(WorkerEvent::TaskStarted {
            task_id: task_id.clone(),
            queue: queue.clone(),
        });

        // Get the executor
        let executor = match executor {
            Some(e) => e,
            None => {
                // No executor found, fail the task
                task.fail(format!("No executor found for task type: {}", task_type), None);
                task_store::update_task(store.as_ref(), &task).await?;

                let history = TaskExecutionHistory::from_task(&task);
                task_store::record_history(history_store.as_ref(), &history).await?;

                let _ = event_sender.send(WorkerEvent::TaskFailed {
                    task_id: task_id.clone(),
                    queue: queue.clone(),
                    error: task.error.clone().unwrap_or_default(),
                });

                let _ = lock_manager.release_lock(&lock_resource, &config.worker_id).await;
                return Ok(());
            }
        };

        // Execute the task with timeout
        let start_time = Utc::now();
        let timeout = Duration::from_secs(task.timeout_secs);

        let result = tokio::time::timeout(timeout, executor.execute(&task)).await;

        let duration_ms = (Utc::now() - start_time).num_milliseconds();

        match result {
            Ok(Ok(result_value)) => {
                // Task completed successfully
                task.complete(result_value);
                task_store::update_task(store.as_ref(), &task).await?;

                let history = TaskExecutionHistory::from_task(&task);
                task_store::record_history(history_store.as_ref(), &history).await?;

                let _ = event_sender.send(WorkerEvent::TaskCompleted {
                    task_id: task_id.clone(),
                    queue: queue.clone(),
                    duration_ms,
                });

                info!(
                    task_id = %task_id,
                    queue = %queue,
                    duration_ms = %duration_ms,
                    "Task completed successfully"
                );
            }
            Ok(Err(error)) => {
                // Task failed
                if task.can_retry() && config.auto_retry {
                    // Schedule retry
                    task.schedule_retry();
                    task_store::update_task(store.as_ref(), &task).await?;

                    let _ = event_sender.send(WorkerEvent::TaskRetrying {
                        task_id: task_id.clone(),
                        queue: queue.clone(),
                        attempt: task.retry_attempt,
                    });

                    info!(
                        task_id = %task_id,
                        queue = %queue,
                        attempt = task.retry_attempt,
                        "Task scheduled for retry"
                    );
                } else {
                    // No more retries
                    task.fail(&error, None);
                    task_store::update_task(store.as_ref(), &task).await?;

                    let history = TaskExecutionHistory::from_task(&task);
                    task_store::record_history(history_store.as_ref(), &history).await?;

                    let _ = event_sender.send(WorkerEvent::TaskFailed {
                        task_id: task_id.clone(),
                        queue: queue.clone(),
                        error: error.clone(),
                    });

                    warn!(
                        task_id = %task_id,
                        queue = %queue,
                        error = %error,
                        "Task failed"
                    );
                }
            }
            Err(_) => {
                // Task timed out
                if task.can_retry() && config.auto_retry {
                    task.schedule_retry();
                    task_store::update_task(store.as_ref(), &task).await?;

                    let _ = event_sender.send(WorkerEvent::TaskRetrying {
                        task_id: task_id.clone(),
                        queue: queue.clone(),
                        attempt: task.retry_attempt,
                    });
                } else {
                    task.timeout();
                    task_store::update_task(store.as_ref(), &task).await?;

                    let history = TaskExecutionHistory::from_task(&task);
                    task_store::record_history(history_store.as_ref(), &history).await?;

                    let _ = event_sender.send(WorkerEvent::TaskTimedOut {
                        task_id: task_id.clone(),
                        queue: queue.clone(),
                    });

                    warn!(
                        task_id = %task_id,
                        queue = %queue,
                        timeout_secs = task.timeout_secs,
                        "Task timed out"
                    );
                }
            }
        }

        // Release locks
        let _ = task_store::release_task_lock(store.as_ref(), &task_id, &lock_token).await;
        let _ = lock_manager.release_lock(&lock_resource, &config.worker_id).await;

        Ok(())
    }

    async fn stale_recovery_loop(&self, mut shutdown: mpsc::Receiver<()>) {
        let interval = Duration::from_secs(self.config.stale_check_interval_secs);

        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("Stale recovery loop shutting down");
                    break;
                }
                _ = tokio::time::sleep(interval) => {
                    if !self.is_running() {
                        break;
                    }

                    if let Err(e) = self.recover_stale_tasks().await {
                        error!(error = %e, "Error recovering stale tasks");
                    }
                }
            }
        }
    }

    async fn recover_stale_tasks(&self) -> TaskResult<()> {
        let threshold = Utc::now() - chrono::Duration::seconds(self.config.stale_threshold_secs as i64);
        let stale_tasks = task_store::get_stale_tasks(self.store.as_ref(), threshold).await?;

        for mut task in stale_tasks {
            info!(task_id = %task.id, "Recovering stale task");

            // Check if should retry
            if task.can_retry() && self.config.auto_retry {
                task.schedule_retry();
                task_store::update_task(self.store.as_ref(), &task).await?;

                self.emit(WorkerEvent::TaskRetrying {
                    task_id: task.id.clone(),
                    queue: task.queue.clone(),
                    attempt: task.retry_attempt,
                });
            } else {
                task.timeout();
                task_store::update_task(self.store.as_ref(), &task).await?;

                let history = TaskExecutionHistory::from_task(&task);
                task_store::record_history(self.history_store.as_ref(), &history).await?;

                self.emit(WorkerEvent::TaskTimedOut {
                    task_id: task.id.clone(),
                    queue: task.queue.clone(),
                });
            }

            self.emit(WorkerEvent::StaleTaskRecovered {
                task_id: task.id.clone(),
            });
        }

        Ok(())
    }

    async fn cleanup_loop(&self, mut shutdown: mpsc::Receiver<()>) {
        let interval = Duration::from_secs(self.config.cleanup_interval_secs);

        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("Cleanup loop shutting down");
                    break;
                }
                _ = tokio::time::sleep(interval) => {
                    if !self.is_running() {
                        break;
                    }

                    if let Err(e) = self.cleanup_old_history().await {
                        error!(error = %e, "Error cleaning up history");
                    }
                }
            }
        }
    }

    async fn cleanup_old_history(&self) -> TaskResult<()> {
        let cutoff = Utc::now() - chrono::Duration::days(self.config.history_retention_days as i64);
        let deleted = task_store::cleanup_history(self.history_store.as_ref(), cutoff).await?;

        if deleted > 0 {
            info!(deleted = deleted, "Cleaned up old task history");
        }

        Ok(())
    }
}

// ============================================================================
// Task Creating Executor (Schedule Bridge)
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
        self.task_configs.write().insert(schedule_id.into(), config);
    }

    /// Unregister a task configuration
    pub fn unregister_task_config(&self, schedule_id: &str) {
        self.task_configs.write().remove(schedule_id);
    }
    pub fn get_task_config(&self, schedule_id: &str) -> Option<ScheduledTaskConfig> {
        self.task_configs.read().get(schedule_id).cloned()
    }
}

#[async_trait]
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

pub struct TaskSchedulerManager<TS, THS, SS>
where
    TS: Store<Task> + 'static,
    THS: Store<TaskExecutionHistory> + 'static,
    SS: ScheduleService + 'static,
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
    SS: ScheduleService + 'static,
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

// ============================================================================
// Task Manager
// ============================================================================

pub struct TaskManager<TS, THS, SS, LS>
where
    TS: Store<Task> + 'static,
    THS: Store<TaskExecutionHistory> + 'static,
    SS: ScheduleService + 'static,
    LS: CacheService + 'static,
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
    SS: ScheduleService + 'static,
    LS: CacheService + Clone + Send + Sync + 'static,
{
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
    pub fn subscribe(&self) -> broadcast::Receiver<TaskManagerEvent> {
        self.event_sender.subscribe()
    }
    pub fn subscribe_worker(&self) -> Option<broadcast::Receiver<WorkerEvent>> {
        self.worker.as_ref().map(|w| w.read().subscribe())
    }

    // ========================================================================
    // Executor Registration
    // ========================================================================
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
    pub fn register_default_executor(&self, executor: Arc<dyn TaskExecutor>) {
        *self.default_executor.write() = Some(Arc::clone(&executor));

        if let Some(worker) = &self.worker {
            worker.read().register_default_executor(executor);
        }
    }

    // ========================================================================
    // Task Operations
    // ========================================================================
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
    pub async fn get_task(&self, task_id: &str) -> TaskResult<Option<Task>> {
        task_store::get_task(self.task_store.as_ref(), task_id).await
    }
    pub async fn query_tasks(&self, query: TaskQuery) -> TaskResult<Vec<Task>> {
        task_store::query_tasks(self.task_store.as_ref(), query).await
    }
    /// Count tasks matching query
    pub async fn count_tasks(&self, query: TaskQuery) -> TaskResult<u64> {
        task_store::count_tasks(self.task_store.as_ref(), query).await
    }
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
    pub async fn delete_task(&self, task_id: &str) -> TaskResult<bool> {
        task_store::delete_task(self.task_store.as_ref(), task_id).await
    }
    pub async fn get_task_history(
        &self,
        task_id: &str,
        limit: Option<usize>,
    ) -> TaskResult<Vec<TaskExecutionHistory>> {
        task_store::get_task_history(self.history_store.as_ref(), task_id, limit).await
    }
    pub async fn get_queue_stats(&self, queue: &str) -> TaskResult<QueueStats> {
        task_store::get_queue_stats(self.task_store.as_ref(), queue).await
    }
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
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| TaskError::InvalidConfiguration {
            reason: "Scheduler is not enabled".to_string(),
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
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| TaskError::InvalidConfiguration {
            reason: "Scheduler is not enabled".to_string(),
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
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| TaskError::InvalidConfiguration {
            reason: "Scheduler is not enabled".to_string(),
        })?;

        let schedule = scheduler
            .create_one_time_task(schedule_id, schedule_name, run_at, task_config)
            .await?;

        let _ = self.event_sender.send(TaskManagerEvent::ScheduleCreated {
            schedule_id: schedule.id.clone(),
        });

        Ok(schedule)
    }
    pub async fn get_schedule(&self, schedule_id: &str) -> TaskResult<Option<Schedule>> {
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| TaskError::InvalidConfiguration {
            reason: "Scheduler is not enabled".to_string(),
        })?;

        scheduler.get_schedule(schedule_id).await
    }
    pub async fn enable_schedule(&self, schedule_id: &str) -> TaskResult<Schedule> {
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| TaskError::InvalidConfiguration {
            reason: "Scheduler is not enabled".to_string(),
        })?;

        scheduler.enable_schedule(schedule_id).await
    }
    pub async fn disable_schedule(&self, schedule_id: &str) -> TaskResult<Schedule> {
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| TaskError::InvalidConfiguration {
            reason: "Scheduler is not enabled".to_string(),
        })?;

        scheduler.disable_schedule(schedule_id).await
    }
    pub async fn delete_schedule(&self, schedule_id: &str) -> TaskResult<bool> {
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| TaskError::InvalidConfiguration {
            reason: "Scheduler is not enabled".to_string(),
        })?;

        scheduler.delete_schedule(schedule_id).await
    }
    pub async fn trigger_schedule(&self, schedule_id: &str) -> TaskResult<Job> {
        let scheduler = self.scheduler_manager.as_ref().ok_or_else(|| TaskError::InvalidConfiguration {
            reason: "Scheduler is not enabled".to_string(),
        })?;

        scheduler.trigger_now(schedule_id).await
    }

    // ========================================================================
    // Lock Operations
    // ========================================================================
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
    pub async fn is_locked(&self, resource_id: &str) -> TaskResult<bool> {
        self.lock_manager
            .is_locked(resource_id)
            .await
            .map_err(|e| TaskError::LockFailed {
                resource: e.to_string(),
            })
    }
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

pub struct TaskSubmitBuilder<'a, TS, THS, SS, LS>
where
    TS: Store<Task> + Clone + 'static,
    THS: Store<TaskExecutionHistory> + Clone + 'static,
    SS: ScheduleService + 'static,
    LS: CacheService + Clone + Send + Sync + 'static,
{
    manager: &'a TaskManager<TS, THS, SS, LS>,
    task: Task,
}

impl<'a, TS, THS, SS, LS> TaskSubmitBuilder<'a, TS, THS, SS, LS>
where
    TS: Store<Task> + Clone + 'static,
    THS: Store<TaskExecutionHistory> + Clone + 'static,
    SS: ScheduleService + 'static,
    LS: CacheService + Clone + Send + Sync + 'static,
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
