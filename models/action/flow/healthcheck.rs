// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckServiceOutput {
    pub details: String,
    pub healthy: bool,
    pub latency_ms: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckDatabaseOutput {
    pub details: String,
    pub healthy: bool,
    pub latency_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckCacheOutput {
    pub details: String,
    pub healthy: bool,
    pub latency_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckExternalApiOutput {
    pub healthy: bool,
    pub latency_ms: i64,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckDiskSpaceOutput {
    pub free_bytes: i64,
    pub healthy: bool,
    pub percent_used: f64,
    pub total_bytes: i64,
    pub used_bytes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckMemoryOutput {
    pub free_bytes: i64,
    pub healthy: bool,
    pub percent_used: f64,
    pub total_bytes: i64,
    pub used_bytes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckAllOutput {
    pub checks: Vec<String>,
    pub failed_count: i64,
    pub healthy: bool,
    pub passed_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSystemStatusOutput {
    pub metrics: String,
    pub status: String,
    pub uptime_seconds: i64,
    pub version: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthDetails {
    pub message: String,
    pub error: String,
    pub extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthCheck {
    pub name: String,
    pub healthy: bool,
    pub latency_ms: i64,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub active_connections: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Healthcheck {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    #[serde(default)]
    pub is_active: bool,
    pub interval_seconds: i64,
    pub name: String,
    pub timeout_seconds: i64,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckRoute {
    pub expected_status_codes: String,
    pub expected_body_contains: String,
    pub headers: String,
    pub healthcheck_id: String,
    pub method: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckResponseTime {
    pub healthcheck_id: String,
    pub route_id: String,
    pub threshold_ms: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckJobExecution {
    pub healthcheck_id: String,
    pub job_name: String,
    pub max_duration_seconds: i64,
    pub max_failures_percent: f64,
    pub queue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckEventRate {
    pub comparison: String,
    pub event_type: String,
    pub healthcheck_id: String,
    pub source: String,
    pub threshold: f64,
    pub window: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckCustom {
    pub healthcheck_id: String,
    pub metric_id: String,
    pub query: String,
    pub success_condition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckResult {
    #[serde(default = "chrono::Utc::now")]
    pub checked_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: i64,
    pub error_message: String,
    pub healthcheck_id: String,
    pub metadata: String,
    pub status: String,
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
    pub last_triggered_at: chrono::DateTime<chrono::Utc>,
    pub notification_channel: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait HealthcheckAction {
    async fn check_service(&self, service_name: String, timeout: i64) -> Result<CheckServiceOutput, Box<dyn std::error::Error>>;
    async fn check_database(&self, connection_name: String, timeout: i64) -> Result<CheckDatabaseOutput, Box<dyn std::error::Error>>;
    async fn check_cache(&self, cache_name: String, timeout: i64) -> Result<CheckCacheOutput, Box<dyn std::error::Error>>;
    async fn check_external_api(&self, expected_status: i64, method: String, timeout: i64, url: String) -> Result<CheckExternalApiOutput, Box<dyn std::error::Error>>;
    async fn check_disk_space(&self, path: String, threshold_percent: i64) -> Result<CheckDiskSpaceOutput, Box<dyn std::error::Error>>;
    async fn check_memory(&self, threshold_percent: i64) -> Result<CheckMemoryOutput, Box<dyn std::error::Error>>;
    async fn check_all(&self, fail_fast: bool, timeout: i64) -> Result<CheckAllOutput, Box<dyn std::error::Error>>;
    async fn get_system_status(&self, include_metrics: bool) -> Result<GetSystemStatusOutput, Box<dyn std::error::Error>>;
}
