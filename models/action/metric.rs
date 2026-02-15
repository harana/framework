// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordMetricInput {
    pub name: String,
    pub tags: std::collections::HashMap<String, String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordMetricOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IncrementCounterInput {
    pub amount: f64,
    pub name: String,
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IncrementCounterOutput {
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetGaugeInput {
    pub name: String,
    pub tags: std::collections::HashMap<String, String>,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetGaugeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordHistogramInput {
    pub buckets: Vec<f64>,
    pub name: String,
    pub tags: std::collections::HashMap<String, String>,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RecordHistogramOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartTimerInput {
    pub name: String,
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StartTimerOutput {
    pub timer_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StopTimerInput {
    pub timer_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StopTimerOutput {
    pub duration_ms: f64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryMetricsInput {
    pub aggregation: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub interval: String,
    pub name: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryMetricsOutput {
    pub datapoints: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetMetricSummaryInput {
    pub name: String,
    pub period: String,
    pub tags: std::collections::HashMap<String, String>,
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
pub struct ListMetricsInput {
    pub limit: i64,
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListMetricsOutput {
    pub metrics: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMetricInput {
    pub before: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteMetricOutput {
    pub deleted_count: i64,
    pub success: bool,
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

#[async_trait]
pub trait MetricAction {
    async fn record_metric(&self, input: RecordMetricInput) -> Result<RecordMetricOutput, Box<dyn std::error::Error>>;
    async fn increment_counter(&self, input: IncrementCounterInput) -> Result<IncrementCounterOutput, Box<dyn std::error::Error>>;
    async fn set_gauge(&self, input: SetGaugeInput) -> Result<SetGaugeOutput, Box<dyn std::error::Error>>;
    async fn record_histogram(&self, input: RecordHistogramInput) -> Result<RecordHistogramOutput, Box<dyn std::error::Error>>;
    async fn start_timer(&self, input: StartTimerInput) -> Result<StartTimerOutput, Box<dyn std::error::Error>>;
    async fn stop_timer(&self, input: StopTimerInput) -> Result<StopTimerOutput, Box<dyn std::error::Error>>;
    async fn query_metrics(&self, input: QueryMetricsInput) -> Result<QueryMetricsOutput, Box<dyn std::error::Error>>;
    async fn get_metric_summary(&self, input: GetMetricSummaryInput) -> Result<GetMetricSummaryOutput, Box<dyn std::error::Error>>;
    async fn list_metrics(&self, input: ListMetricsInput) -> Result<ListMetricsOutput, Box<dyn std::error::Error>>;
    async fn delete_metric(&self, input: DeleteMetricInput) -> Result<DeleteMetricOutput, Box<dyn std::error::Error>>;
}
