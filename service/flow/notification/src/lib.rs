// Harana Actions - Notification Module
// This module provides notification actions and functionality.

pub mod output;

use chrono::Utc;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use output::*;
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

/// Storage for notifications
static NOTIFICATIONS: Lazy<DashMap<String, StoredNotification>> = Lazy::new(DashMap::new);
/// Index by user_id for quick lookup
static USER_NOTIFICATIONS: Lazy<DashMap<String, Vec<String>>> = Lazy::new(DashMap::new);
/// Storage for device tokens
static DEVICE_TOKENS: Lazy<DashMap<String, DeviceRegistration>> = Lazy::new(DashMap::new);
/// Index by user_id for device lookup
static USER_DEVICES: Lazy<DashMap<String, Vec<String>>> = Lazy::new(DashMap::new);

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct StoredNotification {
    notification_id: String,
    user_id: String,
    title: String,
    body: String,
    notification_type: String,
    channel: String,
    data: Option<HashMap<String, Value>>,
    status: String,
    sent_at: String,
    delivered_at: Option<String>,
    read_at: Option<String>,
    error: Option<String>,
    action_url: Option<String>,
    badge: Option<i32>,
    sound: Option<String>,
    priority: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct DeviceRegistration {
    device_token: String,
    user_id: String,
    platform: String,
    registered_at: String,
}

/// Send push notification
pub async fn send_push(
    user_id: &str,
    title: &str,
    body: &str,
    data: Option<HashMap<String, Value>>,
    badge: Option<i32>,
    sound: Option<&str>,
    priority: Option<&str>,
) -> Result<SendPushOutput, String> {
    let notification_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let notification = StoredNotification {
        notification_id: notification_id.clone(),
        user_id: user_id.to_string(),
        title: title.to_string(),
        body: body.to_string(),
        notification_type: "push".to_string(),
        channel: "push".to_string(),
        data: data.clone(),
        status: "sent".to_string(),
        sent_at: now.clone(),
        delivered_at: Some(now),
        read_at: None,
        error: None,
        action_url: None,
        badge,
        sound: sound.map(|s| s.to_string()),
        priority: priority.map(|p| p.to_string()),
    };
    
    NOTIFICATIONS.insert(notification_id.clone(), notification);
    
    USER_NOTIFICATIONS
        .entry(user_id.to_string())
        .or_insert_with(Vec::new)
        .push(notification_id.clone());
    
    Ok(SendPushOutput {
        notification_id,
        success: true,
    })
}

/// Send SMS message
pub async fn send_sms(
    phone_number: &str,
    message: &str,
    sender_id: Option<&str>,
) -> Result<SendSmsOutput, String> {
    let message_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let notification = StoredNotification {
        notification_id: message_id.clone(),
        user_id: phone_number.to_string(),
        title: sender_id.unwrap_or("SMS").to_string(),
        body: message.to_string(),
        notification_type: "sms".to_string(),
        channel: "sms".to_string(),
        data: None,
        status: "sent".to_string(),
        sent_at: now.clone(),
        delivered_at: Some(now),
        read_at: None,
        error: None,
        action_url: None,
        badge: None,
        sound: None,
        priority: None,
    };
    
    NOTIFICATIONS.insert(message_id.clone(), notification);
    
    Ok(SendSmsOutput {
        message_id,
        success: true,
    })
}

/// Send in-app notification
pub async fn send_in_app(
    user_id: &str,
    title: &str,
    message: &str,
    notification_type: Option<&str>,
    action_url: Option<&str>,
    data: Option<HashMap<String, Value>>,
) -> Result<SendInAppOutput, String> {
    let notification_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let notification = StoredNotification {
        notification_id: notification_id.clone(),
        user_id: user_id.to_string(),
        title: title.to_string(),
        body: message.to_string(),
        notification_type: notification_type.unwrap_or("info").to_string(),
        channel: "in_app".to_string(),
        data: data.clone(),
        status: "sent".to_string(),
        sent_at: now.clone(),
        delivered_at: Some(now),
        read_at: None,
        error: None,
        action_url: action_url.map(|s| s.to_string()),
        badge: None,
        sound: None,
        priority: None,
    };
    
    NOTIFICATIONS.insert(notification_id.clone(), notification);
    
    USER_NOTIFICATIONS
        .entry(user_id.to_string())
        .or_insert_with(Vec::new)
        .push(notification_id.clone());
    
    Ok(SendInAppOutput {
        notification_id,
        success: true,
    })
}

/// Send bulk notification
pub async fn send_bulk(
    user_ids: Vec<&str>,
    title: &str,
    body: &str,
    channel: Option<&str>,
    data: Option<HashMap<String, Value>>,
) -> Result<SendBulkNotificationOutput, String> {
    let channel = channel.unwrap_or("push");
    let total = user_ids.len() as i32;
    let mut successful = 0;
    
    for user_id in user_ids {
        let notification_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        
        let notification = StoredNotification {
            notification_id: notification_id.clone(),
            user_id: user_id.to_string(),
            title: title.to_string(),
            body: body.to_string(),
            notification_type: channel.to_string(),
            channel: channel.to_string(),
            data: data.clone(),
            status: "sent".to_string(),
            sent_at: now.clone(),
            delivered_at: Some(now),
            read_at: None,
            error: None,
            action_url: None,
            badge: None,
            sound: None,
            priority: None,
        };
        
        NOTIFICATIONS.insert(notification_id.clone(), notification);
        
        USER_NOTIFICATIONS
            .entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .push(notification_id.clone());
        
        successful += 1;
    }
    
    Ok(SendBulkNotificationOutput {
        total,
        successful,
        failed: 0,
    })
}

/// Get notification status
pub async fn status(
    notification_id: &str,
) -> Result<GetNotificationStatusOutput, String> {
    let notification = NOTIFICATIONS
        .get(notification_id)
        .ok_or_else(|| "Notification not found".to_string())?;
    
    Ok(GetNotificationStatusOutput {
        status: notification.status.clone(),
        sent_at: Some(notification.sent_at.clone()),
        delivered_at: notification.delivered_at.clone(),
        read_at: notification.read_at.clone(),
        error: notification.error.clone(),
    })
}

/// Mark notification as read
pub async fn mark_as_read(
    notification_id: &str,
) -> Result<MarkAsReadOutput, String> {
    if let Some(mut notification) = NOTIFICATIONS.get_mut(notification_id) {
        notification.read_at = Some(Utc::now().to_rfc3339());
        notification.status = "read".to_string();
        Ok(MarkAsReadOutput { success: true })
    } else {
        Err("Notification not found".to_string())
    }
}

/// List user notifications
pub async fn list(
    user_id: &str,
    unread_only: Option<bool>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<ListNotificationsOutput, String> {
    let unread_only = unread_only.unwrap_or(false);
    let limit = limit.unwrap_or(100).max(1) as usize;
    let offset = offset.unwrap_or(0).max(0) as usize;
    
    let notification_ids = USER_NOTIFICATIONS
        .get(user_id)
        .map(|ids| ids.clone())
        .unwrap_or_default();
    
    let notifications: Vec<HashMap<String, Value>> = notification_ids
        .iter()
        .filter_map(|id| NOTIFICATIONS.get(id).map(|n| n.clone()))
        .filter(|n| !unread_only || n.read_at.is_none())
        .skip(offset)
        .take(limit)
        .map(|n| {
            let mut map = HashMap::new();
            map.insert("notification_id".to_string(), json!(n.notification_id));
            map.insert("title".to_string(), json!(n.title));
            map.insert("body".to_string(), json!(n.body));
            map.insert("type".to_string(), json!(n.notification_type));
            map.insert("status".to_string(), json!(n.status));
            map.insert("sent_at".to_string(), json!(n.sent_at));
            if let Some(read_at) = &n.read_at {
                map.insert("read_at".to_string(), json!(read_at));
            }
            if let Some(action_url) = &n.action_url {
                map.insert("action_url".to_string(), json!(action_url));
            }
            if let Some(data) = &n.data {
                map.insert("data".to_string(), json!(data));
            }
            map
        })
        .collect();
    
    let total = notifications.len() as i32;
    let unread_count = notification_ids
        .iter()
        .filter_map(|id| NOTIFICATIONS.get(id))
        .filter(|n| n.read_at.is_none())
        .count() as i32;
    
    Ok(ListNotificationsOutput {
        notifications,
        total,
        unread_count,
    })
}

/// Delete notification
pub async fn delete(
    notification_id: &str,
) -> Result<DeleteNotificationOutput, String> {
    if let Some((_, notification)) = NOTIFICATIONS.remove(notification_id) {
        // Remove from user index
        if let Some(mut user_notifs) = USER_NOTIFICATIONS.get_mut(&notification.user_id) {
            user_notifs.retain(|id| id != notification_id);
        }
        Ok(DeleteNotificationOutput { success: true })
    } else {
        Err("Notification not found".to_string())
    }
}

/// Register device token
pub async fn register_device(
    user_id: &str,
    device_token: &str,
    platform: &str,
) -> Result<RegisterDeviceOutput, String> {
    let registration = DeviceRegistration {
        device_token: device_token.to_string(),
        user_id: user_id.to_string(),
        platform: platform.to_string(),
        registered_at: Utc::now().to_rfc3339(),
    };
    
    DEVICE_TOKENS.insert(device_token.to_string(), registration);
    
    USER_DEVICES
        .entry(user_id.to_string())
        .or_insert_with(Vec::new)
        .push(device_token.to_string());
    
    Ok(RegisterDeviceOutput { success: true })
}

/// Unregister device token
pub async fn unregister_device(
    device_token: &str,
) -> Result<UnregisterDeviceOutput, String> {
    if let Some((_, registration)) = DEVICE_TOKENS.remove(device_token) {
        // Remove from user index
        if let Some(mut user_devices) = USER_DEVICES.get_mut(&registration.user_id) {
            user_devices.retain(|token| token != device_token);
        }
        Ok(UnregisterDeviceOutput { success: true })
    } else {
        Err("Device token not found".to_string())
    }
}

#[cfg(test)]
mod tests;
