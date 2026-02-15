use crate::*;
use std::sync::atomic::{AtomicU32, Ordering};

// ============================================================================
// Channel Tests
// ============================================================================

#[test]
fn test_channel_config_defaults() {
    let config = ChannelConfig::default();
    assert!(!config.persistent);
    assert!(!config.durable);
    assert!(config.allowed_event_types.is_empty());
}

#[test]
fn test_channel_config_persistent() {
    let config = ChannelConfig::persistent();
    assert!(config.persistent);
    assert!(config.durable);
}

#[test]
fn test_allowed_event_types() {
    let config = ChannelConfig::new().with_allowed_types(vec!["user.created", "user.updated"]);

    assert!(config.is_event_type_allowed("user.created"));
    assert!(config.is_event_type_allowed("user.updated"));
    assert!(!config.is_event_type_allowed("user.deleted"));
}

#[test]
fn test_empty_allowed_types_allows_all() {
    let config = ChannelConfig::new();
    assert!(config.is_event_type_allowed("any.event.type"));
}

// ============================================================================
// Event Tests
// ============================================================================

#[test]
fn test_event_creation() {
    let event = Event::new("user.created", "users");
    assert_eq!(event.event_type, "user.created");
    assert_eq!(event.channel, "users");
    assert_eq!(event.status, EventStatus::Pending);
    assert!(event.is_ready());
    assert!(!event.is_expired());
}

#[test]
fn test_event_with_payload() {
    let event =
        Event::new("order.completed", "orders").with_payload(serde_json::json!({"order_id": "123", "total": 99.99}));

    assert!(event.payload.is_object());
    assert_eq!(event.payload["order_id"], "123");
}

#[test]
fn test_event_with_metadata() {
    let metadata = EventMetadata::new()
        .with_source("order-service")
        .with_correlation_id("corr-123")
        .with_tag("important");

    let event = Event::new("order.completed", "orders").with_metadata(metadata);

    assert_eq!(event.metadata.source, Some("order-service".to_string()));
    assert_eq!(event.metadata.correlation_id, Some("corr-123".to_string()));
    assert!(event.metadata.tags.contains(&"important".to_string()));
}

#[test]
fn test_event_priority() {
    assert!(EventPriority::Critical > EventPriority::High);
    assert!(EventPriority::High > EventPriority::Normal);
    assert!(EventPriority::Normal > EventPriority::Low);
}

// ============================================================================
// Subscription Tests
// ============================================================================

fn create_test_event(event_type: &str, channel: &str) -> Event {
    Event::new(event_type, channel)
}

#[test]
fn test_filter_empty_matches_all() {
    let filter = SubscriptionFilter::new();
    let event = create_test_event("user.created", "users");
    assert!(filter.matches(&event));
}

#[test]
fn test_filter_by_event_types() {
    let filter = SubscriptionFilter::for_types(vec!["user.created", "user.updated"]);

    assert!(filter.matches(&create_test_event("user.created", "users")));
    assert!(filter.matches(&create_test_event("user.updated", "users")));
    assert!(!filter.matches(&create_test_event("user.deleted", "users")));
}

#[test]
fn test_filter_by_prefix() {
    let filter = SubscriptionFilter::for_prefix("user.");

    assert!(filter.matches(&create_test_event("user.created", "users")));
    assert!(filter.matches(&create_test_event("user.updated", "users")));
    assert!(!filter.matches(&create_test_event("order.created", "orders")));
}

#[test]
fn test_filter_by_priority() {
    let filter = SubscriptionFilter::new().with_min_priority(EventPriority::High);

    let normal_event = create_test_event("test", "channel");
    let high_event = create_test_event("test", "channel").with_priority(EventPriority::High);
    let critical_event = create_test_event("test", "channel").with_priority(EventPriority::Critical);

    assert!(!filter.matches(&normal_event));
    assert!(filter.matches(&high_event));
    assert!(filter.matches(&critical_event));
}

#[test]
fn test_filter_by_tags() {
    let filter = SubscriptionFilter::new().with_required_tags(vec!["important"]);

    let event_with_tag = create_test_event("test", "channel").with_metadata(EventMetadata::new().with_tag("important"));
    let event_without_tag = create_test_event("test", "channel");

    assert!(filter.matches(&event_with_tag));
    assert!(!filter.matches(&event_without_tag));
}

#[test]
fn test_subscription_should_receive() {
    let handler = SubscriptionHandler::callback("test_handler");
    let mut subscription =
        Subscription::new("users", handler).with_filter(SubscriptionFilter::for_types(vec!["user.created"]));

    let event1 = create_test_event("user.created", "users");
    let event2 = create_test_event("user.deleted", "users");
    let event3 = create_test_event("user.created", "orders");

    assert!(subscription.should_receive(&event1));
    assert!(!subscription.should_receive(&event2)); // wrong type
    assert!(!subscription.should_receive(&event3)); // wrong channel

    // Acknowledge event1
    subscription.acknowledge(&event1.id);
    assert!(!subscription.should_receive(&event1)); // already acknowledged
}

// ============================================================================
// In-Memory Service Tests
// ============================================================================

#[tokio::test]
async fn test_in_memory_store_basic_operations() {
    let store = InMemoryEventService::new();

    // Create a channel
    let channel = Channel::new("test-channel");
    store.upsert_channel(&channel).await.unwrap();

    // Store an event
    let event = Event::new("test.event", "test-channel").with_payload(serde_json::json!({"key": "value"}));
    store.store_event(&event).await.unwrap();

    // Retrieve the event
    let retrieved = store.get_event(&event.id).await.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().event_type, "test.event");

    // Query events
    let events = store
        .query_events(EventQuery::for_channel("test-channel"))
        .await
        .unwrap();
    assert_eq!(events.len(), 1);

    // Delete the event
    let deleted = store.delete_event(&event.id).await.unwrap();
    assert!(deleted);

    let retrieved = store.get_event(&event.id).await.unwrap();
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_in_memory_store_subscriptions() {
    let store = InMemoryEventService::new();

    // Create a channel and subscription
    let channel = Channel::new("test-channel");
    store.upsert_channel(&channel).await.unwrap();

    let subscription = Subscription::new("test-channel", SubscriptionHandler::callback("test"));
    store.upsert_subscription(&subscription).await.unwrap();

    // Store an event
    let event = Event::new("test.event", "test-channel");
    store.store_event(&event).await.unwrap();

    // Get pending events
    let pending = store.get_pending_events(&subscription.id, None).await.unwrap();
    assert_eq!(pending.len(), 1);

    // Acknowledge the event
    store.acknowledge_event(&subscription.id, &event.id).await.unwrap();

    // Pending should be empty now
    let pending = store.get_pending_events(&subscription.id, None).await.unwrap();
    assert_eq!(pending.len(), 0);
}

#[tokio::test]
async fn test_in_memory_store_max_events() {
    let store = InMemoryEventService::new().with_max_events(3);

    let channel = Channel::new("test-channel");
    store.upsert_channel(&channel).await.unwrap();

    // Store 5 events
    for i in 0..5 {
        let event = Event::new(format!("test.event.{}", i), "test-channel");
        store.store_event(&event).await.unwrap();
    }

    // Should only have 3 events (oldest ones removed)
    let events = store
        .query_events(EventQuery::for_channel("test-channel"))
        .await
        .unwrap();
    assert_eq!(events.len(), 3);
}

// ============================================================================
// Event Bus Tests
// ============================================================================

#[tokio::test]
async fn test_event_bus_publish_subscribe() {
    let bus = EventBus::new();
    let counter = std::sync::Arc::new(AtomicU32::new(0));
    let counter_clone = counter.clone();

    // Subscribe to channel
    let _sub_id = bus
        .subscribe("test-channel", move |_event| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        })
        .await
        .unwrap();

    // Publish events
    bus.emit("test-channel", "test.event", serde_json::json!({"key": "value"}))
        .await
        .unwrap();
    bus.emit("test-channel", "test.event", serde_json::json!({"key": "value2"}))
        .await
        .unwrap();

    // Give time for callbacks to execute
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    assert_eq!(counter.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn test_event_bus_filtered_subscription() {
    let bus = EventBus::new();
    let counter = std::sync::Arc::new(AtomicU32::new(0));
    let counter_clone = counter.clone();

    // Subscribe only to user.created events
    let filter = SubscriptionFilter::for_types(vec!["user.created"]);
    let _sub_id = bus
        .subscribe_with_filter("users", filter, move |_event| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        })
        .await
        .unwrap();

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
    let counter = std::sync::Arc::new(AtomicU32::new(0));
    let counter_clone = counter.clone();

    let _sub_id = bus
        .subscribe_async("test-channel", move |_event| {
            let counter = counter_clone.clone();
            async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        })
        .await
        .unwrap();

    bus.emit("test-channel", "test.event", serde_json::json!({}))
        .await
        .unwrap();

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
    bus.emit("test-channel", "test.event", serde_json::json!({"id": 1}))
        .await
        .unwrap();

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
    let event_ids = bus
        .broadcast(
            "broadcast.event",
            serde_json::json!({}),
            Some(vec!["channel-2".to_string()]),
        )
        .await
        .unwrap();

    assert_eq!(event_ids.len(), 2);
}

#[tokio::test]
async fn test_event_bus_acknowledgment() {
    let bus = EventBus::new();

    // Create subscription
    let sub_id = bus.subscribe("test-channel", |_| {}).await.unwrap();

    // Publish event
    let event_id = bus
        .emit("test-channel", "test.event", serde_json::json!({}))
        .await
        .unwrap();

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

// ============================================================================
// Integration Tests
// ============================================================================

#[tokio::test]
async fn test_event_creation_and_lifecycle() {
    let event = Event::new("user.created", "users")
        .with_payload(serde_json::json!({"user_id": "123", "email": "test@example.com"}))
        .with_metadata(
            EventMetadata::new()
                .with_source("user-service")
                .with_correlation_id("corr-123"),
        )
        .with_priority(EventPriority::High);

    assert_eq!(event.event_type, "user.created");
    assert_eq!(event.channel, "users");
    assert_eq!(event.priority, EventPriority::High);
    assert_eq!(event.status, EventStatus::Pending);
    assert!(event.is_ready());
}

#[tokio::test]
async fn test_event_with_ttl() {
    let event = Event::new("temp.event", "channel").with_ttl(1); // 1 second TTL

    assert!(event.expires_at.is_some());
    assert!(!event.is_expired());

    // Wait for expiration
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    assert!(event.is_expired());
}

#[tokio::test]
async fn test_event_delayed_delivery() {
    let event = Event::new("delayed.event", "channel").delayed(2); // 2 seconds delay

    assert!(event.scheduled_at.is_some());
    assert!(!event.is_ready());

    // Wait for scheduled time
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    assert!(event.is_ready());
}

#[tokio::test]
async fn test_in_memory_store_crud() {
    let store = InMemoryEventService::new();

    // Create channel
    let channel = Channel::new("test-channel");
    store.upsert_channel(&channel).await.unwrap();

    // Create event
    let event = Event::new("test.event", "test-channel");
    let event_id = event.id.clone();
    store.store_event(&event).await.unwrap();

    // Read event
    let retrieved = store.get_event(&event_id).await.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().event_type, "test.event");

    // Update status
    store
        .update_event_status(&event_id, EventStatus::Delivered)
        .await
        .unwrap();
    let updated = store.get_event(&event_id).await.unwrap().unwrap();
    assert_eq!(updated.status, EventStatus::Delivered);

    // Delete event
    let deleted = store.delete_event(&event_id).await.unwrap();
    assert!(deleted);

    let retrieved = store.get_event(&event_id).await.unwrap();
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_event_bus_full_workflow() {
    let bus = EventBus::new();

    // 1. Create channel
    bus.create_channel_with_config("orders", ChannelConfig::persistent())
        .await
        .unwrap();

    // 2. Subscribe to channel
    let received_events = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let received_clone = received_events.clone();

    let sub_id = bus
        .subscribe_with_filter(
            "orders",
            SubscriptionFilter::for_types(vec!["order.created", "order.completed"]),
            move |event| {
                received_clone.lock().unwrap().push(event);
            },
        )
        .await
        .unwrap();

    // 3. Publish events
    let event1_id = bus
        .emit(
            "orders",
            "order.created",
            serde_json::json!({
                "order_id": "ORD-001",
                "total": 99.99
            }),
        )
        .await
        .unwrap();

    let _event2_id = bus
        .emit(
            "orders",
            "order.shipped",
            serde_json::json!({
                "order_id": "ORD-001"
            }),
        )
        .await
        .unwrap(); // This won't match the filter

    let event3_id = bus
        .emit(
            "orders",
            "order.completed",
            serde_json::json!({
                "order_id": "ORD-001"
            }),
        )
        .await
        .unwrap();

    // Give time for callbacks
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // 4. Verify received events
    let received = received_events.lock().unwrap();
    assert_eq!(received.len(), 2);
    assert!(received.iter().any(|e| e.event_type == "order.created"));
    assert!(received.iter().any(|e| e.event_type == "order.completed"));
    drop(received);

    // 5. Get pending events and acknowledge
    let pending = bus.get_pending_events(&sub_id, None).await.unwrap();
    assert_eq!(pending.len(), 2);

    bus.acknowledge(&sub_id, &event1_id).await.unwrap();
    bus.acknowledge(&sub_id, &event3_id).await.unwrap();

    let pending = bus.get_pending_events(&sub_id, None).await.unwrap();
    assert_eq!(pending.len(), 0);

    // 6. Replay events
    let replayed = bus
        .replay(
            "orders",
            chrono::Utc::now() - chrono::Duration::hours(1),
            None,
            Some(vec!["order.created".to_string()]),
        )
        .await
        .unwrap();
    assert_eq!(replayed.len(), 1);
    assert_eq!(replayed[0].event_type, "order.created");

    // 7. Unsubscribe
    bus.unsubscribe(&sub_id).await.unwrap();
}

#[tokio::test]
async fn test_subscription_filter_combinations() {
    let bus = EventBus::new();

    let received = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let received_clone = received.clone();

    // Complex filter: high priority user events from auth-service
    let filter = SubscriptionFilter::new()
        .with_prefix("user.")
        .with_min_priority(EventPriority::High)
        .with_source("auth-service");

    bus.subscribe_with_filter("users", filter, move |event| {
        received_clone.lock().unwrap().push(event);
    })
    .await
    .unwrap();

    // Event that matches all criteria
    bus.publish(
        Event::new("user.login", "users")
            .with_priority(EventPriority::High)
            .with_metadata(EventMetadata::new().with_source("auth-service")),
    )
    .await
    .unwrap();

    // Event with wrong priority
    bus.publish(
        Event::new("user.login", "users")
            .with_priority(EventPriority::Normal)
            .with_metadata(EventMetadata::new().with_source("auth-service")),
    )
    .await
    .unwrap();

    // Event with wrong source
    bus.publish(
        Event::new("user.login", "users")
            .with_priority(EventPriority::High)
            .with_metadata(EventMetadata::new().with_source("other-service")),
    )
    .await
    .unwrap();

    // Event with wrong type prefix
    bus.publish(
        Event::new("order.created", "users")
            .with_priority(EventPriority::High)
            .with_metadata(EventMetadata::new().with_source("auth-service")),
    )
    .await
    .unwrap();

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let received = received.lock().unwrap();
    assert_eq!(received.len(), 1);
    assert_eq!(received[0].event_type, "user.login");
}

#[tokio::test]
async fn test_multiple_channels() {
    let bus = EventBus::new();

    bus.create_channel("orders").await.unwrap();
    bus.create_channel("users").await.unwrap();
    bus.create_channel("notifications").await.unwrap();

    let channels = bus.list_channels().await.unwrap();
    assert_eq!(channels.len(), 3);

    // Publish to different channels
    bus.emit("orders", "order.created", serde_json::json!({}))
        .await
        .unwrap();
    bus.emit("users", "user.created", serde_json::json!({})).await.unwrap();

    // Query events per channel
    let order_events = bus.list_events("orders", None, None).await.unwrap();
    let user_events = bus.list_events("users", None, None).await.unwrap();

    assert_eq!(order_events.len(), 1);
    assert_eq!(user_events.len(), 1);
}

#[tokio::test]
async fn test_event_metadata_tracking() {
    let bus = EventBus::new();

    let correlation_id = "correlation-123";
    let _causation_id = "event-000";

    let event_id = bus
        .publish(
            Event::new("step1", "workflow").with_metadata(
                EventMetadata::new()
                    .with_correlation_id(correlation_id)
                    .with_user_id("user-456")
                    .with_tenant_id("tenant-789")
                    .with_tags(vec!["important", "audit"]),
            ),
        )
        .await
        .unwrap();

    // Second event caused by first
    bus.publish(
        Event::new("step2", "workflow").with_metadata(
            EventMetadata::new()
                .with_correlation_id(correlation_id)
                .with_causation_id(&event_id),
        ),
    )
    .await
    .unwrap();

    // Filter by user
    let filter = SubscriptionFilter::new().with_user_id("user-456");
    let event = bus.get_event(&event_id).await.unwrap().unwrap();
    assert!(filter.matches(&event));

    // Filter by tenant
    let filter = SubscriptionFilter::new().with_tenant_id("tenant-789");
    assert!(filter.matches(&event));

    // Filter by tag
    let filter = SubscriptionFilter::new().with_any_tags(vec!["important"]);
    assert!(filter.matches(&event));
}
