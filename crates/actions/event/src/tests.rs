#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_emit_and_get_event() {
        let result = emit_event(
            "test-channel",
            "user.created",
            Some(HashMap::from([
                ("user_id".to_string(), json!("123")),
            ])),
            Some(json!({"name": "Alice"})),
        )
        .await
        .unwrap();

        assert!(result.success);
        assert!(!result.event_id.is_empty());

        // Get the event
        let event = get_event(&result.event_id).await.unwrap();
        assert_eq!(event.event_type, "user.created");
        assert_eq!(event.channel, "test-channel");
        assert!(!event.created_at.is_empty());
    }

    #[tokio::test]
    async fn test_subscribe_and_unsubscribe() {
        let sub_result = subscribe_channel(
            "my_handler",
            "notifications",
            Some(vec!["alert".to_string(), "warning".to_string()]),
        )
        .await
        .unwrap();

        assert!(sub_result.success);
        assert!(!sub_result.subscription_id.is_empty());

        // Unsubscribe
        let unsub_result = unsubscribe_channel(&sub_result.subscription_id)
            .await
            .unwrap();
        assert!(unsub_result.success);

        // Try to unsubscribe again - should fail
        let unsub_again = unsubscribe_channel(&sub_result.subscription_id)
            .await
            .unwrap();
        assert!(!unsub_again.success);
    }

    #[tokio::test]
    async fn test_list_events() {
        let channel = "test-list-channel";

        // Emit multiple events
        emit_event(channel, "event1", None, Some(json!("data1")))
            .await
            .unwrap();
        emit_event(channel, "event2", None, Some(json!("data2")))
            .await
            .unwrap();
        emit_event(channel, "event1", None, Some(json!("data3")))
            .await
            .unwrap();

        // List all events
        let list_result = list_events(channel, None, None, None, None)
            .await
            .unwrap();
        assert!(list_result.total >= 3);
        assert!(list_result.events.len() >= 3);

        // List with event type filter
        let filtered = list_events(
            channel,
            Some(vec!["event1".to_string()]),
            None,
            None,
            None,
        )
        .await
        .unwrap();
        assert!(filtered.total >= 2);
        for event in &filtered.events {
            let event_type = event.get("event_type").and_then(|v| v.as_str()).unwrap();
            assert_eq!(event_type, "event1");
        }

        // List with limit
        let limited = list_events(channel, None, None, Some(2), None)
            .await
            .unwrap();
        assert_eq!(limited.total, 2);
    }

    #[tokio::test]
    async fn test_list_events_empty_channel() {
        let result = list_events("non-existent-channel", None, None, None, None)
            .await
            .unwrap();
        assert_eq!(result.total, 0);
        assert_eq!(result.events.len(), 0);
    }

    #[tokio::test]
    async fn test_broadcast_event() {
        // Create multiple channels with events
        emit_event("channel1", "init", None, Some(json!("init1")))
            .await
            .unwrap();
        emit_event("channel2", "init", None, Some(json!("init2")))
            .await
            .unwrap();
        emit_event("channel3", "init", None, Some(json!("init3")))
            .await
            .unwrap();

        // Broadcast to all channels
        let broadcast_result = broadcast_event("global.update", None, None, Some(json!("update")))
            .await
            .unwrap();

        assert!(!broadcast_result.event_id.is_empty());
        assert!(broadcast_result.recipients >= 3);
    }

    #[tokio::test]
    async fn test_broadcast_with_exclusions() {
        // Create channels
        emit_event("include1", "init", None, Some(json!("i1")))
            .await
            .unwrap();
        emit_event("include2", "init", None, Some(json!("i2")))
            .await
            .unwrap();
        emit_event("exclude1", "init", None, Some(json!("e1")))
            .await
            .unwrap();

        // Broadcast excluding one channel
        let broadcast_result = broadcast_event(
            "announcement",
            None,
            Some(vec!["exclude1".to_string()]),
            Some(json!("message")),
        )
        .await
        .unwrap();

        // Should broadcast to at least 2 channels (include1, include2)
        assert!(broadcast_result.recipients >= 2);
    }

    #[tokio::test]
    async fn test_acknowledge_event() {
        // Create a subscription
        let sub = subscribe_channel("handler", "ack-channel", None)
            .await
            .unwrap();

        // Emit an event
        let event = emit_event("ack-channel", "task.complete", None, Some(json!("done")))
            .await
            .unwrap();

        // Acknowledge the event
        let ack_result = acknowledge_event(&event.event_id, &sub.subscription_id)
            .await
            .unwrap();
        assert!(ack_result.success);

        // Acknowledge again should still succeed
        let ack_again = acknowledge_event(&event.event_id, &sub.subscription_id)
            .await
            .unwrap();
        assert!(ack_again.success);
    }

    #[tokio::test]
    async fn test_acknowledge_invalid_event() {
        let sub = subscribe_channel("handler", "test", None).await.unwrap();

        let result = acknowledge_event("non-existent-event-id", &sub.subscription_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_acknowledge_invalid_subscription() {
        let event = emit_event("test", "event", None, Some(json!("data")))
            .await
            .unwrap();

        let result = acknowledge_event(&event.event_id, "invalid-subscription-id").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_replay_events() {
        let channel = "replay-channel";

        // Record current time
        let start_time = chrono::Utc::now().to_rfc3339();

        // Wait a bit to ensure time difference
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Emit some events
        emit_event(channel, "replay1", None, Some(json!("r1")))
            .await
            .unwrap();
        emit_event(channel, "replay2", None, Some(json!("r2")))
            .await
            .unwrap();
        emit_event(channel, "replay1", None, Some(json!("r3")))
            .await
            .unwrap();

        // Replay all events from start_time
        let replay_result = replay_events(&start_time, channel, None, None)
            .await
            .unwrap();
        assert!(replay_result.success);
        assert_eq!(replay_result.events_replayed, 3);

        // Replay with event type filter
        let filtered_replay = replay_events(
            &start_time,
            channel,
            None,
            Some(vec!["replay1".to_string()]),
        )
        .await
        .unwrap();
        assert!(filtered_replay.success);
        assert_eq!(filtered_replay.events_replayed, 2);
    }

    #[tokio::test]
    async fn test_replay_empty_channel() {
        let start_time = chrono::Utc::now().to_rfc3339();
        let result = replay_events(&start_time, "empty-channel", None, None)
            .await
            .unwrap();
        assert!(result.success);
        assert_eq!(result.events_replayed, 0);
    }

    #[tokio::test]
    async fn test_replay_invalid_time() {
        let result = replay_events("invalid-time", "channel", None, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_nonexistent_event() {
        let result = get_event("non-existent-id").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_event_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), json!("api"));
        metadata.insert("version".to_string(), json!("1.0"));
        metadata.insert("correlation_id".to_string(), json!("abc-123"));

        let result = emit_event("meta-channel", "with.metadata", Some(metadata.clone()), None)
            .await
            .unwrap();

        let event = get_event(&result.event_id).await.unwrap();
        assert_eq!(event.metadata.get("source").unwrap(), &json!("api"));
        assert_eq!(event.metadata.get("version").unwrap(), &json!("1.0"));
        assert_eq!(
            event.metadata.get("correlation_id").unwrap(),
            &json!("abc-123")
        );
    }

    #[tokio::test]
    async fn test_multiple_subscriptions_same_channel() {
        let sub1 = subscribe_channel("handler1", "multi-sub", None)
            .await
            .unwrap();
        let sub2 = subscribe_channel("handler2", "multi-sub", None)
            .await
            .unwrap();

        assert_ne!(sub1.subscription_id, sub2.subscription_id);
        assert!(sub1.success);
        assert!(sub2.success);

        // Both should be able to unsubscribe independently
        unsubscribe_channel(&sub1.subscription_id)
            .await
            .unwrap();
        unsubscribe_channel(&sub2.subscription_id)
            .await
            .unwrap();
    }
}
