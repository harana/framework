// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_ecr_repository
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcrRepository {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub encryption_type: String,
    pub image_tag_mutability: String,
    pub kms_key: Option<String>,
    pub region: Option<String>,
    pub registry_id: Option<String>,
    pub repository_arn: Option<String>,
    pub repository_name: String,
    pub repository_uri: Option<String>,
    #[serde(default)]
    pub scan_on_push: bool,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsEcrRepository {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ecr_image
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcrImage {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub image_digest: String,
    pub image_manifest_media_type: Option<String>,
    pub image_pushed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub image_size_bytes: Option<i64>,
    pub image_tags: Option<String>,
    pub last_scan_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Reference: aws_ecr_repository.id
    pub repository_id: String,
    pub scan_status: String,
    pub vulnerability_count: i64,
}

impl AwsEcrImage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ecr_repository_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcrRepositoryPolicy {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub policy_text: String,
    /// Reference: aws_ecr_repository.id
    pub repository_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsEcrRepositoryPolicy {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ecr_lifecycle_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcrLifecyclePolicy {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_evaluated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub lifecycle_policy_text: String,
    /// Reference: aws_ecr_repository.id
    pub repository_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsEcrLifecyclePolicy {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_ecr_image_scan_finding
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcrImageScanFinding {
    pub description: Option<String>,
    /// Reference: aws_ecr_image.id
    pub image_id: String,
    pub name: String,
    pub severity: String,
    pub uri: Option<String>,
}

impl AwsEcrImageScanFinding {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

