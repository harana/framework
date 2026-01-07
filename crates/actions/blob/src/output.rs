// Harana Actions - Blob Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// copy_blob
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyBlobOutput {
    pub etag: String,
    pub success: bool
}

// delete_blob
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBlobOutput {
    pub success: bool
}

// download_blob
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadBlobOutput {
    pub metadata: HashMap<String, Value>,
    pub content_type: String,
    pub content: Vec<u8>,
    pub size: i32
}

// exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistsOutput {
    pub exists: bool
}

// generate_presigned_url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratePresignedUrlOutput {
    pub expires_at: String,
    pub url: String
}

// get_blob_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlobMetadataOutput {
    pub etag: String,
    pub created: String,
    pub metadata: HashMap<String, Value>,
    pub size: i32,
    pub content_type: String,
    pub modified: String
}

// list_blobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBlobsOutput {
    pub next_token: String,
    pub blobs: Vec<HashMap<String, Value>>
}

// upload_blob
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadBlobOutput {
    pub etag: String,
    pub url: String
}
