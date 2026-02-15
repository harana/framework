// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZipInput {
    pub compression_level: String,
    #[serde(default)]
    pub include_hidden: bool,
    pub output_path: String,
    pub password: String,
    pub source_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ZipOutput {
    pub archive_path: String,
    pub file_count: i64,
    pub size: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnzipInput {
    pub archive_path: String,
    pub destination_path: String,
    pub extract_files: Vec<String>,
    #[serde(default)]
    pub overwrite: bool,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnzipOutput {
    pub extracted_path: String,
    pub file_count: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TarInput {
    pub format: String,
    #[serde(default)]
    pub include_hidden: bool,
    pub output_path: String,
    #[serde(default)]
    pub preserve_permissions: bool,
    pub source_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TarOutput {
    pub archive_path: String,
    pub file_count: i64,
    pub size: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UntarInput {
    pub archive_path: String,
    pub destination_path: String,
    pub extract_files: Vec<String>,
    pub format: String,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UntarOutput {
    pub extracted_path: String,
    pub file_count: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GzipInput {
    pub compression_level: String,
    #[serde(default)]
    pub keep_source: bool,
    pub output_path: String,
    pub source_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GzipOutput {
    pub compressed_path: String,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GunzipInput {
    pub archive_path: String,
    #[serde(default)]
    pub keep_archive: bool,
    pub output_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GunzipOutput {
    pub decompressed_path: String,
    pub size: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListInput {
    pub archive_path: String,
    pub archive_type: String,
    pub password: String,
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
pub struct AddToArchiveInput {
    pub archive_path: String,
    pub archive_type: String,
    pub source_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddToArchiveOutput {
    pub added_count: i64,
    pub archive_path: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveFromArchiveInput {
    pub archive_path: String,
    pub archive_type: String,
    pub file_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveFromArchiveOutput {
    pub archive_path: String,
    pub removed_count: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Bzip2Input {
    pub compression_level: String,
    #[serde(default)]
    pub keep_source: bool,
    pub output_path: String,
    pub source_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Bzip2Output {
    pub compressed_path: String,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Bunzip2Input {
    pub archive_path: String,
    #[serde(default)]
    pub keep_archive: bool,
    pub output_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Bunzip2Output {
    pub decompressed_path: String,
    pub size: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XzInput {
    pub compression_level: String,
    #[serde(default)]
    pub keep_source: bool,
    pub output_path: String,
    pub source_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct XzOutput {
    pub compressed_path: String,
    pub compressed_size: i64,
    pub compression_ratio: f64,
    pub original_size: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnxzInput {
    pub archive_path: String,
    #[serde(default)]
    pub keep_archive: bool,
    pub output_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnxzOutput {
    pub decompressed_path: String,
    pub size: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyArchiveInput {
    pub archive_path: String,
    pub archive_type: String,
    pub password: String,
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
pub struct GetArchiveInfoInput {
    pub archive_path: String,
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
    async fn zip(&self, input: ZipInput) -> Result<ZipOutput, Box<dyn std::error::Error>>;
    async fn unzip(&self, input: UnzipInput) -> Result<UnzipOutput, Box<dyn std::error::Error>>;
    async fn tar(&self, input: TarInput) -> Result<TarOutput, Box<dyn std::error::Error>>;
    async fn untar(&self, input: UntarInput) -> Result<UntarOutput, Box<dyn std::error::Error>>;
    async fn gzip(&self, input: GzipInput) -> Result<GzipOutput, Box<dyn std::error::Error>>;
    async fn gunzip(&self, input: GunzipInput) -> Result<GunzipOutput, Box<dyn std::error::Error>>;
    async fn list(&self, input: ListInput) -> Result<ListOutput, Box<dyn std::error::Error>>;
    async fn add_to_archive(&self, input: AddToArchiveInput) -> Result<AddToArchiveOutput, Box<dyn std::error::Error>>;
    async fn remove_from_archive(&self, input: RemoveFromArchiveInput) -> Result<RemoveFromArchiveOutput, Box<dyn std::error::Error>>;
    async fn bzip2(&self, input: Bzip2Input) -> Result<Bzip2Output, Box<dyn std::error::Error>>;
    async fn bunzip2(&self, input: Bunzip2Input) -> Result<Bunzip2Output, Box<dyn std::error::Error>>;
    async fn xz(&self, input: XzInput) -> Result<XzOutput, Box<dyn std::error::Error>>;
    async fn unxz(&self, input: UnxzInput) -> Result<UnxzOutput, Box<dyn std::error::Error>>;
    async fn verify_archive(&self, input: VerifyArchiveInput) -> Result<VerifyArchiveOutput, Box<dyn std::error::Error>>;
    async fn get_archive_info(&self, input: GetArchiveInfoInput) -> Result<GetArchiveInfoOutput, Box<dyn std::error::Error>>;
}
