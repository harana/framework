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

// TODO: Add remaining output types - see core/schema/actions/aws_s3.yml
