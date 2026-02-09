mod error;
mod manager;

#[cfg(test)]
mod tests;

// Re-export core types
pub use error::{LockError, LockResult};
pub use manager::{
    DistributedLock, DistributedLockManager, LockConfig, LockHandle, LockManager, MultiLockGuard,
    channel_lock_resource, event_lock_resource, job_lock_resource, schedule_lock_resource, subscription_lock_resource,
    worker_lock_resource,
};
