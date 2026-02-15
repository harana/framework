// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadBlobInput {
    pub bucket: String,
    pub content: String,
    pub content_type: String,
    pub key: String,
    pub metadata: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadBlobOutput {
    pub etag: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DownloadBlobInput {
    pub bucket: String,
    pub key: String,
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
pub struct DeleteBlobInput {
    pub bucket: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteBlobOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListBlobsInput {
    pub bucket: String,
    pub continuation_token: String,
    pub max_results: i64,
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListBlobsOutput {
    pub blobs: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExistsInput {
    pub bucket: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExistsOutput {
    pub exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBlobMetadataInput {
    pub bucket: String,
    pub key: String,
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
pub struct CopyBlobInput {
    pub dest_bucket: String,
    pub dest_key: String,
    pub source_bucket: String,
    pub source_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopyBlobOutput {
    pub etag: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GeneratePresignedUrlInput {
    pub bucket: String,
    pub expires_in: i64,
    pub key: String,
    pub operation: String,
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

#[async_trait]
pub trait BlobAction {
    async fn upload_blob(&self, input: UploadBlobInput) -> Result<UploadBlobOutput, Box<dyn std::error::Error>>;
    async fn download_blob(&self, input: DownloadBlobInput) -> Result<DownloadBlobOutput, Box<dyn std::error::Error>>;
    async fn delete_blob(&self, input: DeleteBlobInput) -> Result<DeleteBlobOutput, Box<dyn std::error::Error>>;
    async fn list_blobs(&self, input: ListBlobsInput) -> Result<ListBlobsOutput, Box<dyn std::error::Error>>;
    async fn exists(&self, input: ExistsInput) -> Result<ExistsOutput, Box<dyn std::error::Error>>;
    async fn get_blob_metadata(&self, input: GetBlobMetadataInput) -> Result<GetBlobMetadataOutput, Box<dyn std::error::Error>>;
    async fn copy_blob(&self, input: CopyBlobInput) -> Result<CopyBlobOutput, Box<dyn std::error::Error>>;
    async fn generate_presigned_url(&self, input: GeneratePresignedUrlInput) -> Result<GeneratePresignedUrlOutput, Box<dyn std::error::Error>>;
}
