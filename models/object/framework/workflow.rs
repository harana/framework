// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Workflow {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub trigger_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
}

impl Workflow {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowStep {
    pub method: String,
    pub condition: Option<String>,
    pub on_failure: String,
    pub retry_count: i64,
    pub sort_order: i64,
    pub timeout_seconds: i64,
    pub workflow_id: String,
}

impl WorkflowStep {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowExecution {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub input: Option<String>,
    pub output: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub triggered_by: Option<String>,
    pub workflow_id: String,
}

impl WorkflowExecution {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowStepExecution {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error_message: Option<String>,
    pub input: Option<String>,
    pub output: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub step_id: String,
}

impl WorkflowStepExecution {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowContext {
    pub values: std::collections::HashMap<String, String>,
}

impl WorkflowContext {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowExecutionInfo {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub execution_id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub workflow_id: String,
}

impl WorkflowExecutionInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowHistoryEvent {
    pub data: String,
    pub event_id: String,
    pub event_type: String,
    pub step_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl WorkflowHistoryEvent {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowSignal {
    pub payload: String,
    #[serde(default = "chrono::Utc::now")]
    pub received_at: chrono::DateTime<chrono::Utc>,
    pub signal_name: String,
}

impl WorkflowSignal {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowRuntimeExecution {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub context: std::collections::HashMap<String, String>,
    pub current_step: String,
    pub error: Option<String>,
    pub execution_id: String,
    pub input: String,
    pub output: String,
    pub pause_reason: Option<String>,
    pub progress: f64,
    #[serde(default = "chrono::Utc::now")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub workflow_id: String,
}

impl WorkflowRuntimeExecution {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowCancelOutput {
    pub success: bool,
}

impl WorkflowCancelOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowGetResultOutput {
    pub completed: bool,
    pub error: Option<String>,
    pub output: Option<String>,
}

impl WorkflowGetResultOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowGetStatusOutput {
    pub completed_at: Option<String>,
    pub current_step: String,
    pub error: Option<String>,
    pub progress: f64,
    pub started_at: String,
    pub status: String,
}

impl WorkflowGetStatusOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowHistoryOutput {
    pub events: String,
    pub total: i64,
}

impl WorkflowHistoryOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowListExecutionsOutput {
    pub executions: String,
    pub total: i64,
}

impl WorkflowListExecutionsOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowPauseOutput {
    pub success: bool,
}

impl WorkflowPauseOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowResumeOutput {
    pub success: bool,
}

impl WorkflowResumeOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowRetryStepOutput {
    pub success: bool,
}

impl WorkflowRetryStepOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowSignalOutput {
    pub success: bool,
}

impl WorkflowSignalOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowSkipStepOutput {
    pub success: bool,
}

impl WorkflowSkipStepOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowStartOutput {
    pub execution_id: String,
    pub status: String,
    pub success: bool,
}

impl WorkflowStartOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowTerminateAllOutput {
    pub success: bool,
    pub terminated_count: i64,
}

impl WorkflowTerminateAllOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowWaitForEventOutput {
    pub payload: Option<String>,
    pub received: bool,
    pub timed_out: bool,
}

impl WorkflowWaitForEventOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

