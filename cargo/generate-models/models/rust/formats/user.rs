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

/// user_permission_conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPermissionConditions {
    pub conditions: String,
}

impl UserPermissionConditions {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// user_permission_info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPermissionInfo {
    pub permission_id: String,
    pub method: String,
    pub resource: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl UserPermissionInfo {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// user_role_info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserRoleInfo {
    pub role_id: String,
    pub name: String,
    pub assigned_at: chrono::DateTime<chrono::Utc>,
}

impl UserRoleInfo {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// permission_source
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PermissionSource {
    pub source_type: String,
    pub source_id: String,
    pub name: String,
}

impl PermissionSource {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

