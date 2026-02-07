// Harana Actions - Cloudflare R2 Module Output Types

use serde::{Deserialize, Serialize};

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub body: Vec<u8>,
    pub custom_metadata: serde_json::Value,
    pub etag: String,
    pub http_etag: String,
    pub http_metadata: R2HttpMetadata,
    pub key: String,
    pub size: i32,
    pub uploaded: String,
    pub version: String,
}

// head
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadOutput {
    pub custom_metadata: serde_json::Value,
    pub etag: String,
    pub http_etag: String,
    pub http_metadata: R2HttpMetadata,
    pub key: String,
    pub size: i32,
    pub uploaded: String,
    pub version: String,
}

// put
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutOutput {
    pub etag: String,
    pub key: String,
    pub size: i32,
    pub success: bool,
    pub uploaded: String,
    pub version: String,
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub success: bool,
}

// delete_many
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteManyOutput {
    pub success: bool,
}

// list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOutput {
    pub cursor: String,
    pub delimited_prefixes: Vec<String>,
    pub objects: Vec<R2Object>,
    pub truncated: bool,
}

// create_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMultipartUploadOutput {
    pub key: String,
    pub upload_id: String,
}

// upload_part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartOutput {
    pub etag: String,
    pub part_number: i32,
}

// complete_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteMultipartUploadOutput {
    pub etag: String,
    pub key: String,
    pub size: i32,
    pub success: bool,
    pub uploaded: String,
    pub version: String,
}

// abort_multipart_upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbortMultipartUploadOutput {
    pub success: bool,
}

// Helper structs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R2HttpMetadata {
    pub content_type: Option<String>,
    pub content_language: Option<String>,
    pub content_disposition: Option<String>,
    pub content_encoding: Option<String>,
    pub cache_control: Option<String>,
    pub cache_expiry: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R2Conditional {
    pub etag_matches: Option<String>,
    pub etag_does_not_match: Option<String>,
    pub uploaded_before: Option<String>,
    pub uploaded_after: Option<String>,
    pub seconds_granularity: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R2Range {
    pub offset: Option<i64>,
    pub length: Option<i64>,
    pub suffix: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R2Object {
    pub key: String,
    pub size: i32,
    pub etag: String,
    pub http_etag: String,
    pub uploaded: String,
    pub version: String,
    pub custom_metadata: Option<serde_json::Value>,
    pub http_metadata: Option<R2HttpMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R2UploadedPart {
    pub part_number: i32,
    pub etag: String,
}
