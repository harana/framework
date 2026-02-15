// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateBucketInput {
    pub acl: String,
    pub bucket: String,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateBucketOutput {
    pub bucket: String,
    pub location: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteBucketInput {
    pub bucket: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteBucketOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListBucketsInput {
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListBucketsOutput {
    pub buckets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutObjectInput {
    pub acl: String,
    pub bucket: String,
    pub content: String,
    pub content_type: String,
    pub key: String,
    pub metadata: String,
    pub region: String,
    pub server_side_encryption: String,
    pub storage_class: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutObjectOutput {
    pub etag: String,
    pub success: bool,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetObjectInput {
    pub bucket: String,
    pub key: String,
    pub region: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetObjectOutput {
    pub content: String,
    pub content_type: String,
    pub etag: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub metadata: String,
    pub size: i64,
    pub storage_class: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteObjectInput {
    pub bucket: String,
    pub key: String,
    pub region: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteObjectOutput {
    pub success: bool,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteObjectsInput {
    pub bucket: String,
    pub keys: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteObjectsOutput {
    pub deleted: Vec<String>,
    pub errors: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListObjectsInput {
    pub bucket: String,
    pub continuation_token: String,
    pub delimiter: String,
    pub max_keys: i64,
    pub prefix: String,
    pub region: String,
    pub start_after: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListObjectsOutput {
    pub contents: Vec<String>,
    pub continuation_token: String,
    pub is_truncated: bool,
    pub key_count: i64,
    pub next_continuation_token: String,
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopyObjectInput {
    pub destination_bucket: String,
    pub destination_key: String,
    pub metadata: String,
    pub metadata_directive: String,
    pub region: String,
    pub source_bucket: String,
    pub source_key: String,
    pub source_version_id: String,
    pub storage_class: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CopyObjectOutput {
    pub etag: String,
    pub success: bool,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HeadObjectInput {
    pub bucket: String,
    pub key: String,
    pub region: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HeadObjectOutput {
    pub content_length: i64,
    pub content_type: String,
    pub etag: String,
    pub exists: bool,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub metadata: String,
    pub storage_class: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPresignedUrlInput {
    pub bucket: String,
    pub expires_in: i64,
    pub key: String,
    pub operation: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPresignedUrlOutput {
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutBucketPolicyInput {
    pub bucket: String,
    pub policy: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutBucketPolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBucketPolicyInput {
    pub bucket: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBucketPolicyOutput {
    pub policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteBucketPolicyInput {
    pub bucket: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteBucketPolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutBucketVersioningInput {
    pub bucket: String,
    pub region: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutBucketVersioningOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBucketVersioningInput {
    pub bucket: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBucketVersioningOutput {
    pub mfa_delete: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutBucketEncryptionInput {
    pub bucket: String,
    pub kms_master_key_id: String,
    pub region: String,
    pub sse_algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutBucketEncryptionOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBucketEncryptionInput {
    pub bucket: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBucketEncryptionOutput {
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutBucketCorsInput {
    pub bucket: String,
    pub cors_rules: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutBucketCorsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBucketCorsInput {
    pub bucket: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBucketCorsOutput {
    pub cors_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteBucketCorsInput {
    pub bucket: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteBucketCorsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutBucketLifecycleInput {
    pub bucket: String,
    pub lifecycle_rules: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutBucketLifecycleOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBucketLifecycleInput {
    pub bucket: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetBucketLifecycleOutput {
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutObjectTaggingInput {
    pub bucket: String,
    pub key: String,
    pub region: String,
    pub tags: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutObjectTaggingOutput {
    pub success: bool,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetObjectTaggingInput {
    pub bucket: String,
    pub key: String,
    pub region: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetObjectTaggingOutput {
    pub tags: String,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMultipartUploadInput {
    pub bucket: String,
    pub content_type: String,
    pub key: String,
    pub metadata: String,
    pub region: String,
    pub storage_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMultipartUploadOutput {
    pub success: bool,
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadPartInput {
    pub body: String,
    pub bucket: String,
    pub key: String,
    pub part_number: i64,
    pub region: String,
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadPartOutput {
    pub etag: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompleteMultipartUploadInput {
    pub bucket: String,
    pub key: String,
    pub parts: Vec<String>,
    pub region: String,
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompleteMultipartUploadOutput {
    pub bucket: String,
    pub etag: String,
    pub key: String,
    pub location: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AbortMultipartUploadInput {
    pub bucket: String,
    pub key: String,
    pub region: String,
    pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AbortMultipartUploadOutput {
    pub success: bool,
}

#[async_trait]
pub trait S3Action {
    async fn create_bucket(&self, input: CreateBucketInput) -> Result<CreateBucketOutput, Box<dyn std::error::Error>>;
    async fn delete_bucket(&self, input: DeleteBucketInput) -> Result<DeleteBucketOutput, Box<dyn std::error::Error>>;
    async fn list_buckets(&self, input: ListBucketsInput) -> Result<ListBucketsOutput, Box<dyn std::error::Error>>;
    async fn put_object(&self, input: PutObjectInput) -> Result<PutObjectOutput, Box<dyn std::error::Error>>;
    async fn get_object(&self, input: GetObjectInput) -> Result<GetObjectOutput, Box<dyn std::error::Error>>;
    async fn delete_object(&self, input: DeleteObjectInput) -> Result<DeleteObjectOutput, Box<dyn std::error::Error>>;
    async fn delete_objects(&self, input: DeleteObjectsInput) -> Result<DeleteObjectsOutput, Box<dyn std::error::Error>>;
    async fn list_objects(&self, input: ListObjectsInput) -> Result<ListObjectsOutput, Box<dyn std::error::Error>>;
    async fn copy_object(&self, input: CopyObjectInput) -> Result<CopyObjectOutput, Box<dyn std::error::Error>>;
    async fn head_object(&self, input: HeadObjectInput) -> Result<HeadObjectOutput, Box<dyn std::error::Error>>;
    async fn get_presigned_url(&self, input: GetPresignedUrlInput) -> Result<GetPresignedUrlOutput, Box<dyn std::error::Error>>;
    async fn put_bucket_policy(&self, input: PutBucketPolicyInput) -> Result<PutBucketPolicyOutput, Box<dyn std::error::Error>>;
    async fn get_bucket_policy(&self, input: GetBucketPolicyInput) -> Result<GetBucketPolicyOutput, Box<dyn std::error::Error>>;
    async fn delete_bucket_policy(&self, input: DeleteBucketPolicyInput) -> Result<DeleteBucketPolicyOutput, Box<dyn std::error::Error>>;
    async fn put_bucket_versioning(&self, input: PutBucketVersioningInput) -> Result<PutBucketVersioningOutput, Box<dyn std::error::Error>>;
    async fn get_bucket_versioning(&self, input: GetBucketVersioningInput) -> Result<GetBucketVersioningOutput, Box<dyn std::error::Error>>;
    async fn put_bucket_encryption(&self, input: PutBucketEncryptionInput) -> Result<PutBucketEncryptionOutput, Box<dyn std::error::Error>>;
    async fn get_bucket_encryption(&self, input: GetBucketEncryptionInput) -> Result<GetBucketEncryptionOutput, Box<dyn std::error::Error>>;
    async fn put_bucket_cors(&self, input: PutBucketCorsInput) -> Result<PutBucketCorsOutput, Box<dyn std::error::Error>>;
    async fn get_bucket_cors(&self, input: GetBucketCorsInput) -> Result<GetBucketCorsOutput, Box<dyn std::error::Error>>;
    async fn delete_bucket_cors(&self, input: DeleteBucketCorsInput) -> Result<DeleteBucketCorsOutput, Box<dyn std::error::Error>>;
    async fn put_bucket_lifecycle(&self, input: PutBucketLifecycleInput) -> Result<PutBucketLifecycleOutput, Box<dyn std::error::Error>>;
    async fn get_bucket_lifecycle(&self, input: GetBucketLifecycleInput) -> Result<GetBucketLifecycleOutput, Box<dyn std::error::Error>>;
    async fn put_object_tagging(&self, input: PutObjectTaggingInput) -> Result<PutObjectTaggingOutput, Box<dyn std::error::Error>>;
    async fn get_object_tagging(&self, input: GetObjectTaggingInput) -> Result<GetObjectTaggingOutput, Box<dyn std::error::Error>>;
    async fn create_multipart_upload(&self, input: CreateMultipartUploadInput) -> Result<CreateMultipartUploadOutput, Box<dyn std::error::Error>>;
    async fn upload_part(&self, input: UploadPartInput) -> Result<UploadPartOutput, Box<dyn std::error::Error>>;
    async fn complete_multipart_upload(&self, input: CompleteMultipartUploadInput) -> Result<CompleteMultipartUploadOutput, Box<dyn std::error::Error>>;
    async fn abort_multipart_upload(&self, input: AbortMultipartUploadInput) -> Result<AbortMultipartUploadOutput, Box<dyn std::error::Error>>;
}
