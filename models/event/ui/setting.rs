// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SettingChanged {
    pub setting_id: String,
    pub setting_key: String,
    pub setting_category: Option<String>,
    pub old_value: Option<String>,
    pub new_value: String,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl SettingChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PreferenceToggled {
    pub preference_id: String,
    pub preference_key: String,
    pub preference_category: Option<String>,
    pub new_state: bool,
    #[serde(default = "chrono::Utc::now")]
    pub toggled_at: chrono::DateTime<chrono::Utc>,
}

impl PreferenceToggled {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ThemeSwitched {
    pub from_theme: Option<String>,
    pub to_theme: String,
    pub theme_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub switched_at: chrono::DateTime<chrono::Utc>,
}

impl ThemeSwitched {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LanguageChanged {
    pub from_language: Option<String>,
    pub to_language: String,
    pub language_code: String,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl LanguageChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

