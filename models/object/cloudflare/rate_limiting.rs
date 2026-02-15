// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareRateLimiter {
    pub account_id: String,
    pub binding: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub limiter_name: String,
    pub period_seconds: i64,
    pub requests_per_period: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareRateLimiter {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

