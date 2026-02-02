// Harana Components - Distributed Locking
//
// A distributed locking mechanism that uses the storage component for persistence.
// Includes deadlock prevention through:
// 1. Lock timeouts (TTL) - prevents indefinite holding
// 2. Lock ordering - prevents circular waits
// 3. Wait timeout - prevents indefinite waiting
// 4. Fencing tokens - prevents stale lock holders from making changes

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
