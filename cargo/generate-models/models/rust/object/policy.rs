// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Policy {
    pub actions: String,
    pub conditions: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub effect: String,
    #[serde(default)]
    pub is_active: bool,
    pub name: String,
    pub resources: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Policy {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// policy_attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyAttachment {
    #[serde(default = "chrono::Utc::now")]
    pub attached_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub attached_by: Option<String>,
    pub entity_id: String,
    pub entity_type: String,
    /// Reference: policy.id
    pub policy_id: String,
}

impl PolicyAttachment {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// policy_evaluation_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyEvaluationLog {
    pub action: String,
    #[serde(default)]
    pub allowed: bool,
    #[serde(default = "chrono::Utc::now")]
    pub evaluated_at: chrono::DateTime<chrono::Utc>,
    /// Reference: policy.id
    pub policy_id: String,
    pub reason: Option<String>,
    pub resource: String,
    /// Reference: user.id
    pub user_id: Option<String>,
}

impl PolicyEvaluationLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

