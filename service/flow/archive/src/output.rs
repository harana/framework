// Harana Actions - Archive Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// add_to_archive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToArchiveOutput {
    pub archive_path: String,
    pub added_count: i32,
    pub success: bool
}

// bunzip2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bunzip2Output {
    pub decompressed_path: String,
    pub size: i32,
    pub success: bool
}

// bzip2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bzip2Output {
    pub compressed_size: i32,
    pub compression_ratio: f64,
    pub compressed_path: String,
    pub success: bool,
    pub original_size: i32
}

// get_archive_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArchiveInfoOutput {
    pub compressed_size: i32,
    pub archive_type: String,
    pub is_encrypted: bool,
    pub created_at: i32,
    pub file_count: i32,
    pub size: i32,
    pub compression_ratio: f64
}

// gunzip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GunzipOutput {
    pub decompressed_path: String,
    pub success: bool,
    pub size: i32
}

// gzip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GzipOutput {
    pub original_size: i32,
    pub success: bool,
    pub compression_ratio: f64,
    pub compressed_path: String,
    pub compressed_size: i32
}

// list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOutput {
    pub files: Vec<HashMap<String, Value>>,
    pub total_size: i32,
    pub total_files: i32
}

// remove_from_archive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveFromArchiveOutput {
    pub removed_count: i32,
    pub archive_path: String,
    pub success: bool
}

// tar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TarOutput {
    pub success: bool,
    pub size: i32,
    pub archive_path: String,
    pub file_count: i32
}

// untar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UntarOutput {
    pub success: bool,
    pub extracted_path: String,
    pub file_count: i32
}

// unxz
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnxzOutput {
    pub size: i32,
    pub success: bool,
    pub decompressed_path: String
}

// unzip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnzipOutput {
    pub file_count: i32,
    pub success: bool,
    pub extracted_path: String
}

// verify_archive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyArchiveOutput {
    pub valid: bool,
    pub file_count: i32,
    pub error: String
}

// xz
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XzOutput {
    pub original_size: i32,
    pub compressed_path: String,
    pub success: bool,
    pub compressed_size: i32,
    pub compression_ratio: f64
}

// zip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZipOutput {
    pub success: bool,
    pub size: i32,
    pub archive_path: String,
    pub file_count: i32
}
