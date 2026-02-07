// Harana Actions - Webhook Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub events: Vec<String>,
    pub url: String,
    pub active: bool,
    pub created_at: String,
    pub description: String
}

// get_deliveries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDeliveriesOutput {
    pub deliveries: Vec<HashMap<String, Value>>,
    pub total: i32
}

// lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListsOutput {
    pub webhooks: Vec<HashMap<String, Value>>,
    pub total: i32
}

// register
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterOutput {
    pub success: bool,
    pub secret: String,
    pub webhook_id: String
}

// retry_delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryDeliveryOutput {
    pub status_code: i32,
    pub success: bool
}

// rotate_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateSecretOutput {
    pub new_secret: String,
    pub success: bool
}

// test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestOutput {
    pub success: bool,
    pub status_code: i32,
    pub error: String,
    pub response_time_ms: i32
}

// trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerOutput {
    pub success: bool,
    pub triggered_count: i32
}

// unregister
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnregisterOutput {
    pub success: bool
}

// update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOutput {
    pub success: bool
}

// verify_signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifySignatureOutput {
    pub valid: bool
}
