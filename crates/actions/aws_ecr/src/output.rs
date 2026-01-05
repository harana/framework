// Harana Actions - AWS ECR Module Output Types
// Auto-generated output structs for AWS ECR action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// create_repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRepositoryOutput {
    pub registry_id: String,
    pub repository_arn: String,
    pub repository_uri: String,
    pub success: bool,
}

// delete_repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRepositoryOutput {
    pub success: bool,
}

// describe_repositories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeRepositoriesOutput {
    pub repositories: Vec<HashMap<String, Value>>,
}

// list_images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListImagesOutput {
    pub image_ids: Vec<HashMap<String, Value>>,
    pub next_token: String,
}

// describe_images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeImagesOutput {
    pub image_details: Vec<HashMap<String, Value>>,
}

// put_image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutImageOutput {
    pub image: HashMap<String, Value>,
    pub success: bool,
}

// batch_delete_image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteImageOutput {
    pub failures: Vec<HashMap<String, Value>>,
    pub image_ids: Vec<HashMap<String, Value>>,
    pub success: bool,
}

// get_auth_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthTokenOutput {
    pub authorization_token: String,
    pub expires_at: String, // datetime
    pub proxy_endpoint: String,
    pub success: bool,
}

// get_download_url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDownloadUrlOutput {
    pub download_url: String,
    pub success: bool,
}

// tag_image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagImageOutput {
    pub success: bool,
}

// set_repository_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetRepositoryPolicyOutput {
    pub policy_text: String,
    pub registry_id: String,
    pub repository_name: String,
    pub success: bool,
}

// get_repository_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRepositoryPolicyOutput {
    pub policy_text: String,
    pub registry_id: String,
    pub repository_name: String,
}

// delete_repository_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRepositoryPolicyOutput {
    pub success: bool,
}

// start_image_scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartImageScanOutput {
    pub image_id: HashMap<String, Value>,
    pub scan_status: String,
    pub success: bool,
}

// describe_scan_findings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeScanFindingsOutput {
    pub findings: Vec<HashMap<String, Value>>,
    pub image_scan_status: HashMap<String, Value>,
    pub next_token: String,
}

// put_lifecycle_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutLifecyclePolicyOutput {
    pub lifecycle_policy_text: String,
    pub registry_id: String,
    pub repository_name: String,
    pub success: bool,
}

// get_lifecycle_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLifecyclePolicyOutput {
    pub last_evaluated_at: String, // datetime
    pub lifecycle_policy_text: String,
    pub registry_id: String,
    pub repository_name: String,
}
