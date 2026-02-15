// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAnalyticsDataPointWritten {
    pub binding: String,
    pub index_count: Option<i64>,
    pub double_count: Option<i64>,
    pub blob_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub written_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAnalyticsDataPointWritten {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAnalyticsQueried {
    pub binding: String,
    pub row_count: Option<i64>,
    pub time_start: Option<chrono::DateTime<chrono::Utc>>,
    pub time_end: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub queried_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAnalyticsQueried {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

