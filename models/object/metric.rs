// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Metric {
    pub description: Option<String>,
    pub labels: Option<String>,
    pub type: String,
    pub unit: Option<String>,
}

impl Metric {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetricValue {
    pub labels: Option<String>,
    pub metric_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub value: f64,
}

impl MetricValue {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetricAlert {
    pub comparison: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    pub last_triggered_at: Option<chrono::DateTime<chrono::Utc>>,
    pub metric_id: String,
    pub notification_channel: Option<String>,
    pub threshold: f64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl MetricAlert {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetricDatapoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub value: f64,
}

impl MetricDatapoint {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MetricInfo {
    pub name: String,
    pub type: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl MetricInfo {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

