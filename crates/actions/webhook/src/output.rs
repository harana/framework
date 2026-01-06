// Harana Actions - Webhook Module Output Types
// Auto-generated output structs for Webhook action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// register_webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterWebhookOutput {
    pub secret: String,
    pub success: bool,
    pub webhook_id: String,
}

// update_webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWebhookOutput {
    pub success: bool,
}

// unregister_webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnregisterWebhookOutput {
    pub success: bool,
}

// trigger_webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerWebhookOutput {
    pub success: bool,
    pub triggered_count: i32,
}

// get_webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWebhookOutput {
    pub active: bool,
    pub created_at: String, // datetime
    pub description: String,
    pub events: Vec<String>,
    pub url: String,
}

// list_webhooks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWebhooksOutput {
    pub total: i32,
    pub webhooks: Vec<HashMap<String, Value>>,
}

// test_webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestWebhookOutput {
    pub error: Option<String>,
    pub response_time_ms: i32,
    pub status_code: i32,
    pub success: bool,
}

// get_webhook_deliveries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWebhookDeliveriesOutput {
    pub deliveries: Vec<HashMap<String, Value>>,
    pub total: i32,
}

// retry_delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryDeliveryOutput {
    pub status_code: i32,
    pub success: bool,
}

// verify_signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifySignatureOutput {
    pub valid: bool,
}

// rotate_webhook_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateWebhookSecretOutput {
    pub new_secret: String,
    pub success: bool,
}

// register
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterOutput {
    pub webhook_id: String,
    pub secret: String,
    pub success: bool
}

// update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOutput {
    pub success: bool
}

// unregister
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnregisterOutput {
    pub success: bool
}

// trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerOutput {
    pub success: bool,
    pub triggered_count: i32
}

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub description: String,
    pub events: Vec<String>,
    pub url: String,
    pub created_at: String,
    pub active: bool
}

// lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListsOutput {
    pub total: i32,
    pub webhooks: Vec<HashMap<String, Value>>
}

// test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestOutput {
    pub status_code: i32,
    pub success: bool,
    pub error: String,
    pub response_time_ms: i32
}

// get_deliveries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDeliveriesOutput {
    pub deliveries: Vec<HashMap<String, Value>>,
    pub total: i32
}

// rotate_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateSecretOutput {
    pub success: bool,
    pub new_secret: String
}