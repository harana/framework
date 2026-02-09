use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScheduleError {
    #[error("Schedule not found: {schedule_id}")]
    ScheduleNotFound { schedule_id: String },

    #[error("Job not found: {job_id}")]
    JobNotFound { job_id: String },

    #[error("Execution not found: {execution_id}")]
    ExecutionNotFound { execution_id: String },

    #[error("Invalid cron expression: {expression} - {reason}")]
    InvalidCronExpression { expression: String, reason: String },

    #[error("Invalid timezone: {timezone}")]
    InvalidTimezone { timezone: String },

    #[error("Invalid interval: {reason}")]
    InvalidInterval { reason: String },

    #[error("Schedule already exists: {schedule_id}")]
    ScheduleAlreadyExists { schedule_id: String },

    #[error("Schedule is disabled: {schedule_id}")]
    ScheduleDisabled { schedule_id: String },

    #[error("Schedule is paused: {schedule_id}")]
    SchedulePaused { schedule_id: String },

    #[error("Execution already completed: {execution_id}")]
    ExecutionAlreadyCompleted { execution_id: String },

    #[error("Execution timeout: {execution_id}")]
    ExecutionTimeout { execution_id: String },

    #[error("Max retries exceeded for job: {job_id}")]
    MaxRetriesExceeded { job_id: String },

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Lock acquisition failed: {resource}")]
    LockFailed { resource: String },

    #[error("Scheduler not running")]
    SchedulerNotRunning,

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for schedule operations
pub type ScheduleResult<T> = Result<T, ScheduleError>;

impl From<serde_json::Error> for ScheduleError {
    fn from(err: serde_json::Error) -> Self {
        ScheduleError::SerializationError(err.to_string())
    }
}

impl From<harana_components_lock::LockError> for ScheduleError {
    fn from(err: harana_components_lock::LockError) -> Self {
        ScheduleError::LockFailed {
            resource: err.to_string(),
        }
    }
}
