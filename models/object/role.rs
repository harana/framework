// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Role {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_system: bool,
    pub name: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Role {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RolePermission {
    pub action: String,
    pub conditions: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub granted_at: chrono::DateTime<chrono::Utc>,
    pub granted_by: Option<String>,
    pub resource: String,
    pub role_id: String,
}

impl RolePermission {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RoleAssignment {
    #[serde(default = "chrono::Utc::now")]
    pub assigned_at: chrono::DateTime<chrono::Utc>,
    pub assigned_by: Option<String>,
    pub entity_id: String,
    pub entity_type: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub role_id: String,
}

impl RoleAssignment {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RoleInfo {
    pub role_id: String,
    pub name: String,
    pub description: String,
    pub user_count: i64,
}

impl RoleInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RolePermissionConditions {
    pub conditions: std::collections::HashMap<String, String>,
}

impl RolePermissionConditions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

