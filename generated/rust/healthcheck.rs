// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Healthcheck
/// Class: healthcheck
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Healthcheck {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub interval_seconds: i64,
    pub name: String,
    pub timeout_seconds: i64,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Healthcheck {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Healthcheck Route
/// Class: healthcheck_route
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckRoute {
    pub expected_status_codes: String,
    pub expected_body_contains: Option<String>,
    pub headers: Option<String>,
    pub healthcheck_id: String,
    pub method: String,
    pub url: String,
}

impl HealthcheckRoute {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Healthcheck Response Time
/// Class: healthcheck_response_time
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckResponseTime {
    pub healthcheck_id: String,
    /// Reference: Route.id
    pub route_id: Option<String>,
    pub threshold_ms: i64,
    pub url: Option<String>,
}

impl HealthcheckResponseTime {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Healthcheck Job Execution
/// Class: healthcheck_job_execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckJobExecution {
    pub healthcheck_id: String,
    pub job_name: String,
    pub max_duration_seconds: Option<i64>,
    pub max_failures_percent: Option<f64>,
    pub queue: Option<String>,
}

impl HealthcheckJobExecution {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Healthcheck Event Rate
/// Class: healthcheck_event_rate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckEventRate {
    pub comparison: String,
    pub event_type: String,
    pub healthcheck_id: String,
    pub source: Option<String>,
    pub threshold: f64,
    pub window: String,
}

impl HealthcheckEventRate {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Healthcheck Custom
/// Class: healthcheck_custom
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckCustom {
    pub healthcheck_id: String,
    /// Reference: Metric.id
    pub metric_id: Option<String>,
    pub query: String,
    pub success_condition: String,
}

impl HealthcheckCustom {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Healthcheck Result
/// Class: healthcheck_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckResult {
    #[serde(default = "chrono::Utc::now")]
    pub checked_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    pub healthcheck_id: String,
    pub metadata: Option<String>,
    pub status: String,
}

impl HealthcheckResult {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Healthcheck Alert
/// Class: healthcheck_alert
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckAlert {
    pub consecutive_failures: i64,
    pub cooldown_seconds: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub healthcheck_id: String,
    #[serde(default)]
    pub is_active: bool,
    pub last_triggered_at: Option<chrono::DateTime<chrono::Utc>>,
    pub notification_channel: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl HealthcheckAlert {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

