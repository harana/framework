use async_trait::async_trait;
use serde_json::Value;
use tracing::info;

use crate::Job;

/// Trait for executing scheduled jobs
#[async_trait]
pub trait JobExecutor: Send + Sync {
    /// Execute a job and return the result
    async fn execute(&self, job: &Job) -> Result<Option<Value>, String>;
    /// Check if this executor can handle a given action type
    fn can_handle(&self, action_type: &str) -> bool;
}

pub struct LoggingExecutor;

#[async_trait]
impl JobExecutor for LoggingExecutor {
    async fn execute(&self, job: &Job) -> Result<Option<Value>, String> {
        info!(
            job_id = %job.id,
            schedule_id = %job.schedule_id,
            action_type = %job.action_type,
            "Executing job"
        );
        Ok(Some(serde_json::json!({ "status": "logged" })))
    }

    fn can_handle(&self, _action_type: &str) -> bool {
        true
    }
}
