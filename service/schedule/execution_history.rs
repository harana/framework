use chrono::{DateTime, Utc};
use harana_components_storage::Entity;
use serde::{Deserialize, Serialize};

use crate::job::Job;
use crate::job_status::JobStatus;

/// Historical record of a schedule execution attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionHistory {
    pub id: String,
    pub schedule_id: String,
    pub job_id: String,
    pub status: JobStatus,
    pub scheduled_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<i64>,
    pub retry_attempt: u32,
    pub error: Option<String>,
    pub worker_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl ExecutionHistory {
    pub fn from_job(job: &Job) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            schedule_id: job.schedule_id.clone(),
            job_id: job.id.clone(),
            status: job.status.clone(),
            scheduled_at: job.scheduled_at,
            started_at: job.started_at,
            completed_at: job.completed_at,
            duration_ms: job.duration_ms,
            retry_attempt: job.retry_attempt,
            error: job.error.clone(),
            worker_id: job.worker_id.clone(),
            created_at: Utc::now(),
        }
    }
}

impl Entity for ExecutionHistory {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type() -> &'static str {
        "execution_history"
    }
}
