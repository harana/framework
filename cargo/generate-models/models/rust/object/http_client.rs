// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// http_client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpClient {
    pub base_url: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_headers: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub max_retries: i64,
    pub timeout_seconds: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl HttpClient {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// http_request_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpRequestLog {
    /// Reference: http_client.id
    pub client_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    pub method: String,
    pub request_body: Option<String>,
    pub request_headers: Option<String>,
    pub response_body: Option<String>,
    pub response_headers: Option<String>,
    pub status_code: Option<i64>,
    pub url: String,
}

impl HttpRequestLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

