// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ButtonClicked {
    pub button_id: String,
    pub button_name: Option<String>,
    pub button_type: String,
    pub button_variant: String,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl ButtonClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ButtonDoubleClicked {
    pub button_id: String,
    pub button_name: Option<String>,
    pub button_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub double_clicked_at: chrono::DateTime<chrono::Utc>,
}

impl ButtonDoubleClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ButtonLongPressed {
    pub button_id: String,
    pub button_name: Option<String>,
    pub press_duration_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub long_pressed_at: chrono::DateTime<chrono::Utc>,
}

impl ButtonLongPressed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ButtonDisabledClicked {
    pub button_id: String,
    pub button_name: Option<String>,
    pub disabled_reason: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl ButtonDisabledClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

