// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// workflow
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// workflow_step
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowStep {
    pub method: String,
    pub condition: Option<String>,
    pub on_failure: String,
    pub retry_count: i64,
    pub sort_order: i64,
    pub timeout_seconds: i64,
    /// Reference: workflow.id
    pub workflow_id: String,
}

impl WorkflowStep {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// workflow_execution
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
    /// Reference: user.id
    pub triggered_by: Option<String>,
    /// Reference: workflow.id
    pub workflow_id: String,
}

impl WorkflowExecution {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// workflow_step_execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowStepExecution {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error_message: Option<String>,
    pub input: Option<String>,
    pub output: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    /// Reference: workflow_step.id
    pub step_id: String,
}

impl WorkflowStepExecution {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// workflow_context
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowContext {
    pub values: String,
}

impl WorkflowContext {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// workflow_execution_info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowExecutionInfo {
    pub execution_id: String,
    pub workflow_id: String,
    pub status: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

impl WorkflowExecutionInfo {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// workflow_history_event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowHistoryEvent {
    pub event_id: String,
    pub event_type: String,
    pub step_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: String,
}

impl WorkflowHistoryEvent {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

