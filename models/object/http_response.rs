// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpResponseTemplate {
    pub body_template: Option<String>,
    pub content_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub headers: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub status_code: i64,
    pub template_engine: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl HttpResponseTemplate {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpResponseLog {
    pub content_type: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: Option<i64>,
    pub request_method: String,
    pub request_path: String,
    pub response_size: Option<i64>,
    pub status_code: i64,
    pub template_id: Option<String>,
}

impl HttpResponseLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

