// Harana Components - Task
//
// A unified task management component that integrates with:
// - Lock component for distributed coordination
// - Storage component for persistence
// - Schedule component for recurring tasks
//
// This component provides:
// - Durable task queues with priority support
// - Distributed task execution with locking
// - Task scheduling (cron, interval, one-time)
// - Retry policies with exponential backoff
// - Task progress tracking and history
// - Worker management for task execution

mod entity;
mod error;
mod executor;
mod manager;
mod scheduler;
mod store;
mod worker;

#[cfg(test)]
mod tests;

// Re-export core types
pub use entity::{
    BackoffStrategy, RetryConfig, Task, TaskExecutionHistory, TaskPriority, TaskStatus, queue_lock_resource,
    task_lock_resource,
};
pub use error::{TaskError, TaskResult};
pub use executor::{AsyncFnExecutor, CompositeExecutor, FnExecutor, LoggingExecutor, TaskExecutor};
pub use manager::{TaskManager, TaskManagerConfig, TaskManagerEvent, TaskSubmitBuilder};
pub use scheduler::{ScheduledTaskConfig, TaskCreatingExecutor, TaskSchedulerEvent, TaskSchedulerManager};
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
pub use worker::{TaskWorker, WorkerConfig, WorkerEvent};

// Re-export commonly used types from dependencies
pub use harana_components_lock::{
    DistributedLock, DistributedLockManager, LockConfig, LockHandle, LockManager, MultiLockGuard,
};
pub use harana_components_schedule::{
    DurableScheduler, Job, Schedule, ScheduleQuery, ScheduleStats, ScheduleStatus, ScheduleStore, ScheduleType,
    SchedulerConfig,
};
pub use harana_components_storage::{Entity, FilterCondition, QueryOptions, StorageError, Store};
