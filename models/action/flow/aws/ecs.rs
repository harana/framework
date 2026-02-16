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
pub struct AwsEcsCluster {
    pub account_id: String,
    pub active_services_count: i64,
    pub capacity_providers: String,
    pub cluster_arn: String,
    pub cluster_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub pending_tasks_count: i64,
    pub region: String,
    pub registered_container_instances_count: i64,
    pub running_tasks_count: i64,
    pub settings: String,
    pub status: String,
    pub tags: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcsService {
    pub cluster_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub desired_count: i64,
    pub launch_type: String,
    pub load_balancers: String,
    pub network_configuration: String,
    pub pending_count: i64,
    pub platform_version: String,
    pub running_count: i64,
    pub scheduling_strategy: String,
    pub service_arn: String,
    pub service_name: String,
    pub status: String,
    pub tags: String,
    pub task_definition: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcsTaskDefinition {
    pub account_id: String,
    pub container_definitions: String,
    pub cpu: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub execution_role_arn: String,
    pub family: String,
    pub memory: String,
    pub network_mode: String,
    pub region: String,
    pub requires_compatibilities: String,
    pub revision: i64,
    pub status: String,
    pub tags: String,
    pub task_definition_arn: String,
    pub task_role_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcsTask {
    pub cluster_id: String,
    pub connectivity: String,
    pub container_instance_arn: String,
    pub cpu: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub desired_status: String,
    pub group: String,
    pub last_status: String,
    pub launch_type: String,
    pub memory: String,
    pub platform_version: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub stopped_at: chrono::DateTime<chrono::Utc>,
    pub stopped_reason: String,
    pub tags: String,
    pub task_arn: String,
    pub task_definition_arn: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait EcsAction {
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
