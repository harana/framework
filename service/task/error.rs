// Harana Components - Task Error Types

use thiserror::Error;

/// Task-specific errors
#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Task not found: {task_id}")]
    TaskNotFound { task_id: String },

    #[error("Task already exists: {task_id}")]
    TaskAlreadyExists { task_id: String },

    #[error("Task queue not found: {queue_name}")]
    QueueNotFound { queue_name: String },

    #[error("Task execution failed: {reason}")]
    ExecutionFailed { reason: String },

    #[error("Task timeout: {task_id} after {timeout_secs}s")]
    Timeout { task_id: String, timeout_secs: u64 },

    #[error("Task cancelled: {task_id}")]
    Cancelled { task_id: String },

    #[error("Max retries exceeded for task: {task_id}")]
    MaxRetriesExceeded { task_id: String },

    #[error("Invalid task configuration: {reason}")]
    InvalidConfiguration { reason: String },

    #[error("Worker not running")]
    WorkerNotRunning,

    #[error("Lock acquisition failed: {resource}")]
    LockFailed { resource: String },

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Store error: {reason}")]
    StoreError { reason: String },

    #[error("Schedule error: {0}")]
    ScheduleError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for task operations
pub type TaskResult<T> = Result<T, TaskError>;

impl From<serde_json::Error> for TaskError {
    fn from(err: serde_json::Error) -> Self {
        TaskError::SerializationError(err.to_string())
    }
}

impl From<harana_components_lock::LockError> for TaskError {
    fn from(err: harana_components_lock::LockError) -> Self {
        TaskError::LockFailed {
            resource: err.to_string(),
        }
    }
}

impl From<harana_components_storage::StorageError> for TaskError {
    fn from(err: harana_components_storage::StorageError) -> Self {
        TaskError::StorageError(err.to_string())
    }
}

impl From<harana_components_schedule::ScheduleError> for TaskError {
    fn from(err: harana_components_schedule::ScheduleError) -> Self {
        TaskError::ScheduleError(err.to_string())
    }
}
