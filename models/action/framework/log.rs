// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DebugInput {
    pub context: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DebugOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InfoInput {
    pub context: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InfoOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WarnInput {
    pub context: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WarnOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ErrorInput {
    pub context: String,
    pub error: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ErrorOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StructuredInput {
    pub data: String,
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StructuredOutput {
    pub success: bool,
}

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

#[async_trait]
pub trait LogAction {
    async fn debug(&self, input: DebugInput) -> Result<DebugOutput, Box<dyn std::error::Error>>;
    async fn info(&self, input: InfoInput) -> Result<InfoOutput, Box<dyn std::error::Error>>;
    async fn warn(&self, input: WarnInput) -> Result<WarnOutput, Box<dyn std::error::Error>>;
    async fn error(&self, input: ErrorInput) -> Result<ErrorOutput, Box<dyn std::error::Error>>;
    async fn structured(&self, input: StructuredInput) -> Result<StructuredOutput, Box<dyn std::error::Error>>;
}
