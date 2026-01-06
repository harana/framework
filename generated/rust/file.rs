// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// File
/// Class: file
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct File {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub path: String,
    pub full_path: String,
    /// Reference: File.id
    pub parent_id: Option<String>,
    pub type: String,
    pub extension: Option<String>,
    pub mime_type: Option<String>,
    pub size: i64,
    pub content_hash: Option<String>,
    #[serde(default)]
    pub is_hidden: bool,
    #[serde(default)]
    pub is_system: bool,
    #[serde(default)]
    pub is_readonly: bool,
    #[serde(default)]
    pub is_archived: bool,
    pub permissions: Option<String>,
    pub owner: Option<String>,
    pub group: Option<String>,
    /// Reference: BlobObject.id
    pub blob_id: Option<String>,
    /// Reference: User.id
    pub created_by: Option<String>,
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Reference: User.id
    pub modified_by: Option<String>,
    pub accessed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl File {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// File Version
/// Class: file_version
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileVersion {
    pub file_id: String,
    pub version_number: i64,
    pub size: i64,
    pub content_hash: String,
    /// Reference: BlobObject.id
    pub blob_id: Option<String>,
    pub comment: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: User.id
    pub created_by: Option<String>,
    #[serde(default)]
    pub is_current: bool,
}

impl FileVersion {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// File Metadata
/// Class: file_metadata
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

impl FileMetadata {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// File Tag
/// Class: file_tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileTag {
    pub color: Option<String>,
    pub description: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: User.id
    pub created_by: Option<String>,
}

impl FileTag {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// File Tag Association
/// Class: file_tag_association
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileTagAssociation {
    pub file_id: String,
    pub tag_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: User.id
    pub created_by: Option<String>,
}

impl FileTagAssociation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// File Share
/// Class: file_share
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileShare {
    pub file_id: String,
    pub shared_by: String,
    /// Reference: User.id
    pub shared_with: Option<String>,
    pub share_token: String,
    pub permissions: String,
    pub password: Option<String>,
    pub download_limit: Option<i64>,
    pub download_count: i64,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub accessed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FileShare {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// File Operation
/// Class: file_operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileOperation {
    pub file_id: String,
    pub operation: String,
    pub status: String,
    pub source_path: Option<String>,
    pub destination_path: Option<String>,
    pub bytes_processed: i64,
    pub total_bytes: Option<i64>,
    pub progress_percent: i64,
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub performed_at: chrono::DateTime<chrono::Utc>,
    /// Reference: User.id
    pub performed_by: Option<String>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FileOperation {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// File Format
/// Class: file_format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileFormat {
    pub format_type: String,
    pub encoding: Option<String>,
    pub page_count: Option<i64>,
    pub word_count: Option<i64>,
    pub line_count: Option<i64>,
    pub column_count: Option<i64>,
    pub row_count: Option<i64>,
    #[serde(default)]
    pub has_macros: bool,
    #[serde(default)]
    pub is_encrypted: bool,
    pub language: Option<String>,
    pub author: Option<String>,
    pub title: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl FileFormat {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// File Index
/// Class: file_index
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileIndex {
    pub file_id: String,
    pub index_type: String,
    pub content: Option<String>,
    pub content_vector: Option<String>,
    pub language: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub indexed_at: chrono::DateTime<chrono::Utc>,
    pub index_version: Option<String>,
    #[serde(default)]
    pub is_current: bool,
}

impl FileIndex {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// File Watch
/// Class: file_watch
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileWatch {
    pub path: String,
    pub pattern: Option<String>,
    pub watch_type: String,
    #[serde(default)]
    pub is_recursive: bool,
    #[serde(default)]
    pub is_active: bool,
    pub callback_url: Option<String>,
    pub callback_event: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: User.id
    pub created_by: Option<String>,
    pub last_triggered_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FileWatch {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// File Crawl
/// Class: file_crawl
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileCrawl {
    pub root_path: String,
    pub pattern: Option<String>,
    pub exclude_pattern: Option<String>,
    pub max_depth: Option<i64>,
    pub max_files: Option<i64>,
    #[serde(default)]
    pub follow_symlinks: bool,
    pub status: String,
    pub files_found: i64,
    pub files_processed: i64,
    pub total_size: i64,
    pub error_message: Option<String>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: User.id
    pub created_by: Option<String>,
}

impl FileCrawl {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

