// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SliderChanged {
    pub slider_id: String,
    pub slider_name: Option<String>,
    pub old_value: Option<f64>,
    pub new_value: f64,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub step: Option<f64>,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl SliderChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SliderDragStarted {
    pub slider_id: String,
    pub slider_name: Option<String>,
    pub start_value: f64,
    #[serde(default = "chrono::Utc::now")]
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl SliderDragStarted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SliderDragEnded {
    pub slider_id: String,
    pub slider_name: Option<String>,
    pub start_value: Option<f64>,
    pub end_value: f64,
    pub drag_duration_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub ended_at: chrono::DateTime<chrono::Utc>,
}

impl SliderDragEnded {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RangeAdjusted {
    pub slider_id: String,
    pub slider_name: Option<String>,
    pub old_min: Option<f64>,
    pub old_max: Option<f64>,
    pub new_min: f64,
    pub new_max: f64,
    #[serde(default = "chrono::Utc::now")]
    pub adjusted_at: chrono::DateTime<chrono::Utc>,
}

impl RangeAdjusted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

