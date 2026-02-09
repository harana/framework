// Core entity types - now split into separate modules for better organization
mod backoff;
mod execution_history;
mod lock_helpers;
mod priority;
mod retry_config;
mod status;
mod task;

// Other modules
mod error;
mod executor;
mod manager;
mod scheduler;
mod store;
mod worker;
mod worker_config;
mod worker_event;

#[cfg(test)]
mod tests;

// Re-export core types from their individual modules
pub use backoff::BackoffStrategy;
pub use error::{TaskError, TaskResult};
pub use execution_history::TaskExecutionHistory;
pub use executor::{AsyncFnExecutor, CompositeExecutor, FnExecutor, LoggingExecutor, TaskExecutor};
pub use lock_helpers::{queue_lock_resource, task_lock_resource};
pub use manager::{TaskManager, TaskManagerConfig, TaskManagerEvent, TaskSubmitBuilder};
pub use priority::TaskPriority;
pub use retry_config::RetryConfig;
pub use scheduler::{ScheduledTaskConfig, TaskCreatingExecutor, TaskSchedulerEvent, TaskSchedulerManager};
pub use status::TaskStatus;
pub use store::{
    QueueStats,
    TaskQuery,
    cleanup_history,
    count_tasks,
    // Task store functions
    create_task,
    delete_task,
    extend_task_lock,
    get_queue_history,
    // Statistics functions
    get_queue_stats,
    get_queues,
    get_runnable_tasks,
    get_stale_tasks,
    get_task,
    get_task_history,
    query_tasks,
    // History functions
    record_history,
    release_task_lock,
    try_lock_task,
    update_task,
};
pub use task::Task;
pub use worker::TaskWorker;
pub use worker_config::WorkerConfig;
pub use worker_event::WorkerEvent;

// Re-export commonly used types from dependencies
pub use harana_components_lock::{
    DistributedLock, DistributedLockManager, LockConfig, LockHandle, LockManager, MultiLockGuard,
};
pub use harana_components_schedule::{
    DurableScheduler, Job, Schedule, ScheduleQuery, ScheduleStats, ScheduleStatus, ScheduleStore, ScheduleType,
    SchedulerConfig,
};
pub use harana_components_storage::{Entity, FilterCondition, QueryOptions, StorageError, Store};
