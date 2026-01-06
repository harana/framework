// Harana Actions - AWS S3 Module Output Types
// Auto-generated output structs for AWS S3 action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// create_bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBucketOutput {
    pub bucket: String,
    pub location: String,
    pub success: bool,
}

// delete_bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketOutput {
    pub success: bool,
}

// list_buckets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBucketsOutput {
    pub buckets: Vec<HashMap<String, Value>>,
}

// put_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutObjectOutput {
    pub etag: String,
    pub success: bool,
    pub version_id: String,
}

// get_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetObjectOutput {
    pub content: Vec<u8>,
    pub content_type: String,
    pub etag: String,
    pub last_modified: String,
    pub metadata: HashMap<String, Value>,
    pub size: i32,
    pub storage_class: String,
    pub version_id: String,
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
    pub success: bool,
    pub deleted: Vec<HashMap<String, Value>>,
    pub errors: Vec<HashMap<String, Value>>
}

// list_objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListObjectsOutput {
    pub continuation_token: String,
    pub contents: Vec<HashMap<String, Value>>,
    pub next_continuation_token: String,
    pub key_count: i32,
    pub is_truncated: bool,
    pub prefix: String
}

// copy_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyObjectOutput {
    pub version_id: String,
    pub etag: String,
    pub success: bool
}

// head_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadObjectOutput {
    pub content_length: i32,
    pub last_modified: String,
    pub metadata: HashMap<String, Value>,
    pub storage_class: String,
    pub content_type: String,
    pub exists: bool,
    pub etag: String,
    pub version_id: String
}

// get_presigned_url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPresignedUrlOutput {
    pub expires_at: String,
    pub url: String
}

// put_bucket_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketPolicyOutput {
    pub success: bool
}

// get_bucket_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketPolicyOutput {
    pub policy: String
}

// delete_bucket_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketPolicyOutput {
    pub success: bool
}

// put_bucket_versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketVersioningOutput {
    pub success: bool
}

// get_bucket_versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketVersioningOutput {
    pub mfa_delete: String,
    pub status: String
}

// put_bucket_encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketEncryptionOutput {
    pub success: bool
}

// get_bucket_encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketEncryptionOutput {
    pub rules: Vec<HashMap<String, Value>>
}

// put_bucket_cors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketCorsOutput {
    pub success: bool
}

// get_bucket_cors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketCorsOutput {
    pub cors_rules: Vec<HashMap<String, Value>>
}

// delete_bucket_cors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketCorsOutput {
    pub success: bool
}

// put_bucket_lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketLifecycleOutput {
    pub success: bool
}

// get_bucket_lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketLifecycleOutput {
    pub rules: Vec<HashMap<String, Value>>
}

// put_object_tagging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutObjectTaggingOutput {
    pub version_id: String,
    pub success: bool
}

// get_object_tagging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetObjectTaggingOutput {
    pub version_id: String,
    pub tags: HashMap<String, Value>
}

// create_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMultipartUploadOutput {
    pub success: bool,
    pub upload_id: String
}

// upload_part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartOutput {
    pub etag: String,
    pub success: bool
}

// complete_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteMultipartUploadOutput {
    pub key: String,
    pub location: String,
    pub bucket: String,
    pub etag: String,
    pub success: bool
}

// abort_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbortMultipartUploadOutput {
    pub success: bool
}
// TODO: Add remaining output types - see core/schema/actions/aws_s3.yml
