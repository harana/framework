// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareR2ObjectUploaded {
    pub bucket: String,
    pub key: String,
    pub size: Option<i64>,
    pub etag: Option<String>,
    pub storage_class: String,
    #[serde(default = "chrono::Utc::now")]
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareR2ObjectUploaded {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareR2ObjectDownloaded {
    pub bucket: String,
    pub key: String,
    pub size: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub downloaded_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareR2ObjectDownloaded {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareR2ObjectDeleted {
    pub bucket: String,
    pub key: String,
    #[serde(default = "chrono::Utc::now")]
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareR2ObjectDeleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareR2ObjectsDeleted {
    pub bucket: String,
    pub key_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareR2ObjectsDeleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareR2ObjectsListed {
    pub bucket: String,
    pub prefix: Option<String>,
    pub object_count: Option<i64>,
    pub truncated: Option<bool>,
    #[serde(default = "chrono::Utc::now")]
    pub listed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareR2ObjectsListed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareR2MultipartUploadStarted {
    pub bucket: String,
    pub key: String,
    pub upload_id: String,
    pub storage_class: String,
    #[serde(default = "chrono::Utc::now")]
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareR2MultipartUploadStarted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareR2MultipartUploadCompleted {
    pub bucket: String,
    pub key: String,
    pub upload_id: String,
    pub part_count: Option<i64>,
    pub size: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareR2MultipartUploadCompleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareR2MultipartUploadAborted {
    pub bucket: String,
    pub key: String,
    pub upload_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub aborted_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareR2MultipartUploadAborted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

