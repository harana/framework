// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduleOutput {
    pub job_id: String,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
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
pub struct ListsOutput {
    pub jobs: Vec<String>,
    pub total: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobSchedule {
    pub cron_expression: String,
    #[serde(default)]
    pub is_active: bool,
    pub job_name: String,
    pub last_run_at: chrono::DateTime<chrono::Utc>,
    pub next_run_at: chrono::DateTime<chrono::Utc>,
    pub payload: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobLog {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub job_id: String,
    pub level: String,
    pub message: String,
}

#[async_trait]
pub trait JobAction {
    async fn schedule(&self, cron: String, handler: String, name: String, payload: String, priority: String, run_at: chrono::DateTime<chrono::Utc>) -> Result<ScheduleOutput, Box<dyn std::error::Error>>;
    async fn cancel(&self, job_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_status(&self, job_id: String) -> Result<GetStatusOutput, Box<dyn std::error::Error>>;
    async fn retry(&self, delay_seconds: i64, job_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn lists(&self, end_date: chrono::DateTime<chrono::Utc>, handler: String, limit: i64, offset: i64, start_date: chrono::DateTime<chrono::Utc>, status: String) -> Result<ListsOutput, Box<dyn std::error::Error>>;
    async fn pause(&self, job_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn resume(&self, job_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_result(&self, job_id: String, timeout: i64) -> Result<GetResultOutput, Box<dyn std::error::Error>>;
    async fn update_progress(&self, job_id: String, message: String, progress: f64) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, job_id: String) -> Result<(), Box<dyn std::error::Error>>;
}
