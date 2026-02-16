// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZipOutput {
    pub archive_path: String,
    pub file_count: i64,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnzipOutput {
    pub extracted_path: String,
    pub file_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TarOutput {
    pub archive_path: String,
    pub file_count: i64,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UntarOutput {
    pub extracted_path: String,
    pub file_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GzipOutput {
    pub compressed_path: String,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GunzipOutput {
    pub decompressed_path: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListOutput {
    pub files: Vec<String>,
    pub total_files: i64,
    pub total_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddToArchiveOutput {
    pub added_count: i64,
    pub archive_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveFromArchiveOutput {
    pub archive_path: String,
    pub removed_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Bzip2Output {
    pub compressed_path: String,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Bunzip2Output {
    pub decompressed_path: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XzOutput {
    pub compressed_path: String,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnxzOutput {
    pub decompressed_path: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyArchiveOutput {
    pub error: String,
    pub file_count: i64,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetArchiveInfoOutput {
    pub archive_type: String,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub created_at: i64,
    pub file_count: i64,
    pub is_encrypted: bool,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Archive {
    pub path: String,
    pub archive_type: String,
    pub size: i64,
    pub compressed_size: i64,
    pub file_count: i64,
    pub is_encrypted: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveEntry {
    pub path: String,
    pub size: i64,
    pub compressed_size: i64,
    pub is_directory: bool,
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait ArchiveAction {
    async fn zip(&self, compression_level: String, include_hidden: bool, output_path: String, password: String, source_paths: Vec<String>) -> Result<ZipOutput, Box<dyn std::error::Error>>;
    async fn unzip(&self, archive_path: String, destination_path: String, extract_files: Vec<String>, overwrite: bool, password: String) -> Result<UnzipOutput, Box<dyn std::error::Error>>;
    async fn tar(&self, format: String, include_hidden: bool, output_path: String, preserve_permissions: bool, source_paths: Vec<String>) -> Result<TarOutput, Box<dyn std::error::Error>>;
    async fn untar(&self, archive_path: String, destination_path: String, extract_files: Vec<String>, format: String, overwrite: bool) -> Result<UntarOutput, Box<dyn std::error::Error>>;
    async fn gzip(&self, compression_level: String, keep_source: bool, output_path: String, source_path: String) -> Result<GzipOutput, Box<dyn std::error::Error>>;
    async fn gunzip(&self, archive_path: String, keep_archive: bool, output_path: String) -> Result<GunzipOutput, Box<dyn std::error::Error>>;
    async fn list(&self, archive_path: String, archive_type: String, password: String) -> Result<ListOutput, Box<dyn std::error::Error>>;
    async fn add_to_archive(&self, archive_path: String, archive_type: String, source_paths: Vec<String>) -> Result<AddToArchiveOutput, Box<dyn std::error::Error>>;
    async fn remove_from_archive(&self, archive_path: String, archive_type: String, file_paths: Vec<String>) -> Result<RemoveFromArchiveOutput, Box<dyn std::error::Error>>;
    async fn bzip2(&self, compression_level: String, keep_source: bool, output_path: String, source_path: String) -> Result<Bzip2Output, Box<dyn std::error::Error>>;
    async fn bunzip2(&self, archive_path: String, keep_archive: bool, output_path: String) -> Result<Bunzip2Output, Box<dyn std::error::Error>>;
    async fn xz(&self, compression_level: String, keep_source: bool, output_path: String, source_path: String) -> Result<XzOutput, Box<dyn std::error::Error>>;
    async fn unxz(&self, archive_path: String, keep_archive: bool, output_path: String) -> Result<UnxzOutput, Box<dyn std::error::Error>>;
    async fn verify_archive(&self, archive_path: String, archive_type: String, password: String) -> Result<VerifyArchiveOutput, Box<dyn std::error::Error>>;
    async fn get_archive_info(&self, archive_path: String) -> Result<GetArchiveInfoOutput, Box<dyn std::error::Error>>;
}
