// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckServiceInput {
    pub service_name: String,
    pub timeout: i64,
}

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
pub struct CheckDatabaseInput {
    pub connection_name: String,
    pub timeout: i64,
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
pub struct CheckCacheInput {
    pub cache_name: String,
    pub timeout: i64,
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
pub struct CheckExternalApiInput {
    pub expected_status: i64,
    pub method: String,
    pub timeout: i64,
    pub url: String,
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
pub struct CheckDiskSpaceInput {
    pub path: String,
    pub threshold_percent: i64,
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
pub struct CheckMemoryInput {
    pub threshold_percent: i64,
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
pub struct CheckAllInput {
    #[serde(default)]
    pub fail_fast: bool,
    pub timeout: i64,
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
pub struct GetSystemStatusInput {
    #[serde(default)]
    pub include_metrics: bool,
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

#[async_trait]
pub trait HealthcheckAction {
    async fn check_service(&self, input: CheckServiceInput) -> Result<CheckServiceOutput, Box<dyn std::error::Error>>;
    async fn check_database(&self, input: CheckDatabaseInput) -> Result<CheckDatabaseOutput, Box<dyn std::error::Error>>;
    async fn check_cache(&self, input: CheckCacheInput) -> Result<CheckCacheOutput, Box<dyn std::error::Error>>;
    async fn check_external_api(&self, input: CheckExternalApiInput) -> Result<CheckExternalApiOutput, Box<dyn std::error::Error>>;
    async fn check_disk_space(&self, input: CheckDiskSpaceInput) -> Result<CheckDiskSpaceOutput, Box<dyn std::error::Error>>;
    async fn check_memory(&self, input: CheckMemoryInput) -> Result<CheckMemoryOutput, Box<dyn std::error::Error>>;
    async fn check_all(&self, input: CheckAllInput) -> Result<CheckAllOutput, Box<dyn std::error::Error>>;
    async fn get_system_status(&self, input: GetSystemStatusInput) -> Result<GetSystemStatusOutput, Box<dyn std::error::Error>>;
}
