// Harana Actions - Aws Ecr Module
// This module provides aws ecr actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Batch Delete ECR Image
pub async fn batch_delete_image(
    image_ids: Vec<HashMap<String, Value>>,
    repository_name: &str,
    region: Option<&str>,
) -> Result<BatchDeleteImageOutput, String> {
    unimplemented!("batch_delete_image")
}

/// Create ECR Repository
pub async fn create_repository(
    repository_name: &str,
    scan_on_push: Option<bool>,
    tags: Option<HashMap<String, Value>>,
    kms_key: Option<&str>,
    image_tag_mutability: Option<&str>,
    encryption_type: Option<&str>,
    region: Option<&str>,
) -> Result<CreateRepositoryOutput, String> {
    unimplemented!("create_repository")
}

/// Delete ECR Repository
pub async fn delete_repository(
    repository_name: &str,
    region: Option<&str>,
    force: Option<bool>,
) -> Result<DeleteRepositoryOutput, String> {
    unimplemented!("delete_repository")
}

/// Delete Repository Policy
pub async fn delete_repository_policy(
    repository_name: &str,
    region: Option<&str>,
) -> Result<DeleteRepositoryPolicyOutput, String> {
    unimplemented!("delete_repository_policy")
}

/// Describe ECR Images
pub async fn describe_images(
    repository_name: &str,
    image_ids: Option<Vec<HashMap<String, Value>>>,
    region: Option<&str>,
) -> Result<DescribeImagesOutput, String> {
    unimplemented!("describe_images")
}

/// Describe ECR Repositories
pub async fn describe_repositories(
    region: Option<&str>,
    repository_names: Option<Vec<String>>,
) -> Result<DescribeRepositoriesOutput, String> {
    unimplemented!("describe_repositories")
}

/// Describe Image Scan Findings
pub async fn describe_scan_findings(
    image_id: HashMap<String, Value>,
    repository_name: &str,
    max_results: Option<i32>,
    region: Option<&str>,
) -> Result<DescribeScanFindingsOutput, String> {
    unimplemented!("describe_scan_findings")
}

/// Get ECR Authorization Token
pub async fn get_auth_token(
    registry_ids: Option<Vec<String>>,
    region: Option<&str>,
) -> Result<GetAuthTokenOutput, String> {
    unimplemented!("get_auth_token")
}

/// Get ECR Download URL
pub async fn get_download_url(
    repository_name: &str,
    image_digest: &str,
    region: Option<&str>,
) -> Result<GetDownloadUrlOutput, String> {
    unimplemented!("get_download_url")
}

/// Get Lifecycle Policy
pub async fn get_lifecycle_policy(
    repository_name: &str,
    region: Option<&str>,
) -> Result<GetLifecyclePolicyOutput, String> {
    unimplemented!("get_lifecycle_policy")
}

/// Get Repository Policy
pub async fn get_repository_policy(
    repository_name: &str,
    region: Option<&str>,
) -> Result<GetRepositoryPolicyOutput, String> {
    unimplemented!("get_repository_policy")
}

/// List ECR Images
pub async fn list_images(
    repository_name: &str,
    filter: Option<HashMap<String, Value>>,
    region: Option<&str>,
    max_results: Option<i32>,
) -> Result<ListImagesOutput, String> {
    unimplemented!("list_images")
}

/// Put ECR Image
pub async fn put_image(
    repository_name: &str,
    image_manifest: &str,
    image_tag: &str,
    region: Option<&str>,
) -> Result<PutImageOutput, String> {
    unimplemented!("put_image")
}

/// Put Lifecycle Policy
pub async fn put_lifecycle_policy(
    lifecycle_policy_text: &str,
    repository_name: &str,
    region: Option<&str>,
) -> Result<PutLifecyclePolicyOutput, String> {
    unimplemented!("put_lifecycle_policy")
}

/// Set Repository Policy
pub async fn set_repository_policy(
    repository_name: &str,
    policy_text: &str,
    region: Option<&str>,
    force: Option<bool>,
) -> Result<SetRepositoryPolicyOutput, String> {
    unimplemented!("set_repository_policy")
}

/// Start Image Scan
pub async fn start_image_scan(
    image_id: HashMap<String, Value>,
    repository_name: &str,
    region: Option<&str>,
) -> Result<StartImageScanOutput, String> {
    unimplemented!("start_image_scan")
}

/// Tag ECR Image
pub async fn tag_image(
    repository_name: &str,
    image_digest: &str,
    image_tag: &str,
    region: Option<&str>,
) -> Result<TagImageOutput, String> {
    unimplemented!("tag_image")
}
