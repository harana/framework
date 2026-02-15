// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// cf_r2_bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfR2Bucket {
    pub account_id: String,
    pub bucket_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub location: Option<String>,
    pub storage_class: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CfR2Bucket {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cf_r2_object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfR2Object {
    /// Reference: cf_r2_bucket.id
    pub bucket_id: String,
    pub custom_metadata: Option<String>,
    pub etag: Option<String>,
    pub http_etag: Option<String>,
    pub key: String,
    pub size: Option<i64>,
    pub storage_class: String,
    pub uploaded: Option<chrono::DateTime<chrono::Utc>>,
    pub version: Option<String>,
}

impl CfR2Object {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cf_r2_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfR2MultipartUpload {
    /// Reference: cf_r2_bucket.id
    pub bucket_id: String,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub custom_metadata: Option<String>,
    pub key: String,
    pub status: String,
    pub storage_class: String,
    pub upload_id: String,
}

impl CfR2MultipartUpload {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cf_r2_multipart_part
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfR2MultipartPart {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub etag: Option<String>,
    pub part_number: i64,
    pub size: Option<i64>,
    /// Reference: cf_r2_multipart_upload.id
    pub upload_id: String,
}

impl CfR2MultipartPart {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

