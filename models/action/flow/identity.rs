// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthenticateOutput {
    pub access_token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub refresh_token: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyTokenOutput {
    pub claims: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RefreshTokenOutput {
    pub access_token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetUserOutput {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub email: String,
    pub metadata: String,
    pub roles: Vec<String>,
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListUsersOutput {
    pub total: i64,
    pub users: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub metadata: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_login: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TokenClaims {
    pub claims: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserMetadata {
    pub values: std::collections::HashMap<String, String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Group {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub id: String,
    #[serde(default)]
    pub is_active: bool,
    pub name: String,
    pub parent_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupMember {
    #[serde(default = "chrono::Utc::now")]
    pub added_at: chrono::DateTime<chrono::Utc>,
    pub added_by: String,
    pub group_id: String,
    #[serde(default)]
    pub is_admin: bool,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupRole {
    #[serde(default = "chrono::Utc::now")]
    pub assigned_at: chrono::DateTime<chrono::Utc>,
    pub assigned_by: String,
    pub group_id: String,
    pub role_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Role {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub id: String,
    #[serde(default)]
    pub is_system: bool,
    pub name: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RolePermission {
    #[serde(default = "chrono::Utc::now")]
    pub granted_at: chrono::DateTime<chrono::Utc>,
    pub granted_by: String,
    pub permission_id: String,
    pub role_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserRole {
    #[serde(default = "chrono::Utc::now")]
    pub assigned_at: chrono::DateTime<chrono::Utc>,
    pub assigned_by: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub role_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserProfile {
    pub bio: String,
    pub date_of_birth: chrono::DateTime<chrono::Utc>,
    pub job_title: String,
    pub location: String,
    pub organisation: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
    pub website: String,
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

#[async_trait]
pub trait IdentityAction {
    async fn authenticate(&self, mfa_code: String, password: String, username: String) -> Result<AuthenticateOutput, Box<dyn std::error::Error>>;
    async fn verify_token(&self, token: String) -> Result<VerifyTokenOutput, Box<dyn std::error::Error>>;
    async fn refresh_token(&self, refresh_token: String) -> Result<RefreshTokenOutput, Box<dyn std::error::Error>>;
    async fn revoke_token(&self, token: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_user(&self, user_id: String) -> Result<GetUserOutput, Box<dyn std::error::Error>>;
    async fn create_user(&self, email: String, metadata: String, password: String, roles: Vec<String>, username: String, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn update_user(&self, email: String, metadata: String, roles: Vec<String>, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_user(&self, hard_delete: bool, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn change_password(&self, current_password: String, new_password: String, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn request_password_reset(&self, email: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn reset_password(&self, new_password: String, reset_token: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_users(&self, limit: i64, offset: i64, roles: Vec<String>, search: String) -> Result<ListUsersOutput, Box<dyn std::error::Error>>;
}
