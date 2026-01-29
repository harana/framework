// Harana Actions - Blob Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// upload_blob
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadBlobOutput {
    pub etag: String,
    pub url: String,
}

// download_blob
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadBlobOutput {
    pub content: Vec<u8>,
    pub content_type: String,
    pub metadata: HashMap<String, String>,
    pub size: i32,
}

// delete_blob
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBlobOutput {
    pub success: bool,
}

// list_blobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBlobsOutput {
    pub blobs: Vec<BlobInfo>,
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobInfo {
    pub key: String,
    pub size: i32,
    pub last_modified: String,
    pub etag: String,
}

// exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistsOutput {
    pub exists: bool,
}

// get_blob_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBlobMetadataOutput {
    pub content_type: String,
    pub created: String,
    pub etag: String,
    pub metadata: HashMap<String, String>,
    pub modified: String,
    pub size: i32,
}

// copy_blob
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyBlobOutput {
    pub etag: String,
    pub success: bool,
}

// generate_presigned_url
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratePresignedUrlOutput {
    pub expires_at: String,
    pub url: String,
}
