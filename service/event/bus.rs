// Harana Components - Events Bus (Main Event System Interface)

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};

use crate::{
    Channel, ChannelConfig, Event, EventError, EventMetadata,
    EventResult, EventStore, InMemoryEventStore, Subscription,
    SubscriptionFilter, SubscriptionHandler,
};

#[derive(Debug, Clone)]
pub struct EventBusConfig {
    pub default_channel_config: ChannelConfig,
    pub broadcast_buffer_size: usize,
    pub auto_cleanup: bool,
    pub cleanup_interval_secs: u64,
    pub deduplicate: bool,
    pub dedup_window_secs: u64,
}

impl Default for EventBusConfig {
    fn default() -> Self {
        Self {
            default_channel_config: ChannelConfig::default(),
            broadcast_buffer_size: 1024,
            auto_cleanup: true,
            cleanup_interval_secs: 60,
            deduplicate: true,
            dedup_window_secs: 300,
        }
    }
}

impl EventBusConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a client-side configuration (in-memory only)
    pub fn client() -> Self {
        Self {
            default_channel_config: ChannelConfig::in_memory(),
            broadcast_buffer_size: 256,
            auto_cleanup: true,
            cleanup_interval_secs: 30,
            deduplicate: true,
            dedup_window_secs: 60,
        }
    }

    /// Create a server-side configuration (with persistence)
    pub fn server() -> Self {
        Self {
            default_channel_config: ChannelConfig::persistent(),
            broadcast_buffer_size: 4096,
            auto_cleanup: true,
            cleanup_interval_secs: 300,
            deduplicate: true,
            dedup_window_secs: 600,
        }
    }

    pub fn with_broadcast_buffer(mut self, size: usize) -> Self {
        self.broadcast_buffer_size = size;
        self
    }

    pub fn with_cleanup(mut self, enabled: bool, interval_secs: u64) -> Self {
        self.auto_cleanup = enabled;
        self.cleanup_interval_secs = interval_secs;
        self
    }

    pub fn with_deduplication(mut self, enabled: bool, window_secs: u64) -> Self {
        self.deduplicate = enabled;
        self.dedup_window_secs = window_secs;
        self
    }
}

/// Callback type for event handlers
pub type EventCallback = Arc<dyn Fn(Event) + Send + Sync>;

/// Async callback type for event handlers
pub type AsyncEventCallback = Arc<dyn Fn(Event) -> Pin<Box<dyn std::future::Future<Output = EventResult<()>> + Send>> + Send + Sync>;

struct CallbackEntry {
    subscription_id: String,
    filter: SubscriptionFilter,
    callback: EventCallback,
}

struct AsyncCallbackEntry {
    subscription_id: String,
    filter: SubscriptionFilter,
    callback: AsyncEventCallback,
}

struct ChannelBroadcaster {
    sender: broadcast::Sender<Event>,
    _receiver: broadcast::Receiver<Event>,
}

impl ChannelBroadcaster {
    fn new(capacity: usize) -> Self {
        let (sender, _receiver) = broadcast::channel(capacity);
        Self { sender, _receiver }
    }
}

pub struct EventBus<S: EventStore = InMemoryEventStore> {
    store: Arc<S>,
    config: EventBusConfig,
    broadcasters: DashMap<String, Arc<ChannelBroadcaster>>,
    callbacks: DashMap<String, Vec<CallbackEntry>>, // channel -> [CallbackEntry]
    async_callbacks: DashMap<String, Vec<AsyncCallbackEntry>>,
    dedup_cache: DashMap<String, DateTime<Utc>>,
    #[allow(dead_code)] // Reserved for graceful shutdown implementation
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl EventBus<InMemoryEventStore> {
    /// Create a new event bus with in-memory storage
    pub fn new() -> Self {
        Self::with_store(InMemoryEventStore::new())
    }

    /// Create a new event bus with configuration
    pub fn with_config(config: EventBusConfig) -> Self {
        Self::with_store_and_config(InMemoryEventStore::new(), config)
    }
}

impl Default for EventBus<InMemoryEventStore> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: EventStore + 'static> EventBus<S> {
    /// Create a new event bus with a custom store
    pub fn with_store(store: S) -> Self {
        Self::with_store_and_config(store, EventBusConfig::default())
    }

    /// Create a new event bus with a custom store and configuration
    pub fn with_store_and_config(store: S, config: EventBusConfig) -> Self {
        Self {
            store: Arc::new(store),
            config,
            broadcasters: DashMap::new(),
            callbacks: DashMap::new(),
            async_callbacks: DashMap::new(),
            dedup_cache: DashMap::new(),
            shutdown_tx: None,
        }
    }

    /// Get the underlying store
    pub fn store(&self) -> &S {
        &self.store
    }

    /// Get the configuration
    pub fn config(&self) -> &EventBusConfig {
        &self.config
    }

    // ========== Channel Management ==========

    /// Create a new channel with default configuration
    pub async fn create_channel(&self, name: impl Into<String>) -> EventResult<Channel> {
        let name = name.into();
        let channel = Channel::with_config(&name, self.config.default_channel_config.clone());
        self.store.upsert_channel(&channel).await?;
        
        // Create broadcaster for the channel
        self.broadcasters.entry(name.clone()).or_insert_with(|| {
            Arc::new(ChannelBroadcaster::new(self.config.broadcast_buffer_size))
        });
        
        Ok(channel)
    }

    /// Create a new channel with custom configuration
    pub async fn create_channel_with_config(&self, name: impl Into<String>, config: ChannelConfig) -> EventResult<Channel> {
        let name = name.into();
        let channel = Channel::with_config(&name, config);
        self.store.upsert_channel(&channel).await?;
        
        let buffer_size = channel.config.buffer_size;
        self.broadcasters.entry(name.clone()).or_insert_with(|| {
            Arc::new(ChannelBroadcaster::new(buffer_size))
        });
        
        Ok(channel)
    }

    /// Get a channel by name
    pub async fn get_channel(&self, name: &str) -> EventResult<Option<Channel>> {
        self.store.get_channel(name).await
    }

    /// List all channels
    pub async fn list_channels(&self) -> EventResult<Vec<Channel>> {
        self.store.list_channels().await
    }

    /// Delete a channel and all its events/subscriptions
    pub async fn delete_channel(&self, name: &str) -> EventResult<bool> {
        self.broadcasters.remove(name);
        self.callbacks.remove(name);
        self.async_callbacks.remove(name);
        self.store.delete_channel(name).await
    }

    // ========== Event Publishing ==========

    /// Publish an event to a channel
    pub async fn publish(&self, event: Event) -> EventResult<String> {
        // Ensure channel exists
        let channel = self.store.get_channel(&event.channel).await?;
        if channel.is_none() {
            // Auto-create channel with default config
            self.create_channel(&event.channel).await?;
        }

        // Check deduplication
        if self.config.deduplicate {
            if self.dedup_cache.contains_key(&event.id) {
                return Err(EventError::DuplicateEvent { event_id: event.id });
            }
            self.dedup_cache.insert(event.id.clone(), Utc::now());
        }

        // Store the event
        self.store.store_event(&event).await?;

        // Broadcast to real-time subscribers
        if let Some(broadcaster) = self.broadcasters.get(&event.channel) {
            let _ = broadcaster.sender.send(event.clone());
        }

        // Execute sync callbacks (only if filter matches)
        if let Some(callbacks) = self.callbacks.get(&event.channel) {
            for entry in callbacks.iter() {
                if entry.filter.matches(&event) {
                    (entry.callback)(event.clone());
                }
            }
        }

        // Execute async callbacks (only if filter matches)
        if let Some(callbacks) = self.async_callbacks.get(&event.channel) {
            for entry in callbacks.iter() {
                if entry.filter.matches(&event) {
                    let _ = (entry.callback)(event.clone()).await;
                }
            }
        }

        Ok(event.id)
    }

    /// Publish a simple event with just type and payload
    pub async fn emit(
        &self,
        channel: impl Into<String>,
        event_type: impl Into<String>,
        payload: impl serde::Serialize,
    ) -> EventResult<String> {
        let event = Event::new(event_type, channel)
            .with_payload(payload);
        self.publish(event).await
    }

    /// Publish an event with metadata
    pub async fn emit_with_metadata(
        &self,
        channel: impl Into<String>,
        event_type: impl Into<String>,
        payload: impl serde::Serialize,
        metadata: EventMetadata,
    ) -> EventResult<String> {
        let event = Event::new(event_type, channel)
            .with_payload(payload)
            .with_metadata(metadata);
        self.publish(event).await
    }

    /// Broadcast an event to all channels
    pub async fn broadcast(
        &self,
        event_type: impl Into<String>,
        payload: impl serde::Serialize,
        exclude_channels: Option<Vec<String>>,
    ) -> EventResult<Vec<String>> {
        let event_type = event_type.into();
        let payload = serde_json::to_value(payload)?;
        let exclude = exclude_channels.unwrap_or_default();

        let channels = self.store.list_channels().await?;
        let mut event_ids = Vec::new();

        for channel in channels {
            if exclude.contains(&channel.name) {
                continue;
            }

            let event = Event::new(&event_type, &channel.name)
                .with_payload(payload.clone());
            
            if let Ok(id) = self.publish(event).await {
                event_ids.push(id);
            }
        }

        Ok(event_ids)
    }

    // ========== Subscription Management ==========

    /// Subscribe to a channel with a sync callback
    pub async fn subscribe(
        &self,
        channel: impl Into<String>,
        callback: impl Fn(Event) + Send + Sync + 'static,
    ) -> EventResult<String> {
        self.subscribe_with_filter(channel, SubscriptionFilter::new(), callback).await
    }

    /// Subscribe to a channel with a filter and sync callback
    pub async fn subscribe_with_filter(
        &self,
        channel: impl Into<String>,
        filter: SubscriptionFilter,
        callback: impl Fn(Event) + Send + Sync + 'static,
    ) -> EventResult<String> {
        let channel = channel.into();
        
        // Ensure channel exists
        if self.store.get_channel(&channel).await?.is_none() {
            self.create_channel(&channel).await?;
        }

        let handler = SubscriptionHandler::callback("callback");
        let subscription = Subscription::new(&channel, handler)
            .with_filter(filter.clone());
        
        self.store.upsert_subscription(&subscription).await?;

        // Add callback with filter
        self.callbacks
            .entry(channel)
            .or_default()
            .push(CallbackEntry {
                subscription_id: subscription.id.clone(),
                filter,
                callback: Arc::new(callback),
            });

        Ok(subscription.id)
    }

    /// Subscribe to a channel with an async callback
    pub async fn subscribe_async<F, Fut>(
        &self,
        channel: impl Into<String>,
        callback: F,
    ) -> EventResult<String>
    where
        F: Fn(Event) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = EventResult<()>> + Send + 'static,
    {
        self.subscribe_async_with_filter(channel, SubscriptionFilter::new(), callback).await
    }

    /// Subscribe to a channel with a filter and async callback
    pub async fn subscribe_async_with_filter<F, Fut>(
        &self,
        channel: impl Into<String>,
        filter: SubscriptionFilter,
        callback: F,
    ) -> EventResult<String>
    where
        F: Fn(Event) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = EventResult<()>> + Send + 'static,
    {
        let channel = channel.into();
        
        // Ensure channel exists
        if self.store.get_channel(&channel).await?.is_none() {
            self.create_channel(&channel).await?;
        }

        let handler = SubscriptionHandler::callback("async_callback");
        let subscription = Subscription::new(&channel, handler)
            .with_filter(filter.clone());
        
        self.store.upsert_subscription(&subscription).await?;

        // Wrap the callback
        let wrapped_callback: AsyncEventCallback = Arc::new(move |event| {
            let fut = callback(event);
            Box::pin(fut)
        });

        // Add callback with filter
        self.async_callbacks
            .entry(channel)
            .or_default()
            .push(AsyncCallbackEntry {
                subscription_id: subscription.id.clone(),
                filter,
                callback: wrapped_callback,
            });

        Ok(subscription.id)
    }

    /// Get a broadcast receiver for a channel (for streaming events)
    pub fn receiver(&self, channel: &str) -> Option<broadcast::Receiver<Event>> {
        self.broadcasters.get(channel).map(|b| b.sender.subscribe())
    }

    /// Unsubscribe from a channel
    pub async fn unsubscribe(&self, subscription_id: &str) -> EventResult<bool> {
        // Remove from callbacks
        for mut callbacks in self.callbacks.iter_mut() {
            callbacks.value_mut().retain(|entry| entry.subscription_id != subscription_id);
        }
        for mut callbacks in self.async_callbacks.iter_mut() {
            callbacks.value_mut().retain(|entry| entry.subscription_id != subscription_id);
        }

        self.store.delete_subscription(subscription_id).await
    }

    /// Get a subscription by ID
    pub async fn get_subscription(&self, subscription_id: &str) -> EventResult<Option<Subscription>> {
        self.store.get_subscription(subscription_id).await
    }

    /// List subscriptions for a channel
    pub async fn list_subscriptions(&self, channel: Option<&str>) -> EventResult<Vec<Subscription>> {
        self.store.list_subscriptions(channel).await
    }

    // ========== Event Retrieval ==========

    /// Get an event by ID
    pub async fn get_event(&self, event_id: &str) -> EventResult<Option<Event>> {
        self.store.get_event(event_id).await
    }

    /// List events from a channel
    pub async fn list_events(
        &self,
        channel: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> EventResult<Vec<Event>> {
        use crate::store::EventQuery;
        
        let mut query = EventQuery::for_channel(channel).descending();
        if let Some(limit) = limit {
            query = query.with_limit(limit);
        }
        if let Some(offset) = offset {
            query = query.with_offset(offset);
        }
        
        self.store.query_events(query).await
    }

    /// Query events with custom filters
    pub async fn query_events(&self, query: crate::store::EventQuery) -> EventResult<Vec<Event>> {
        self.store.query_events(query).await
    }

    // ========== Event Acknowledgment ==========

    /// Acknowledge an event for a subscription
    pub async fn acknowledge(&self, subscription_id: &str, event_id: &str) -> EventResult<()> {
        self.store.acknowledge_event(subscription_id, event_id).await
    }

    /// Get pending (unacknowledged) events for a subscription
    pub async fn get_pending_events(&self, subscription_id: &str, limit: Option<usize>) -> EventResult<Vec<Event>> {
        self.store.get_pending_events(subscription_id, limit).await
    }

    // ========== Replay ==========

    /// Replay events from a specific time
    pub async fn replay(
        &self,
        channel: &str,
        start_time: DateTime<Utc>,
        end_time: Option<DateTime<Utc>>,
        event_types: Option<Vec<String>>,
    ) -> EventResult<Vec<Event>> {
        use crate::store::EventQuery;
        
        let mut query = EventQuery::for_channel(channel)
            .with_time_range(Some(start_time), end_time)
            .ascending();
        
        if let Some(types) = event_types {
            query = query.with_types(types);
        }
        
        self.store.query_events(query).await
    }

    // ========== Cleanup ==========

    /// Delete expired events
    pub async fn cleanup_expired(&self) -> EventResult<u64> {
        self.store.delete_expired_events().await
    }

    /// Delete events older than a given time
    pub async fn cleanup_before(&self, before: DateTime<Utc>) -> EventResult<u64> {
        self.store.delete_events_before(before).await
    }

    /// Clean up the deduplication cache
    pub fn cleanup_dedup_cache(&self) {
        let cutoff = Utc::now() - chrono::Duration::seconds(self.config.dedup_window_secs as i64);
        self.dedup_cache.retain(|_, time| *time > cutoff);
    }

    /// Start the background cleanup task
    pub fn start_cleanup_task(self: Arc<Self>) {
        if !self.config.auto_cleanup {
            return;
        }

        let interval = self.config.cleanup_interval_secs;
        let bus = self.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval));
            loop {
                interval.tick().await;
                let _ = bus.cleanup_expired().await;
                bus.cleanup_dedup_cache();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn test_event_bus_publish_subscribe() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        // Subscribe to channel
        let _sub_id = bus.subscribe("test-channel", move |_event| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }).await.unwrap();

        // Publish events
        bus.emit("test-channel", "test.event", serde_json::json!({"key": "value"})).await.unwrap();
        bus.emit("test-channel", "test.event", serde_json::json!({"key": "value2"})).await.unwrap();

        // Give time for callbacks to execute
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_event_bus_filtered_subscription() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        // Subscribe only to user.created events
        let filter = SubscriptionFilter::for_types(vec!["user.created"]);
        let _sub_id = bus.subscribe_with_filter("users", filter, move |_event| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }).await.unwrap();

        // Publish matching and non-matching events
        bus.emit("users", "user.created", serde_json::json!({})).await.unwrap();
        bus.emit("users", "user.updated", serde_json::json!({})).await.unwrap();
        bus.emit("users", "user.created", serde_json::json!({})).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Only user.created events should trigger the callback
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_event_bus_async_subscription() {
        let bus = EventBus::new();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let _sub_id = bus.subscribe_async("test-channel", move |_event| {
            let counter = counter_clone.clone();
            async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        }).await.unwrap();

        bus.emit("test-channel", "test.event", serde_json::json!({})).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_event_bus_receiver() {
        let bus = EventBus::new();
        
        // Create channel and get receiver
        bus.create_channel("test-channel").await.unwrap();
        let mut receiver = bus.receiver("test-channel").unwrap();

        // Publish event
        bus.emit("test-channel", "test.event", serde_json::json!({"id": 1})).await.unwrap();

        // Receive event
        let event = receiver.recv().await.unwrap();
        assert_eq!(event.event_type, "test.event");
    }

    #[tokio::test]
    async fn test_event_bus_broadcast() {
        let bus = EventBus::new();
        
        // Create multiple channels
        bus.create_channel("channel-1").await.unwrap();
        bus.create_channel("channel-2").await.unwrap();
        bus.create_channel("channel-3").await.unwrap();

        // Broadcast to all except channel-2
        let event_ids = bus.broadcast(
            "broadcast.event",
            serde_json::json!({}),
            Some(vec!["channel-2".to_string()]),
        ).await.unwrap();

        assert_eq!(event_ids.len(), 2);
    }

    #[tokio::test]
    async fn test_event_bus_acknowledgment() {
        let bus = EventBus::new();
        
        // Create subscription
        let sub_id = bus.subscribe("test-channel", |_| {}).await.unwrap();

        // Publish event
        let event_id = bus.emit("test-channel", "test.event", serde_json::json!({})).await.unwrap();

        // Get pending events
        let pending = bus.get_pending_events(&sub_id, None).await.unwrap();
        assert_eq!(pending.len(), 1);

        // Acknowledge
        bus.acknowledge(&sub_id, &event_id).await.unwrap();

        // Pending should be empty
        let pending = bus.get_pending_events(&sub_id, None).await.unwrap();
        assert_eq!(pending.len(), 0);
    }

    #[tokio::test]
    async fn test_event_bus_deduplication() {
        let bus = EventBus::with_config(EventBusConfig {
            deduplicate: true,
            dedup_window_secs: 60,
            ..Default::default()
        });

        let event = Event::with_id("unique-id", "test.event", "test-channel");
        
        // First publish should succeed
        let result = bus.publish(event.clone()).await;
        assert!(result.is_ok());

        // Second publish with same ID should fail
        let result = bus.publish(event).await;
        assert!(matches!(result, Err(EventError::DuplicateEvent { .. })));
    }
}
