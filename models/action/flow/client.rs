// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientConnection {
    pub client_type: String,
    pub connection_retry_attempts: i64,
    pub connection_retry_delay_ms: i64,
    pub connection_string: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub database: String,
    pub host: String,
    #[serde(default)]
    pub is_active: bool,
    pub max_connections: i64,
    pub metadata: String,
    pub min_connections: i64,
    pub password: String,
    pub port: i64,
    pub ssl_cert_path: String,
    pub timeout_seconds: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub use_ssl: bool,
    pub username: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientQuery {
    pub bytes_returned: i64,
    pub connection_id: String,
    pub duration_ms: i64,
    pub error_message: String,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub executed_by: String,
    pub parameters: String,
    pub query_hash: String,
    pub query_text: String,
    pub query_type: String,
    pub rows_affected: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientHealthCheck {
    #[serde(default = "chrono::Utc::now")]
    pub checked_at: chrono::DateTime<chrono::Utc>,
    pub connection_id: String,
    pub error_message: String,
    #[serde(default)]
    pub is_healthy: bool,
    pub metadata: String,
    pub response_time_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientConfig {
    pub connection_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    #[serde(default)]
    pub is_encrypted: bool,
    #[serde(default)]
    pub is_required: bool,
    pub key: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub value: String,
}

