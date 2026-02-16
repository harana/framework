// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryOutput {
    pub data: Vec<String>,
    pub rows: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfAnalyticsEngineDataset {
    pub account_id: String,
    pub binding: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dataset_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfAnalyticsEngineDataPoint {
    pub blobs: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub dataset_id: String,
    pub doubles: String,
    pub indexes: String,
    #[serde(default = "chrono::Utc::now")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait AnalyticsEngineAction {
    async fn write_data_point(&self, binding: String, blobs: Vec<String>, doubles: Vec<f64>, indexes: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn query(&self, binding: String, query: String, time_end: chrono::DateTime<chrono::Utc>, time_start: chrono::DateTime<chrono::Utc>) -> Result<QueryOutput, Box<dyn std::error::Error>>;
}
