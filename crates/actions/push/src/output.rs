// Harana Actions - Push Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// send_apns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendApnsOutput {
    pub error: String,
    pub apns_id: String,
    pub success: bool
}

// send_fcm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendFcmOutput {
    pub success: bool,
    pub error: String,
    pub message_id: String
}

// send_multicast_push
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMulticastPushOutput {
    pub failed: i32,
    pub total: i32,
    pub failures: Vec<HashMap<String, Value>>,
    pub successful: i32
}

// send_topic_push
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTopicPushOutput {
    pub success: bool,
    pub message_id: String
}

// send_web_push
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendWebPushOutput {
    pub error: String,
    pub success: bool
}

// subscribe_to_topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeToTopicOutput {
    pub failed_tokens: Vec<String>,
    pub success: bool
}

// unsubscribe_from_topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeFromTopicOutput {
    pub failed_tokens: Vec<String>,
    pub success: bool
}

// validate_push_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatePushTokenOutput {
    pub valid: bool,
    pub error: String
}
