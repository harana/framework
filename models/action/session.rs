// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateInput {
    pub data: String,
    pub ttl: i64,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOutput {
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInput {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub data: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub found: bool,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateInput {
    pub data: String,
    #[serde(default)]
    pub merge: bool,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DestroyInput {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DestroyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RefreshInput {
    pub session_id: String,
    pub ttl: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RefreshOutput {
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetValueInput {
    pub key: String,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetValueOutput {
    pub found: bool,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetValueInput {
    pub key: String,
    pub session_id: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetValueOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteValueInput {
    pub key: String,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteValueOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListUsersInput {
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListUsersOutput {
    pub sessions: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DestroyUsersInput {
    pub except_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DestroyUsersOutput {
    pub destroyed_count: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateInput {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
    pub data: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SessionData {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SessionInfo {
    pub session_id: String,
    pub user_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait SessionAction {
    async fn create(&self, input: CreateInput) -> Result<CreateOutput, Box<dyn std::error::Error>>;
    async fn get(&self, input: GetInput) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn update(&self, input: UpdateInput) -> Result<UpdateOutput, Box<dyn std::error::Error>>;
    async fn destroy(&self, input: DestroyInput) -> Result<DestroyOutput, Box<dyn std::error::Error>>;
    async fn refresh(&self, input: RefreshInput) -> Result<RefreshOutput, Box<dyn std::error::Error>>;
    async fn get_value(&self, input: GetValueInput) -> Result<GetValueOutput, Box<dyn std::error::Error>>;
    async fn set_value(&self, input: SetValueInput) -> Result<SetValueOutput, Box<dyn std::error::Error>>;
    async fn delete_value(&self, input: DeleteValueInput) -> Result<DeleteValueOutput, Box<dyn std::error::Error>>;
    async fn list_users(&self, input: ListUsersInput) -> Result<ListUsersOutput, Box<dyn std::error::Error>>;
    async fn destroy_users(&self, input: DestroyUsersInput) -> Result<DestroyUsersOutput, Box<dyn std::error::Error>>;
    async fn validate(&self, input: ValidateInput) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
}
