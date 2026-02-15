// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// healthcheck
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

/// healthcheck_route
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckRoute {
    pub expected_status_codes: String,
    pub expected_body_contains: Option<String>,
    pub headers: Option<String>,
    /// Reference: healthcheck.id
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

/// healthcheck_response_time
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckResponseTime {
    /// Reference: healthcheck.id
    pub healthcheck_id: String,
    /// Reference: route.id
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

/// healthcheck_job_execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckJobExecution {
    /// Reference: healthcheck.id
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

/// healthcheck_event_rate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckEventRate {
    pub comparison: String,
    pub event_type: String,
    /// Reference: healthcheck.id
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

/// healthcheck_custom
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckCustom {
    /// Reference: healthcheck.id
    pub healthcheck_id: String,
    /// Reference: metric.id
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

/// healthcheck_result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckResult {
    #[serde(default = "chrono::Utc::now")]
    pub checked_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    /// Reference: healthcheck.id
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

/// healthcheck_alert
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckAlert {
    pub consecutive_failures: i64,
    pub cooldown_seconds: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: healthcheck.id
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

/// health_status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthStatus {
    pub service_name: String,
    pub healthy: bool,
    pub status: String,
    pub latency_ms: i64,
    pub details: String,
    pub checked_at: chrono::DateTime<chrono::Utc>,
}

impl HealthStatus {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// health_details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthDetails {
    pub message: String,
    pub error: String,
    pub extra: String,
}

impl HealthDetails {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// health_check
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthCheck {
    pub name: String,
    pub healthy: bool,
    pub latency_ms: i64,
    pub error: String,
}

impl HealthCheck {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// system_metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub active_connections: i64,
}

impl SystemMetrics {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

