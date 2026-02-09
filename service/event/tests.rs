use crate::*;

#[tokio::test]
async fn test_event_creation_and_lifecycle() {
    let event = Event::new("user.created", "users")
        .with_payload(serde_json::json!({"user_id": "123", "email": "test@example.com"}))
        .with_metadata(
            EventMetadata::new()
                .with_source("user-service")
                .with_correlation_id("corr-123")
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
    let event = Event::new("temp.event", "channel")
        .with_ttl(1); // 1 second TTL

    assert!(event.expires_at.is_some());
    assert!(!event.is_expired());

    // Wait for expiration
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    assert!(event.is_expired());
}

#[tokio::test]
async fn test_event_delayed_delivery() {
    let event = Event::new("delayed.event", "channel")
        .delayed(2); // 2 seconds delay

    assert!(event.scheduled_at.is_some());
    assert!(!event.is_ready());

    // Wait for scheduled time
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    assert!(event.is_ready());
}

#[tokio::test]
async fn test_in_memory_store_crud() {
    let store = InMemoryEventStore::new();

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
    store.update_event_status(&event_id, EventStatus::Delivered).await.unwrap();
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
    bus.create_channel_with_config("orders", ChannelConfig::persistent()).await.unwrap();

    // 2. Subscribe to channel
    let received_events = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let received_clone = received_events.clone();

    let sub_id = bus.subscribe_with_filter(
        "orders",
        SubscriptionFilter::for_types(vec!["order.created", "order.completed"]),
        move |event| {
            received_clone.lock().unwrap().push(event);
        }
    ).await.unwrap();

    // 3. Publish events
    let event1_id = bus.emit("orders", "order.created", serde_json::json!({
        "order_id": "ORD-001",
        "total": 99.99
    })).await.unwrap();

    let _event2_id = bus.emit("orders", "order.shipped", serde_json::json!({
        "order_id": "ORD-001"
    })).await.unwrap(); // This won't match the filter

    let event3_id = bus.emit("orders", "order.completed", serde_json::json!({
        "order_id": "ORD-001"
    })).await.unwrap();

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
    let replayed = bus.replay(
        "orders",
        chrono::Utc::now() - chrono::Duration::hours(1),
        None,
        Some(vec!["order.created".to_string()])
    ).await.unwrap();
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
    }).await.unwrap();

    // Event that matches all criteria
    bus.publish(
        Event::new("user.login", "users")
            .with_priority(EventPriority::High)
            .with_metadata(EventMetadata::new().with_source("auth-service"))
    ).await.unwrap();

    // Event with wrong priority
    bus.publish(
        Event::new("user.login", "users")
            .with_priority(EventPriority::Normal)
            .with_metadata(EventMetadata::new().with_source("auth-service"))
    ).await.unwrap();

    // Event with wrong source
    bus.publish(
        Event::new("user.login", "users")
            .with_priority(EventPriority::High)
            .with_metadata(EventMetadata::new().with_source("other-service"))
    ).await.unwrap();

    // Event with wrong type prefix
    bus.publish(
        Event::new("order.created", "users")
            .with_priority(EventPriority::High)
            .with_metadata(EventMetadata::new().with_source("auth-service"))
    ).await.unwrap();

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
    bus.emit("orders", "order.created", serde_json::json!({})).await.unwrap();
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

    let event_id = bus.publish(
        Event::new("step1", "workflow")
            .with_metadata(
                EventMetadata::new()
                    .with_correlation_id(correlation_id)
                    .with_user_id("user-456")
                    .with_tenant_id("tenant-789")
                    .with_tags(vec!["important", "audit"])
            )
    ).await.unwrap();

    // Second event caused by first
    bus.publish(
        Event::new("step2", "workflow")
            .with_metadata(
                EventMetadata::new()
                    .with_correlation_id(correlation_id)
                    .with_causation_id(&event_id)
            )
    ).await.unwrap();

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
