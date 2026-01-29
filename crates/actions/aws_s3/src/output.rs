//! Output types for AWS S3 actions
//!
//! This module contains all the output structs and helper types used by the AWS S3 actions.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Output for abort_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbortMultipartUploadOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for complete_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteMultipartUploadOutput {
    /// Whether the operation was successful
    pub success: bool,
    /// The object key
    pub key: String,
    /// The name of the bucket
    pub bucket: String,
    /// Entity tag for the uploaded object
    pub etag: String,
    /// The URI that identifies the newly created object
    pub location: String,
}

/// Output for copy_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyObjectOutput {
    /// Whether the operation was successful
    pub success: bool,
    /// Entity tag of the copied object
    pub etag: String,
    /// Version ID of the copy
    pub version_id: String,
}

/// Output for create_bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBucketOutput {
    /// The location of the bucket
    pub location: String,
    /// Whether the operation was successful
    pub success: bool,
    /// The name of the bucket
    pub bucket: String,
}

/// Output for create_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMultipartUploadOutput {
    /// Whether the operation was successful
    pub success: bool,
    /// Upload ID identifying the multipart upload
    pub upload_id: String,
}

/// Output for delete_bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for delete_bucket_cors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketCorsOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for delete_bucket_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBucketPolicyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for delete_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteObjectOutput {
    /// Whether the operation was successful
    pub success: bool,
    /// Version ID of the delete marker
    pub version_id: String,
}

/// Output for delete_objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteObjectsOutput {
    /// Container for errors that occurred during the delete operation
    pub errors: Vec<HashMap<String, Value>>,
    /// Whether the operation was successful (no errors)
    pub success: bool,
    /// Container for objects that were deleted
    pub deleted: Vec<HashMap<String, Value>>,
}

/// Output for get_bucket_cors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketCorsOutput {
    /// A set of origins and methods enabled for this bucket
    pub cors_rules: Vec<HashMap<String, Value>>,
}

/// Output for get_bucket_encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketEncryptionOutput {
    /// Server-side encryption configuration rules
    pub rules: Vec<HashMap<String, Value>>,
}

/// Output for get_bucket_lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketLifecycleOutput {
    /// Container for lifecycle rules
    pub rules: Vec<HashMap<String, Value>>,
}

/// Output for get_bucket_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketPolicyOutput {
    /// The bucket policy as a JSON string
    pub policy: String,
}

/// Output for get_bucket_versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBucketVersioningOutput {
    /// Specifies whether MFA delete is enabled in the bucket versioning configuration
    pub mfa_delete: String,
    /// The versioning state of the bucket
    pub status: String,
}

/// Output for get_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetObjectOutput {
    /// Entity tag for the object
    pub etag: String,
    /// The date and time the object was last modified
    pub last_modified: String,
    /// A map of metadata to store with the object
    pub metadata: HashMap<String, Value>,
    /// Version ID of the object
    pub version_id: String,
    /// Provides storage class information of the object
    pub storage_class: String,
    /// The object content as bytes
    pub content: Vec<u8>,
    /// A standard MIME type describing the format of the object data
    pub content_type: String,
    /// Size of the body in bytes
    pub size: i32,
}

/// Output for get_object_tagging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetObjectTaggingOutput {
    /// Contains the tag set
    pub tags: HashMap<String, Value>,
    /// The version ID of the object for which you got the tagging information
    pub version_id: String,
}

/// Output for get_presigned_url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPresignedUrlOutput {
    /// The time at which the URL expires
    pub expires_at: String,
    /// The presigned URL
    pub url: String,
}

/// Output for head_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadObjectOutput {
    /// Size of the body in bytes
    pub content_length: i32,
    /// Specifies whether the object exists
    pub exists: bool,
    /// A map of metadata to store with the object
    pub metadata: HashMap<String, Value>,
    /// A standard MIME type describing the format of the object data
    pub content_type: String,
    /// Provides storage class information of the object
    pub storage_class: String,
    /// The date and time the object was last modified
    pub last_modified: String,
    /// Entity tag for the object
    pub etag: String,
    /// Version ID of the object
    pub version_id: String,
}

/// Output for list_buckets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBucketsOutput {
    /// The list of buckets owned by the authenticated sender
    pub buckets: Vec<HashMap<String, Value>>,
}

/// Output for list_objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListObjectsOutput {
    /// If ContinuationToken was sent with the request, it is included in the response
    pub continuation_token: String,
    /// Metadata about each object returned
    pub contents: Vec<HashMap<String, Value>>,
    /// Set to false if all of the results were returned
    pub is_truncated: bool,
    /// Token that can be used in subsequent request to get next set of results
    pub next_continuation_token: String,
    /// Number of keys returned with this response
    pub key_count: i32,
    /// Keys that begin with the indicated prefix
    pub prefix: String,
}

/// Output for put_bucket_cors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketCorsOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for put_bucket_encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketEncryptionOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for put_bucket_lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketLifecycleOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for put_bucket_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketPolicyOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for put_bucket_versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBucketVersioningOutput {
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for put_object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutObjectOutput {
    /// Whether the operation was successful
    pub success: bool,
    /// Version ID of the object
    pub version_id: String,
    /// Entity tag for the uploaded object
    pub etag: String,
}

/// Output for put_object_tagging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutObjectTaggingOutput {
    /// The version ID of the object the tag-set was added to
    pub version_id: String,
    /// Whether the operation was successful
    pub success: bool,
}

/// Output for upload_part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartOutput {
    /// Whether the operation was successful
    pub success: bool,
    /// Entity tag for the uploaded part
    pub etag: String,
}
