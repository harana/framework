// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebdriverCookieLog {
    pub action: String,
    pub cookie_domain: Option<String>,
    pub cookie_name: Option<String>,
    pub cookie_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub performed_at: chrono::DateTime<chrono::Utc>,
    pub session_id: String,
}

impl WebdriverCookieLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

