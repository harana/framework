// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskBackoffStrategy {
    pub value: String,
}

impl TaskBackoffStrategy {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskPriority {
    pub value: String,
}

impl TaskPriority {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskStatus {
    pub value: String,
}

impl TaskStatus {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskRetryConfig {
    pub backoff_strategy: String,
    pub max_delay_secs: i64,
    pub max_retries: i64,
    pub no_retry_on_errors: Option<Vec<String>>,
    pub retry_delay_secs: i64,
    pub retry_on_errors: Option<Vec<String>>,
}

impl TaskRetryConfig {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Task {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub correlation_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub duration_ms: Option<i64>,
    pub error: Option<String>,
    pub error_details: Option<String>,
    pub id: String,
    pub lock_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub lock_token: Option<String>,
    pub metadata: std::collections::HashMap<String, String>,
    pub name: String,
    pub owner_id: Option<String>,
    pub parent_task_id: Option<String>,
    pub payload: std::collections::HashMap<String, String>,
    pub priority: String,
    pub progress: Option<i64>,
    pub progress_message: Option<String>,
    pub queue: String,
    pub result: Option<String>,
    pub retry_attempt: i64,
    pub retry_at: Option<chrono::DateTime<chrono::Utc>>,
    pub retry_config: String,
    pub schedule_id: Option<String>,
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub tags: Vec<String>,
    pub task_type: String,
    pub timeout_secs: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
    pub worker_id: Option<String>,
}

impl Task {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskExecutionHistory {
    pub attempt: i64,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub correlation_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: Option<i64>,
    pub error: Option<String>,
    pub error_details: Option<String>,
    pub id: String,
    pub queue: String,
    pub result: Option<String>,
    pub schedule_id: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub task_id: String,
    pub task_name: String,
    pub task_type: String,
    pub worker_id: Option<String>,
}

impl TaskExecutionHistory {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskQuery {
    pub correlation_id: Option<String>,
    pub created_after: Option<chrono::DateTime<chrono::Utc>>,
    pub created_before: Option<chrono::DateTime<chrono::Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub owner_id: Option<String>,
    pub parent_task_id: Option<String>,
    pub priority: String,
    pub queue: Option<String>,
    pub schedule_id: Option<String>,
    pub scheduled_after: Option<chrono::DateTime<chrono::Utc>>,
    pub scheduled_before: Option<chrono::DateTime<chrono::Utc>>,
    pub search: Option<String>,
    pub status: String,
    pub tags: Option<Vec<String>>,
    pub task_type: Option<String>,
    pub worker_id: Option<String>,
}

impl TaskQuery {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskQueueStats {
    pub average_duration_ms: Option<f64>,
    pub average_wait_time_ms: Option<f64>,
    pub cancelled: i64,
    pub completed: i64,
    pub failed: i64,
    pub pending: i64,
    pub retrying: i64,
    pub running: i64,
    pub scheduled: i64,
    pub timed_out: i64,
    pub total: i64,
}

impl TaskQueueStats {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskWorkerConfig {
    #[serde(default)]
    pub auto_retry: bool,
    pub batch_size: i64,
    pub cleanup_interval_secs: i64,
    pub history_retention_days: i64,
    pub lock_duration_secs: i64,
    pub max_concurrent_tasks: i64,
    pub poll_interval_ms: i64,
    #[serde(default)]
    pub process_scheduled: bool,
    pub queues: Vec<String>,
    pub stale_check_interval_secs: i64,
    pub stale_threshold_secs: i64,
    pub worker_id: String,
}

impl TaskWorkerConfig {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskScheduledConfig {
    pub description: Option<String>,
    pub name: String,
    pub owner_id: Option<String>,
    pub payload: std::collections::HashMap<String, String>,
    pub priority: String,
    pub queue: String,
    pub tags: Vec<String>,
    pub task_type: String,
    pub timeout_secs: i64,
}

impl TaskScheduledConfig {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskManagerConfig {
    #[serde(default)]
    pub enable_scheduler: bool,
    #[serde(default)]
    pub enable_worker: bool,
    pub lock_config: String,
    pub scheduler_config: String,
    pub worker_config: String,
}

impl TaskManagerConfig {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

