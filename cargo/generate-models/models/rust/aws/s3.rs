// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_s3_bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsS3Bucket {
    pub account_id: String,
    pub acl: String,
    pub bucket_name: String,
    #[serde(default)]
    pub cors_enabled: bool,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub encryption_algorithm: String,
    #[serde(default)]
    pub is_versioned: bool,
    pub kms_key_id: Option<String>,
    pub lifecycle_rules_count: i64,
    pub location: Option<String>,
    pub region: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsS3Bucket {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_s3_object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsS3Object {
    /// Reference: aws_s3_bucket.id
    pub bucket_id: String,
    pub content_type: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub etag: Option<String>,
    #[serde(default)]
    pub is_delete_marker: bool,
    pub key: String,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: Option<String>,
    pub size: Option<i64>,
    pub storage_class: String,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version_id: Option<String>,
}

impl AwsS3Object {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_s3_bucket_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsS3BucketPolicy {
    /// Reference: aws_s3_bucket.id
    pub bucket_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub policy_document: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsS3BucketPolicy {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_s3_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsS3MultipartUpload {
    /// Reference: aws_s3_bucket.id
    pub bucket_id: String,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub content_type: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub key: String,
    pub metadata: Option<String>,
    pub status: String,
    pub storage_class: Option<String>,
    pub upload_id: String,
}

impl AwsS3MultipartUpload {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

