// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPermission {
    pub action: String,
    pub conditions: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub granted_at: chrono::DateTime<chrono::Utc>,
    pub granted_by: Option<String>,
    pub resource: String,
    pub user_id: String,
}

impl UserPermission {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EffectivePermission {
    pub action: String,
    #[serde(default)]
    pub allowed: bool,
    pub resource: String,
    pub source_id: Option<String>,
    pub source_type: String,
    pub user_id: String,
}

impl EffectivePermission {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPermissionConditions {
    pub conditions: std::collections::HashMap<String, String>,
}

impl UserPermissionConditions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPermissionInfo {
    pub permission_id: String,
    pub method: String,
    pub resource: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl UserPermissionInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserRoleInfo {
    pub role_id: String,
    pub name: String,
    pub assigned_at: chrono::DateTime<chrono::Utc>,
}

impl UserRoleInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PermissionSource {
    pub source_type: String,
    pub source_id: String,
    pub name: String,
}

impl PermissionSource {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

