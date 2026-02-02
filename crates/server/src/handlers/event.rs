//! Datastar SSE handlers for real-time updates

use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use asynk_strim::{stream_fn, Yielder};
use core::time::Duration;
use datastar::prelude::{MergeFragments, MergeSignals, ReadSignals, Sse};
use harana_components_events::{Event, EventMetadata, SubscriptionFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use dashmap::DashMap;
use parking_lot::RwLock;

use crate::state::AppState;

// ============================================================================
// Channel Permission System
// ============================================================================

/// Channel visibility/access type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChannelAccess {
    /// Only the owner can access
    Private,
    /// Any authenticated user can access
    Public,
    /// Specific users can access (in addition to owner)
    Shared,
}

impl Default for ChannelAccess {
    fn default() -> Self {
        Self::Private
    }
}

/// Channel permission metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPermission {
        pub channel: String,
        pub owner_id: String,
        pub access: ChannelAccess,
        pub shared_with: HashSet<String>,
        pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ChannelPermission {
    /// Create a new private channel owned by the user
    pub fn private(channel: impl Into<String>, owner_id: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            owner_id: owner_id.into(),
            access: ChannelAccess::Private,
            shared_with: HashSet::new(),
            created_at: chrono::Utc::now(),
        }
    }

    /// Create a new public channel owned by the user
    pub fn public(channel: impl Into<String>, owner_id: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            owner_id: owner_id.into(),
            access: ChannelAccess::Public,
            shared_with: HashSet::new(),
            created_at: chrono::Utc::now(),
        }
    }

    /// Create a new shared channel
    pub fn shared(channel: impl Into<String>, owner_id: impl Into<String>, users: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            channel: channel.into(),
            owner_id: owner_id.into(),
            access: ChannelAccess::Shared,
            shared_with: users.into_iter().map(Into::into).collect(),
            created_at: chrono::Utc::now(),
        }
    }

    /// Check if a user can access this channel
    pub fn can_access(&self, user_id: &str) -> bool {
        match self.access {
            ChannelAccess::Private => self.owner_id == user_id,
            ChannelAccess::Public => true,
            ChannelAccess::Shared => self.owner_id == user_id || self.shared_with.contains(user_id),
        }
    }

    /// Check if a user is the owner
    pub fn is_owner(&self, user_id: &str) -> bool {
        self.owner_id == user_id
    }

    /// Add a user to shared access
    pub fn share_with(&mut self, user_id: impl Into<String>) {
        self.shared_with.insert(user_id.into());
    }

    /// Remove a user from shared access
    pub fn unshare_with(&mut self, user_id: &str) {
        self.shared_with.remove(user_id);
    }
}

/// Channel permission registry
#[derive(Debug, Default)]
pub struct ChannelRegistry {
    permissions: DashMap<String, ChannelPermission>,
        system_channels: RwLock<HashSet<String>>,
}

impl ChannelRegistry {
    /// Create a new channel registry
    pub fn new() -> Self {
        let registry = Self::default();
        // Register system channels
        {
            let mut system = registry.system_channels.write();
            system.insert("datastar".to_string());
            system.insert("notifications".to_string());
            system.insert("system".to_string());
            system.insert("broadcast".to_string());
        }
        registry
    }

    /// Check if a channel is a system channel
    pub fn is_system_channel(&self, channel: &str) -> bool {
        self.system_channels.read().contains(channel)
    }

    /// Register a channel with permissions
    pub fn register(&self, permission: ChannelPermission) {
        self.permissions.insert(permission.channel.clone(), permission);
    }

    /// Get channel permission
    pub fn get(&self, channel: &str) -> Option<ChannelPermission> {
        self.permissions.get(channel).map(|p| p.clone())
    }

    /// Check if user can access a channel
    pub fn can_access(&self, channel: &str, user_id: &str) -> bool {
        // System channels are accessible to all authenticated users
        if self.is_system_channel(channel) {
            return true;
        }

        // User-prefixed channels (e.g., "user:123:events") belong to that user
        if let Some(owner) = Self::extract_user_prefix(channel) {
            return owner == user_id;
        }

        // Check registered permissions
        if let Some(permission) = self.get(channel) {
            return permission.can_access(user_id);
        }

        // Default: channel doesn't exist or no permissions set - deny
        false
    }

    /// Check if user is owner of a channel
    pub fn is_owner(&self, channel: &str, user_id: &str) -> bool {
        // System channels have no owner
        if self.is_system_channel(channel) {
            return false;
        }

        // User-prefixed channels
        if let Some(owner) = Self::extract_user_prefix(channel) {
            return owner == user_id;
        }

        // Check registered permissions
        if let Some(permission) = self.get(channel) {
            return permission.is_owner(user_id);
        }

        false
    }

    /// Extract user ID from user-prefixed channel name (e.g., "user:123:events" -> "123")
    fn extract_user_prefix(channel: &str) -> Option<&str> {
        if channel.starts_with("user:") {
            channel.strip_prefix("user:")?.split(':').next()
        } else {
            None
        }
    }

    /// Get user's default private channel name
    pub fn user_channel(user_id: &str) -> String {
        format!("user:{}:events", user_id)
    }

    /// Get user's notification channel name
    pub fn user_notifications(user_id: &str) -> String {
        format!("user:{}:notifications", user_id)
    }

    /// List channels accessible to a user
    pub fn list_accessible(&self, user_id: &str) -> Vec<ChannelPermission> {
        let mut channels = Vec::new();

        // Add system channels as public
        for channel in self.system_channels.read().iter() {
            channels.push(ChannelPermission::public(channel, "system"));
        }

        // Add user's own channels
        for entry in self.permissions.iter() {
            if entry.value().can_access(user_id) {
                channels.push(entry.value().clone());
            }
        }

        channels
    }

    /// Remove a channel
    pub fn remove(&self, channel: &str) -> Option<ChannelPermission> {
        self.permissions.remove(channel).map(|(_, v)| v)
    }
}

/// Global channel registry instance
static CHANNEL_REGISTRY: std::sync::OnceLock<Arc<ChannelRegistry>> = std::sync::OnceLock::new();

/// Get the global channel registry
pub fn channel_registry() -> &'static Arc<ChannelRegistry> {
    CHANNEL_REGISTRY.get_or_init(|| Arc::new(ChannelRegistry::new()))
}

/// Signals received from the client
#[derive(Debug, Clone, Deserialize)]
pub struct DatastarSignals {
    /// Delay in milliseconds between updates
    #[serde(default = "default_delay")]
    pub delay: u64,
    /// Message to display
    #[serde(default)]
    pub message: String,
    /// Counter value
    #[serde(default)]
    pub count: i32,
}

fn default_delay() -> u64 {
    100
}

/// Hello world endpoint that streams a message character by character
pub async fn hello_world(ReadSignals(signals): ReadSignals<DatastarSignals>) -> impl IntoResponse {
    const MESSAGE: &str = "Hello from Harana! 🌺";

    Sse(stream_fn(move |mut yielder: Yielder<MergeFragments>| async move {
        for i in 0..MESSAGE.len() {
            // Get valid UTF-8 slice
            let end = MESSAGE
                .char_indices()
                .nth(i + 1)
                .map(|(idx, _)| idx)
                .unwrap_or(MESSAGE.len());
            
            if end <= MESSAGE.len() {
                let text = &MESSAGE[0..end];
                let fragment = MergeFragments::new(format!(r#"<span data-text="$message">{}</span>"#, text));
                yielder.yield_item(fragment).await;
            }

            tokio::time::sleep(Duration::from_millis(signals.delay.max(50))).await;
        }
    }))
}

/// Increment counter endpoint
pub async fn increment(ReadSignals(signals): ReadSignals<DatastarSignals>) -> impl IntoResponse {
    let new_count = signals.count + 1;
    
    Sse(stream_fn(move |mut yielder: Yielder<MergeSignals>| async move {
        let signals_update = MergeSignals::new(format!(r#"{{"count":{}}}"#, new_count));
        yielder.yield_item(signals_update).await;
    }))
}

/// Send a hello message
pub async fn hello_message(ReadSignals(_signals): ReadSignals<DatastarSignals>) -> impl IntoResponse {
    Sse(stream_fn(|mut yielder: Yielder<MergeSignals>| async move {
        let signals_update = MergeSignals::new(r#"{"message":"Hello from the server! 👋"}"#);
        yielder.yield_item(signals_update).await;
    }))
}

/// Real-time events endpoint - streams events from user's private channel and system channels
pub async fn event_stream(
    State(state): State<AppState>,
    auth: crate::extractors::Auth,
    ReadSignals(signals): ReadSignals<DatastarSignals>,
) -> impl IntoResponse {
    let user_id = auth.user_id.clone();
    let event_bus = state.event_bus.clone();
    
    // Use user's private channel
    let user_channel = ChannelRegistry::user_channel(&user_id);
    let _ = event_bus.create_channel(&user_channel).await;
    
    // Also subscribe to the system "datastar" channel for broadcasts
    let _ = event_bus.create_channel("global").await;
    
    // Get receivers for both channels
    let user_receiver = event_bus.receiver(&user_channel);
    let system_receiver = event_bus.receiver("global");

    Sse(stream_fn(move |mut yielder: Yielder<MergeFragments>| async move {
        // Send initial connection message
        let fragment = MergeFragments::new(format!(
            r#"<div id="event-stream">
                <p>User: {}</p>
                <p>Connected to private channel: {}</p>
                <p>Connected at {}</p>
            </div>"#,
            user_id,
            user_channel,
            chrono::Utc::now().format("%H:%M:%S")
        ));
        yielder.yield_item(fragment).await;

        let mut user_rx = user_receiver;
        let mut system_rx = system_receiver;
        let filter = SubscriptionFilter::new().with_user_id(&user_id);
        let mut count = 0;
        
        loop {
            tokio::select! {
                // Events from user's private channel
                result = async { 
                    match &mut user_rx {
                        Some(rx) => Some(rx.recv().await),
                        None => None,
                    }
                }, if user_rx.is_some() => {
                    if let Some(result) = result {
                        match result {
                            Ok(event) => {
                                count += 1;
                                let fragment = MergeFragments::new(format!(
                                    r#"<div id="event-stream">
                                        <p>User: {}</p>
                                        <p>Event #{} [{}]: {}</p>
                                        <p>Source: private</p>
                                        <p>Received at: {}</p>
                                    </div>"#,
                                    user_id,
                                    count,
                                    event.event_type,
                                    event.payload,
                                    chrono::Utc::now().format("%H:%M:%S")
                                ));
                                yielder.yield_item(fragment).await;
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                                let fragment = MergeFragments::new(format!(
                                    r#"<div id="event-stream">
                                        <p class="warning">Missed {} private events</p>
                                    </div>"#,
                                    n
                                ));
                                yielder.yield_item(fragment).await;
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                                user_rx = None;
                            }
                        }
                    }
                }
                // Events from system channel (broadcasts)
                result = async {
                    match &mut system_rx {
                        Some(rx) => Some(rx.recv().await),
                        None => None,
                    }
                }, if system_rx.is_some() => {
                    if let Some(result) = result {
                        match result {
                            Ok(event) => {
                                // Only show if it's a broadcast (no user_id) or targeted at this user
                                if event.metadata.user_id.is_none() || filter.matches(&event) {
                                    count += 1;
                                    let fragment = MergeFragments::new(format!(
                                        r#"<div id="event-stream">
                                            <p>User: {}</p>
                                            <p>Event #{} [{}]: {}</p>
                                            <p>Source: broadcast</p>
                                            <p>Received at: {}</p>
                                        </div>"#,
                                        user_id,
                                        count,
                                        event.event_type,
                                        event.payload,
                                        chrono::Utc::now().format("%H:%M:%S")
                                    ));
                                    yielder.yield_item(fragment).await;
                                }
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                                let fragment = MergeFragments::new(format!(
                                    r#"<div id="event-stream">
                                        <p class="warning">Missed {} broadcast events</p>
                                    </div>"#,
                                    n
                                ));
                                yielder.yield_item(fragment).await;
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                                system_rx = None;
                            }
                        }
                    }
                }
                // Heartbeat
                _ = tokio::time::sleep(Duration::from_millis(signals.delay.max(1000))) => {
                    let fragment = MergeFragments::new(format!(
                        r#"<div id="event-stream">
                            <p>User: {}</p>
                            <p>Heartbeat at {}</p>
                        </div>"#,
                        user_id,
                        chrono::Utc::now().format("%H:%M:%S")
                    ));
                    yielder.yield_item(fragment).await;
                }
            }

            // Stop after 100 events or if both channels are closed
            if count >= 100 || (user_rx.is_none() && system_rx.is_none()) {
                break;
            }
        }
    }))
}

/// Server time endpoint - streams current time
pub async fn server_time() -> impl IntoResponse {
    Sse(stream_fn(|mut yielder: Yielder<MergeFragments>| async move {
        loop {
            let time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
            let fragment = MergeFragments::new(format!(
                r#"<span id="server-time">{}</span>"#,
                time
            ));
            yielder.yield_item(fragment).await;

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }))
}

/// Signals for notification subscription
#[derive(Debug, Clone, Deserialize)]
pub struct NotificationSignals {
    /// Last notification ID received (for resumable subscriptions)
    #[serde(default)]
    pub last_id: Option<String>,
}

/// Notification stream endpoint - streams notifications from user's channel and system notifications
pub async fn notifications(
    State(state): State<AppState>,
    auth: crate::extractors::Auth,
    ReadSignals(_signals): ReadSignals<NotificationSignals>,
) -> impl IntoResponse {
    let user_id = auth.user_id.clone();
    let event_bus = state.event_bus.clone();
    
    // User's private notification channel
    let user_notifications = ChannelRegistry::user_notifications(&user_id);
    let _ = event_bus.create_channel(&user_notifications).await;
    
    // System notifications channel (for broadcasts)
    let _ = event_bus.create_channel("notifications").await;
    
    // Get receivers
    let user_receiver = event_bus.receiver(&user_notifications);
    let system_receiver = event_bus.receiver("notifications");

    Sse(stream_fn(move |mut yielder: Yielder<MergeFragments>| async move {
        // Send welcome message
        let fragment = MergeFragments::new(format!(
            r#"<div id="notifications" class="notification-container">
                <div class="notification info">Welcome back, {}!</div>
            </div>"#,
            user_id
        ));
        yielder.yield_item(fragment).await;

        let mut user_rx = user_receiver;
        let mut system_rx = system_receiver;
        let filter = SubscriptionFilter::new().with_user_id(&user_id);
        
        loop {
            tokio::select! {
                // User's private notifications
                result = async { 
                    match &mut user_rx {
                        Some(rx) => Some(rx.recv().await),
                        None => None,
                    }
                }, if user_rx.is_some() => {
                    if let Some(result) = result {
                        match result {
                            Ok(event) => {
                                let level = event.metadata.tags.iter()
                                    .find(|t| ["info", "success", "warning", "error"].contains(&t.as_str()))
                                    .cloned()
                                    .unwrap_or_else(|| "info".to_string());
                                
                                let message = event.payload.as_str()
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| event.payload.to_string());
                                
                                let fragment = MergeFragments::new(format!(
                                    r#"<div id="notifications" class="notification-container">
                                        <div class="notification {}">{}</div>
                                    </div>"#,
                                    level, message
                                ));
                                yielder.yield_item(fragment).await;
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                                user_rx = None;
                            }
                        }
                    }
                }
                // System broadcast notifications
                result = async {
                    match &mut system_rx {
                        Some(rx) => Some(rx.recv().await),
                        None => None,
                    }
                }, if system_rx.is_some() => {
                    if let Some(result) = result {
                        match result {
                            Ok(event) => {
                                // Only show broadcasts (no user_id) or events targeted at this user
                                if event.metadata.user_id.is_none() || filter.matches(&event) {
                                    let level = event.metadata.tags.iter()
                                        .find(|t| ["info", "success", "warning", "error"].contains(&t.as_str()))
                                        .cloned()
                                        .unwrap_or_else(|| "info".to_string());
                                    
                                    let message = event.payload.as_str()
                                        .map(|s| s.to_string())
                                        .unwrap_or_else(|| event.payload.to_string());
                                    
                                    let fragment = MergeFragments::new(format!(
                                        r#"<div id="notifications" class="notification-container">
                                            <div class="notification {} broadcast">{}</div>
                                        </div>"#,
                                        level, message
                                    ));
                                    yielder.yield_item(fragment).await;
                                }
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                                system_rx = None;
                            }
                        }
                    }
                }
                // Heartbeat
                _ = tokio::time::sleep(Duration::from_secs(30)) => {
                    let fragment = MergeFragments::new(format!(
                        r#"<div id="notifications" class="notification-container">
                            <div class="notification ping">Still connected at {}</div>
                        </div>"#,
                        chrono::Utc::now().format("%H:%M:%S")
                    ));
                    yielder.yield_item(fragment).await;
                }
            }

            // Exit if both channels closed
            if user_rx.is_none() && system_rx.is_none() {
                break;
            }
        }
    }))
}

// ============================================================================
// Event Publishing Endpoints
// ============================================================================

/// Request to publish an event to a channel
#[derive(Debug, Clone, Deserialize)]
pub struct PublishEventRequest {
        pub channel: String,
        pub event_type: String,
        pub payload: serde_json::Value,
    /// Target user ID (optional, for user-specific events)
    #[serde(default)]
    pub user_id: Option<String>,
    /// Tags to attach to the event
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Response after publishing an event
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub struct PublishEventResponse {
        pub event_id: String,
        pub channel: String,
        pub event_type: String,
}

/// Publish an event to a channel (requires authentication and channel access)
pub async fn publish_event(
    State(state): State<AppState>,
    auth: crate::extractors::Auth,
    Json(request): Json<PublishEventRequest>,
) -> impl IntoResponse {
    let registry = channel_registry();
    
    // Check if user can publish to this channel
    if !registry.can_access(&request.channel, &auth.user_id) {
        return Json(serde_json::json!({
            "success": false,
            "error": "Access denied: you don't have permission to publish to this channel"
        }));
    }
    
    let event_bus = state.event_bus.clone();
    
    // Build metadata with authenticated user
    let mut metadata = EventMetadata::new()
        .with_source("datastar")
        .with_user_id(&auth.user_id);
    
    // Allow targeting a specific user only for user-owned or shared channels
    if let Some(target_user_id) = &request.user_id {
        // Can only target users if publishing to their channel or a shared channel
        let target_channel = ChannelRegistry::user_channel(target_user_id);
        if registry.can_access(&target_channel, &auth.user_id) || registry.is_system_channel(&request.channel) {
            metadata = metadata.with_user_id(target_user_id);
        }
    }
    
    for tag in request.tags {
        metadata = metadata.with_tag(tag);
    }
    
    // Ensure channel exists
    let _ = event_bus.create_channel(&request.channel).await;
    
    // Create and publish the event
    let event = Event::new(&request.event_type, &request.channel)
        .with_payload(request.payload)
        .with_metadata(metadata);
    
    match event_bus.publish(event).await {
        Ok(event_id) => Json(serde_json::json!({
            "success": true,
            "event_id": event_id,
            "channel": request.channel,
            "event_type": request.event_type
        })),
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        })),
    }
}

/// Signals for subscribing to a channel
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelSubscribeSignals {
        pub channel: String,
    /// Filter by event types (optional)
    #[serde(default)]
    pub event_types: Vec<String>,
    /// Heartbeat delay in milliseconds
    #[serde(default = "default_delay")]
    pub delay: u64,
}

/// Subscribe to a specific channel and stream events (requires authentication and channel access)
pub async fn channel_stream(
    State(state): State<AppState>,
    auth: crate::extractors::Auth,
    ReadSignals(signals): ReadSignals<ChannelSubscribeSignals>,
) -> impl IntoResponse {
    let user_id = auth.user_id.clone();
    let event_bus = state.event_bus.clone();
    let channel_name = signals.channel.clone();
    let registry = channel_registry();
    
    // Check if user can access this channel
    let has_access = registry.can_access(&channel_name, &user_id);
    
    // Get receiver only if access is granted
    let receiver = if has_access {
        let _ = event_bus.create_channel(&channel_name).await;
        event_bus.receiver(&channel_name)
    } else {
        None
    };
    
    // Build filter if event types are specified
    let filter = if signals.event_types.is_empty() {
        SubscriptionFilter::new()
    } else {
        SubscriptionFilter::for_types(signals.event_types.clone())
    };

    Sse(stream_fn(move |mut yielder: Yielder<MergeFragments>| async move {
        // Check access first
        if !has_access {
            let fragment = MergeFragments::new(format!(
                r#"<div id="channel-stream-{}">
                    <p class="error">Access denied: you don't have permission to access channel '{}'</p>
                </div>"#,
                channel_name, channel_name
            ));
            yielder.yield_item(fragment).await;
            return;
        }
        
        // Send initial connection message
        let fragment = MergeFragments::new(format!(
            r#"<div id="channel-stream-{}">
                <p>User: {} subscribed to channel: {}</p>
                <p>Connected at {}</p>
            </div>"#,
            channel_name,
            user_id,
            channel_name,
            chrono::Utc::now().format("%H:%M:%S")
        ));
        yielder.yield_item(fragment).await;

        match receiver {
            Some(mut rx) => {
                let mut count = 0;
                
                loop {
                    tokio::select! {
                        result = rx.recv() => {
                            match result {
                                Ok(event) => {
                                    // Check if event matches filter
                                    if filter.matches(&event) {
                                        count += 1;
                                        let fragment = MergeFragments::new(format!(
                                            r#"<div id="channel-stream-{}">
                                                <p>Event #{} [{}]</p>
                                                <pre>{}</pre>
                                                <p>Received at: {}</p>
                                            </div>"#,
                                            channel_name,
                                            count,
                                            event.event_type,
                                            serde_json::to_string_pretty(&event.payload).unwrap_or_default(),
                                            chrono::Utc::now().format("%H:%M:%S")
                                        ));
                                        yielder.yield_item(fragment).await;
                                    }
                                }
                                Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                                    let fragment = MergeFragments::new(format!(
                                        r#"<div id="channel-stream-{}">
                                            <p class="warning">Missed {} events</p>
                                        </div>"#,
                                        channel_name, n
                                    ));
                                    yielder.yield_item(fragment).await;
                                }
                                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                                    break;
                                }
                            }
                        }
                        _ = tokio::time::sleep(Duration::from_millis(signals.delay.max(5000))) => {
                            // Heartbeat
                            let fragment = MergeFragments::new(format!(
                                r#"<div id="channel-stream-{}">
                                    <p>Heartbeat - {} events received</p>
                                </div>"#,
                                channel_name, count
                            ));
                            yielder.yield_item(fragment).await;
                        }
                    }

                    if count >= 1000 {
                        break;
                    }
                }
            }
            None => {
                let fragment = MergeFragments::new(format!(
                    r#"<div id="channel-stream-{}">
                        <p class="error">Channel {} not available</p>
                    </div>"#,
                    channel_name, channel_name
                ));
                yielder.yield_item(fragment).await;
            }
        }
    }))
}

/// List channels accessible to the authenticated user
pub async fn list_channels(
    State(state): State<AppState>,
    auth: crate::extractors::Auth,
) -> impl IntoResponse {
    let event_bus = state.event_bus.clone();
    let registry = channel_registry();
    
    // Get permissions for channels the user can access
    let accessible = registry.list_accessible(&auth.user_id);
    
    match event_bus.list_channels().await {
        Ok(channels) => {
            // Filter channels to only those the user can access
            let channel_info: Vec<_> = channels.iter()
                .filter(|c| registry.can_access(&c.name, &auth.user_id))
                .map(|c| {
                    // Find permission info if available
                    let perm = accessible.iter().find(|p| p.channel == c.name);
                    serde_json::json!({
                        "name": c.name,
                        "subscriber_count": c.subscriber_count,
                        "total_events": c.total_events,
                        "active": c.active,
                        "access": perm.map(|p| &p.access).unwrap_or(&ChannelAccess::Public),
                        "is_owner": perm.map(|p| p.is_owner(&auth.user_id)).unwrap_or(false),
                    })
                })
                .collect();
            
            // Also include user's implicit channels
            let user_channel = ChannelRegistry::user_channel(&auth.user_id);
            let user_notifications = ChannelRegistry::user_notifications(&auth.user_id);
            
            Json(serde_json::json!({
                "success": true,
                "channels": channel_info,
                "user_channels": {
                    "events": user_channel,
                    "notifications": user_notifications,
                }
            }))
        }
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        })),
    }
}

// ============================================================================
// Channel Management Endpoints
// ============================================================================

/// Request to create a new channel
#[derive(Debug, Clone, Deserialize)]
pub struct CreateChannelRequest {
        pub name: String,
    /// Access type: "private", "public", or "shared"
    #[serde(default)]
    pub access: ChannelAccess,
    /// Users to share with (for shared channels)
    #[serde(default)]
    pub shared_with: Vec<String>,
}

/// Create a new channel owned by the authenticated user
pub async fn create_channel(
    State(state): State<AppState>,
    auth: crate::extractors::Auth,
    Json(request): Json<CreateChannelRequest>,
) -> impl IntoResponse {
    let registry = channel_registry();
    
    // Check if channel name is reserved (system channels)
    if registry.is_system_channel(&request.name) {
        return Json(serde_json::json!({
            "success": false,
            "error": "Cannot create a channel with a reserved system name"
        }));
    }
    
    // Check if channel name starts with "user:" (reserved for user-scoped channels)
    if request.name.starts_with("user:") {
        return Json(serde_json::json!({
            "success": false,
            "error": "Channel names starting with 'user:' are reserved for system use"
        }));
    }
    
    // Check if channel already exists
    if registry.get(&request.name).is_some() {
        return Json(serde_json::json!({
            "success": false,
            "error": "Channel already exists"
        }));
    }
    
    // Create permission based on access type
    let permission = match request.access {
        ChannelAccess::Private => ChannelPermission::private(&request.name, &auth.user_id),
        ChannelAccess::Public => ChannelPermission::public(&request.name, &auth.user_id),
        ChannelAccess::Shared => ChannelPermission::shared(&request.name, &auth.user_id, request.shared_with),
    };
    
    // Register the channel
    registry.register(permission.clone());
    
    // Create the channel in the event bus
    let _ = state.event_bus.create_channel(&request.name).await;
    
    Json(serde_json::json!({
        "success": true,
        "channel": {
            "name": permission.channel,
            "owner_id": permission.owner_id,
            "access": permission.access,
            "shared_with": permission.shared_with.iter().collect::<Vec<_>>(),
        }
    }))
}

/// Request to share a channel with users
#[derive(Debug, Clone, Deserialize)]
pub struct ShareChannelRequest {
        pub channel: String,
        pub users: Vec<String>,
}

/// Share a channel with additional users (owner only)
pub async fn share_channel(
    auth: crate::extractors::Auth,
    Json(request): Json<ShareChannelRequest>,
) -> impl IntoResponse {
    let registry = channel_registry();
    
    // Get existing permission
    let mut permission = match registry.get(&request.channel) {
        Some(p) => p,
        None => {
            return Json(serde_json::json!({
                "success": false,
                "error": "Channel not found"
            }));
        }
    };
    
    // Check if user is the owner
    if !permission.is_owner(&auth.user_id) {
        return Json(serde_json::json!({
            "success": false,
            "error": "Only the channel owner can share the channel"
        }));
    }
    
    // Update access type to shared and add users
    permission.access = ChannelAccess::Shared;
    for user in &request.users {
        permission.share_with(user);
    }
    
    // Update registry
    registry.register(permission.clone());
    
    Json(serde_json::json!({
        "success": true,
        "channel": {
            "name": permission.channel,
            "access": permission.access,
            "shared_with": permission.shared_with.iter().collect::<Vec<_>>(),
        }
    }))
}

/// Request to unshare a channel
#[derive(Debug, Clone, Deserialize)]
pub struct UnshareChannelRequest {
        pub channel: String,
        pub users: Vec<String>,
}

/// Remove users from a shared channel (owner only)
pub async fn unshare_channel(
    auth: crate::extractors::Auth,
    Json(request): Json<UnshareChannelRequest>,
) -> impl IntoResponse {
    let registry = channel_registry();
    
    // Get existing permission
    let mut permission = match registry.get(&request.channel) {
        Some(p) => p,
        None => {
            return Json(serde_json::json!({
                "success": false,
                "error": "Channel not found"
            }));
        }
    };
    
    // Check if user is the owner
    if !permission.is_owner(&auth.user_id) {
        return Json(serde_json::json!({
            "success": false,
            "error": "Only the channel owner can modify sharing"
        }));
    }
    
    // Remove users
    for user in &request.users {
        permission.unshare_with(user);
    }
    
    // If no more shared users, convert back to private
    if permission.shared_with.is_empty() && permission.access == ChannelAccess::Shared {
        permission.access = ChannelAccess::Private;
    }
    
    // Update registry
    registry.register(permission.clone());
    
    Json(serde_json::json!({
        "success": true,
        "channel": {
            "name": permission.channel,
            "access": permission.access,
            "shared_with": permission.shared_with.iter().collect::<Vec<_>>(),
        }
    }))
}

/// Request to delete a channel
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteChannelRequest {
        pub channel: String,
}

/// Delete a channel (owner only)
pub async fn delete_channel(
    State(state): State<AppState>,
    auth: crate::extractors::Auth,
    Json(request): Json<DeleteChannelRequest>,
) -> impl IntoResponse {
    let registry = channel_registry();
    
    // Check if it's a system channel
    if registry.is_system_channel(&request.channel) {
        return Json(serde_json::json!({
            "success": false,
            "error": "Cannot delete system channels"
        }));
    }
    
    // Get existing permission
    let permission = match registry.get(&request.channel) {
        Some(p) => p,
        None => {
            return Json(serde_json::json!({
                "success": false,
                "error": "Channel not found"
            }));
        }
    };
    
    // Check if user is the owner
    if !permission.is_owner(&auth.user_id) {
        return Json(serde_json::json!({
            "success": false,
            "error": "Only the channel owner can delete the channel"
        }));
    }
    
    // Remove from registry
    registry.remove(&request.channel);
    
    // Delete from event bus
    let _ = state.event_bus.delete_channel(&request.channel).await;
    
    Json(serde_json::json!({
        "success": true,
        "message": format!("Channel '{}' deleted", request.channel)
    }))
}
