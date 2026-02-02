use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// === Action Outputs ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadBlobOutput {
    pub url: String,
    pub etag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadBlobOutput {
    pub content: Vec<u8>,
    pub content_type: String,
    pub size: i64,
    pub metadata: BlobMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBlobOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBlobsOutput {
    pub blobs: Vec<BlobInfo>,
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistsOutput {
    pub exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlobMetadataOutput {
    pub content_type: String,
    pub size: i64,
    pub etag: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub metadata: BlobMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyBlobOutput {
    pub success: bool,
    pub etag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratePresignedUrlOutput {
    pub url: String,
    pub expires_at: DateTime<Utc>,
}

// === Class Types ===

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Blob {
    pub bucket: String,
    pub key: String,
    pub etag: String,
    pub url: String,
    pub content_type: String,
    pub size: i64,
    pub metadata: BlobMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlobMetadata {
    pub values: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobInfo {
    pub key: String,
    pub size: i64,
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub content_type: String,
}

// === Enums ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresignedUrlOperation {
    Get,
    Put,
}

impl Default for PresignedUrlOperation {
    fn default() -> Self {
        PresignedUrlOperation::Get
    }
}

// === Internal Storage Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredBlob {
    pub bucket: String,
    pub key: String,
    pub content: Vec<u8>,
    pub content_type: String,
    pub etag: String,
    pub metadata: BlobMetadata,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}
