// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// job
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Job {
    pub action_config: String,
    pub action_type: String,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: Option<i64>,
    pub error: Option<String>,
    pub error_details: String,
    pub lock_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub lock_token: Option<String>,
    pub max_retries: i64,
    pub metadata: String,
    pub result: Option<String>,
    pub retry_attempt: i64,
    pub retry_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Reference: schedule.id
    pub schedule_id: String,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub worker_id: Option<String>,
}

impl Job {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// job_status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobStatus {
    pub value: String,
}

impl JobStatus {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// job_query
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub schedule_id: Option<String>,
    pub scheduled_after: Option<chrono::DateTime<chrono::Utc>>,
    pub scheduled_before: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
}

impl JobQuery {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// job_info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobInfo {
    pub action_type: String,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration_ms: Option<i64>,
    pub error: Option<String>,
    pub job_id: String,
    pub retry_attempt: i64,
    pub schedule_id: String,
    pub schedule_name: String,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub worker_id: Option<String>,
}

impl JobInfo {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

