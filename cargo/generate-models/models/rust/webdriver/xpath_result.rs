// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// webdriver_xpath_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebdriverXpathResult {
    pub element_count: i64,
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub queried_at: chrono::DateTime<chrono::Utc>,
    pub results: Option<String>,
    /// Reference: web_driver_session.id
    pub session_id: String,
    pub status: String,
    pub xpath: String,
}

impl WebdriverXpathResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

