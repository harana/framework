// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleInput {
    pub cron: String,
    pub handler: String,
    pub name: String,
    pub payload: String,
    pub priority: String,
    pub run_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleOutput {
    pub job_id: String,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelInput {
    pub job_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetStatusInput {
    pub job_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetStatusOutput {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub error: String,
    pub progress: f64,
    pub result: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetryInput {
    pub delay_seconds: i64,
    pub job_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetryOutput {
    pub new_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListsInput {
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub handler: String,
    pub limit: i64,
    pub offset: i64,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListsOutput {
    pub jobs: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PauseInput {
    pub job_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PauseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeInput {
    pub job_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetResultInput {
    pub job_id: String,
    pub timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetResultOutput {
    pub completed: bool,
    pub error: String,
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateProgressInput {
    pub job_id: String,
    pub message: String,
    pub progress: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateProgressOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteInput {
    pub job_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Job {
    pub job_id: String,
    pub name: String,
    pub handler: String,
    pub payload: String,
    pub status: String,
    pub priority: String,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub progress: f64,
    pub result: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobInfo {
    pub job_id: String,
    pub name: String,
    pub handler: String,
    pub status: String,
    pub priority: String,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait JobAction {
    async fn schedule(&self, input: ScheduleInput) -> Result<ScheduleOutput, Box<dyn std::error::Error>>;
    async fn cancel(&self, input: CancelInput) -> Result<CancelOutput, Box<dyn std::error::Error>>;
    async fn get_status(&self, input: GetStatusInput) -> Result<GetStatusOutput, Box<dyn std::error::Error>>;
    async fn retry(&self, input: RetryInput) -> Result<RetryOutput, Box<dyn std::error::Error>>;
    async fn lists(&self, input: ListsInput) -> Result<ListsOutput, Box<dyn std::error::Error>>;
    async fn pause(&self, input: PauseInput) -> Result<PauseOutput, Box<dyn std::error::Error>>;
    async fn resume(&self, input: ResumeInput) -> Result<ResumeOutput, Box<dyn std::error::Error>>;
    async fn get_result(&self, input: GetResultInput) -> Result<GetResultOutput, Box<dyn std::error::Error>>;
    async fn update_progress(&self, input: UpdateProgressInput) -> Result<UpdateProgressOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, input: DeleteInput) -> Result<DeleteOutput, Box<dyn std::error::Error>>;
}
