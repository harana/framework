// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateScheduleInput {
    pub cron_expression: String,
    pub description: String,
    #[serde(default)]
    pub enabled: bool,
    pub metadata: String,
    pub name: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateScheduleOutput {
    pub name: String,
    pub next_run: i64,
    pub schedule_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateScheduleInput {
    pub cron_expression: String,
    pub description: String,
    pub enabled: bool,
    pub metadata: String,
    pub name: String,
    pub schedule_id: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateScheduleOutput {
    pub next_run: i64,
    pub schedule_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteScheduleInput {
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteScheduleOutput {
    pub schedule_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnableScheduleInput {
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnableScheduleOutput {
    pub enabled: bool,
    pub next_run: i64,
    pub schedule_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisableScheduleInput {
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DisableScheduleOutput {
    pub enabled: bool,
    pub schedule_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetScheduleInput {
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
pub struct ListSchedulesInput {
    pub enabled: bool,
    pub limit: i64,
    pub offset: i64,
    pub search: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListSchedulesOutput {
    pub schedules: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TriggerScheduleInput {
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TriggerScheduleOutput {
    pub execution_id: String,
    pub schedule_id: String,
    pub success: bool,
    pub triggered_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetNextRunInput {
    pub count: i64,
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetNextRunOutput {
    pub next_runs: Vec<i64>,
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateCronInput {
    pub cron_expression: String,
    pub timezone: String,
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
pub struct CreateOneTimeInput {
    pub description: String,
    pub metadata: String,
    pub name: String,
    pub run_at: i64,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOneTimeOutput {
    pub run_at: i64,
    pub schedule_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PauseScheduleInput {
    pub resume_at: i64,
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PauseScheduleOutput {
    pub paused: bool,
    pub resume_at: i64,
    pub schedule_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeScheduleInput {
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeScheduleOutput {
    pub next_run: i64,
    pub paused: bool,
    pub schedule_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetScheduleHistoryInput {
    pub end_date: i64,
    pub limit: i64,
    pub schedule_id: String,
    pub start_date: i64,
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
pub struct GetScheduleStatsInput {
    pub end_date: i64,
    pub schedule_id: String,
    pub start_date: i64,
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
pub struct BulkEnableInput {
    pub schedule_ids: Vec<String>,
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
pub struct BulkDisableInput {
    pub schedule_ids: Vec<String>,
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
pub struct CreateIntervalInput {
    pub description: String,
    #[serde(default)]
    pub enabled: bool,
    pub end_time: i64,
    pub interval_seconds: i64,
    pub name: String,
    pub start_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIntervalOutput {
    pub next_run: i64,
    pub schedule_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateActionInput {
    pub action_config: String,
    pub action_type: String,
    pub schedule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateActionOutput {
    pub schedule_id: String,
    pub success: bool,
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
    pub success: bool,
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

#[async_trait]
pub trait ScheduleAction {
    async fn create_schedule(&self, input: CreateScheduleInput) -> Result<CreateScheduleOutput, Box<dyn std::error::Error>>;
    async fn update_schedule(&self, input: UpdateScheduleInput) -> Result<UpdateScheduleOutput, Box<dyn std::error::Error>>;
    async fn delete_schedule(&self, input: DeleteScheduleInput) -> Result<DeleteScheduleOutput, Box<dyn std::error::Error>>;
    async fn enable_schedule(&self, input: EnableScheduleInput) -> Result<EnableScheduleOutput, Box<dyn std::error::Error>>;
    async fn disable_schedule(&self, input: DisableScheduleInput) -> Result<DisableScheduleOutput, Box<dyn std::error::Error>>;
    async fn get_schedule(&self, input: GetScheduleInput) -> Result<GetScheduleOutput, Box<dyn std::error::Error>>;
    async fn list_schedules(&self, input: ListSchedulesInput) -> Result<ListSchedulesOutput, Box<dyn std::error::Error>>;
    async fn trigger_schedule(&self, input: TriggerScheduleInput) -> Result<TriggerScheduleOutput, Box<dyn std::error::Error>>;
    async fn get_next_run(&self, input: GetNextRunInput) -> Result<GetNextRunOutput, Box<dyn std::error::Error>>;
    async fn validate_cron(&self, input: ValidateCronInput) -> Result<ValidateCronOutput, Box<dyn std::error::Error>>;
    async fn create_one_time(&self, input: CreateOneTimeInput) -> Result<CreateOneTimeOutput, Box<dyn std::error::Error>>;
    async fn pause_schedule(&self, input: PauseScheduleInput) -> Result<PauseScheduleOutput, Box<dyn std::error::Error>>;
    async fn resume_schedule(&self, input: ResumeScheduleInput) -> Result<ResumeScheduleOutput, Box<dyn std::error::Error>>;
    async fn get_schedule_history(&self, input: GetScheduleHistoryInput) -> Result<GetScheduleHistoryOutput, Box<dyn std::error::Error>>;
    async fn get_schedule_stats(&self, input: GetScheduleStatsInput) -> Result<GetScheduleStatsOutput, Box<dyn std::error::Error>>;
    async fn bulk_enable(&self, input: BulkEnableInput) -> Result<BulkEnableOutput, Box<dyn std::error::Error>>;
    async fn bulk_disable(&self, input: BulkDisableInput) -> Result<BulkDisableOutput, Box<dyn std::error::Error>>;
    async fn create_interval(&self, input: CreateIntervalInput) -> Result<CreateIntervalOutput, Box<dyn std::error::Error>>;
    async fn update_action(&self, input: UpdateActionInput) -> Result<UpdateActionOutput, Box<dyn std::error::Error>>;
}
