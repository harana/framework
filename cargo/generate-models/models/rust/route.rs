// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// route
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route_group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteGroup {
    pub middleware: Option<String>,
    pub prefix: String,
}

impl RouteGroup {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route_group_assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteGroupAssignment {
    /// Reference: route_group.id
    pub group_id: String,
    /// Reference: route.id
    pub route_id: String,
}

impl RouteGroupAssignment {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route_auth_options
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route_upload_options
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
    /// Reference: blob_storage.id
    pub storage_id: String,
    pub storage_path: Option<String>,
    #[serde(default)]
    pub strip_exif: bool,
}

impl RouteUploadOptions {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route_download_options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteDownloadOptions {
    pub cache_control: Option<String>,
    pub content_disposition: String,
    #[serde(default)]
    pub require_signed_url: bool,
    pub signed_url_ttl: i64,
    /// Reference: blob_storage.id
    pub storage_id: String,
    pub storage_path: Option<String>,
}

impl RouteDownloadOptions {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route_websocket_options
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route_sse_options
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route_transfer_log
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
    /// Reference: route.id
    pub route_id: String,
    pub status: String,
    pub user_agent: Option<String>,
    /// Reference: user.id
    pub user_id: Option<String>,
}

impl RouteTransferLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

