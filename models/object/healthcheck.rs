// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckResponseTime {
    pub healthcheck_id: String,
    pub route_id: Option<String>,
    pub threshold_ms: i64,
    pub url: Option<String>,
}

impl HealthcheckResponseTime {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckCustom {
    pub healthcheck_id: String,
    pub metric_id: Option<String>,
    pub query: String,
    pub success_condition: String,
}

impl HealthcheckCustom {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthDetails {
    pub message: String,
    pub error: String,
    pub extra: std::collections::HashMap<String, String>,
}

impl HealthDetails {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthCheck {
    pub name: String,
    pub healthy: bool,
    pub latency_ms: i64,
    pub error: String,
}

impl HealthCheck {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub active_connections: i64,
}

impl SystemMetrics {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

