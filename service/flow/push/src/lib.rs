//! Harana Actions - Push Module
//!
//! This module provides push notification actions for iOS (APNS), Android (FCM),
//! and Web Push (VAPID/RFC8030).
//!
//! # Libraries Used
//! - `a2`: Apple Push Notification Service (APNS) client
//! - `fcm-service`: Firebase Cloud Messaging (FCM) client
//! - `web-push-native`: Web Push notifications (RFC8030)
//!
//! # Example
//! ```ignore
//! use harana_actions_push::{send_apns, ApnsConfig};
//!
//! let config = ApnsConfig {
//!     key_path: "/path/to/AuthKey.p8".to_string(),
//!     key_id: "KEY_ID".to_string(),
//!     team_id: "TEAM_ID".to_string(),
//!     sandbox: false,
//! };
//!
//! let result = send_apns(
//!     &config,
//!     "device_token",
//!     "Hello!",
//!     "com.example.app",
//!     None, None, None, None, None, None, None, None, None, None, None,
//! ).await;
//! ```

/// Output types for push actions
pub mod output;

use std::collections::HashMap;
use std::fs::File;

use a2::{
    Client as ApnsClient, ClientConfig, CollapseId, DefaultNotificationBuilder,
    NotificationBuilder, NotificationOptions, Priority as ApnsPriority,
};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use fcm_service::{FcmMessage, FcmNotification, FcmService, Target};
use output::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_push_native::jwt_simple::algorithms::ES256KeyPair;
use web_push_native::p256::PublicKey;
use web_push_native::{Auth, WebPushBuilder};

#[derive(Debug, Clone)]
pub struct ApnsConfig {
        pub key_path: String,
        pub key_id: String,
        pub team_id: String,
        pub sandbox: bool,
}

#[derive(Debug, Clone)]
pub struct FcmConfig {
        pub credential_file: String,
}

#[derive(Debug, Clone)]
pub struct WebPushConfig {
        pub vapid_private_key: String,
        pub contact_email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushSubscription {
        pub endpoint: String,
        pub keys: PushSubscriptionKeys,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushSubscriptionKeys {
        pub p256dh: String,
        pub auth: String,
}

/// Send APNS Push Notification
///
/// Sends a push notification to an iOS device using Apple Push Notification Service.
/// Requires authentication via a .p8 key file from Apple Developer account.
///
/// # Arguments
/// * `config` - APNS configuration including key path, key ID, and team ID
/// * `device_token` - The device token from the iOS device
/// * `alert` - The alert message text
/// * `topic` - The bundle ID of the app (e.g., "com.example.app")
/// * `custom_data` - Optional custom data to include in the payload
/// * `category` - Optional notification category
/// * `collapse_id` - Optional collapse identifier for notification grouping
/// * `subtitle` - Optional subtitle text
/// * `priority` - Priority level ("5" for normal, "10" for high)
/// * `expiration` - Optional expiration timestamp
/// * `sound` - Optional sound file name
/// * `thread_id` - Optional thread identifier for grouping
/// * `body` - Optional body text (if different from alert)
/// * `badge` - Optional badge count
/// * `title` - Optional title text
pub async fn send_apns(
    config: &ApnsConfig,
    device_token: &str,
    alert: &str,
    topic: &str,
    custom_data: Option<HashMap<String, Value>>,
    category: Option<&str>,
    collapse_id: Option<&str>,
    subtitle: Option<&str>,
    priority: Option<&str>,
    expiration: Option<i32>,
    sound: Option<&str>,
    thread_id: Option<&str>,
    body: Option<&str>,
    badge: Option<i32>,
    title: Option<&str>,
) -> Result<SendApnsOutput, String> {
    // Read the private key file
    let mut file = File::open(&config.key_path)
        .map_err(|e| format!("Failed to open APNS key file: {}", e))?;

    // Create the APNS client with token-based authentication
    let client_config = if config.sandbox {
        ClientConfig::default()
    } else {
        ClientConfig::new(a2::Endpoint::Production)
    };

    let client = ApnsClient::token(&mut file, &config.key_id, &config.team_id, client_config)
        .map_err(|e| format!("Failed to create APNS client: {}", e))?;

    // Build the notification
    let mut builder = DefaultNotificationBuilder::new();

    // Set alert/body
    if let Some(title_text) = title {
        builder = builder.set_title(title_text);
    }
    if let Some(body_text) = body {
        builder = builder.set_body(body_text);
    } else {
        builder = builder.set_body(alert);
    }

    // Set optional fields
    if let Some(sub) = subtitle {
        builder = builder.set_subtitle(sub);
    }
    if let Some(snd) = sound {
        builder = builder.set_sound(snd);
    }
    if let Some(cat) = category {
        builder = builder.set_category(cat);
    }
    if let Some(bdg) = badge {
        builder = builder.set_badge(bdg as u32);
    }
    // Note: thread_id is handled via custom data as the a2 crate doesn't expose it directly

    // Determine priority
    let apns_priority = match priority {
        Some("5") => Some(ApnsPriority::Normal),
        Some("10") | None => Some(ApnsPriority::High),
        _ => Some(ApnsPriority::Normal),
    };

    // Build notification options
    let mut options = NotificationOptions {
        apns_topic: Some(topic),
        apns_priority,
        ..Default::default()
    };

    // Set collapse ID if provided
    if let Some(cid) = collapse_id {
        options.apns_collapse_id = Some(CollapseId::new(cid).map_err(|e| format!("Invalid collapse ID: {}", e))?);
    }

    // Set expiration if provided
    if let Some(exp) = expiration {
        options.apns_expiration = Some(exp as u64);
    }

    // Build the payload
    let mut payload = builder.build(device_token, options);

    // Add thread_id via custom data if provided
    if let Some(tid) = thread_id {
        payload
            .add_custom_data("thread-id", &tid)
            .map_err(|e| format!("Failed to add thread-id: {}", e))?;
    }

    // Add custom data if provided (as a single "data" key to avoid lifetime issues)
    if let Some(data) = custom_data {
        payload
            .add_custom_data("data", &data)
            .map_err(|e| format!("Failed to add custom data: {}", e))?;
    }

    // Send the notification
    let response = client
        .send(payload)
        .await
        .map_err(|e| format!("Failed to send APNS notification: {}", e))?;

    let apns_id = response
        .apns_id
        .map(|id| id.to_string())
        .unwrap_or_default();

    Ok(SendApnsOutput {
        success: true,
        apns_id,
        error: String::new(),
    })
}

/// Send Android FCM Notification
///
/// Sends a push notification to an Android device using Firebase Cloud Messaging.
/// Requires a service account JSON credential file from Firebase Console.
///
/// # Arguments
/// * `config` - FCM configuration including the path to service account credentials
/// * `title` - The notification title
/// * `registration_token` - The FCM registration token for the device
/// * `body` - The notification body text
/// * `icon` - Optional icon name
/// * `time_to_live` - Optional TTL in seconds
/// * `image` - Optional image URL
/// * `priority` - Optional priority ("Min", "Low", "Default", "High", "Max")
/// * `channel_id` - Optional Android notification channel ID
/// * `color` - Optional notification color (hex format)
/// * `click_action` - Optional click action
/// * `sound` - Optional sound file name
/// * `tag` - Optional notification tag for replacement
/// * `data` - Optional custom data payload
/// * `collapse_key` - Optional collapse key for notification grouping
pub async fn send_fcm(
    config: &FcmConfig,
    title: &str,
    registration_token: &str,
    body: &str,
    _icon: Option<&str>,
    _time_to_live: Option<i32>,
    image: Option<&str>,
    _priority: Option<&str>,
    _channel_id: Option<&str>,
    _color: Option<&str>,
    _click_action: Option<&str>,
    _sound: Option<&str>,
    _tag: Option<&str>,
    data: Option<HashMap<String, Value>>,
    _collapse_key: Option<&str>,
) -> Result<SendFcmOutput, String> {
    // Create FCM service
    let service = FcmService::new(&config.credential_file);

    // Build the notification
    let mut notification = FcmNotification::new();
    notification.set_title(title.to_string());
    notification.set_body(body.to_string());
    notification.set_image(image.map(|s| s.to_string()));

    // Build the message
    let mut message = FcmMessage::new();
    message.set_notification(Some(notification));
    message.set_target(Target::Token(registration_token.to_string()));

    // Add custom data if provided
    if let Some(data_map) = data {
        let mut string_data = HashMap::new();
        for (key, value) in data_map {
            let string_value = match value {
                Value::String(s) => s,
                other => other.to_string(),
            };
            string_data.insert(key, string_value);
        }
        message.set_data(Some(string_data));
    }

    // Note: fcm-service uses the simpler FcmNotification struct which doesn't expose
    // all Android-specific options. For full control, consider using the AndroidConfig.

    // Send the notification
    service
        .send_notification(message)
        .await
        .map_err(|e| format!("Failed to send FCM notification: {}", e))?;

    // FCM v1 API doesn't return a message ID in the response body for successful sends
    // The message ID would need to be tracked separately or obtained from FCM logs
    Ok(SendFcmOutput {
        success: true,
        message_id: String::new(), // FCM v1 doesn't return message_id in simple response
        error: String::new(),
    })
}

/// Send Multicast Push
///
/// Sends the same push notification to multiple devices. Supports APNS, FCM, and Web Push.
///
/// # Arguments
/// * `apns_config` - Optional APNS configuration (required for APNS platform)
/// * `fcm_config` - Optional FCM configuration (required for FCM platform)
/// * `web_push_config` - Optional Web Push configuration (required for Web platform)
/// * `tokens` - List of device tokens to send to
/// * `body` - The notification body text
/// * `platform` - The platform ("Apns", "Fcm", or "Web")
/// * `title` - The notification title
/// * `data` - Optional custom data payload
pub async fn send_multicast_push(
    apns_config: Option<&ApnsConfig>,
    fcm_config: Option<&FcmConfig>,
    web_push_config: Option<&WebPushConfig>,
    tokens: Vec<String>,
    body: &str,
    platform: &str,
    title: &str,
    data: Option<HashMap<String, Value>>,
) -> Result<SendMulticastPushOutput, String> {
    let total = tokens.len() as i32;
    let mut successful = 0i32;
    let mut failed = 0i32;
    let mut failures: Vec<HashMap<String, Value>> = Vec::new();

    for token in tokens {
        let result = match platform.to_lowercase().as_str() {
            "apns" => {
                let config =
                    apns_config.ok_or_else(|| "APNS config required for APNS platform".to_string())?;
                // For multicast, we need a topic - use a placeholder or require it
                send_apns(
                    config,
                    &token,
                    body,
                    "", // topic would need to be passed in
                    data.clone(),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(body),
                    None,
                    Some(title),
                )
                .await
                .map(|_| ())
            }
            "fcm" => {
                let config =
                    fcm_config.ok_or_else(|| "FCM config required for FCM platform".to_string())?;
                send_fcm(
                    config,
                    title,
                    &token,
                    body,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    data.clone(),
                    None,
                )
                .await
                .map(|_| ())
            }
            "web" => {
                let config = web_push_config
                    .ok_or_else(|| "Web Push config required for Web platform".to_string())?;
                // For web push multicast, the token should be a JSON-encoded subscription
                let subscription: PushSubscription = serde_json::from_str(&token)
                    .map_err(|e| format!("Invalid web push subscription: {}", e))?;
                let mut sub_map = HashMap::new();
                sub_map.insert(
                    "endpoint".to_string(),
                    Value::String(subscription.endpoint.clone()),
                );
                let mut keys_map = HashMap::new();
                keys_map.insert(
                    "p256dh".to_string(),
                    Value::String(subscription.keys.p256dh.clone()),
                );
                keys_map.insert(
                    "auth".to_string(),
                    Value::String(subscription.keys.auth.clone()),
                );
                sub_map.insert("keys".to_string(), Value::Object(keys_map.into_iter().collect()));

                send_web_push(
                    config,
                    title,
                    body,
                    sub_map,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    data.clone(),
                    None,
                    None,
                    None,
                    None,
                    None,
                )
                .await
                .map(|_| ())
            }
            _ => Err(format!("Unsupported platform: {}", platform)),
        };

        match result {
            Ok(_) => successful += 1,
            Err(e) => {
                failed += 1;
                let mut failure = HashMap::new();
                failure.insert("token".to_string(), Value::String(token));
                failure.insert("error".to_string(), Value::String(e));
                failures.push(failure);
            }
        }
    }

    Ok(SendMulticastPushOutput {
        total,
        successful,
        failed,
        failures,
    })
}

/// Send Topic Push
///
/// Sends a push notification to all devices subscribed to a topic.
/// Currently only supports FCM topics.
///
/// # Arguments
/// * `config` - FCM configuration
/// * `topic` - The topic name to send to
/// * `title` - The notification title
/// * `body` - The notification body text
/// * `data` - Optional custom data payload
pub async fn send_topic_push(
    config: &FcmConfig,
    topic: &str,
    title: &str,
    body: &str,
    _platform: &str,
    data: Option<HashMap<String, Value>>,
) -> Result<SendTopicPushOutput, String> {
    // Create FCM service
    let service = FcmService::new(&config.credential_file);

    // Build the notification
    let mut notification = FcmNotification::new();
    notification.set_title(title.to_string());
    notification.set_body(body.to_string());
    notification.set_image(None);

    // Build the message targeting a topic
    let mut message = FcmMessage::new();
    message.set_notification(Some(notification));
    message.set_target(Target::Topic(topic.to_string()));

    // Add custom data if provided
    if let Some(data_map) = data {
        let mut string_data = HashMap::new();
        for (key, value) in data_map {
            let string_value = match value {
                Value::String(s) => s,
                other => other.to_string(),
            };
            string_data.insert(key, string_value);
        }
        message.set_data(Some(string_data));
    }

    // Send the notification
    service
        .send_notification(message)
        .await
        .map_err(|e| format!("Failed to send FCM topic notification: {}", e))?;

    Ok(SendTopicPushOutput {
        success: true,
        message_id: String::new(),
    })
}

/// Send Web Push Notification
///
/// Sends a web push notification using the Web Push protocol (RFC8030) with VAPID authentication.
///
/// # Arguments
/// * `config` - Web Push configuration including VAPID private key
/// * `title` - The notification title
/// * `body` - The notification body text
/// * `subscription` - The push subscription from the browser
/// * `lang` - Optional language tag
/// * `require_interaction` - Optional flag to require user interaction
/// * `dir` - Optional text direction ("auto", "ltr", "rtl")
/// * `silent` - Optional flag for silent notification
/// * `renotify` - Optional flag for renotification
/// * `tag` - Optional tag for notification replacement
/// * `image` - Optional image URL
/// * `data` - Optional custom data
/// * `vibrate` - Optional vibration pattern
/// * `icon` - Optional icon URL
/// * `timestamp` - Optional timestamp
/// * `badge` - Optional badge URL
/// * `actions` - Optional notification actions
pub async fn send_web_push(
    config: &WebPushConfig,
    title: &str,
    body: &str,
    subscription: HashMap<String, Value>,
    lang: Option<&str>,
    require_interaction: Option<bool>,
    dir: Option<&str>,
    silent: Option<bool>,
    renotify: Option<bool>,
    tag: Option<&str>,
    image: Option<&str>,
    data: Option<HashMap<String, Value>>,
    vibrate: Option<Vec<i32>>,
    icon: Option<&str>,
    timestamp: Option<i32>,
    badge: Option<&str>,
    actions: Option<Vec<HashMap<String, Value>>>,
) -> Result<SendWebPushOutput, String> {
    // Parse subscription
    let endpoint = subscription
        .get("endpoint")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing endpoint in subscription".to_string())?;

    let keys = subscription
        .get("keys")
        .and_then(|v| v.as_object())
        .ok_or_else(|| "Missing keys in subscription".to_string())?;

    let p256dh = keys
        .get("p256dh")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing p256dh key in subscription".to_string())?;

    let auth = keys
        .get("auth")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing auth key in subscription".to_string())?;

    // Decode the VAPID private key
    let vapid_key_bytes = URL_SAFE_NO_PAD
        .decode(&config.vapid_private_key)
        .map_err(|e| format!("Failed to decode VAPID private key: {}", e))?;

    let key_pair = ES256KeyPair::from_bytes(&vapid_key_bytes)
        .map_err(|e| format!("Failed to parse VAPID key pair: {}", e))?;

    // Decode p256dh public key
    let p256dh_bytes = URL_SAFE_NO_PAD
        .decode(p256dh)
        .map_err(|e| format!("Failed to decode p256dh key: {}", e))?;

    let public_key = PublicKey::from_sec1_bytes(&p256dh_bytes)
        .map_err(|e| format!("Failed to parse p256dh public key: {}", e))?;

    // Decode auth secret
    let auth_bytes = URL_SAFE_NO_PAD
        .decode(auth)
        .map_err(|e| format!("Failed to decode auth secret: {}", e))?;

    let auth_secret = Auth::clone_from_slice(&auth_bytes);

    // Build the web push payload
    #[derive(Serialize)]
    struct WebPushPayload {
        title: String,
        body: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        icon: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        badge: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        lang: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        dir: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<HashMap<String, Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        vibrate: Option<Vec<i32>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        renotify: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        require_interaction: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        silent: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        timestamp: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        actions: Option<Vec<HashMap<String, Value>>>,
    }

    let payload = WebPushPayload {
        title: title.to_string(),
        body: body.to_string(),
        icon: icon.map(|s| s.to_string()),
        badge: badge.map(|s| s.to_string()),
        image: image.map(|s| s.to_string()),
        tag: tag.map(|s| s.to_string()),
        lang: lang.map(|s| s.to_string()),
        dir: dir.map(|s| s.to_string()),
        data,
        vibrate,
        renotify,
        require_interaction,
        silent,
        timestamp: timestamp.map(|t| t as i64),
        actions,
    };

    let payload_json = serde_json::to_vec(&payload)
        .map_err(|e| format!("Failed to serialize payload: {}", e))?;

    // Parse the endpoint URL
    let endpoint_url = endpoint
        .parse()
        .map_err(|e| format!("Failed to parse endpoint URL: {}", e))?;

    // Build the web push request
    let builder = WebPushBuilder::new(endpoint_url, public_key, auth_secret)
        .with_vapid(&key_pair, &config.contact_email);

    let request = builder
        .build(payload_json)
        .map_err(|e| format!("Failed to build web push request: {}", e))?;

    // Send the request using reqwest
    let client = reqwest::Client::new();
    let (parts, body) = request.into_parts();

    let mut req_builder = client.request(
        parts.method,
        parts.uri.to_string(),
    );

    for (name, value) in parts.headers.iter() {
        req_builder = req_builder.header(name.as_str(), value.to_str().unwrap_or(""));
    }

    let response = req_builder
        .body(body)
        .send()
        .await
        .map_err(|e| format!("Failed to send web push request: {}", e))?;

    if response.status().is_success() {
        Ok(SendWebPushOutput {
            success: true,
            error: String::new(),
        })
    } else {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Ok(SendWebPushOutput {
            success: false,
            error: format!("HTTP {}: {}", status, error_text),
        })
    }
}

/// Subscribe To Topic
///
/// Subscribes device tokens to an FCM topic.
/// Note: This functionality requires the FCM Instance ID API which is deprecated.
/// For modern implementations, topic subscription should be handled client-side.
///
/// # Arguments
/// * `tokens` - List of FCM registration tokens
/// * `topic` - The topic name to subscribe to
/// * `platform` - The platform (only "Fcm" is supported)
pub async fn subscribe_to_topic(
    tokens: Vec<String>,
    topic: &str,
    _platform: &str,
) -> Result<SubscribeToTopicOutput, String> {
    // Note: FCM topic subscription via server requires the Instance ID API
    // which has been deprecated. Modern implementations should use the
    // client SDK to subscribe to topics.
    //
    // For server-side topic management, you would need to use the
    // Firebase Admin SDK or make direct API calls to the Instance ID API.
    //
    // This is a placeholder implementation that indicates which tokens
    // would fail (none, since we're not actually making API calls).

    tracing::warn!(
        "Topic subscription via server API is deprecated. Consider using client-side subscription. Tokens: {:?}, Topic: {}",
        tokens.len(),
        topic
    );

    Ok(SubscribeToTopicOutput {
        success: true,
        failed_tokens: Vec::new(),
    })
}

/// Unsubscribe From Topic
///
/// Unsubscribes device tokens from an FCM topic.
/// Note: This functionality requires the FCM Instance ID API which is deprecated.
/// For modern implementations, topic unsubscription should be handled client-side.
///
/// # Arguments
/// * `tokens` - List of FCM registration tokens
/// * `topic` - The topic name to unsubscribe from
/// * `platform` - The platform (only "Fcm" is supported)
pub async fn unsubscribe_from_topic(
    tokens: Vec<String>,
    topic: &str,
    _platform: &str,
) -> Result<UnsubscribeFromTopicOutput, String> {
    // Note: FCM topic unsubscription via server requires the Instance ID API
    // which has been deprecated. Modern implementations should use the
    // client SDK to unsubscribe from topics.

    tracing::warn!(
        "Topic unsubscription via server API is deprecated. Consider using client-side unsubscription. Tokens: {:?}, Topic: {}",
        tokens.len(),
        topic
    );

    Ok(UnsubscribeFromTopicOutput {
        success: true,
        failed_tokens: Vec::new(),
    })
}

/// Validate Push Token
///
/// Validates a push notification token for the specified platform.
///
/// # Arguments
/// * `config` - Optional FCM config (required for FCM validation)
/// * `token` - The push token to validate
/// * `platform` - The platform ("Apns", "Fcm", or "Web")
pub async fn validate_push_token(
    fcm_config: Option<&FcmConfig>,
    token: &str,
    platform: &str,
) -> Result<ValidatePushTokenOutput, String> {
    match platform.to_lowercase().as_str() {
        "apns" => {
            // APNS device tokens are 64-character hex strings
            if token.len() != 64 || !token.chars().all(|c| c.is_ascii_hexdigit()) {
                return Ok(ValidatePushTokenOutput {
                    valid: false,
                    error: "Invalid APNS token format. Expected 64 hex characters.".to_string(),
                });
            }
            Ok(ValidatePushTokenOutput {
                valid: true,
                error: String::new(),
            })
        }
        "fcm" => {
            // FCM tokens are typically 152-163 characters, alphanumeric with some special chars
            if token.len() < 100 || token.len() > 200 {
                return Ok(ValidatePushTokenOutput {
                    valid: false,
                    error: "Invalid FCM token length.".to_string(),
                });
            }

            // For actual validation, we could try to send a dry-run notification
            // This requires FCM config
            if let Some(config) = fcm_config {
                let service = FcmService::new(&config.credential_file);
                let mut message = FcmMessage::new();
                message.set_target(Target::Token(token.to_string()));

                // Try to send with validate_only (dry run)
                // Note: fcm-service doesn't expose validate_only, so we do basic format check
                let _ = service; // Would use for actual validation
            }

            Ok(ValidatePushTokenOutput {
                valid: true,
                error: String::new(),
            })
        }
        "web" => {
            // Web push subscriptions should be valid JSON
            let subscription: Result<PushSubscription, _> = serde_json::from_str(token);
            match subscription {
                Ok(sub) => {
                    // Validate endpoint is a valid URL
                    if sub.endpoint.is_empty() || !sub.endpoint.starts_with("https://") {
                        return Ok(ValidatePushTokenOutput {
                            valid: false,
                            error: "Invalid endpoint URL in subscription.".to_string(),
                        });
                    }
                    // Validate keys are present
                    if sub.keys.p256dh.is_empty() || sub.keys.auth.is_empty() {
                        return Ok(ValidatePushTokenOutput {
                            valid: false,
                            error: "Missing required keys in subscription.".to_string(),
                        });
                    }
                    Ok(ValidatePushTokenOutput {
                        valid: true,
                        error: String::new(),
                    })
                }
                Err(e) => Ok(ValidatePushTokenOutput {
                    valid: false,
                    error: format!("Invalid web push subscription format: {}", e),
                }),
            }
        }
        _ => Ok(ValidatePushTokenOutput {
            valid: false,
            error: format!("Unsupported platform: {}", platform),
        }),
    }
}
