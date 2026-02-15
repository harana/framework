// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartInput {
    pub context: String,
    pub input: String,
    pub workflow_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartOutput {
    pub execution_id: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PauseInput {
    pub execution_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PauseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeInput {
    pub execution_id: String,
    pub input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResumeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelInput {
    pub execution_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetStatusInput {
    pub execution_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetStatusOutput {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub current_step: String,
    pub error: String,
    pub progress: f64,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetResultInput {
    pub execution_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetResultOutput {
    pub completed: bool,
    pub error: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListExecutionsInput {
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub limit: i64,
    pub offset: i64,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub workflow_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListExecutionsOutput {
    pub executions: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignalInput {
    pub execution_id: String,
    pub payload: String,
    pub signal_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignalOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WaitForEventInput {
    pub event_name: String,
    pub execution_id: String,
    pub timeout: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WaitForEventOutput {
    pub payload: String,
    pub received: bool,
    pub timed_out: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HistoryInput {
    pub execution_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HistoryOutput {
    pub events: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetryStepInput {
    pub execution_id: String,
    pub step_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetryStepOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SkipStepInput {
    pub execution_id: String,
    pub output: String,
    pub step_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SkipStepOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TerminateAllInput {
    pub reason: String,
    pub workflow_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TerminateAllOutput {
    pub success: bool,
    pub terminated_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowExecution {
    pub execution_id: String,
    pub workflow_id: String,
    pub status: String,
    pub current_step: String,
    pub input: String,
    pub output: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowContext {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowExecutionInfo {
    pub execution_id: String,
    pub workflow_id: String,
    pub status: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowHistoryEvent {
    pub event_id: String,
    pub event_type: String,
    pub step_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: String,
}

#[async_trait]
pub trait WorkflowAction {
    async fn start(&self, input: StartInput) -> Result<StartOutput, Box<dyn std::error::Error>>;
    async fn pause(&self, input: PauseInput) -> Result<PauseOutput, Box<dyn std::error::Error>>;
    async fn resume(&self, input: ResumeInput) -> Result<ResumeOutput, Box<dyn std::error::Error>>;
    async fn cancel(&self, input: CancelInput) -> Result<CancelOutput, Box<dyn std::error::Error>>;
    async fn get_status(&self, input: GetStatusInput) -> Result<GetStatusOutput, Box<dyn std::error::Error>>;
    async fn get_result(&self, input: GetResultInput) -> Result<GetResultOutput, Box<dyn std::error::Error>>;
    async fn list_executions(&self, input: ListExecutionsInput) -> Result<ListExecutionsOutput, Box<dyn std::error::Error>>;
    async fn signal(&self, input: SignalInput) -> Result<SignalOutput, Box<dyn std::error::Error>>;
    async fn wait_for_event(&self, input: WaitForEventInput) -> Result<WaitForEventOutput, Box<dyn std::error::Error>>;
    async fn history(&self, input: HistoryInput) -> Result<HistoryOutput, Box<dyn std::error::Error>>;
    async fn retry_step(&self, input: RetryStepInput) -> Result<RetryStepOutput, Box<dyn std::error::Error>>;
    async fn skip_step(&self, input: SkipStepInput) -> Result<SkipStepOutput, Box<dyn std::error::Error>>;
    async fn terminate_all(&self, input: TerminateAllInput) -> Result<TerminateAllOutput, Box<dyn std::error::Error>>;
}
