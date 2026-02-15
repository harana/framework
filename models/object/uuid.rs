// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UuidRegistry {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub entity_id: Option<String>,
    pub entity_type: Option<String>,
    pub value: String,
    pub variant: Option<String>,
    pub version: Option<i64>,
}

impl UuidRegistry {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Uuid {
    pub value: String,
    pub version: i64,
    pub variant: String,
    pub timestamp: i64,
}

impl Uuid {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

