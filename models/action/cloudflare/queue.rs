// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendInput {
    pub content_type: String,
    pub delay_seconds: i64,
    pub message: String,
    pub queue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendBatchInput {
    pub messages: Vec<String>,
    pub queue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendBatchOutput {
    pub failed_messages: Vec<String>,
    pub success: bool,
}

#[async_trait]
pub trait QueueAction {
    async fn send(&self, input: SendInput) -> Result<SendOutput, Box<dyn std::error::Error>>;
    async fn send_batch(&self, input: SendBatchInput) -> Result<SendBatchOutput, Box<dyn std::error::Error>>;
}
