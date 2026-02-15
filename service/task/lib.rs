mod error;
mod service;
mod store;

#[cfg(test)]
mod tests;

// Re-export error types
pub use error::{TaskError, TaskResult};

// Re-export service types
pub use service::{
    AsyncFnExecutor, CompositeExecutor, FnExecutor, LoggingExecutor, TaskCreatingExecutor,
    TaskExecutor, TaskManager, TaskSchedulerManager, TaskSubmitBuilder, TaskWorker,
};

// Re-export store functions
pub use store::{
    cleanup_history, count_tasks, create_task, delete_task, extend_task_lock, get_queue_history,
    get_queue_stats, get_queues, get_runnable_tasks, get_stale_tasks, get_task, get_task_history,
    query_tasks, record_history, release_task_lock, try_lock_task, update_task,
};

// Re-export commonly used types from dependencies
pub use harana_components_cache::CacheService;
pub use harana_components_lock::{
    DistributedLock, DistributedLockManager, LockConfig, LockHandle, LockManager, MultiLockGuard,
};
pub use harana_components_schedule::{
    DurableScheduler, Job, Schedule, ScheduleQuery, ScheduleStats, ScheduleStatus, ScheduleService,
    ScheduleType, SchedulerConfig,
};
pub use harana_components_storage::{Entity, FilterCondition, QueryOptions, StorageError, Store};
