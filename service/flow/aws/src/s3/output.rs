//! Output types for AWS S3 actions
//!
//! This module contains all the output structs and helper types used by the AWS S3 actions.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbortMultipartUploadOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteMultipartUploadOutput {
        pub success: bool,
        pub key: String,
        pub bucket: String,
        pub etag: String,
        pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyObjectOutput {
        pub success: bool,
        pub etag: String,
        pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBucketOutput {
        pub location: String,
        pub success: bool,
        pub bucket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMultipartUploadOutput {
        pub success: bool,
        pub upload_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketCorsOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketPolicyOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteObjectOutput {
        pub success: bool,
        pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteObjectsOutput {
        pub errors: Vec<HashMap<String, Value>>,
        pub success: bool,
        pub deleted: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketCorsOutput {
        pub cors_rules: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketEncryptionOutput {
        pub rules: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketLifecycleOutput {
        pub rules: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketPolicyOutput {
        pub policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketVersioningOutput {
        pub mfa_delete: String,
        pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetObjectOutput {
        pub etag: String,
        pub last_modified: String,
        pub metadata: HashMap<String, Value>,
        pub version_id: String,
        pub storage_class: String,
        pub content: Vec<u8>,
        pub content_type: String,
        pub size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetObjectTaggingOutput {
        pub tags: HashMap<String, Value>,
        pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPresignedUrlOutput {
        pub expires_at: String,
        pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadObjectOutput {
        pub content_length: i32,
        pub exists: bool,
        pub metadata: HashMap<String, Value>,
        pub content_type: String,
        pub storage_class: String,
        pub last_modified: String,
        pub etag: String,
        pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBucketsOutput {
        pub buckets: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListObjectsOutput {
        pub continuation_token: String,
        pub contents: Vec<HashMap<String, Value>>,
        pub is_truncated: bool,
        pub next_continuation_token: String,
        pub key_count: i32,
        pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketCorsOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketEncryptionOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketLifecycleOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketPolicyOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketVersioningOutput {
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutObjectOutput {
        pub success: bool,
        pub version_id: String,
        pub etag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutObjectTaggingOutput {
        pub version_id: String,
        pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartOutput {
        pub success: bool,
        pub etag: String,
}
