// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientConnection {
    pub client_type: String,
    pub connection_retry_attempts: i64,
    pub connection_retry_delay_ms: i64,
    pub connection_string: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub database: Option<String>,
    pub host: String,
    #[serde(default)]
    pub is_active: bool,
    pub max_connections: i64,
    pub metadata: Option<String>,
    pub min_connections: i64,
    pub password: Option<String>,
    pub port: i64,
    pub ssl_cert_path: Option<String>,
    pub timeout_seconds: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub use_ssl: bool,
    pub username: Option<String>,
}

impl ClientConnection {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientPool {
    pub active_connections: i64,
    pub connection_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub idle_connections: i64,
    pub idle_timeout_seconds: i64,
    #[serde(default)]
    pub is_healthy: bool,
    pub max_wait_time_ms: i64,
    pub pool_size: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub wait_queue_size: i64,
}

impl ClientPool {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientQuery {
    pub bytes_returned: Option<i64>,
    pub connection_id: String,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub executed_by: Option<String>,
    pub parameters: Option<String>,
    pub query_hash: Option<String>,
    pub query_text: Option<String>,
    pub query_type: String,
    pub rows_affected: Option<i64>,
    pub status: String,
}

impl ClientQuery {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientHealthCheck {
    #[serde(default = "chrono::Utc::now")]
    pub checked_at: chrono::DateTime<chrono::Utc>,
    pub connection_id: String,
    pub error_message: Option<String>,
    #[serde(default)]
    pub is_healthy: bool,
    pub metadata: Option<String>,
    pub response_time_ms: Option<i64>,
}

impl ClientHealthCheck {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientConfig {
    pub connection_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_encrypted: bool,
    #[serde(default)]
    pub is_required: bool,
    pub key: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub value: String,
}

impl ClientConfig {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub body: String,
    pub headers: std::collections::HashMap<String, String>,
    pub status_code: i64,
}

impl GetOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostOutput {
    pub body: String,
    pub headers: std::collections::HashMap<String, String>,
    pub status_code: i64,
}

impl PostOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutOutput {
    pub body: String,
    pub headers: std::collections::HashMap<String, String>,
    pub status_code: i64,
}

impl PutOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PatchOutput {
    pub body: String,
    pub headers: std::collections::HashMap<String, String>,
    pub status_code: i64,
}

impl PatchOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteOutput {
    pub body: String,
    pub headers: std::collections::HashMap<String, String>,
    pub status_code: i64,
}

impl DeleteOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DownloadOutput {
    pub content: String,
    pub content_type: String,
    pub size: i64,
}

impl DownloadOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadOutput {
    pub body: String,
    pub status_code: i64,
}

impl UploadOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphqlQueryOutput {
    pub data: String,
    pub errors: Vec<String>,
}

impl GraphqlQueryOutput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphqlError {
    pub message: String,
    pub path: Vec<String>,
    pub locations: Vec<String>,
}

impl GraphqlError {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphqlErrorLocation {
    pub line: i64,
    pub column: i64,
}

impl GraphqlErrorLocation {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

