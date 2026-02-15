// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSecretInput {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSecretOutput {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub value: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetSecretInput {
    pub description: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetSecretOutput {
    pub success: bool,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSecretInput {
    #[serde(default)]
    pub force: bool,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteSecretOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListSecretsInput {
    pub limit: i64,
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListSecretsOutput {
    pub secrets: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotateSecretInput {
    pub name: String,
    pub new_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotateSecretOutput {
    pub new_version: String,
    pub old_version: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSecretMetadataInput {
    pub name: String,
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
pub struct SecretExistsInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SecretExistsOutput {
    pub exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListSecretVersionsInput {
    pub limit: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListSecretVersionsOutput {
    pub total: i64,
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreSecretVersionInput {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreSecretVersionOutput {
    pub new_version: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateSecretInput {
    pub charset: String,
    pub description: String,
    pub length: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateSecretOutput {
    pub success: bool,
    pub value: String,
    pub version: String,
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

#[async_trait]
pub trait SecretAction {
    async fn get_secret(&self, input: GetSecretInput) -> Result<GetSecretOutput, Box<dyn std::error::Error>>;
    async fn set_secret(&self, input: SetSecretInput) -> Result<SetSecretOutput, Box<dyn std::error::Error>>;
    async fn delete_secret(&self, input: DeleteSecretInput) -> Result<DeleteSecretOutput, Box<dyn std::error::Error>>;
    async fn list_secrets(&self, input: ListSecretsInput) -> Result<ListSecretsOutput, Box<dyn std::error::Error>>;
    async fn rotate_secret(&self, input: RotateSecretInput) -> Result<RotateSecretOutput, Box<dyn std::error::Error>>;
    async fn get_secret_metadata(&self, input: GetSecretMetadataInput) -> Result<GetSecretMetadataOutput, Box<dyn std::error::Error>>;
    async fn secret_exists(&self, input: SecretExistsInput) -> Result<SecretExistsOutput, Box<dyn std::error::Error>>;
    async fn list_secret_versions(&self, input: ListSecretVersionsInput) -> Result<ListSecretVersionsOutput, Box<dyn std::error::Error>>;
    async fn restore_secret_version(&self, input: RestoreSecretVersionInput) -> Result<RestoreSecretVersionOutput, Box<dyn std::error::Error>>;
    async fn generate_secret(&self, input: GenerateSecretInput) -> Result<GenerateSecretOutput, Box<dyn std::error::Error>>;
}
