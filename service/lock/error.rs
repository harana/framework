// Harana Components - Lock Error Types

use thiserror::Error;

/// Lock-specific errors
#[derive(Debug, Error)]
pub enum LockError {
    #[error("Failed to acquire lock on resource: {resource}")]
    LockFailed { resource: String },

    #[error("Lock not found: {resource}")]
    NotFound { resource: String },

    #[error("Lock already held by another owner")]
    AlreadyHeld,

    #[error("Lock expired")]
    Expired,

    #[error("Lock ordering violation: {message}")]
    OrderingViolation { message: String },

    #[error("Maximum locks per owner exceeded: {max}")]
    MaxLocksExceeded { max: usize },

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for lock operations
pub type LockResult<T> = Result<T, LockError>;
