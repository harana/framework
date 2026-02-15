// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// webdriver_screenshot
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebdriverScreenshot {
    #[serde(default = "chrono::Utc::now")]
    pub captured_at: chrono::DateTime<chrono::Utc>,
    pub element_selector: Option<String>,
    pub file_path: Option<String>,
    pub file_size: Option<i64>,
    pub format: String,
    #[serde(default)]
    pub is_element: bool,
    /// Reference: web_driver_session.id
    pub session_id: String,
    pub url: Option<String>,
}

impl WebdriverScreenshot {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

