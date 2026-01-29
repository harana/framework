//! Push Module Output Types
//! 
//! Output structs for push notification action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Output from send_apns action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendApnsOutput {
    /// Error message if the notification failed
    pub error: String,
    /// The APNS ID returned by Apple
    pub apns_id: String,
    /// Whether the notification was sent successfully
    pub success: bool
}

/// Output from send_fcm action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendFcmOutput {
    /// Whether the notification was sent successfully
    pub success: bool,
    /// Error message if the notification failed
    pub error: String,
    /// The message ID returned by FCM
    pub message_id: String
}

/// Output from send_multicast_push action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMulticastPushOutput {
    /// Number of failed sends
    pub failed: i32,
    /// Total number of tokens attempted
    pub total: i32,
    /// Details about each failure
    pub failures: Vec<HashMap<String, Value>>,
    /// Number of successful sends
    pub successful: i32
}

/// Output from send_topic_push action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTopicPushOutput {
    /// Whether the topic notification was sent successfully
    pub success: bool,
    /// The message ID returned by the push service
    pub message_id: String
}

/// Output from send_web_push action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendWebPushOutput {
    /// Error message if the notification failed
    pub error: String,
    /// Whether the notification was sent successfully
    pub success: bool
}

/// Output from subscribe_to_topic action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeToTopicOutput {
    /// Tokens that failed to subscribe
    pub failed_tokens: Vec<String>,
    /// Whether the subscription was successful overall
    pub success: bool
}

/// Output from unsubscribe_from_topic action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeFromTopicOutput {
    /// Tokens that failed to unsubscribe
    pub failed_tokens: Vec<String>,
    /// Whether the unsubscription was successful overall
    pub success: bool
}

/// Output from validate_push_token action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatePushTokenOutput {
    /// Whether the token is valid
    pub valid: bool,
    /// Error message if validation failed
    pub error: String
}
