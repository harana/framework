// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadBlobOutput {
    pub etag: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DownloadBlobOutput {
    pub content: String,
    pub content_type: String,
    pub metadata: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListBlobsOutput {
    pub blobs: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBlobMetadataOutput {
    pub content_type: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub etag: String,
    pub metadata: String,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeneratePresignedUrlOutput {
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobMetadata {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobInfo {
    pub key: String,
    pub size: i64,
    pub etag: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobStorage {
    pub access_key_id: String,
    pub allowed_mime_types: String,
    pub base_path: String,
    pub bucket: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub endpoint: String,
    #[serde(default)]
    pub is_active: bool,
    pub max_file_size: i64,
    pub provider: String,
    pub region: String,
    pub secret_access_key: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobObject {
    pub cdn_url: String,
    pub content_hash: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub file_name: String,
    pub file_size: i64,
    #[serde(default)]
    pub is_encrypted: bool,
    #[serde(default)]
    pub is_public: bool,
    pub key: String,
    pub metadata: String,
    pub mime_type: String,
    pub storage_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub uploaded_by: String,
    pub url: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobAccessLog {
    #[serde(default = "chrono::Utc::now")]
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    pub accessed_by: String,
    pub method: String,
    pub bytes_transferred: i64,
    pub ip_address: String,
    pub object_id: String,
    pub status: String,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobMultipartUpload {
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub completed_parts: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub file_name: String,
    pub key: String,
    pub status: String,
    pub storage_id: String,
    pub total_parts: i64,
    pub total_size: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub upload_id: String,
    pub uploaded_by: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobMetadata {
    pub cache_control: String,
    pub content_disposition: String,
    pub content_encoding: String,
    pub content_type: String,
    pub custom: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobInfo {
    pub content_type: String,
    pub etag: String,
    pub key: String,
    pub last_modified: String,
    pub metadata: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobPutOptions {
    #[serde(default)]
    pub if_not_exists: bool,
    pub metadata: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobListOptions {
    pub cursor: String,
    pub delimiter: String,
    pub limit: i64,
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobListResponse {
    pub cursor: String,
    pub objects: Vec<String>,
    pub prefixes: Vec<String>,
    pub truncated: bool,
}

#[async_trait]
pub trait BlobAction {
    async fn upload_blob(&self, bucket: String, content: String, content_type: String, key: String, metadata: String) -> Result<UploadBlobOutput, Box<dyn std::error::Error>>;
    async fn download_blob(&self, bucket: String, key: String) -> Result<DownloadBlobOutput, Box<dyn std::error::Error>>;
    async fn delete_blob(&self, bucket: String, key: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_blobs(&self, bucket: String, continuation_token: String, max_results: i64, prefix: String) -> Result<ListBlobsOutput, Box<dyn std::error::Error>>;
    async fn exists(&self, bucket: String, key: String) -> Result<bool, Box<dyn std::error::Error>>;
    async fn get_blob_metadata(&self, bucket: String, key: String) -> Result<GetBlobMetadataOutput, Box<dyn std::error::Error>>;
    async fn copy_blob(&self, dest_bucket: String, dest_key: String, source_bucket: String, source_key: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn generate_presigned_url(&self, bucket: String, expires_in: i64, key: String, operation: String) -> Result<GeneratePresignedUrlOutput, Box<dyn std::error::Error>>;
}
