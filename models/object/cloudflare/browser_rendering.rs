// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareBrowserRenderingSession {
    pub account_id: String,
    pub binding: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub session_id: String,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareBrowserRenderingSession {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareBrowserRenderingResult {
    pub content: Option<String>,
    pub content_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub rendered_at: chrono::DateTime<chrono::Utc>,
    pub session_id: String,
    pub status_code: Option<i64>,
    pub title: Option<String>,
    pub url: String,
}

impl CloudflareBrowserRenderingResult {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

