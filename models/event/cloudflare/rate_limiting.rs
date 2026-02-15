// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareRateLimitChecked {
    pub rate_limiter: String,
    pub key: String,
    pub allowed: bool,
    #[serde(default = "chrono::Utc::now")]
    pub checked_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareRateLimitChecked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareRateLimitExceeded {
    pub rate_limiter: String,
    pub key: String,
    #[serde(default = "chrono::Utc::now")]
    pub exceeded_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareRateLimitExceeded {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

