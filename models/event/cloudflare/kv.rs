// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareKvValueRead {
    pub namespace: String,
    pub key: String,
    pub found: bool,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub read_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareKvValueRead {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareKvValueWritten {
    pub namespace: String,
    pub key: String,
    pub expiration_ttl: Option<i64>,
    #[serde(default)]
    pub has_metadata: bool,
    #[serde(default = "chrono::Utc::now")]
    pub written_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareKvValueWritten {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareKvKeyDeleted {
    pub namespace: String,
    pub key: String,
    #[serde(default = "chrono::Utc::now")]
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareKvKeyDeleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareKvKeysListed {
    pub namespace: String,
    pub prefix: Option<String>,
    pub key_count: Option<i64>,
    pub list_complete: Option<bool>,
    #[serde(default = "chrono::Utc::now")]
    pub listed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareKvKeysListed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

