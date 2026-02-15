// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAnalyticsEngineDataset {
    pub account_id: String,
    pub binding: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dataset_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAnalyticsEngineDataset {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareAnalyticsEngineDataPoint {
    pub blobs: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dataset_id: String,
    pub doubles: Option<String>,
    pub indexes: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl CloudflareAnalyticsEngineDataPoint {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

