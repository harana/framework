// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SwitchToggledOn {
    pub switch_id: String,
    pub switch_name: Option<String>,
    pub switch_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub toggled_at: chrono::DateTime<chrono::Utc>,
}

impl SwitchToggledOn {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SwitchToggledOff {
    pub switch_id: String,
    pub switch_name: Option<String>,
    pub switch_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub toggled_at: chrono::DateTime<chrono::Utc>,
}

impl SwitchToggledOff {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SwitchChanged {
    pub switch_id: String,
    pub switch_name: Option<String>,
    pub switch_value: Option<String>,
    pub is_checked: bool,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl SwitchChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SwitchGroupChanged {
    pub group_id: String,
    pub group_name: Option<String>,
    pub active_switches: Option<String>,
    pub active_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl SwitchGroupChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

