// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DateFormat {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub format_string: String,
    #[serde(default)]
    pub is_active: bool,
    pub locale: Option<String>,
}

impl DateFormat {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Timezone {
    pub abbreviation: Option<String>,
    pub dst_offset: i64,
    pub name: String,
    pub utc_offset: i64,
}

impl Timezone {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Date {
    pub timestamp: String,
    pub timezone: String,
    pub year: i64,
    pub month: i64,
    pub day: i64,
    pub hour: i64,
    pub minute: i64,
    pub second: i64,
}

impl Date {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

