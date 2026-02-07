//! Output types for AWS S3 actions
//!
//! This module contains all the output structs and helper types used by the AWS S3 actions.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Output for abort_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbortMultipartUploadOutput {
        pub success: bool,
}

/// Output for complete_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteMultipartUploadOutput {
        pub success: bool,
        pub key: String,
        pub bucket: String,
        pub etag: String,
        pub location: String,
}

/// Output for copy_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyObjectOutput {
        pub success: bool,
        pub etag: String,
        pub version_id: String,
}

/// Output for create_bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBucketOutput {
        pub location: String,
        pub success: bool,
        pub bucket: String,
}

/// Output for create_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMultipartUploadOutput {
        pub success: bool,
        pub upload_id: String,
}

/// Output for delete_bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketOutput {
        pub success: bool,
}

/// Output for delete_bucket_cors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketCorsOutput {
        pub success: bool,
}

/// Output for delete_bucket_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketPolicyOutput {
        pub success: bool,
}

/// Output for delete_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteObjectOutput {
        pub success: bool,
        pub version_id: String,
}

/// Output for delete_objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteObjectsOutput {
        pub errors: Vec<HashMap<String, Value>>,
        pub success: bool,
        pub deleted: Vec<HashMap<String, Value>>,
}

/// Output for get_bucket_cors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketCorsOutput {
        pub cors_rules: Vec<HashMap<String, Value>>,
}

/// Output for get_bucket_encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketEncryptionOutput {
        pub rules: Vec<HashMap<String, Value>>,
}

/// Output for get_bucket_lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketLifecycleOutput {
        pub rules: Vec<HashMap<String, Value>>,
}

/// Output for get_bucket_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketPolicyOutput {
        pub policy: String,
}

/// Output for get_bucket_versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketVersioningOutput {
        pub mfa_delete: String,
        pub status: String,
}

/// Output for get_object
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

/// Output for get_object_tagging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetObjectTaggingOutput {
        pub tags: HashMap<String, Value>,
        pub version_id: String,
}

/// Output for get_presigned_url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPresignedUrlOutput {
        pub expires_at: String,
        pub url: String,
}

/// Output for head_object
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

/// Output for list_buckets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBucketsOutput {
        pub buckets: Vec<HashMap<String, Value>>,
}

/// Output for list_objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListObjectsOutput {
        pub continuation_token: String,
        pub contents: Vec<HashMap<String, Value>>,
        pub is_truncated: bool,
        pub next_continuation_token: String,
        pub key_count: i32,
        pub prefix: String,
}

/// Output for put_bucket_cors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketCorsOutput {
        pub success: bool,
}

/// Output for put_bucket_encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketEncryptionOutput {
        pub success: bool,
}

/// Output for put_bucket_lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketLifecycleOutput {
        pub success: bool,
}

/// Output for put_bucket_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketPolicyOutput {
        pub success: bool,
}

/// Output for put_bucket_versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketVersioningOutput {
        pub success: bool,
}

/// Output for put_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutObjectOutput {
        pub success: bool,
        pub version_id: String,
        pub etag: String,
}

/// Output for put_object_tagging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutObjectTaggingOutput {
        pub version_id: String,
        pub success: bool,
}

/// Output for upload_part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartOutput {
        pub success: bool,
        pub etag: String,
}
