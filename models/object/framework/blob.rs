// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobStorage {
    pub access_key_id: Option<String>,
    pub allowed_mime_types: Option<String>,
    pub base_path: Option<String>,
    pub bucket: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub endpoint: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub max_file_size: i64,
    pub provider: String,
    pub region: Option<String>,
    pub secret_access_key: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl BlobStorage {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobObject {
    pub cdn_url: Option<String>,
    pub content_hash: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub file_name: String,
    pub file_size: i64,
    #[serde(default)]
    pub is_encrypted: bool,
    #[serde(default)]
    pub is_public: bool,
    pub key: String,
    pub metadata: Option<String>,
    pub mime_type: String,
    pub storage_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub uploaded_by: Option<String>,
    pub url: Option<String>,
    pub version: Option<String>,
}

impl BlobObject {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobAccessLog {
    #[serde(default = "chrono::Utc::now")]
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    pub accessed_by: Option<String>,
    pub method: String,
    pub bytes_transferred: Option<i64>,
    pub ip_address: Option<String>,
    pub object_id: String,
    pub status: String,
    pub user_agent: Option<String>,
}

impl BlobAccessLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobMultipartUpload {
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_parts: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub file_name: String,
    pub key: String,
    pub status: String,
    pub storage_id: String,
    pub total_parts: i64,
    pub total_size: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub upload_id: String,
    pub uploaded_by: Option<String>,
}

impl BlobMultipartUpload {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Blob {
    pub bucket: String,
    pub key: String,
    pub etag: String,
    pub url: String,
    pub content_type: String,
    pub size: i64,
    pub metadata: String,
}

impl Blob {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobMetadata {
    pub cache_control: Option<String>,
    pub content_disposition: Option<String>,
    pub content_encoding: Option<String>,
    pub content_type: Option<String>,
    pub custom: std::collections::HashMap<String, String>,
}

impl BlobMetadata {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobInfo {
    pub content_type: Option<String>,
    pub etag: Option<String>,
    pub key: String,
    pub last_modified: Option<String>,
    pub metadata: Option<String>,
    pub size: i64,
}

impl BlobInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobPutOptions {
    #[serde(default)]
    pub if_not_exists: bool,
    pub metadata: Option<String>,
}

impl BlobPutOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobListOptions {
    pub cursor: Option<String>,
    pub delimiter: Option<String>,
    pub limit: Option<i64>,
    pub prefix: Option<String>,
}

impl BlobListOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobListResponse {
    pub cursor: Option<String>,
    pub objects: Vec<String>,
    pub prefixes: Vec<String>,
    pub truncated: bool,
}

impl BlobListResponse {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

