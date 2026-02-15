// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebdriverElementInteraction {
    pub action: String,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    pub input_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub performed_at: chrono::DateTime<chrono::Utc>,
    pub selector_type: String,
    pub selector_value: String,
    pub session_id: String,
    pub status: String,
}

impl WebdriverElementInteraction {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

