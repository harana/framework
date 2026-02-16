// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AllOutput {
    pub completed: i64,
    pub failed: i64,
    pub results: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RaceOutput {
    pub result: String,
    pub winner_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnyOutput {
    pub result: String,
    pub success_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AllSettledOutput {
    pub fulfilled: i64,
    pub rejected: i64,
    pub results: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MapOutput {
    pub errors: Vec<String>,
    pub results: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FilterOutput {
    pub count: i64,
    pub results: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetryOutput {
    pub attempts: i64,
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimeoutOutput {
    pub result: String,
    pub timed_out: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchOutput {
    pub batch_count: i64,
    pub results: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SemaphoreOutput {
    pub completed: i64,
    pub failed: i64,
    pub results: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParallelTask {
    pub id: String,
    pub handler: String,
    pub input: String,
    pub timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParallelResult {
    pub task_id: String,
    pub result: String,
    pub error: String,
    pub duration_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParallelSettledResult {
    pub task_id: String,
    pub status: String,
    pub value: String,
    pub reason: String,
    pub duration_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParallelError {
    pub task_id: String,
    pub index: i64,
    pub message: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParallelExecution {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub completed_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub failed_count: i64,
    pub max_concurrency: i64,
    pub status: String,
    pub strategy: String,
    pub timeout_ms: i64,
    pub total_count: i64,
}

#[async_trait]
pub trait ParallelAction {
    async fn all(&self, fail_fast: bool, max_concurrency: i64, tasks: Vec<String>, timeout: i64) -> Result<AllOutput, Box<dyn std::error::Error>>;
    async fn race(&self, tasks: Vec<String>, timeout: i64) -> Result<RaceOutput, Box<dyn std::error::Error>>;
    async fn any(&self, tasks: Vec<String>, timeout: i64) -> Result<AnyOutput, Box<dyn std::error::Error>>;
    async fn all_settled(&self, max_concurrency: i64, tasks: Vec<String>, timeout: i64) -> Result<AllSettledOutput, Box<dyn std::error::Error>>;
    async fn map(&self, handler: String, items: Vec<String>, max_concurrency: i64) -> Result<MapOutput, Box<dyn std::error::Error>>;
    async fn filter(&self, handler: String, items: Vec<String>, max_concurrency: i64) -> Result<FilterOutput, Box<dyn std::error::Error>>;
    async fn reduce(&self, handler: String, initial_value: String, items: Vec<String>, max_concurrency: i64) -> Result<String, Box<dyn std::error::Error>>;
    async fn retry(&self, backoff_multiplier: f64, delay_ms: i64, max_attempts: i64, task: String) -> Result<RetryOutput, Box<dyn std::error::Error>>;
    async fn timeout(&self, task: String, timeout_ms: i64) -> Result<TimeoutOutput, Box<dyn std::error::Error>>;
    async fn batch(&self, batch_size: i64, handler: String, items: Vec<String>, max_concurrency: i64) -> Result<BatchOutput, Box<dyn std::error::Error>>;
    async fn semaphore(&self, max_concurrent: i64, tasks: Vec<String>) -> Result<SemaphoreOutput, Box<dyn std::error::Error>>;
}
