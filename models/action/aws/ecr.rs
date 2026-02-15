// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRepositoryInput {
    pub encryption_type: String,
    pub image_tag_mutability: String,
    pub kms_key: String,
    pub region: String,
    pub repository_name: String,
    #[serde(default)]
    pub scan_on_push: bool,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRepositoryOutput {
    pub registry_id: String,
    pub repository_arn: String,
    pub repository_uri: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRepositoryInput {
    #[serde(default)]
    pub force: bool,
    pub region: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRepositoryOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeRepositoriesInput {
    pub region: String,
    pub repository_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeRepositoriesOutput {
    pub repositories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListImagesInput {
    pub filter: String,
    pub max_results: i64,
    pub region: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListImagesOutput {
    pub image_ids: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeImagesInput {
    pub image_ids: Vec<String>,
    pub region: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeImagesOutput {
    pub image_details: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutImageInput {
    pub image_manifest: String,
    pub image_tag: String,
    pub region: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutImageOutput {
    pub image: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchDeleteImageInput {
    pub image_ids: Vec<String>,
    pub region: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BatchDeleteImageOutput {
    pub failures: Vec<String>,
    pub image_ids: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAuthTokenInput {
    pub region: String,
    pub registry_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAuthTokenOutput {
    pub authorization_token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub proxy_endpoint: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDownloadUrlInput {
    pub image_digest: String,
    pub region: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDownloadUrlOutput {
    pub download_url: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagImageInput {
    pub image_digest: String,
    pub image_tag: String,
    pub region: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagImageOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetRepositoryPolicyInput {
    #[serde(default)]
    pub force: bool,
    pub policy_text: String,
    pub region: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetRepositoryPolicyOutput {
    pub policy_text: String,
    pub registry_id: String,
    pub repository_name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetRepositoryPolicyInput {
    pub region: String,
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
pub struct DeleteRepositoryPolicyInput {
    pub region: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRepositoryPolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartImageScanInput {
    pub image_id: String,
    pub region: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartImageScanOutput {
    pub image_id: String,
    pub scan_status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeScanFindingsInput {
    pub image_id: String,
    pub max_results: i64,
    pub region: String,
    pub repository_name: String,
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
pub struct PutLifecyclePolicyInput {
    pub lifecycle_policy_text: String,
    pub region: String,
    pub repository_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutLifecyclePolicyOutput {
    pub lifecycle_policy_text: String,
    pub registry_id: String,
    pub repository_name: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetLifecyclePolicyInput {
    pub region: String,
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

#[async_trait]
pub trait EcrAction {
    async fn create_repository(&self, input: CreateRepositoryInput) -> Result<CreateRepositoryOutput, Box<dyn std::error::Error>>;
    async fn delete_repository(&self, input: DeleteRepositoryInput) -> Result<DeleteRepositoryOutput, Box<dyn std::error::Error>>;
    async fn describe_repositories(&self, input: DescribeRepositoriesInput) -> Result<DescribeRepositoriesOutput, Box<dyn std::error::Error>>;
    async fn list_images(&self, input: ListImagesInput) -> Result<ListImagesOutput, Box<dyn std::error::Error>>;
    async fn describe_images(&self, input: DescribeImagesInput) -> Result<DescribeImagesOutput, Box<dyn std::error::Error>>;
    async fn put_image(&self, input: PutImageInput) -> Result<PutImageOutput, Box<dyn std::error::Error>>;
    async fn batch_delete_image(&self, input: BatchDeleteImageInput) -> Result<BatchDeleteImageOutput, Box<dyn std::error::Error>>;
    async fn get_auth_token(&self, input: GetAuthTokenInput) -> Result<GetAuthTokenOutput, Box<dyn std::error::Error>>;
    async fn get_download_url(&self, input: GetDownloadUrlInput) -> Result<GetDownloadUrlOutput, Box<dyn std::error::Error>>;
    async fn tag_image(&self, input: TagImageInput) -> Result<TagImageOutput, Box<dyn std::error::Error>>;
    async fn set_repository_policy(&self, input: SetRepositoryPolicyInput) -> Result<SetRepositoryPolicyOutput, Box<dyn std::error::Error>>;
    async fn get_repository_policy(&self, input: GetRepositoryPolicyInput) -> Result<GetRepositoryPolicyOutput, Box<dyn std::error::Error>>;
    async fn delete_repository_policy(&self, input: DeleteRepositoryPolicyInput) -> Result<DeleteRepositoryPolicyOutput, Box<dyn std::error::Error>>;
    async fn start_image_scan(&self, input: StartImageScanInput) -> Result<StartImageScanOutput, Box<dyn std::error::Error>>;
    async fn describe_scan_findings(&self, input: DescribeScanFindingsInput) -> Result<DescribeScanFindingsOutput, Box<dyn std::error::Error>>;
    async fn put_lifecycle_policy(&self, input: PutLifecyclePolicyInput) -> Result<PutLifecyclePolicyOutput, Box<dyn std::error::Error>>;
    async fn get_lifecycle_policy(&self, input: GetLifecyclePolicyInput) -> Result<GetLifecyclePolicyOutput, Box<dyn std::error::Error>>;
}
