// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// log_entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogEntry {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub level: String,
    pub message: String,
    pub metadata: Option<String>,
    pub request_id: Option<String>,
    pub source: Option<String>,
    pub span_id: Option<String>,
    pub trace_id: Option<String>,
    /// Reference: user.id
    pub user_id: Option<String>,
}

impl LogEntry {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// log_error
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogError {
    pub code: Option<String>,
    /// Reference: log_entry.id
    pub log_entry_id: String,
    pub message: String,
    pub stack_trace: Option<String>,
}

impl LogError {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// log_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogConfig {
    #[serde(default)]
    pub is_enabled: bool,
    pub level: String,
    pub max_retention_days: i64,
    pub source: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl LogConfig {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// log_context
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogContext {
    pub request_id: String,
    pub user_id: String,
    pub trace_id: String,
    pub span_id: String,
}

impl LogContext {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// log_data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogData {
    pub key: String,
    pub value: String,
}

impl LogData {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

