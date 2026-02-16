// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOutput {
    pub arn: String,
    pub name: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetValueOutput {
    pub arn: String,
    pub created_date: i64,
    pub name: String,
    pub secret_value: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateOutput {
    pub arn: String,
    pub name: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOutput {
    pub arn: String,
    pub deletion_date: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RestoreOutput {
    pub arn: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListsOutput {
    pub next_token: String,
    pub secrets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeOutput {
    pub arn: String,
    pub description: String,
    pub kms_key_id: String,
    pub last_changed_date: i64,
    pub last_rotated_date: i64,
    pub name: String,
    pub rotation_enabled: bool,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutValueOutput {
    pub arn: String,
    pub name: String,
    pub version_id: String,
    pub version_stages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotateOutput {
    pub arn: String,
    pub name: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelRotateOutput {
    pub arn: String,
    pub name: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplicateOutput {
    pub arn: String,
    pub replication_status: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveRegionsFromReplicationOutput {
    pub arn: String,
    pub replication_status: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub exists: bool,
    pub scheduled_for_deletion: bool,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSecretsManagerSecret {
    pub account_id: String,
    pub arn: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub kms_key_id: String,
    pub last_changed_at: chrono::DateTime<chrono::Utc>,
    pub last_rotated_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub region: String,
    #[serde(default)]
    pub rotation_enabled: bool,
    pub rotation_lambda_arn: String,
    pub rotation_rules: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSecretsManagerSecretVersion {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub secret_id: String,
    pub version_id: String,
    pub version_stages: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSecretsManagerSecretReplication {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub kms_key_id: String,
    pub last_accessed_at: chrono::DateTime<chrono::Utc>,
    pub region: String,
    pub secret_id: String,
    pub status: String,
    pub status_message: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSecretsManagerAccessLog {
    #[serde(default = "chrono::Utc::now")]
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    pub action: String,
    pub ip_address: String,
    pub secret_id: String,
    #[serde(default)]
    pub success: bool,
    pub user_arn: String,
}

#[async_trait]
pub trait SecretsAction {
    async fn create(&self, description: String, kms_key_id: String, name: String, secret_value: String, tags: String) -> Result<CreateOutput, Box<dyn std::error::Error>>;
    async fn get_value(&self, secret_id: String, version_id: String, version_stage: String) -> Result<GetValueOutput, Box<dyn std::error::Error>>;
    async fn update(&self, description: String, kms_key_id: String, secret_id: String, secret_value: String) -> Result<UpdateOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, force_delete: bool, recovery_window_days: i64, secret_id: String) -> Result<DeleteOutput, Box<dyn std::error::Error>>;
    async fn restore(&self, secret_id: String) -> Result<RestoreOutput, Box<dyn std::error::Error>>;
    async fn lists(&self, filters: Vec<String>, max_results: i64, sort_order: String) -> Result<ListsOutput, Box<dyn std::error::Error>>;
    async fn describe(&self, secret_id: String) -> Result<DescribeOutput, Box<dyn std::error::Error>>;
    async fn put_value(&self, secret_id: String, secret_value: String, version_stages: Vec<String>) -> Result<PutValueOutput, Box<dyn std::error::Error>>;
    async fn rotate(&self, rotation_lambda_arn: String, rotation_rules: String, secret_id: String) -> Result<RotateOutput, Box<dyn std::error::Error>>;
    async fn cancel_rotate(&self, secret_id: String, version_id: String) -> Result<CancelRotateOutput, Box<dyn std::error::Error>>;
    async fn tag(&self, secret_id: String, tags: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn untag(&self, secret_id: String, tag_keys: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn replicate(&self, kms_key_ids: String, regions: Vec<String>, secret_id: String) -> Result<ReplicateOutput, Box<dyn std::error::Error>>;
    async fn remove_regions_from_replication(&self, regions: Vec<String>, secret_id: String) -> Result<RemoveRegionsFromReplicationOutput, Box<dyn std::error::Error>>;
    async fn validate(&self, secret_id: String) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
}
