// Harana Actions - AWS ECS Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

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
    pub repositories: Vec<EcrRepository>,
}

// list_images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListImagesOutput {
    pub image_ids: Vec<ImageId>,
    pub next_token: Option<String>,
}

// describe_images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeImagesOutput {
    pub image_details: Vec<EcrImageDetail>,
}

// put_image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutImageOutput {
    pub image: EcrImage,
    pub success: bool,
}

// batch_delete_image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteImageOutput {
    pub failures: Vec<ImageFailure>,
    pub image_ids: Vec<ImageId>,
    pub success: bool,
}

// get_auth_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthTokenOutput {
    pub authorization_token: String,
    pub expires_at: DateTime<Utc>,
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

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcrRepository {
    pub repository_arn: String,
    pub registry_id: String,
    pub repository_name: String,
    pub repository_uri: String,
    pub created_at: Option<DateTime<Utc>>,
    pub image_tag_mutability: String,
    pub image_scanning_configuration: Option<ImageScanningConfiguration>,
    pub encryption_configuration: Option<EncryptionConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageId {
    pub image_digest: Option<String>,
    pub image_tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcrImageDetail {
    pub registry_id: String,
    pub repository_name: String,
    pub image_digest: String,
    pub image_tags: Option<Vec<String>>,
    pub image_size_in_bytes: Option<i64>,
    pub image_pushed_at: Option<DateTime<Utc>>,
    pub image_scan_status: Option<ImageScanStatus>,
    pub artifact_media_type: Option<String>,
    pub image_manifest_media_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcrImage {
    pub registry_id: String,
    pub repository_name: String,
    pub image_id: ImageId,
    pub image_manifest: String,
    pub image_manifest_media_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFailure {
    pub image_id: Option<ImageId>,
    pub failure_code: String,
    pub failure_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageScanningConfiguration {
    pub scan_on_push: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    pub encryption_type: String,
    pub kms_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageScanStatus {
    pub status: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFilter {
    pub tag_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcrTags {
    pub tags: HashMap<String, String>,
}
