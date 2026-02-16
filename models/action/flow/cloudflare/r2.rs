// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub body: String,
    pub custom_metadata: String,
    pub etag: String,
    pub http_etag: String,
    pub http_metadata: String,
    pub key: String,
    pub size: i64,
    pub uploaded: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HeadOutput {
    pub custom_metadata: String,
    pub etag: String,
    pub http_etag: String,
    pub http_metadata: String,
    pub key: String,
    pub size: i64,
    pub uploaded: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutOutput {
    pub etag: String,
    pub key: String,
    pub size: i64,
    pub uploaded: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListOutput {
    pub cursor: String,
    pub delimited_prefixes: Vec<String>,
    pub objects: Vec<String>,
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMultipartUploadOutput {
    pub key: String,
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadPartOutput {
    pub etag: String,
    pub part_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompleteMultipartUploadOutput {
    pub etag: String,
    pub key: String,
    pub size: i64,
    pub uploaded: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfR2Bucket {
    pub account_id: String,
    pub bucket_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub location: String,
    pub storage_schema: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfR2Object {
    pub bucket_id: String,
    pub custom_metadata: String,
    pub etag: String,
    pub http_etag: String,
    pub key: String,
    pub size: i64,
    pub storage_schema: String,
    pub uploaded: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfR2MultipartUpload {
    pub bucket_id: String,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub custom_metadata: String,
    pub key: String,
    pub status: String,
    pub storage_schema: String,
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfR2MultipartPart {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub etag: String,
    pub part_number: i64,
    pub size: i64,
    pub upload_id: String,
}

#[async_trait]
pub trait R2Action {
    async fn get(&self, bucket: String, key: String, only_if: String, range: String) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn head(&self, bucket: String, key: String) -> Result<HeadOutput, Box<dyn std::error::Error>>;
    async fn put(&self, bucket: String, custom_metadata: String, http_metadata: String, key: String, md5: String, only_if: String, sha1: String, sha256: String, sha384: String, sha512: String, storage_schema: String, value: String) -> Result<PutOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, bucket: String, key: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_many(&self, bucket: String, keys: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn list(&self, bucket: String, cursor: String, delimiter: String, include: String, limit: i64, prefix: String, start_after: String) -> Result<ListOutput, Box<dyn std::error::Error>>;
    async fn create_multipart_upload(&self, bucket: String, custom_metadata: String, http_metadata: String, key: String, storage_schema: String) -> Result<CreateMultipartUploadOutput, Box<dyn std::error::Error>>;
    async fn upload_part(&self, bucket: String, key: String, part_number: i64, upload_id: String, value: String) -> Result<UploadPartOutput, Box<dyn std::error::Error>>;
    async fn complete_multipart_upload(&self, bucket: String, key: String, parts: Vec<String>, upload_id: String) -> Result<CompleteMultipartUploadOutput, Box<dyn std::error::Error>>;
    async fn abort_multipart_upload(&self, bucket: String, key: String, upload_id: String) -> Result<(), Box<dyn std::error::Error>>;
}
