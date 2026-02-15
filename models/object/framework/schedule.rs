// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Schedule {
    pub action_config: std::collections::HashMap<String, String>,
    pub action_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub cron_expression: Option<String>,
    pub description: Option<String>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
    pub execution_count: i64,
    pub id: String,
    pub interval_seconds: Option<i64>,
    pub last_run_at: Option<chrono::DateTime<chrono::Utc>>,
    pub max_executions: Option<i64>,
    pub metadata: std::collections::HashMap<String, String>,
    pub name: String,
    pub next_run_at: Option<chrono::DateTime<chrono::Utc>>,
    pub owner_id: Option<String>,
    pub resume_at: Option<chrono::DateTime<chrono::Utc>>,
    pub retry_config: String,
    pub run_at: Option<chrono::DateTime<chrono::Utc>>,
    pub schedule_type: String,
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub tags: Vec<String>,
    pub timezone: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
}

impl Schedule {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleStatus {
    pub value: String,
}

impl ScheduleStatus {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleType {
    pub value: String,
}

impl ScheduleType {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleExecutionHistory {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: Option<i64>,
    pub error: Option<String>,
    pub job_id: String,
    pub retry_attempt: i64,
    pub schedule_id: String,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub worker_id: Option<String>,
}

impl ScheduleExecutionHistory {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleRetryConfig {
    pub initial_delay_secs: i64,
    #[serde(default)]
    pub jitter: bool,
    pub max_delay_secs: i64,
    pub max_retries: i64,
    pub multiplier: f64,
    pub strategy: String,
}

impl ScheduleRetryConfig {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleBackoffStrategy {
    pub value: String,
}

impl ScheduleBackoffStrategy {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleQuery {
    pub due_before: Option<chrono::DateTime<chrono::Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub owner_id: Option<String>,
    pub schedule_type: String,
    pub search: Option<String>,
    pub status: String,
    pub tags: Option<Vec<String>>,
}

impl ScheduleQuery {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleStats {
    pub average_duration_ms: Option<f64>,
    pub failed_executions: i64,
    pub last_execution_at: Option<chrono::DateTime<chrono::Utc>>,
    pub next_execution_at: Option<chrono::DateTime<chrono::Utc>>,
    pub successful_executions: i64,
    pub total_executions: i64,
}

impl ScheduleStats {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleSchedulerConfig {
    #[serde(default)]
    pub auto_create_jobs: bool,
    pub batch_size: i64,
    pub cleanup_interval_secs: i64,
    pub history_retention_days: i64,
    pub lock_duration_secs: i64,
    pub max_concurrent_jobs: i64,
    pub poll_interval_secs: i64,
    pub stale_check_interval_secs: i64,
    pub stale_threshold_secs: i64,
    pub worker_id: String,
}

impl ScheduleSchedulerConfig {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleInfo {
    pub cron_expression: Option<String>,
    pub enabled: bool,
    pub last_run: Option<i64>,
    pub name: String,
    pub next_run: Option<i64>,
    pub schedule_id: String,
}

impl ScheduleInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

