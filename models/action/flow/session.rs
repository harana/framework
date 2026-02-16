// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOutput {
    pub expires_at: chrono::DateTime<chrono::Utc>,
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
pub struct GetValueOutput {
    pub found: bool,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListUsersOutput {
    pub sessions: Vec<String>,
    pub total: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RefreshToken {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_revoked: bool,
    pub revoked_at: chrono::DateTime<chrono::Utc>,
    pub session_id: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SessionActivity {
    pub method: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub ip_address: String,
    pub metadata: String,
    pub session_id: String,
}

#[async_trait]
pub trait SessionAction {
    async fn create(&self, data: String, ttl: i64, user_id: String) -> Result<CreateOutput, Box<dyn std::error::Error>>;
    async fn get(&self, session_id: String) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn update(&self, data: String, merge: bool, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn destroy(&self, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn refresh(&self, session_id: String, ttl: i64) -> Result<chrono::DateTime<chrono::Utc>, Box<dyn std::error::Error>>;
    async fn get_value(&self, key: String, session_id: String) -> Result<GetValueOutput, Box<dyn std::error::Error>>;
    async fn set_value(&self, key: String, session_id: String, value: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_value(&self, key: String, session_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_users(&self, user_id: String) -> Result<ListUsersOutput, Box<dyn std::error::Error>>;
    async fn destroy_users(&self, except_id: String, user_id: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn validate(&self, session_id: String) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
}
