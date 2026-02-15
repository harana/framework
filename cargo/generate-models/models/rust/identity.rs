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
    pub id: String,
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

/// role
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// role_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RolePermission {
    #[serde(default = "chrono::Utc::now")]
    pub granted_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub granted_by: Option<String>,
    /// Reference: permission.id
    pub permission_id: String,
    /// Reference: role.id
    pub role_id: String,
}

impl RolePermission {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// user_role
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserRole {
    #[serde(default = "chrono::Utc::now")]
    pub assigned_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub assigned_by: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Reference: role.id
    pub role_id: String,
    /// Reference: user.id
    pub user_id: String,
}

impl UserRole {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// user
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// user_profile
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
    /// Reference: user.id
    pub user_id: String,
    pub website: Option<String>,
}

impl UserProfile {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// user_credential
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
    /// Reference: user.id
    pub user_id: String,
}

impl UserCredential {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// token_claims
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TokenClaims {
    pub claims: String,
}

impl TokenClaims {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// user_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserMetadata {
    pub values: String,
}

impl UserMetadata {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// user_info
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

