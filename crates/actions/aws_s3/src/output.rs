// Harana Actions - Aws S3 Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// abort_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbortMultipartUploadOutput {
    pub success: bool
}

// complete_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteMultipartUploadOutput {
    pub success: bool,
    pub key: String,
    pub bucket: String,
    pub etag: String,
    pub location: String
}

// copy_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyObjectOutput {
    pub success: bool,
    pub etag: String,
    pub version_id: String
}

// create_bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBucketOutput {
    pub location: String,
    pub success: bool,
    pub bucket: String
}

// create_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMultipartUploadOutput {
    pub success: bool,
    pub upload_id: String
}

// delete_bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketOutput {
    pub success: bool
}

// delete_bucket_cors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketCorsOutput {
    pub success: bool
}

// delete_bucket_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketPolicyOutput {
    pub success: bool
}

// delete_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteObjectOutput {
    pub success: bool,
    pub version_id: String
}

// delete_objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteObjectsOutput {
    pub errors: Vec<HashMap<String, Value>>,
    pub success: bool,
    pub deleted: Vec<HashMap<String, Value>>
}

// get_bucket_cors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketCorsOutput {
    pub cors_rules: Vec<HashMap<String, Value>>
}

// get_bucket_encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketEncryptionOutput {
    pub rules: Vec<HashMap<String, Value>>
}

// get_bucket_lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketLifecycleOutput {
    pub rules: Vec<HashMap<String, Value>>
}

// get_bucket_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketPolicyOutput {
    pub policy: String
}

// get_bucket_versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketVersioningOutput {
    pub mfa_delete: String,
    pub status: String
}

// get_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetObjectOutput {
    pub etag: String,
    pub last_modified: String,
    pub metadata: HashMap<String, Value>,
    pub version_id: String,
    pub storage_class: String,
    pub content: Vec<u8>,
    pub content_type: String,
    pub size: i32
}

// get_object_tagging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetObjectTaggingOutput {
    pub tags: HashMap<String, Value>,
    pub version_id: String
}

// get_presigned_url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPresignedUrlOutput {
    pub expires_at: String,
    pub url: String
}

// head_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadObjectOutput {
    pub content_length: i32,
    pub exists: bool,
    pub metadata: HashMap<String, Value>,
    pub content_type: String,
    pub storage_class: String,
    pub last_modified: String,
    pub etag: String,
    pub version_id: String
}

// list_buckets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBucketsOutput {
    pub buckets: Vec<HashMap<String, Value>>
}

// list_objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListObjectsOutput {
    pub continuation_token: String,
    pub contents: Vec<HashMap<String, Value>>,
    pub is_truncated: bool,
    pub next_continuation_token: String,
    pub key_count: i32,
    pub prefix: String
}

// put_bucket_cors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketCorsOutput {
    pub success: bool
}

// put_bucket_encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketEncryptionOutput {
    pub success: bool
}

// put_bucket_lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketLifecycleOutput {
    pub success: bool
}

// put_bucket_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketPolicyOutput {
    pub success: bool
}

// put_bucket_versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketVersioningOutput {
    pub success: bool
}

// put_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutObjectOutput {
    pub success: bool,
    pub version_id: String,
    pub etag: String
}

// put_object_tagging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutObjectTaggingOutput {
    pub version_id: String,
    pub success: bool
}

// upload_part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartOutput {
    pub success: bool,
    pub etag: String
}
