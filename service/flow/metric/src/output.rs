// Harana Actions - Metric Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};

// delete_metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMetricOutput {
    pub deleted_count: i32,
}

// get_metric_summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetricSummaryOutput {
    pub min: f64,
    pub avg: f64,
    pub p95: f64,
    pub p99: f64,
    pub sum: f64,
    pub p50: f64,
    pub count: i32,
    pub max: f64,
}

// increment_counter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementCounterOutput {
    pub value: f64,
}

// list_metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMetricsOutput {
    pub metrics: Vec<MetricInfo>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricInfo {
    pub name: String,
    pub metric_type: String,
    pub tags: Vec<String>,
}

// query_metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMetricsOutput {
    pub datapoints: Vec<MetricDatapoint>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDatapoint {
    pub timestamp: i64,
    pub value: f64,
}

// record_histogram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordHistogramOutput {
    pub success: bool,
}

// record_metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordMetricOutput {
    pub success: bool,
}

// set_gauge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetGaugeOutput {
    pub success: bool,
}

// start_timer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartTimerOutput {
    pub timer_id: String,
}

// stop_timer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopTimerOutput {
    pub success: bool,
    pub duration_ms: f64,
}
