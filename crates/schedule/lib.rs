// Harana Components - Durable Scheduler
// A persistent, durable scheduler with support for cron, interval, and one-time schedules.
// Uses the storage component for persistence across restarts.

mod entity;
mod error;
mod scheduler;
mod store;

#[cfg(test)]
mod tests;

// Re-export core types
pub use entity::{
    BackoffStrategy, ExecutionHistory, Job, JobStatus, RetryConfig, Schedule, ScheduleStatus, ScheduleType,
};
pub use error::{ScheduleError, ScheduleResult};

// Re-export locking types from the lock component
pub use harana_components_lock::{
    DistributedLock, DistributedLockManager, LockConfig, LockHandle, LockManager, MultiLockGuard, job_lock_resource,
    schedule_lock_resource, worker_lock_resource,
};

pub use scheduler::{
    DurableScheduler, JobExecutor, LoggingExecutor, SchedulerConfig, SchedulerEvent, get_next_cron_runs, validate_cron,
    validate_timezone,
};
pub use store::{
    InMemoryScheduleStore, JobQuery, PersistentScheduleStore, ScheduleQuery, ScheduleStats, ScheduleStore,
};
