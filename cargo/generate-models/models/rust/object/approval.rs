// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// approval_request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApprovalRequest {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: Option<String>,
    pub priority: String,
    pub request_type: String,
    /// Reference: user.id
    pub requester_id: String,
    pub resource_id: String,
    pub resource_type: String,
    pub status: String,
    pub title: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ApprovalRequest {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// approval_approver
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApprovalApprover {
    /// Reference: user.id
    pub approver_id: String,
    pub comment: Option<String>,
    pub decided_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub is_required: bool,
    pub reason: Option<String>,
    /// Reference: approval_request.id
    pub request_id: String,
    pub status: String,
}

impl ApprovalApprover {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// approval_history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApprovalHistory {
    pub action: String,
    /// Reference: user.id
    pub actor_id: Option<String>,
    pub comment: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: approval_request.id
    pub request_id: String,
}

impl ApprovalHistory {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

