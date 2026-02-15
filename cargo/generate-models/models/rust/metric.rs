// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// metric
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Metric {
    pub description: Option<String>,
    pub labels: Option<String>,
    pub type: String,
    pub unit: Option<String>,
}

impl Metric {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// metric_value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetricValue {
    pub labels: Option<String>,
    /// Reference: metric.id
    pub metric_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub value: f64,
}

impl MetricValue {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// metric_alert
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetricAlert {
    pub comparison: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    pub last_triggered_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Reference: metric.id
    pub metric_id: String,
    pub notification_channel: Option<String>,
    pub threshold: f64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl MetricAlert {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// metric_datapoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetricDatapoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub value: f64,
}

impl MetricDatapoint {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// metric_info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetricInfo {
    pub name: String,
    pub type: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl MetricInfo {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

