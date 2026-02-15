// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebdriverSession {
    pub browser: String,
    pub capabilities: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub headless: bool,
    pub implicit_wait_ms: i64,
    #[serde(default)]
    pub is_active: bool,
    pub page_load_timeout_ms: i64,
    pub script_timeout_ms: i64,
    pub server_url: String,
    pub session_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_agent: Option<String>,
    pub window_height: i64,
    pub window_width: i64,
}

impl WebdriverSession {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

