// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Group {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub name: String,
    /// Reference: group.id
    pub parent_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Group {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// group_member
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupMember {
    #[serde(default = "chrono::Utc::now")]
    pub added_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub added_by: Option<String>,
    /// Reference: group.id
    pub group_id: String,
    #[serde(default)]
    pub is_admin: bool,
    /// Reference: user.id
    pub user_id: String,
}

impl GroupMember {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// group_role
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupRole {
    #[serde(default = "chrono::Utc::now")]
    pub assigned_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub assigned_by: Option<String>,
    /// Reference: group.id
    pub group_id: String,
    /// Reference: role.id
    pub role_id: String,
}

impl GroupRole {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

