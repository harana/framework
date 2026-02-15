// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyAttachment {
    #[serde(default = "chrono::Utc::now")]
    pub attached_at: chrono::DateTime<chrono::Utc>,
    pub attached_by: Option<String>,
    pub entity_id: String,
    pub entity_type: String,
    pub policy_id: String,
}

impl PolicyAttachment {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyEvaluationLog {
    pub action: String,
    #[serde(default)]
    pub allowed: bool,
    #[serde(default = "chrono::Utc::now")]
    pub evaluated_at: chrono::DateTime<chrono::Utc>,
    pub policy_id: String,
    pub reason: Option<String>,
    pub resource: String,
    pub user_id: Option<String>,
}

impl PolicyEvaluationLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyConditions {
    pub conditions: std::collections::HashMap<String, String>,
}

impl PolicyConditions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyInfo {
    pub policy_id: String,
    pub name: String,
    pub description: String,
    pub effect: String,
}

impl PolicyInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyEvaluationContext {
    pub values: std::collections::HashMap<String, String>,
}

impl PolicyEvaluationContext {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyEvaluatedConditions {
    pub results: std::collections::HashMap<String, String>,
}

impl PolicyEvaluatedConditions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

