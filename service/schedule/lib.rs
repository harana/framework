// Core entity types - now split into separate modules for better organization
mod backoff_strategy;
mod execution_history;
mod job;
mod job_status;
mod retry_config;
mod schedule;
mod schedule_status;
mod schedule_type;

// Other modules
mod entity;
mod error;
mod job_executor;
mod job_query;
mod schedule_query;
mod schedule_stats;
mod schedule_store_trait;
mod scheduler;
mod scheduler_config;
mod scheduler_event;
mod store;

#[cfg(test)]
mod tests;

// Re-export core types from their individual modules
pub use backoff_strategy::BackoffStrategy;
pub use error::{ScheduleError, ScheduleResult};
pub use execution_history::ExecutionHistory;
pub use job::Job;
pub use job_status::JobStatus;
pub use retry_config::RetryConfig;
pub use schedule::Schedule;
pub use schedule_status::ScheduleStatus;
pub use schedule_type::ScheduleType;

// Re-export locking types from the lock component
pub use harana_components_lock::{
    DistributedLock, DistributedLockManager, LockConfig, LockHandle, LockManager, MultiLockGuard, job_lock_resource,
    schedule_lock_resource, worker_lock_resource,
};

pub use job_executor::{JobExecutor, LoggingExecutor};
pub use job_query::JobQuery;
pub use schedule_query::ScheduleQuery;
pub use schedule_stats::ScheduleStats;
pub use schedule_store_trait::ScheduleStore;
pub use scheduler::{DurableScheduler, get_next_cron_runs, validate_cron, validate_timezone};
pub use scheduler_config::SchedulerConfig;
pub use scheduler_event::SchedulerEvent;
pub use store::{InMemoryScheduleStore, PersistentScheduleStore};
