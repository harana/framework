// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppStarted {
    pub app_id: String,
    pub app_name: String,
    pub version: String,
    pub environment: String,
    pub host: String,
    pub port: i64,
    #[serde(default = "chrono::Utc::now")]
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl AppStarted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppStopped {
    pub app_id: String,
    pub app_name: String,
    pub reason: String,
    pub exit_code: Option<i64>,
    pub uptime_seconds: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub stopped_at: chrono::DateTime<chrono::Utc>,
}

impl AppStopped {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppConfigUpdated {
    pub app_id: String,
    pub config_key: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub source: String,
    pub updated_by: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AppConfigUpdated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppConfigLoaded {
    pub app_id: String,
    pub config_path: Option<String>,
    pub source: String,
    pub keys_loaded: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub loaded_at: chrono::DateTime<chrono::Utc>,
}

impl AppConfigLoaded {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppHealthCheck {
    pub app_id: String,
    pub status: String,
    pub checks_passed: Option<i64>,
    pub checks_failed: Option<i64>,
    pub response_time_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub checked_at: chrono::DateTime<chrono::Utc>,
}

impl AppHealthCheck {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppReady {
    pub app_id: String,
    pub app_name: String,
    pub startup_time_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub ready_at: chrono::DateTime<chrono::Utc>,
}

impl AppReady {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppShuttingDown {
    pub app_id: String,
    pub reason: String,
    pub grace_period_ms: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub initiated_at: chrono::DateTime<chrono::Utc>,
}

impl AppShuttingDown {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppError {
    pub app_id: String,
    pub error_type: String,
    pub error_message: String,
    pub stack_trace: Option<String>,
    pub severity: String,
    #[serde(default = "chrono::Utc::now")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

impl AppError {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppConnectionPoolCreated {
    pub app_id: String,
    pub pool_name: String,
    pub pool_type: String,
    pub min_connections: Option<i64>,
    pub max_connections: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl AppConnectionPoolCreated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppConnectionPoolExhausted {
    pub app_id: String,
    pub pool_name: String,
    pub active_connections: i64,
    pub max_connections: i64,
    pub waiting_requests: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

impl AppConnectionPoolExhausted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

