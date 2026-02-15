// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// user_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPermission {
    pub action: String,
    pub conditions: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub granted_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub granted_by: Option<String>,
    pub resource: String,
    /// Reference: user.id
    pub user_id: String,
}

impl UserPermission {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// effective_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EffectivePermission {
    pub action: String,
    #[serde(default)]
    pub allowed: bool,
    pub resource: String,
    pub source_id: Option<String>,
    pub source_type: String,
    /// Reference: user.id
    pub user_id: String,
}

impl EffectivePermission {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

