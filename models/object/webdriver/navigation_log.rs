// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebdriverNavigationLog {
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub navigated_at: chrono::DateTime<chrono::Utc>,
    pub page_title: Option<String>,
    pub session_id: String,
    pub status: String,
    pub url: String,
}

impl WebdriverNavigationLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

