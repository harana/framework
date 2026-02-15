// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Group {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub name: String,
    pub parent_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Group {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupMember {
    #[serde(default = "chrono::Utc::now")]
    pub added_at: chrono::DateTime<chrono::Utc>,
    pub added_by: Option<String>,
    pub group_id: String,
    #[serde(default)]
    pub is_admin: bool,
    pub user_id: String,
}

impl GroupMember {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupRole {
    #[serde(default = "chrono::Utc::now")]
    pub assigned_at: chrono::DateTime<chrono::Utc>,
    pub assigned_by: Option<String>,
    pub group_id: String,
    pub role_id: String,
}

impl GroupRole {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

