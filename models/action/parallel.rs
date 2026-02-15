// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AllInput {
    #[serde(default)]
    pub fail_fast: bool,
    pub max_concurrency: i64,
    pub tasks: Vec<String>,
    pub timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AllOutput {
    pub completed: i64,
    pub failed: i64,
    pub results: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RaceInput {
    pub tasks: Vec<String>,
    pub timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RaceOutput {
    pub result: String,
    pub success: bool,
    pub winner_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnyInput {
    pub tasks: Vec<String>,
    pub timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnyOutput {
    pub result: String,
    pub success: bool,
    pub success_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AllSettledInput {
    pub max_concurrency: i64,
    pub tasks: Vec<String>,
    pub timeout: i64,
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
pub struct MapInput {
    pub handler: String,
    pub items: Vec<String>,
    pub max_concurrency: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MapOutput {
    pub errors: Vec<String>,
    pub results: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FilterInput {
    pub handler: String,
    pub items: Vec<String>,
    pub max_concurrency: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FilterOutput {
    pub count: i64,
    pub results: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReduceInput {
    pub handler: String,
    pub initial_value: String,
    pub items: Vec<String>,
    pub max_concurrency: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReduceOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetryInput {
    pub backoff_multiplier: f64,
    pub delay_ms: i64,
    pub max_attempts: i64,
    pub task: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetryOutput {
    pub attempts: i64,
    pub result: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimeoutInput {
    pub task: String,
    pub timeout_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimeoutOutput {
    pub result: String,
    pub success: bool,
    pub timed_out: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchInput {
    pub batch_size: i64,
    pub handler: String,
    pub items: Vec<String>,
    pub max_concurrency: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchOutput {
    pub batch_count: i64,
    pub results: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SemaphoreInput {
    pub max_concurrent: i64,
    pub tasks: Vec<String>,
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
    pub success: bool,
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

#[async_trait]
pub trait ParallelAction {
    async fn all(&self, input: AllInput) -> Result<AllOutput, Box<dyn std::error::Error>>;
    async fn race(&self, input: RaceInput) -> Result<RaceOutput, Box<dyn std::error::Error>>;
    async fn any(&self, input: AnyInput) -> Result<AnyOutput, Box<dyn std::error::Error>>;
    async fn all_settled(&self, input: AllSettledInput) -> Result<AllSettledOutput, Box<dyn std::error::Error>>;
    async fn map(&self, input: MapInput) -> Result<MapOutput, Box<dyn std::error::Error>>;
    async fn filter(&self, input: FilterInput) -> Result<FilterOutput, Box<dyn std::error::Error>>;
    async fn reduce(&self, input: ReduceInput) -> Result<ReduceOutput, Box<dyn std::error::Error>>;
    async fn retry(&self, input: RetryInput) -> Result<RetryOutput, Box<dyn std::error::Error>>;
    async fn timeout(&self, input: TimeoutInput) -> Result<TimeoutOutput, Box<dyn std::error::Error>>;
    async fn batch(&self, input: BatchInput) -> Result<BatchOutput, Box<dyn std::error::Error>>;
    async fn semaphore(&self, input: SemaphoreInput) -> Result<SemaphoreOutput, Box<dyn std::error::Error>>;
}
