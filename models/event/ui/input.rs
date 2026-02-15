// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InputCleared {
    pub input_id: String,
    pub input_name: Option<String>,
    pub input_type: String,
    pub previous_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub cleared_at: chrono::DateTime<chrono::Utc>,
}

impl InputCleared {
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
pub struct CharacterLimitReached {
    pub input_id: String,
    pub input_name: Option<String>,
    pub character_limit: i64,
    pub current_length: i64,
    #[serde(default = "chrono::Utc::now")]
    pub reached_at: chrono::DateTime<chrono::Utc>,
}

impl CharacterLimitReached {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

