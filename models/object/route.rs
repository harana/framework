// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Route {
    pub auth: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub download: Option<String>,
    pub handler: String,
    #[serde(default)]
    pub is_active: bool,
    pub method: String,
    pub middleware: Option<String>,
    pub path: String,
    pub rate_limit: Option<i64>,
    pub sse: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub upload: Option<String>,
    pub websocket: Option<String>,
}

impl Route {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteGroup {
    pub middleware: Option<String>,
    pub prefix: String,
}

impl RouteGroup {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteGroupAssignment {
    pub group_id: String,
    pub route_id: String,
}

impl RouteGroupAssignment {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteAuthOptions {
    #[serde(default)]
    pub allow_anonymous_fallback: bool,
    #[serde(default)]
    pub auth_required: bool,
    pub auth_type: String,
    pub max_token_age: Option<i64>,
    pub required_permissions: Option<String>,
    pub required_roles: Option<String>,
    pub required_scopes: Option<String>,
}

impl RouteAuthOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteUploadOptions {
    pub allowed_extensions: Option<String>,
    pub allowed_mime_types: Option<String>,
    #[serde(default)]
    pub is_public: bool,
    pub max_file_size: i64,
    pub max_files: i64,
    #[serde(default)]
    pub overwrite_existing: bool,
    pub storage_id: String,
    pub storage_path: Option<String>,
    #[serde(default)]
    pub strip_exif: bool,
}

impl RouteUploadOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteDownloadOptions {
    pub cache_control: Option<String>,
    pub content_disposition: String,
    #[serde(default)]
    pub require_signed_url: bool,
    pub signed_url_ttl: i64,
    pub storage_id: String,
    pub storage_path: Option<String>,
}

impl RouteDownloadOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteWebsocketOptions {
    pub allowed_origins: Option<String>,
    pub heartbeat_interval: i64,
    pub idle_timeout: i64,
    pub max_connections: Option<i64>,
    pub max_frame_size: i64,
    pub max_message_size: i64,
    pub protocol: Option<String>,
}

impl RouteWebsocketOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteSseOptions {
    pub channel: Option<String>,
    pub heartbeat_interval: i64,
    pub idle_timeout: i64,
    pub max_connections: Option<i64>,
    pub retry_timeout: i64,
}

impl RouteSseOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteTransferLog {
    pub blob_key: String,
    pub bytes_transferred: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub direction: String,
    pub duration_ms: Option<i64>,
    pub file_name: Option<String>,
    pub ip_address: Option<String>,
    pub mime_type: Option<String>,
    pub route_id: String,
    pub status: String,
    pub user_agent: Option<String>,
    pub user_id: Option<String>,
}

impl RouteTransferLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

