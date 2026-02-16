// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSecretOutput {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub value: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListSecretsOutput {
    pub secrets: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotateSecretOutput {
    pub new_version: String,
    pub old_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSecretMetadataOutput {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListSecretVersionsOutput {
    pub total: i64,
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Secret {
    pub name: String,
    pub value: String,
    pub version: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecretInfo {
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecretVersion {
    pub version: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecretAccessLog {
    #[serde(default = "chrono::Utc::now")]
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    pub method: String,
    pub ip_address: String,
    pub secret_id: String,
    pub user_id: String,
}

#[async_trait]
pub trait SecretAction {
    async fn get_secret(&self, name: String, version: String) -> Result<GetSecretOutput, Box<dyn std::error::Error>>;
    async fn set_secret(&self, description: String, expires_at: chrono::DateTime<chrono::Utc>, name: String, value: String, version: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_secret(&self, force: bool, name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_secrets(&self, limit: i64, prefix: String) -> Result<ListSecretsOutput, Box<dyn std::error::Error>>;
    async fn rotate_secret(&self, name: String, new_value: String) -> Result<RotateSecretOutput, Box<dyn std::error::Error>>;
    async fn get_secret_metadata(&self, name: String) -> Result<GetSecretMetadataOutput, Box<dyn std::error::Error>>;
    async fn secret_exists(&self, name: String) -> Result<bool, Box<dyn std::error::Error>>;
    async fn list_secret_versions(&self, limit: i64, name: String) -> Result<ListSecretVersionsOutput, Box<dyn std::error::Error>>;
    async fn restore_secret_version(&self, name: String, version: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn generate_secret(&self, charset: String, description: String, length: i64, name: String, value: String, version: String) -> Result<(), Box<dyn std::error::Error>>;
}
