// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Permission {
    pub method: String,
    pub description: Option<String>,
    pub resource: String,
}

impl Permission {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Role {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_system: bool,
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
    pub permission_id: String,
    pub role_id: String,
}

impl RolePermission {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserRole {
    #[serde(default = "chrono::Utc::now")]
    pub assigned_at: chrono::DateTime<chrono::Utc>,
    pub assigned_by: Option<String>,
    pub role_id: String,
    pub user_id: String,
}

impl UserRole {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

