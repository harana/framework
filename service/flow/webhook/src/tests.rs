#[cfg(test)]
mod tests {
    use crate::storage::StorageType;
    use crate::WebhookService;

    async fn create_test_service() -> WebhookService {
        WebhookService::new(StorageType::InMemory).await.unwrap()
    }

    #[tokio::test]
    async fn test_register_webhook() {
        let service = create_test_service().await;
        let result = service
            .register(
                "https://example.com/webhook",
                vec!["user.created".to_string(), "user.updated".to_string()],
                Some(true),
                None,
                Some("Test webhook"),
            )
            .await
            .unwrap();

        assert!(result.success);
        assert!(!result.webhook_id.is_empty());
        assert!(!result.secret.is_empty());
    }

    #[tokio::test]
    async fn test_register_with_custom_secret() {
        let service = create_test_service().await;
        let custom_secret = "my-secret-key";
        let result = service
            .register(
                "https://example.com/webhook",
                vec!["order.created".to_string()],
                Some(true),
                Some(custom_secret),
                None,
            )
            .await
            .unwrap();

        assert!(result.success);
        assert_eq!(result.secret, custom_secret);
    }

    #[tokio::test]
    async fn test_get_webhook() {
        let service = create_test_service().await;
        let register_result = service
            .register(
                "https://example.com/webhook",
                vec!["test.event".to_string()],
                Some(true),
                None,
                Some("Test description"),
            )
            .await
            .unwrap();

        let get_result = service.get(&register_result.webhook_id).await.unwrap();

        assert_eq!(get_result.url, "https://example.com/webhook");
        assert!(get_result.active);
        assert_eq!(get_result.events, vec!["test.event".to_string()]);
        assert_eq!(get_result.description, "Test description");
        assert!(!get_result.created_at.is_empty());
    }

    #[tokio::test]
    async fn test_get_nonexistent_webhook() {
        let service = create_test_service().await;
        let result = service.get("nonexistent-id").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[tokio::test]
    async fn test_update_webhook() {
        let service = create_test_service().await;
        let register_result = service
            .register(
                "https://example.com/webhook",
                vec!["event1".to_string()],
                Some(true),
                None,
                None,
            )
            .await
            .unwrap();

        let update_result = service
            .update(
                &register_result.webhook_id,
                Some(vec![
                    "event1".to_string(),
                    "event2".to_string(),
                    "event3".to_string(),
                ]),
                Some(false),
                Some("Updated description"),
                None,
                Some("https://example.com/new-webhook"),
            )
            .await
            .unwrap();

        assert!(update_result.success);

        let get_result = service.get(&register_result.webhook_id).await.unwrap();
        assert_eq!(get_result.url, "https://example.com/new-webhook");
        assert!(!get_result.active);
        assert_eq!(get_result.events.len(), 3);
        assert_eq!(get_result.description, "Updated description");
    }

    #[tokio::test]
    async fn test_unregister_webhook() {
        let service = create_test_service().await;
        let register_result = service
            .register(
                "https://example.com/webhook",
                vec!["event1".to_string()],
                Some(true),
                None,
                None,
            )
            .await
            .unwrap();

        let unregister_result = service.unregister(&register_result.webhook_id).await.unwrap();
        assert!(unregister_result.success);

        let get_result = service.get(&register_result.webhook_id).await;
        assert!(get_result.is_err());
    }

    #[tokio::test]
    async fn test_list_webhooks() {
        let service = create_test_service().await;

        // Register multiple webhooks
        service
            .register(
                "https://example.com/webhook1",
                vec!["user.created".to_string()],
                Some(true),
                None,
                None,
            )
            .await
            .unwrap();

        service
            .register(
                "https://example.com/webhook2",
                vec!["user.updated".to_string()],
                Some(true),
                None,
                None,
            )
            .await
            .unwrap();

        service
            .register(
                "https://example.com/webhook3",
                vec!["user.deleted".to_string()],
                Some(false),
                None,
                None,
            )
            .await
            .unwrap();

        // List all webhooks
        let list_result = service.lists(None, None, None, None).await.unwrap();
        assert!(list_result.total >= 3);

        // List active webhooks only
        let active_result = service.lists(None, Some(true), None, None).await.unwrap();
        assert!(active_result.total >= 2);

        // List webhooks for specific event
        let event_result = service.lists(Some("user.created"), None, None, None).await.unwrap();
        assert!(event_result.total >= 1);
    }

    #[tokio::test]
    async fn test_list_webhooks_pagination() {
        let service = create_test_service().await;

        // Register webhooks
        for i in 0..5 {
            service
                .register(
                    &format!("https://example.com/webhook{}", i),
                    vec!["pagination.test".to_string()],
                    Some(true),
                    None,
                    None,
                )
                .await
                .unwrap();
        }

        // Test pagination
        let page1 = service
            .lists(Some("pagination.test"), None, Some(2), Some(0))
            .await
            .unwrap();
        assert_eq!(page1.webhooks.len(), 2);

        let page2 = service
            .lists(Some("pagination.test"), None, Some(2), Some(2))
            .await
            .unwrap();
        assert!(page2.webhooks.len() >= 1);
    }

    #[tokio::test]
    async fn test_verify_signature_sha256() {
        let service = create_test_service().await;
        let secret = "my-secret-key";
        let payload = r#"{"event": "test"}"#;

        // Compute signature using the module function
        let signature = crate::compute_signature(payload, secret, "Sha256").unwrap();

        // Verify signature
        let result = service
            .verify_signature(&signature, payload, secret, Some("Sha256"))
            .await
            .unwrap();

        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_verify_signature_sha512() {
        let service = create_test_service().await;
        let secret = "my-secret-key";
        let payload = r#"{"event": "test"}"#;

        let signature = crate::compute_signature(payload, secret, "Sha512").unwrap();
        let result = service
            .verify_signature(&signature, payload, secret, Some("Sha512"))
            .await
            .unwrap();

        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_verify_invalid_signature() {
        let service = create_test_service().await;
        let secret = "my-secret-key";
        let payload = r#"{"event": "test"}"#;

        let result = service
            .verify_signature("invalid-signature", payload, secret, Some("Sha256"))
            .await
            .unwrap();

        assert!(!result.valid);
    }

    #[tokio::test]
    async fn test_rotate_secret() {
        let service = create_test_service().await;
        let register_result = service
            .register(
                "https://example.com/webhook",
                vec!["event1".to_string()],
                Some(true),
                Some("original-secret"),
                None,
            )
            .await
            .unwrap();

        assert_eq!(register_result.secret, "original-secret");

        let rotate_result = service.rotate_secret(&register_result.webhook_id).await.unwrap();
        assert!(rotate_result.success);
        assert_ne!(rotate_result.new_secret, "original-secret");

        let _get_result = service.get(&register_result.webhook_id).await.unwrap();
        // We can't directly check the secret in the output, but we rotated it
        assert!(!rotate_result.new_secret.is_empty());
    }

    #[tokio::test]
    async fn test_trigger_event() {
        let service = create_test_service().await;
        // This test won't actually send HTTP requests to a real server
        // but will test the trigger logic
        let register_result = service
            .register(
                "https://httpbin.org/post",
                vec!["test.event".to_string()],
                Some(true),
                None,
                None,
            )
            .await
            .unwrap();

        let trigger_result = service
            .trigger(
                "test.event",
                r#"{"data": "test"}"#,
                Some(vec![register_result.webhook_id.clone()]),
            )
            .await
            .unwrap();

        // Should have attempted to trigger 1 webhook
        assert_eq!(trigger_result.triggered_count, 1);
    }

    #[tokio::test]
    async fn test_trigger_multiple_webhooks() {
        let service = create_test_service().await;
        let _webhook1 = service
            .register(
                "https://httpbin.org/post",
                vec!["multi.event".to_string()],
                Some(true),
                None,
                None,
            )
            .await
            .unwrap();

        let _webhook2 = service
            .register(
                "https://httpbin.org/post",
                vec!["multi.event".to_string()],
                Some(true),
                None,
                None,
            )
            .await
            .unwrap();

        let trigger_result = service
            .trigger("multi.event", r#"{"test": true}"#, None)
            .await
            .unwrap();

        assert!(trigger_result.triggered_count >= 2);
    }

    #[tokio::test]
    async fn test_get_deliveries() {
        let service = create_test_service().await;
        let register_result = service
            .register(
                "https://httpbin.org/post",
                vec!["delivery.test".to_string()],
                Some(true),
                None,
                None,
            )
            .await
            .unwrap();

        // Trigger an event to create a delivery
        service
            .trigger(
                "delivery.test",
                r#"{"data": "test"}"#,
                Some(vec![register_result.webhook_id.clone()]),
            )
            .await
            .unwrap();

        // Wait a bit for the delivery to be recorded
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let deliveries = service
            .get_deliveries(&register_result.webhook_id, None, None, None)
            .await
            .unwrap();

        assert!(deliveries.total >= 1);
        assert!(!deliveries.deliveries.is_empty());
    }

    #[tokio::test]
    async fn test_test_webhook() {
        let service = create_test_service().await;
        let register_result = service
            .register(
                "https://httpbin.org/post",
                vec!["test".to_string()],
                Some(true),
                None,
                None,
            )
            .await
            .unwrap();

        let test_result = service
            .test(&register_result.webhook_id, Some(r#"{"test": true}"#))
            .await
            .unwrap();

        // httpbin.org should return 200
        assert!(test_result.status_code > 0);
        assert!(test_result.response_time_ms >= 0);
    }

    #[tokio::test]
    async fn test_storage_type_enum() {
        // Test InMemory
        let inmem = StorageType::InMemory;
        assert!(matches!(inmem, StorageType::InMemory));

        // Test Redis variant
        let redis = StorageType::Redis {
            url: "redis://localhost:6379".to_string(),
        };
        assert!(matches!(redis, StorageType::Redis { .. }));

        // Test MongoDB variant
        let mongo = StorageType::MongoDB {
            url: "mongodb://localhost:27017".to_string(),
            database: "webhooks".to_string(),
        };
        assert!(matches!(mongo, StorageType::MongoDB { .. }));

        // Test Default
        let default = StorageType::default();
        assert!(matches!(default, StorageType::InMemory));
    }
}
