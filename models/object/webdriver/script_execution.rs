// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebdriverScriptExecution {
    pub arguments: Option<String>,
    #[serde(default)]
    pub async: bool,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub result: Option<String>,
    pub script: String,
    pub session_id: String,
    pub status: String,
}

impl WebdriverScriptExecution {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

