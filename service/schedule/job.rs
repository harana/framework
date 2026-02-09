use chrono::{DateTime, Utc};
use harana_components_storage::Entity;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::job_status::JobStatus;
use crate::schedule::Schedule;

/// Job entity representing a single instance of a scheduled execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub schedule_id: String,
    pub schedule_name: String,
    pub status: JobStatus,
    pub scheduled_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub action_type: String,
    pub action_config: HashMap<String, Value>,
    pub result: Option<Value>,
    pub error: Option<String>,
    pub error_details: Option<String>,
    pub retry_attempt: u32,
    pub max_retries: u32,
    pub retry_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<i64>,
    pub worker_id: Option<String>,
    pub lock_token: Option<String>,
    pub lock_expires_at: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Job {
    pub fn new(schedule: &Schedule, scheduled_at: DateTime<Utc>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            schedule_id: schedule.id.clone(),
            schedule_name: schedule.name.clone(),
            status: JobStatus::Pending,
            scheduled_at,
            started_at: None,
            completed_at: None,
            action_type: schedule.action_type.clone(),
            action_config: schedule.action_config.clone(),
            result: None,
            error: None,
            error_details: None,
            retry_attempt: 0,
            max_retries: schedule.retry_config.max_retries,
            retry_at: None,
            duration_ms: None,
            worker_id: None,
            lock_token: None,
            lock_expires_at: None,
            metadata: schedule.metadata.clone(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn start(&mut self, worker_id: Option<String>) {
        self.status = JobStatus::Running;
        self.started_at = Some(Utc::now());
        self.worker_id = worker_id;
        self.updated_at = Utc::now();
    }

    /// Complete the job successfully
    pub fn complete(&mut self, result: Option<Value>) {
        let now = Utc::now();
        self.status = JobStatus::Completed;
        self.completed_at = Some(now);
        self.result = result;
        if let Some(started) = self.started_at {
            self.duration_ms = Some((now - started).num_milliseconds());
        }
        self.updated_at = now;
    }

    /// Fail the job
    pub fn fail(&mut self, error: String, details: Option<String>) {
        let now = Utc::now();
        self.status = JobStatus::Failed;
        self.completed_at = Some(now);
        self.error = Some(error);
        self.error_details = details;
        if let Some(started) = self.started_at {
            self.duration_ms = Some((now - started).num_milliseconds());
        }
        self.updated_at = now;
    }

    /// Mark job for retry
    pub fn retry(&mut self, retry_at: DateTime<Utc>) {
        self.status = JobStatus::Retrying;
        self.retry_attempt += 1;
        self.retry_at = Some(retry_at);
        self.started_at = None;
        self.updated_at = Utc::now();
    }

    /// Cancel the job
    pub fn cancel(&mut self) {
        self.status = JobStatus::Cancelled;
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn can_retry(&self) -> bool {
        self.retry_attempt < self.max_retries && !self.status.is_terminal()
    }
}

impl Entity for Job {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type() -> &'static str {
        "job"
    }
}
