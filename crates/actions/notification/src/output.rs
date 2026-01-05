// Harana Actions - Notification Module Output Types
// Auto-generated output structs for Notification action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// send_push
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPushOutput {
    pub notification_id: String,
    pub success: bool,
}

// send_sms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendSmsOutput {
    pub message_id: String,
    pub success: bool,
}

// send_in_app
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendInAppOutput {
    pub notification_id: String,
    pub success: bool,
}

// send_bulk_notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBulkNotificationOutput {
    pub failed: i32,
    pub successful: i32,
    pub total: i32,
}

// get_notification_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNotificationStatusOutput {
    pub delivered_at: Option<String>, // datetime
    pub error: Option<String>,
    pub read_at: Option<String>, // datetime
    pub sent_at: Option<String>, // datetime
    pub status: String,          // pending | sent | delivered | read | failed
}

// mark_as_read
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkAsReadOutput {
    pub success: bool,
}

// list_notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListNotificationsOutput {
    pub notifications: Vec<HashMap<String, Value>>,
    pub total: i32,
    pub unread_count: i32,
}

// delete_notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteNotificationOutput {
    pub success: bool,
}

// register_device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterDeviceOutput {
    pub success: bool,
}

// unregister_device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnregisterDeviceOutput {
    pub success: bool,
}
