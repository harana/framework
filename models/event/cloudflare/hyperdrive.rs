// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareHyperdriveConnected {
    pub binding: String,
    pub host: Option<String>,
    pub port: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareHyperdriveConnected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareHyperdriveConnectionFailed {
    pub binding: String,
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub failed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareHyperdriveConnectionFailed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

