// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// cloudflare_hyperdrive_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareHyperdriveConfig {
    pub account_id: String,
    pub binding: String,
    #[serde(default)]
    pub caching_disabled: bool,
    pub config_id: Option<String>,
    pub config_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub database: String,
    pub host: String,
    pub port: i64,
    pub scheme: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user: String,
}

impl CloudflareHyperdriveConfig {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

