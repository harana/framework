// Harana Actions - Cloudflare Queue Module Output Types

use serde::{Deserialize, Serialize};

// send
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendOutput {
    pub success: bool,
}

// send_batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBatchOutput {
    pub failed_messages: Vec<QueueFailedMessage>,
    pub success: bool,
}

// ack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AckOutput {
    pub success: bool,
}

// ack_all
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AckAllOutput {
    pub success: bool,
}

// retry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryOutput {
    pub success: bool,
}

// retry_all
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryAllOutput {
    pub success: bool,
}

// get_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessageOutput {
    pub body: serde_json::Value,
    pub id: String,
    pub timestamp: String,
}

// process_batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessBatchOutput {
    pub messages: Vec<QueueMessage>,
    pub queue: String,
}

// Helper structs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueBatchMessage {
    pub body: serde_json::Value,
    pub content_type: Option<String>,
    pub delay_seconds: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueFailedMessage {
    pub body: serde_json::Value,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueMessage {
    pub body: serde_json::Value,
    pub id: String,
    pub timestamp: String,
}
