// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateInput {
    pub binding: String,
    pub id: String,
    pub params: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOutput {
    pub id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateBatchInput {
    pub binding: String,
    pub instances: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateBatchOutput {
    pub instances: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInput {
    pub binding: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StatusInput {
    pub binding: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StatusOutput {
    pub error: String,
    pub output: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PauseInput {
    pub binding: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PauseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeInput {
    pub binding: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestartInput {
    pub binding: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestartOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TerminateInput {
    pub binding: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TerminateOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendEventInput {
    pub binding: String,
    pub id: String,
    pub payload: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendEventOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepDoInput {
    pub callback: String,
    pub name: String,
    pub retries_backoff: String,
    pub retries_delay: String,
    pub retries_limit: i64,
    pub timeout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepDoOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepSleepInput {
    pub duration: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepSleepOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepSleepUntilInput {
    pub name: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepSleepUntilOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepWaitForEventInput {
    pub name: String,
    pub timeout: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StepWaitForEventOutput {
    pub payload: String,
    pub type: String,
}

#[async_trait]
pub trait WorkflowsAction {
    async fn create(&self, input: CreateInput) -> Result<CreateOutput, Box<dyn std::error::Error>>;
    async fn create_batch(&self, input: CreateBatchInput) -> Result<CreateBatchOutput, Box<dyn std::error::Error>>;
    async fn get(&self, input: GetInput) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn status(&self, input: StatusInput) -> Result<StatusOutput, Box<dyn std::error::Error>>;
    async fn pause(&self, input: PauseInput) -> Result<PauseOutput, Box<dyn std::error::Error>>;
    async fn resume(&self, input: ResumeInput) -> Result<ResumeOutput, Box<dyn std::error::Error>>;
    async fn restart(&self, input: RestartInput) -> Result<RestartOutput, Box<dyn std::error::Error>>;
    async fn terminate(&self, input: TerminateInput) -> Result<TerminateOutput, Box<dyn std::error::Error>>;
    async fn send_event(&self, input: SendEventInput) -> Result<SendEventOutput, Box<dyn std::error::Error>>;
    async fn step_do(&self, input: StepDoInput) -> Result<StepDoOutput, Box<dyn std::error::Error>>;
    async fn step_sleep(&self, input: StepSleepInput) -> Result<StepSleepOutput, Box<dyn std::error::Error>>;
    async fn step_sleep_until(&self, input: StepSleepUntilInput) -> Result<StepSleepUntilOutput, Box<dyn std::error::Error>>;
    async fn step_wait_for_event(&self, input: StepWaitForEventInput) -> Result<StepWaitForEventOutput, Box<dyn std::error::Error>>;
}
