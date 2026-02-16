// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRepositoryOutput {
    pub registry_id: String,
    pub repository_arn: String,
    pub repository_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListImagesOutput {
    pub image_ids: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchDeleteImageOutput {
    pub failures: Vec<String>,
    pub image_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAuthTokenOutput {
    pub authorization_token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub proxy_endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetRepositoryPolicyOutput {
    pub policy_text: String,
    pub registry_id: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetRepositoryPolicyOutput {
    pub policy_text: String,
    pub registry_id: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartImageScanOutput {
    pub image_id: String,
    pub scan_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeScanFindingsOutput {
    pub findings: Vec<String>,
    pub image_scan_status: String,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutLifecyclePolicyOutput {
    pub lifecycle_policy_text: String,
    pub registry_id: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetLifecyclePolicyOutput {
    pub last_evaluated_at: chrono::DateTime<chrono::Utc>,
    pub lifecycle_policy_text: String,
    pub registry_id: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcrRepository {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub encryption_type: String,
    pub image_tag_mutability: String,
    pub kms_key: String,
    pub region: String,
    pub registry_id: String,
    pub repository_arn: String,
    pub repository_name: String,
    pub repository_uri: String,
    #[serde(default)]
    pub scan_on_push: bool,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcrImage {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub image_digest: String,
    pub image_manifest_media_type: String,
    pub image_pushed_at: chrono::DateTime<chrono::Utc>,
    pub image_size_bytes: i64,
    pub image_tags: String,
    pub last_scan_at: chrono::DateTime<chrono::Utc>,
    pub repository_id: String,
    pub scan_status: String,
    pub vulnerability_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcrRepositoryPolicy {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub policy_text: String,
    pub repository_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcrLifecyclePolicy {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_evaluated_at: chrono::DateTime<chrono::Utc>,
    pub lifecycle_policy_text: String,
    pub repository_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcrImageScanFinding {
    pub description: String,
    pub image_id: String,
    pub name: String,
    pub severity: String,
    pub uri: String,
}

#[async_trait]
pub trait EcrAction {
    async fn create_repository(&self, encryption_type: String, image_tag_mutability: String, kms_key: String, region: String, repository_name: String, scan_on_push: bool, tags: String) -> Result<CreateRepositoryOutput, Box<dyn std::error::Error>>;
    async fn delete_repository(&self, force: bool, region: String, repository_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_repositories(&self, region: String, repository_names: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn list_images(&self, filter: String, max_results: i64, region: String, repository_name: String) -> Result<ListImagesOutput, Box<dyn std::error::Error>>;
    async fn describe_images(&self, image_ids: Vec<String>, region: String, repository_name: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn put_image(&self, image_manifest: String, image_tag: String, region: String, repository_name: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn batch_delete_image(&self, image_ids: Vec<String>, region: String, repository_name: String) -> Result<BatchDeleteImageOutput, Box<dyn std::error::Error>>;
    async fn get_auth_token(&self, region: String, registry_ids: Vec<String>) -> Result<GetAuthTokenOutput, Box<dyn std::error::Error>>;
    async fn get_download_url(&self, image_digest: String, region: String, repository_name: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn tag_image(&self, image_digest: String, image_tag: String, region: String, repository_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn set_repository_policy(&self, force: bool, policy_text: String, region: String, repository_name: String) -> Result<SetRepositoryPolicyOutput, Box<dyn std::error::Error>>;
    async fn get_repository_policy(&self, region: String, repository_name: String) -> Result<GetRepositoryPolicyOutput, Box<dyn std::error::Error>>;
    async fn delete_repository_policy(&self, region: String, repository_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn start_image_scan(&self, image_id: String, region: String, repository_name: String) -> Result<StartImageScanOutput, Box<dyn std::error::Error>>;
    async fn describe_scan_findings(&self, image_id: String, max_results: i64, region: String, repository_name: String) -> Result<DescribeScanFindingsOutput, Box<dyn std::error::Error>>;
    async fn put_lifecycle_policy(&self, lifecycle_policy_text: String, region: String, repository_name: String) -> Result<PutLifecyclePolicyOutput, Box<dyn std::error::Error>>;
    async fn get_lifecycle_policy(&self, region: String, repository_name: String) -> Result<GetLifecyclePolicyOutput, Box<dyn std::error::Error>>;
}
