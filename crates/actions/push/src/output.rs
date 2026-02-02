//! Push Module Output Types
//! 
//! Output structs for push notification action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Output from send_apns action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendApnsOutput {
        pub error: String,
        pub apns_id: String,
        pub success: bool
}

/// Output from send_fcm action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendFcmOutput {
        pub success: bool,
        pub error: String,
        pub message_id: String
}

/// Output from send_multicast_push action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMulticastPushOutput {
        pub failed: i32,
        pub total: i32,
        pub failures: Vec<HashMap<String, Value>>,
        pub successful: i32
}

/// Output from send_topic_push action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTopicPushOutput {
        pub success: bool,
        pub message_id: String
}

/// Output from send_web_push action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendWebPushOutput {
        pub error: String,
        pub success: bool
}

/// Output from subscribe_to_topic action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeToTopicOutput {
        pub failed_tokens: Vec<String>,
        pub success: bool
}

/// Output from unsubscribe_from_topic action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeFromTopicOutput {
        pub failed_tokens: Vec<String>,
        pub success: bool
}

/// Output from validate_push_token action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatePushTokenOutput {
        pub valid: bool,
        pub error: String
}
