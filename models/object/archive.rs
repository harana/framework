// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Archive {
    pub archive_type: String,
    pub compression_level: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<String>,
    pub file_count: i64,
    #[serde(default)]
    pub is_encrypted: bool,
    pub original_size: Option<i64>,
    pub output_path: String,
    pub size: Option<i64>,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Archive {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ArchiveEntry {
    pub archive_id: String,
    pub compressed_size: Option<i64>,
    #[serde(default)]
    pub is_directory: bool,
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub original_size: Option<i64>,
    pub path: String,
    pub permissions: Option<String>,
}

impl ArchiveEntry {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

