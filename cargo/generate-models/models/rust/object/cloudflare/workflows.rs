// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// cf_workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkflow {
    pub account_id: String,
    pub binding: String,
    pub class_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub script_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub workflow_name: String,
}

impl CfWorkflow {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cf_workflow_instance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkflowInstance {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub error: Option<String>,
    pub instance_id: String,
    pub output: Option<String>,
    pub params: Option<String>,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Reference: cf_workflow.id
    pub workflow_id: String,
}

impl CfWorkflowInstance {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cf_workflow_step
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkflowStep {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: cf_workflow_instance.id
    pub instance_id: String,
    pub result: Option<String>,
    pub retries_backoff: String,
    pub retries_delay: Option<String>,
    pub retries_limit: Option<i64>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub step_name: String,
    pub step_type: String,
    pub timeout: Option<String>,
}

impl CfWorkflowStep {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cf_workflow_event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfWorkflowEvent {
    pub event_type: String,
    /// Reference: cf_workflow_instance.id
    pub instance_id: String,
    pub payload: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub sent_at: chrono::DateTime<chrono::Utc>,
}

impl CfWorkflowEvent {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

