// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateScheduleOutput {
    pub name: String,
    pub next_run: i64,
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateScheduleOutput {
    pub next_run: i64,
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnableScheduleOutput {
    pub enabled: bool,
    pub next_run: i64,
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisableScheduleOutput {
    pub enabled: bool,
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetScheduleOutput {
    pub created_at: i64,
    pub cron_expression: String,
    pub enabled: bool,
    pub last_run: i64,
    pub metadata: String,
    pub name: String,
    pub next_run: i64,
    pub schedule_id: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListSchedulesOutput {
    pub schedules: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TriggerScheduleOutput {
    pub execution_id: String,
    pub schedule_id: String,
    pub triggered_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetNextRunOutput {
    pub next_runs: Vec<i64>,
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateCronOutput {
    pub error: String,
    pub next_run: i64,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOneTimeOutput {
    pub run_at: i64,
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PauseScheduleOutput {
    pub paused: bool,
    pub resume_at: i64,
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeScheduleOutput {
    pub next_run: i64,
    pub paused: bool,
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetScheduleHistoryOutput {
    pub executions: Vec<String>,
    pub schedule_id: String,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetScheduleStatsOutput {
    pub average_duration: f64,
    pub failed_executions: i64,
    pub last_execution: String,
    pub schedule_id: String,
    pub successful_executions: i64,
    pub total_executions: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkEnableOutput {
    pub failed: i64,
    pub results: Vec<String>,
    pub successful: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkDisableOutput {
    pub failed: i64,
    pub results: Vec<String>,
    pub successful: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIntervalOutput {
    pub next_run: i64,
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Schedule {
    pub schedule_id: String,
    pub name: String,
    pub cron_expression: String,
    pub timezone: String,
    pub enabled: bool,
    pub next_run: chrono::DateTime<chrono::Utc>,
    pub last_run: chrono::DateTime<chrono::Utc>,
    pub metadata: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleMetadata {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleInfo {
    pub schedule_id: String,
    pub name: String,
    pub cron_expression: String,
    pub enabled: bool,
    pub next_run: i64,
    pub last_run: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleExecution {
    pub execution_id: String,
    pub schedule_id: String,
    pub started_at: i64,
    pub completed_at: i64,
    pub status: String,
    pub duration_ms: i64,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleBulkResult {
    pub schedule_id: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleActionConfig {
    pub url: String,
    pub method: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Schedule {
    pub action_config: std::collections::HashMap<String, String>,
    pub action_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub cron_expression: String,
    pub description: String,
    pub end_at: chrono::DateTime<chrono::Utc>,
    pub execution_count: i64,
    pub id: String,
    pub interval_seconds: i64,
    pub last_run_at: chrono::DateTime<chrono::Utc>,
    pub max_executions: i64,
    pub metadata: std::collections::HashMap<String, String>,
    pub name: String,
    pub next_run_at: chrono::DateTime<chrono::Utc>,
    pub owner_id: String,
    pub resume_at: chrono::DateTime<chrono::Utc>,
    pub retry_config: String,
    pub run_at: chrono::DateTime<chrono::Utc>,
    pub schedule_type: String,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub tags: Vec<String>,
    pub timezone: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleStatus {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleType {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleExecutionHistory {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: i64,
    pub error: String,
    pub job_id: String,
    pub retry_attempt: i64,
    pub schedule_id: String,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub worker_id: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleBackoffStrategy {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleQuery {
    pub due_before: chrono::DateTime<chrono::Utc>,
    pub limit: i64,
    pub offset: i64,
    pub owner_id: String,
    pub schedule_type: String,
    pub search: String,
    pub status: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleStats {
    pub average_duration_ms: f64,
    pub failed_executions: i64,
    pub last_execution_at: chrono::DateTime<chrono::Utc>,
    pub next_execution_at: chrono::DateTime<chrono::Utc>,
    pub successful_executions: i64,
    pub total_executions: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleInfo {
    pub cron_expression: String,
    pub enabled: bool,
    pub last_run: i64,
    pub name: String,
    pub next_run: i64,
    pub schedule_id: String,
}

#[async_trait]
pub trait ScheduleAction {
    async fn create_schedule(&self, cron_expression: String, description: String, enabled: bool, metadata: String, name: String, timezone: String) -> Result<CreateScheduleOutput, Box<dyn std::error::Error>>;
    async fn update_schedule(&self, cron_expression: String, description: String, enabled: bool, metadata: String, name: String, schedule_id: String, timezone: String) -> Result<UpdateScheduleOutput, Box<dyn std::error::Error>>;
    async fn delete_schedule(&self, schedule_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn enable_schedule(&self, schedule_id: String) -> Result<EnableScheduleOutput, Box<dyn std::error::Error>>;
    async fn disable_schedule(&self, schedule_id: String) -> Result<DisableScheduleOutput, Box<dyn std::error::Error>>;
    async fn get_schedule(&self, schedule_id: String) -> Result<GetScheduleOutput, Box<dyn std::error::Error>>;
    async fn list_schedules(&self, enabled: bool, limit: i64, offset: i64, search: String) -> Result<ListSchedulesOutput, Box<dyn std::error::Error>>;
    async fn trigger_schedule(&self, schedule_id: String) -> Result<TriggerScheduleOutput, Box<dyn std::error::Error>>;
    async fn get_next_run(&self, count: i64, schedule_id: String) -> Result<GetNextRunOutput, Box<dyn std::error::Error>>;
    async fn validate_cron(&self, cron_expression: String, timezone: String) -> Result<ValidateCronOutput, Box<dyn std::error::Error>>;
    async fn create_one_time(&self, description: String, metadata: String, name: String, run_at: i64, timezone: String) -> Result<CreateOneTimeOutput, Box<dyn std::error::Error>>;
    async fn pause_schedule(&self, resume_at: i64, schedule_id: String) -> Result<PauseScheduleOutput, Box<dyn std::error::Error>>;
    async fn resume_schedule(&self, schedule_id: String) -> Result<ResumeScheduleOutput, Box<dyn std::error::Error>>;
    async fn get_schedule_history(&self, end_date: i64, limit: i64, schedule_id: String, start_date: i64) -> Result<GetScheduleHistoryOutput, Box<dyn std::error::Error>>;
    async fn get_schedule_stats(&self, end_date: i64, schedule_id: String, start_date: i64) -> Result<GetScheduleStatsOutput, Box<dyn std::error::Error>>;
    async fn bulk_enable(&self, schedule_ids: Vec<String>) -> Result<BulkEnableOutput, Box<dyn std::error::Error>>;
    async fn bulk_disable(&self, schedule_ids: Vec<String>) -> Result<BulkDisableOutput, Box<dyn std::error::Error>>;
    async fn create_interval(&self, description: String, enabled: bool, end_time: i64, interval_seconds: i64, name: String, start_time: i64) -> Result<CreateIntervalOutput, Box<dyn std::error::Error>>;
    async fn update_action(&self, action_config: String, action_type: String, schedule_id: String) -> Result<String, Box<dyn std::error::Error>>;
}
