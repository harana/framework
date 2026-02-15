// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextareaChanged {
    pub textarea_id: String,
    pub textarea_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl TextareaChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextareaFocused {
    pub textarea_id: String,
    pub textarea_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub focused_at: chrono::DateTime<chrono::Utc>,
}

impl TextareaFocused {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextareaBlurred {
    pub textarea_id: String,
    pub textarea_name: Option<String>,
    pub final_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub blurred_at: chrono::DateTime<chrono::Utc>,
}

impl TextareaBlurred {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextareaInput {
    pub textarea_id: String,
    pub textarea_name: Option<String>,
    pub current_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub input_at: chrono::DateTime<chrono::Utc>,
}

impl TextareaInput {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextareaResized {
    pub textarea_id: String,
    pub textarea_name: Option<String>,
    pub old_width: Option<i64>,
    pub old_height: Option<i64>,
    pub new_width: Option<i64>,
    pub new_height: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub resized_at: chrono::DateTime<chrono::Utc>,
}

impl TextareaResized {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextareaCharacterLimitReached {
    pub textarea_id: String,
    pub textarea_name: Option<String>,
    pub character_limit: i64,
    pub current_length: i64,
    #[serde(default = "chrono::Utc::now")]
    pub reached_at: chrono::DateTime<chrono::Utc>,
}

impl TextareaCharacterLimitReached {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

