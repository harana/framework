use chrono::Utc;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Semaphore, broadcast, mpsc};
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

use harana_components_lock::{DistributedLock, DistributedLockManager, LockConfig, LockManager};
use harana_components_storage::Store;

use crate::{
    Task, TaskExecutionHistory, TaskExecutor, TaskResult, WorkerConfig, WorkerEvent, store as task_store,
    task_lock_resource,
};

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
    L: Store<DistributedLock> + 'static,
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
    L: Store<DistributedLock> + Send + Sync + 'static,
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

struct WorkerTaskHandle<S: Store<Task>, H: Store<TaskExecutionHistory>, L: Store<DistributedLock>> {
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
    L: Store<DistributedLock> + Send + Sync + 'static,
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
            let tasks = task_store::get_runnable_tasks(self.store.as_ref(), &queue, self.config.batch_size).await?;

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
        let lock_token =
            match task_store::try_lock_task(store.as_ref(), &task_id, &config.worker_id, config.lock_duration_secs)
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
