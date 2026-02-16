// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskBackoffStrategy {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskPriority {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskStatus {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskRetryConfig {
    pub backoff_strategy: String,
    pub max_delay_secs: i64,
    pub max_retries: i64,
    pub no_retry_on_errors: Vec<String>,
    pub retry_delay_secs: i64,
    pub retry_on_errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Task {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub correlation_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub duration_ms: i64,
    pub error: String,
    pub error_details: String,
    pub id: String,
    pub lock_expires_at: chrono::DateTime<chrono::Utc>,
    pub lock_token: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub name: String,
    pub owner_id: String,
    pub parent_task_id: String,
    pub payload: std::collections::HashMap<String, String>,
    pub priority: String,
    pub progress: i64,
    pub progress_message: String,
    pub queue: String,
    pub result: String,
    pub retry_attempt: i64,
    pub retry_at: chrono::DateTime<chrono::Utc>,
    pub retry_config: String,
    pub schedule_id: String,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub tags: Vec<String>,
    pub task_type: String,
    pub timeout_secs: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
    pub worker_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskExecutionHistory {
    pub attempt: i64,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub correlation_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: i64,
    pub error: String,
    pub error_details: String,
    pub id: String,
    pub queue: String,
    pub result: String,
    pub schedule_id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub task_id: String,
    pub task_name: String,
    pub task_type: String,
    pub worker_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskQuery {
    pub correlation_id: String,
    pub created_after: chrono::DateTime<chrono::Utc>,
    pub created_before: chrono::DateTime<chrono::Utc>,
    pub limit: i64,
    pub offset: i64,
    pub owner_id: String,
    pub parent_task_id: String,
    pub priority: String,
    pub queue: String,
    pub schedule_id: String,
    pub scheduled_after: chrono::DateTime<chrono::Utc>,
    pub scheduled_before: chrono::DateTime<chrono::Utc>,
    pub search: String,
    pub status: String,
    pub tags: Vec<String>,
    pub task_type: String,
    pub worker_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskQueueStats {
    pub average_duration_ms: f64,
    pub average_wait_time_ms: f64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskScheduledConfig {
    pub description: String,
    pub name: String,
    pub owner_id: String,
    pub payload: std::collections::HashMap<String, String>,
    pub priority: String,
    pub queue: String,
    pub tags: Vec<String>,
    pub task_type: String,
    pub timeout_secs: i64,
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

