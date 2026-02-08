//! Push Module Output Types
//! 
//! Output structs for push notification action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendApnsOutput {
        pub error: String,
        pub apns_id: String,
        pub success: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendFcmOutput {
        pub success: bool,
        pub error: String,
        pub message_id: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMulticastPushOutput {
        pub failed: i32,
        pub total: i32,
        pub failures: Vec<HashMap<String, Value>>,
        pub successful: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTopicPushOutput {
        pub success: bool,
        pub message_id: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendWebPushOutput {
        pub error: String,
        pub success: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeToTopicOutput {
        pub failed_tokens: Vec<String>,
        pub success: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeFromTopicOutput {
        pub failed_tokens: Vec<String>,
        pub success: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatePushTokenOutput {
        pub valid: bool,
        pub error: String
}
