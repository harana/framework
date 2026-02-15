// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Group {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub id: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Role {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub id: String,
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
    #[serde(default = "chrono::Utc::now")]
    pub granted_at: chrono::DateTime<chrono::Utc>,
    pub granted_by: Option<String>,
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
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub role_id: String,
    pub user_id: String,
}

impl UserRole {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct User {
    pub avatar_url: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub email: String,
    pub first_name: Option<String>,
    pub id: String,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default)]
    pub is_verified: bool,
    pub last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub timezone: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub username: String,
}

impl User {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserProfile {
    pub bio: Option<String>,
    pub date_of_birth: Option<chrono::DateTime<chrono::Utc>>,
    pub job_title: Option<String>,
    pub location: Option<String>,
    pub organisation: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
    pub website: Option<String>,
}

impl UserProfile {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserCredential {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub credential_type: String,
    pub credential_value: String,
    #[serde(default)]
    pub is_primary: bool,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

impl UserCredential {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TokenClaims {
    pub claims: std::collections::HashMap<String, String>,
}

impl TokenClaims {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserMetadata {
    pub values: std::collections::HashMap<String, String>,
}

impl UserMetadata {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserInfo {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl UserInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

