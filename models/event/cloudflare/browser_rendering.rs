// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareBrowserPageRendered {
    pub binding: String,
    pub url: String,
    pub status_code: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub rendered_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareBrowserPageRendered {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareBrowserScreenshotTaken {
    pub binding: String,
    pub url: String,
    pub type: String,
    pub width: Option<i64>,
    pub height: Option<i64>,
    #[serde(default)]
    pub full_page: bool,
    #[serde(default = "chrono::Utc::now")]
    pub taken_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareBrowserScreenshotTaken {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareBrowserContentExtracted {
    pub binding: String,
    pub url: String,
    pub selector: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub extracted_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareBrowserContentExtracted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareBrowserPdfGenerated {
    pub binding: String,
    pub url: String,
    pub format: String,
    #[serde(default)]
    pub landscape: bool,
    #[serde(default = "chrono::Utc::now")]
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareBrowserPdfGenerated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

