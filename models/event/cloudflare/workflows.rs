// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkflowInstanceCreated {
    pub binding: String,
    pub instance_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkflowInstanceCreated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkflowInstanceBatchCreated {
    pub binding: String,
    pub instance_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkflowInstanceBatchCreated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkflowInstanceStatusChanged {
    pub binding: String,
    pub instance_id: String,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkflowInstanceStatusChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkflowInstancePaused {
    pub binding: String,
    pub instance_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub paused_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkflowInstancePaused {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkflowInstanceResumed {
    pub binding: String,
    pub instance_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub resumed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkflowInstanceResumed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkflowInstanceRestarted {
    pub binding: String,
    pub instance_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub restarted_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkflowInstanceRestarted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkflowInstanceTerminated {
    pub binding: String,
    pub instance_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub terminated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkflowInstanceTerminated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkflowInstanceCompleted {
    pub binding: String,
    pub instance_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkflowInstanceCompleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkflowInstanceErrored {
    pub binding: String,
    pub instance_id: String,
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub errored_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkflowInstanceErrored {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkflowEventSent {
    pub binding: String,
    pub instance_id: String,
    pub event_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub sent_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkflowEventSent {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkflowStepExecuted {
    pub step_name: String,
    pub retries_limit: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkflowStepExecuted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkflowStepFailed {
    pub step_name: String,
    pub error_message: Option<String>,
    pub retry_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub failed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkflowStepFailed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

