// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInput {
    pub bucket: String,
    pub key: String,
    pub only_if: String,
    pub range: String,
}

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
pub struct HeadInput {
    pub bucket: String,
    pub key: String,
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
pub struct PutInput {
    pub bucket: String,
    pub custom_metadata: String,
    pub http_metadata: String,
    pub key: String,
    pub md5: String,
    pub only_if: String,
    pub sha1: String,
    pub sha256: String,
    pub sha384: String,
    pub sha512: String,
    pub storage_class: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutOutput {
    pub etag: String,
    pub key: String,
    pub size: i64,
    pub success: bool,
    pub uploaded: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteInput {
    pub bucket: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteManyInput {
    pub bucket: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteManyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListInput {
    pub bucket: String,
    pub cursor: String,
    pub delimiter: String,
    pub include: String,
    pub limit: i64,
    pub prefix: String,
    pub start_after: String,
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
pub struct CreateMultipartUploadInput {
    pub bucket: String,
    pub custom_metadata: String,
    pub http_metadata: String,
    pub key: String,
    pub storage_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMultipartUploadOutput {
    pub key: String,
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadPartInput {
    pub bucket: String,
    pub key: String,
    pub part_number: i64,
    pub upload_id: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadPartOutput {
    pub etag: String,
    pub part_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompleteMultipartUploadInput {
    pub bucket: String,
    pub key: String,
    pub parts: Vec<String>,
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompleteMultipartUploadOutput {
    pub etag: String,
    pub key: String,
    pub size: i64,
    pub success: bool,
    pub uploaded: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AbortMultipartUploadInput {
    pub bucket: String,
    pub key: String,
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AbortMultipartUploadOutput {
    pub success: bool,
}

#[async_trait]
pub trait R2Action {
    async fn get(&self, input: GetInput) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn head(&self, input: HeadInput) -> Result<HeadOutput, Box<dyn std::error::Error>>;
    async fn put(&self, input: PutInput) -> Result<PutOutput, Box<dyn std::error::Error>>;
    async fn delete(&self, input: DeleteInput) -> Result<DeleteOutput, Box<dyn std::error::Error>>;
    async fn delete_many(&self, input: DeleteManyInput) -> Result<DeleteManyOutput, Box<dyn std::error::Error>>;
    async fn list(&self, input: ListInput) -> Result<ListOutput, Box<dyn std::error::Error>>;
    async fn create_multipart_upload(&self, input: CreateMultipartUploadInput) -> Result<CreateMultipartUploadOutput, Box<dyn std::error::Error>>;
    async fn upload_part(&self, input: UploadPartInput) -> Result<UploadPartOutput, Box<dyn std::error::Error>>;
    async fn complete_multipart_upload(&self, input: CompleteMultipartUploadInput) -> Result<CompleteMultipartUploadOutput, Box<dyn std::error::Error>>;
    async fn abort_multipart_upload(&self, input: AbortMultipartUploadInput) -> Result<AbortMultipartUploadOutput, Box<dyn std::error::Error>>;
}
