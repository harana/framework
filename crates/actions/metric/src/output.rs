// Harana Actions - Metric Module Output Types
// Auto-generated output structs for Metric action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// record_metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordMetricOutput {
    pub success: bool,
}

// increment_counter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementCounterOutput {
    pub value: f32,
}

// set_gauge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetGaugeOutput {
    pub success: bool,
}

// record_histogram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordHistogramOutput {
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
    pub duration_ms: f32,
    pub success: bool,
}

// query_metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMetricsOutput {
    pub datapoints: Vec<HashMap<String, Value>>,
    pub total: i32,
}

// get_metric_summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetricSummaryOutput {
    pub avg: f32,
    pub count: i32,
    pub max: f32,
    pub min: f32,
    pub p50: f32,
    pub p95: f32,
    pub p99: f32,
    pub sum: f32,
}

// list_metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMetricsOutput {
    pub metrics: Vec<HashMap<String, Value>>,
    pub total: i32,
}

// delete_metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMetricOutput {
    pub deleted_count: i32,
    pub success: bool,
}
