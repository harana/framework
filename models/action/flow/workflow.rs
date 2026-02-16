// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartOutput {
    pub execution_id: String,
    pub status: String,
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
pub struct GetResultOutput {
    pub completed: bool,
    pub error: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListExecutionsOutput {
    pub executions: Vec<String>,
    pub total: i64,
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
pub struct HistoryOutput {
    pub events: Vec<String>,
    pub total: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Workflow {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    #[serde(default)]
    pub is_active: bool,
    pub trigger_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowStep {
    pub method: String,
    pub condition: String,
    pub on_failure: String,
    pub retry_count: i64,
    pub sort_order: i64,
    pub timeout_seconds: i64,
    pub workflow_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowStepExecution {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub error_message: String,
    pub input: String,
    pub output: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub step_id: String,
}

#[async_trait]
pub trait WorkflowAction {
    async fn start(&self, context: String, input: String, workflow_id: String) -> Result<StartOutput, Box<dyn std::error::Error>>;
    async fn pause(&self, execution_id: String, reason: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn resume(&self, execution_id: String, input: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn cancel(&self, execution_id: String, reason: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_status(&self, execution_id: String) -> Result<GetStatusOutput, Box<dyn std::error::Error>>;
    async fn get_result(&self, execution_id: String) -> Result<GetResultOutput, Box<dyn std::error::Error>>;
    async fn list_executions(&self, end_date: chrono::DateTime<chrono::Utc>, limit: i64, offset: i64, start_date: chrono::DateTime<chrono::Utc>, status: String, workflow_id: String) -> Result<ListExecutionsOutput, Box<dyn std::error::Error>>;
    async fn signal(&self, execution_id: String, payload: String, signal_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn wait_for_event(&self, event_name: String, execution_id: String, timeout: i64) -> Result<WaitForEventOutput, Box<dyn std::error::Error>>;
    async fn history(&self, execution_id: String) -> Result<HistoryOutput, Box<dyn std::error::Error>>;
    async fn retry_step(&self, execution_id: String, step_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn skip_step(&self, execution_id: String, output: String, step_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn terminate_all(&self, reason: String, workflow_id: String, terminated_count: i64) -> Result<(), Box<dyn std::error::Error>>;
}
