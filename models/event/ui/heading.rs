// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HeadingClicked {
    pub heading_id: String,
    pub heading_content: Option<String>,
    pub heading_level: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl HeadingClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubheadingClicked {
    pub subheading_id: String,
    pub subheading_content: Option<String>,
    pub subheading_level: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl SubheadingClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

