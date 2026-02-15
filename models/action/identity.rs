// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthenticateInput {
    pub mfa_code: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthenticateOutput {
    pub access_token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub refresh_token: String,
    pub success: bool,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyTokenInput {
    pub token: String,
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
pub struct RefreshTokenInput {
    pub refresh_token: String,
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
pub struct RevokeTokenInput {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RevokeTokenOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetUserInput {
    pub user_id: String,
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
pub struct CreateUserInput {
    pub email: String,
    pub metadata: String,
    pub password: String,
    pub roles: Vec<String>,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateUserOutput {
    pub success: bool,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateUserInput {
    pub email: String,
    pub metadata: String,
    pub roles: Vec<String>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateUserOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteUserInput {
    #[serde(default)]
    pub hard_delete: bool,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteUserOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangePasswordInput {
    pub current_password: String,
    pub new_password: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChangePasswordOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestPasswordResetInput {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestPasswordResetOutput {
    pub reset_token: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResetPasswordInput {
    pub new_password: String,
    pub reset_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResetPasswordOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListUsersInput {
    pub limit: i64,
    pub offset: i64,
    pub roles: Vec<String>,
    pub search: String,
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

#[async_trait]
pub trait IdentityAction {
    async fn authenticate(&self, input: AuthenticateInput) -> Result<AuthenticateOutput, Box<dyn std::error::Error>>;
    async fn verify_token(&self, input: VerifyTokenInput) -> Result<VerifyTokenOutput, Box<dyn std::error::Error>>;
    async fn refresh_token(&self, input: RefreshTokenInput) -> Result<RefreshTokenOutput, Box<dyn std::error::Error>>;
    async fn revoke_token(&self, input: RevokeTokenInput) -> Result<RevokeTokenOutput, Box<dyn std::error::Error>>;
    async fn get_user(&self, input: GetUserInput) -> Result<GetUserOutput, Box<dyn std::error::Error>>;
    async fn create_user(&self, input: CreateUserInput) -> Result<CreateUserOutput, Box<dyn std::error::Error>>;
    async fn update_user(&self, input: UpdateUserInput) -> Result<UpdateUserOutput, Box<dyn std::error::Error>>;
    async fn delete_user(&self, input: DeleteUserInput) -> Result<DeleteUserOutput, Box<dyn std::error::Error>>;
    async fn change_password(&self, input: ChangePasswordInput) -> Result<ChangePasswordOutput, Box<dyn std::error::Error>>;
    async fn request_password_reset(&self, input: RequestPasswordResetInput) -> Result<RequestPasswordResetOutput, Box<dyn std::error::Error>>;
    async fn reset_password(&self, input: ResetPasswordInput) -> Result<ResetPasswordOutput, Box<dyn std::error::Error>>;
    async fn list_users(&self, input: ListUsersInput) -> Result<ListUsersOutput, Box<dyn std::error::Error>>;
}
