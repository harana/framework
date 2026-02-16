// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryMetricsOutput {
    pub datapoints: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetMetricSummaryOutput {
    pub avg: f64,
    pub count: i64,
    pub max: f64,
    pub min: f64,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
    pub sum: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListMetricsOutput {
    pub metrics: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub tags: std::collections::HashMap<String, String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetricDatapoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetricInfo {
    pub name: String,
    pub type: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetricValue {
    pub labels: String,
    pub metric_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetricAlert {
    pub comparison: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    pub last_triggered_at: chrono::DateTime<chrono::Utc>,
    pub metric_id: String,
    pub notification_channel: String,
    pub threshold: f64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait MetricAction {
    async fn record_metric(&self, name: String, tags: std::collections::HashMap<String, String>, timestamp: chrono::DateTime<chrono::Utc>, value: f64) -> Result<(), Box<dyn std::error::Error>>;
    async fn increment_counter(&self, amount: f64, name: String, tags: std::collections::HashMap<String, String>) -> Result<f64, Box<dyn std::error::Error>>;
    async fn set_gauge(&self, name: String, tags: std::collections::HashMap<String, String>, value: f64) -> Result<(), Box<dyn std::error::Error>>;
    async fn record_histogram(&self, buckets: Vec<f64>, name: String, tags: std::collections::HashMap<String, String>, value: f64) -> Result<(), Box<dyn std::error::Error>>;
    async fn start_timer(&self, name: String, tags: std::collections::HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn stop_timer(&self, timer_id: String) -> Result<f64, Box<dyn std::error::Error>>;
    async fn query_metrics(&self, aggregation: String, end_time: chrono::DateTime<chrono::Utc>, interval: String, name: String, start_time: chrono::DateTime<chrono::Utc>, tags: std::collections::HashMap<String, String>) -> Result<QueryMetricsOutput, Box<dyn std::error::Error>>;
    async fn get_metric_summary(&self, name: String, period: String, tags: std::collections::HashMap<String, String>) -> Result<GetMetricSummaryOutput, Box<dyn std::error::Error>>;
    async fn list_metrics(&self, limit: i64, prefix: String) -> Result<ListMetricsOutput, Box<dyn std::error::Error>>;
    async fn delete_metric(&self, before: chrono::DateTime<chrono::Utc>, name: String, tags: std::collections::HashMap<String, String>) -> Result<i64, Box<dyn std::error::Error>>;
}
