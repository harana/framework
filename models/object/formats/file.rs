// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileDocument {
    pub content_type: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_directory: bool,
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub path: String,
    pub permissions: Option<String>,
    pub size: i64,
}

impl FileDocument {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct File {
    pub path: String,
    pub name: String,
    pub size: i64,
    pub content_type: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub is_directory: bool,
    pub permissions: String,
}

impl File {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

