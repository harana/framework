// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Client Connection
/// Class: client_connection
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Client Pool
/// Class: client_pool
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Client Query
/// Class: client_query
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientQuery {
    pub bytes_returned: Option<i64>,
    pub connection_id: String,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
    /// Reference: User.id
    pub executed_by: Option<String>,
    pub parameters: Option<String>,
    pub query_hash: Option<String>,
    pub query_text: Option<String>,
    pub query_type: String,
    pub rows_affected: Option<i64>,
    pub status: String,
}

impl ClientQuery {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Client Health Check
/// Class: client_health_check
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Client Configuration
/// Class: client_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ClientConfiguration {
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

impl ClientConfiguration {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

