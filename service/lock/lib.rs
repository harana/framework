mod error;
mod service;

#[cfg(test)]
mod tests;

// Re-export core types
pub use error::{LockError, LockResult};
pub use service::{DistributedLockManager, LockHandle, LockManager, MultiLockGuard};
