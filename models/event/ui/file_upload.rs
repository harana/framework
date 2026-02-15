// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileSelected {
    pub upload_id: String,
    pub file_name: String,
    pub file_size: Option<i64>,
    pub file_type: Option<String>,
    pub mime_type: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub selected_at: chrono::DateTime<chrono::Utc>,
}

impl FileSelected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileRemoved {
    pub upload_id: String,
    pub file_name: String,
    pub file_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub removed_at: chrono::DateTime<chrono::Utc>,
}

impl FileRemoved {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileUploadStarted {
    pub upload_id: String,
    pub file_name: String,
    pub file_size: Option<i64>,
    pub file_type: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl FileUploadStarted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileUploadCompleted {
    pub upload_id: String,
    pub file_name: String,
    pub file_size: Option<i64>,
    pub file_id: Option<String>,
    pub upload_duration_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

impl FileUploadCompleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileUploadFailed {
    pub upload_id: String,
    pub file_name: String,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub failed_at: chrono::DateTime<chrono::Utc>,
}

impl FileUploadFailed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DragEntered {
    pub dropzone_id: String,
    pub dropzone_name: Option<String>,
    pub file_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub entered_at: chrono::DateTime<chrono::Utc>,
}

impl DragEntered {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DragLeft {
    pub dropzone_id: String,
    pub dropzone_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub left_at: chrono::DateTime<chrono::Utc>,
}

impl DragLeft {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileDropped {
    pub dropzone_id: String,
    pub dropzone_name: Option<String>,
    pub file_count: i64,
    pub total_size: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub dropped_at: chrono::DateTime<chrono::Utc>,
}

impl FileDropped {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

