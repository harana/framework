// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub repository_id: String,
    pub scan_status: String,
    pub vulnerability_count: i64,
}

impl AwsEcrImage {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl AwsEcrRepositoryPolicy {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcrLifecyclePolicy {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_evaluated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub lifecycle_policy_text: String,
    pub repository_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsEcrLifecyclePolicy {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsEcrImageScanFinding {
    pub description: Option<String>,
    pub image_id: String,
    pub name: String,
    pub severity: String,
    pub uri: Option<String>,
}

impl AwsEcrImageScanFinding {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

