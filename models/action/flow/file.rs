// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct File {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub path: String,
    pub full_path: String,
    pub parent_id: String,
    pub type: String,
    pub extension: String,
    pub mime_type: String,
    pub size: i64,
    pub content_hash: String,
    #[serde(default)]
    pub is_hidden: bool,
    #[serde(default)]
    pub is_system: bool,
    #[serde(default)]
    pub is_readonly: bool,
    #[serde(default)]
    pub is_archived: bool,
    pub permissions: String,
    pub owner: String,
    pub group: String,
    pub blob_id: String,
    pub created_by: String,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    pub modified_by: String,
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Version {
    pub file_id: String,
    pub version_number: i64,
    pub size: i64,
    pub content_hash: String,
    pub blob_id: String,
    pub comment: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
    #[serde(default)]
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Metadata {
    pub file_id: String,
    pub key: String,
    pub value: String,
    pub value_type: String,
    pub source: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tag {
    pub color: String,
    pub description: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagAssociation {
    pub file_id: String,
    pub tag_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Share {
    pub file_id: String,
    pub shared_by: String,
    pub shared_with: String,
    pub share_token: String,
    pub permissions: String,
    pub password: String,
    pub download_limit: i64,
    pub download_count: i64,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub accessed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Operation {
    pub file_id: String,
    pub operation: String,
    pub status: String,
    pub source_path: String,
    pub destination_path: String,
    pub bytes_processed: i64,
    pub total_bytes: i64,
    pub progress_percent: i64,
    pub error_message: String,
    #[serde(default = "chrono::Utc::now")]
    pub performed_at: chrono::DateTime<chrono::Utc>,
    pub performed_by: String,
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Format {
    pub format_type: String,
    pub encoding: String,
    pub page_count: i64,
    pub word_count: i64,
    pub line_count: i64,
    pub column_count: i64,
    pub row_count: i64,
    #[serde(default)]
    pub has_macros: bool,
    #[serde(default)]
    pub is_encrypted: bool,
    pub language: String,
    pub author: String,
    pub title: String,
    pub subject: String,
    pub keywords: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Index {
    pub file_id: String,
    pub index_type: String,
    pub content: String,
    pub content_vector: String,
    pub language: String,
    #[serde(default = "chrono::Utc::now")]
    pub indexed_at: chrono::DateTime<chrono::Utc>,
    pub index_version: String,
    #[serde(default)]
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Watch {
    pub path: String,
    pub pattern: String,
    pub watch_type: String,
    #[serde(default)]
    pub is_recursive: bool,
    #[serde(default)]
    pub is_active: bool,
    pub callback_url: String,
    pub callback_event: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
    pub last_triggered_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Crawl {
    pub root_path: String,
    pub pattern: String,
    pub exclude_pattern: String,
    pub max_depth: i64,
    pub max_files: i64,
    #[serde(default)]
    pub follow_symlinks: bool,
    pub status: String,
    pub files_found: i64,
    pub files_processed: i64,
    pub total_size: i64,
    pub error_message: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileVersion {
    pub file_id: String,
    pub version_number: i64,
    pub size: i64,
    pub content_hash: String,
    pub blob_id: String,
    pub comment: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
    #[serde(default)]
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileMetadata {
    pub file_id: String,
    pub key: String,
    pub value: String,
    pub value_type: String,
    pub source: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileTag {
    pub color: String,
    pub description: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileTagAssociation {
    pub file_id: String,
    pub tag_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileShare {
    pub file_id: String,
    pub shared_by: String,
    pub shared_with: String,
    pub share_token: String,
    pub permissions: String,
    pub password: String,
    pub download_limit: i64,
    pub download_count: i64,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub accessed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileOperation {
    pub file_id: String,
    pub operation: String,
    pub status: String,
    pub source_path: String,
    pub destination_path: String,
    pub bytes_processed: i64,
    pub total_bytes: i64,
    pub progress_percent: i64,
    pub error_message: String,
    #[serde(default = "chrono::Utc::now")]
    pub performed_at: chrono::DateTime<chrono::Utc>,
    pub performed_by: String,
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileFormat {
    pub format_type: String,
    pub encoding: String,
    pub page_count: i64,
    pub word_count: i64,
    pub line_count: i64,
    pub column_count: i64,
    pub row_count: i64,
    #[serde(default)]
    pub has_macros: bool,
    #[serde(default)]
    pub is_encrypted: bool,
    pub language: String,
    pub author: String,
    pub title: String,
    pub subject: String,
    pub keywords: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileIndex {
    pub file_id: String,
    pub index_type: String,
    pub content: String,
    pub content_vector: String,
    pub language: String,
    #[serde(default = "chrono::Utc::now")]
    pub indexed_at: chrono::DateTime<chrono::Utc>,
    pub index_version: String,
    #[serde(default)]
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileWatch {
    pub path: String,
    pub pattern: String,
    pub watch_type: String,
    #[serde(default)]
    pub is_recursive: bool,
    #[serde(default)]
    pub is_active: bool,
    pub callback_url: String,
    pub callback_event: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
    pub last_triggered_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileCrawl {
    pub root_path: String,
    pub pattern: String,
    pub exclude_pattern: String,
    pub max_depth: i64,
    pub max_files: i64,
    #[serde(default)]
    pub follow_symlinks: bool,
    pub status: String,
    pub files_found: i64,
    pub files_processed: i64,
    pub total_size: i64,
    pub error_message: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
}

