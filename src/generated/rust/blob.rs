// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Blob Storage
/// Class: blob_storage
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Blob Object
/// Class: blob_object
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
    /// Reference: User.id
    pub uploaded_by: Option<String>,
    pub url: Option<String>,
    pub version: Option<String>,
}

impl BlobObject {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Blob Access Log
/// Class: blob_access_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobAccessLog {
    #[serde(default = "chrono::Utc::now")]
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    /// Reference: User.id
    pub accessed_by: Option<String>,
    pub action: String,
    pub bytes_transferred: Option<i64>,
    pub ip_address: Option<String>,
    pub object_id: String,
    pub status: String,
    pub user_agent: Option<String>,
}

impl BlobAccessLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Blob Multipart Upload
/// Class: blob_multipart_upload
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
    /// Reference: User.id
    pub uploaded_by: Option<String>,
}

impl BlobMultipartUpload {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

