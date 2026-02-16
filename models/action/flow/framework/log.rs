// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogEntry {
    pub level: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub context: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogContext {
    pub request_id: String,
    pub user_id: String,
    pub trace_id: String,
    pub span_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogError {
    pub message: String,
    pub stack_trace: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogData {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogEntry {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub level: String,
    pub message: String,
    pub metadata: String,
    pub request_id: String,
    pub source: String,
    pub span_id: String,
    pub trace_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogError {
    pub code: String,
    pub log_entry_id: String,
    pub message: String,
    pub stack_trace: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogContext {
    pub request_id: String,
    pub user_id: String,
    pub trace_id: String,
    pub span_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogData {
    pub key: String,
    pub value: String,
}

#[async_trait]
pub trait LogAction {
    async fn debug(&self, context: String, message: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn info(&self, context: String, message: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn warn(&self, context: String, message: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn error(&self, context: String, error: String, message: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn structured(&self, data: String, level: String) -> Result<(), Box<dyn std::error::Error>>;
}
