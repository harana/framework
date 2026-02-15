// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WriteDataPointInput {
    pub binding: String,
    pub blobs: Vec<String>,
    pub doubles: Vec<f64>,
    pub indexes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WriteDataPointOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryInput {
    pub binding: String,
    pub query: String,
    pub time_end: chrono::DateTime<chrono::Utc>,
    pub time_start: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryOutput {
    pub data: Vec<String>,
    pub rows: i64,
}

#[async_trait]
pub trait AnalyticsEngineAction {
    async fn write_data_point(&self, input: WriteDataPointInput) -> Result<WriteDataPointOutput, Box<dyn std::error::Error>>;
    async fn query(&self, input: QueryInput) -> Result<QueryOutput, Box<dyn std::error::Error>>;
}
