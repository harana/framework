#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_set_and_get() {
        let result = set("test_key", "\"hello world\"", Some("test_ns"), None)
            .await
            .unwrap();
        assert!(result.success);

        let get_result = get("test_key", Some("test_ns")).await.unwrap();
        assert!(get_result.found);
        assert_eq!(get_result.value, "\"hello world\"");
    }

    #[tokio::test]
    async fn test_get_not_found() {
        let result = get("nonexistent_key", Some("test_get_ns")).await.unwrap();
        assert!(!result.found);
        assert!(result.value.is_empty());
    }

    #[tokio::test]
    async fn test_set_json_value() {
        let json_value = r#"{"name": "test", "count": 42}"#;
        let result = set("json_key", json_value, Some("json_ns"), None)
            .await
            .unwrap();
        assert!(result.success);

        let get_result = get("json_key", Some("json_ns")).await.unwrap();
        assert!(get_result.found);
        assert!(get_result.value.contains("name"));
        assert!(get_result.value.contains("test"));
    }

    #[tokio::test]
    async fn test_delete() {
        // First set a value
        set("delete_key", "\"to be deleted\"", Some("delete_ns"), None)
            .await
            .unwrap();

        // Verify it exists
        let exists_result = exists("delete_key", Some("delete_ns")).await.unwrap();
        assert!(exists_result.exists);

        // Delete it
        let delete_result = delete("delete_key", Some("delete_ns")).await.unwrap();
        assert!(delete_result.success);

        // Verify it's gone
        let exists_after = exists("delete_key", Some("delete_ns")).await.unwrap();
        assert!(!exists_after.exists);
    }

    #[tokio::test]
    async fn test_exists() {
        let result = exists("nonexistent", Some("exists_ns")).await.unwrap();
        assert!(!result.exists);

        set("existing_key", "\"value\"", Some("exists_ns"), None)
            .await
            .unwrap();

        let result = exists("existing_key", Some("exists_ns")).await.unwrap();
        assert!(result.exists);
    }

    #[tokio::test]
    async fn test_clear_namespace() {
        // Set multiple keys
        set("clear_key1", "\"val1\"", Some("clear_ns"), None)
            .await
            .unwrap();
        set("clear_key2", "\"val2\"", Some("clear_ns"), None)
            .await
            .unwrap();
        set("clear_key3", "\"val3\"", Some("clear_ns"), None)
            .await
            .unwrap();

        // Clear all keys in namespace
        let clear_result = clear(Some("clear_ns"), None).await.unwrap();
        assert_eq!(clear_result.keys_deleted, 3);

        // Verify keys are gone
        let exists1 = exists("clear_key1", Some("clear_ns")).await.unwrap();
        assert!(!exists1.exists);
    }

    #[tokio::test]
    async fn test_clear_with_pattern() {
        // Set keys with different patterns
        set("user_1", "\"val1\"", Some("pattern_ns"), None)
            .await
            .unwrap();
        set("user_2", "\"val2\"", Some("pattern_ns"), None)
            .await
            .unwrap();
        set("admin_1", "\"val3\"", Some("pattern_ns"), None)
            .await
            .unwrap();

        // Clear only keys matching "user"
        let clear_result = clear(Some("pattern_ns"), Some("user")).await.unwrap();
        assert_eq!(clear_result.keys_deleted, 2);

        // Verify user keys are gone but admin remains
        let exists_admin = exists("admin_1", Some("pattern_ns")).await.unwrap();
        assert!(exists_admin.exists);
    }

    #[tokio::test]
    async fn test_get_many() {
        set("multi_key1", "\"value1\"", Some("multi_ns"), None)
            .await
            .unwrap();
        set("multi_key2", "\"value2\"", Some("multi_ns"), None)
            .await
            .unwrap();
        set("multi_key3", "\"value3\"", Some("multi_ns"), None)
            .await
            .unwrap();

        let keys = vec![
            "multi_key1".to_string(),
            "multi_key2".to_string(),
            "nonexistent".to_string(),
        ];
        let result = get_many(keys, Some("multi_ns")).await.unwrap();

        assert_eq!(result.values.len(), 2);
        assert!(result.values.contains_key("multi_key1"));
        assert!(result.values.contains_key("multi_key2"));
        assert!(!result.values.contains_key("nonexistent"));
    }

    #[tokio::test]
    async fn test_set_many() {
        let mut entries = HashMap::new();
        entries.insert("batch_key1".to_string(), json!("batch_val1"));
        entries.insert("batch_key2".to_string(), json!({"nested": true}));
        entries.insert("batch_key3".to_string(), json!(123));

        let result = set_many(entries, None, Some("batch_ns")).await.unwrap();
        assert!(result.success);

        // Verify all keys were set
        let exists1 = exists("batch_key1", Some("batch_ns")).await.unwrap();
        let exists2 = exists("batch_key2", Some("batch_ns")).await.unwrap();
        let exists3 = exists("batch_key3", Some("batch_ns")).await.unwrap();

        assert!(exists1.exists);
        assert!(exists2.exists);
        assert!(exists3.exists);
    }

    #[tokio::test]
    async fn test_increment() {
        // Start with no value (should default to 0)
        let result = increment("counter", Some("inc_ns"), None).await.unwrap();
        assert_eq!(result.value, 1);

        // Increment again
        let result = increment("counter", Some("inc_ns"), None).await.unwrap();
        assert_eq!(result.value, 2);

        // Increment by custom amount
        let result = increment("counter", Some("inc_ns"), Some(5)).await.unwrap();
        assert_eq!(result.value, 7);
    }

    #[tokio::test]
    async fn test_decrement() {
        // Set initial value
        set("dec_counter", "10", Some("dec_ns"), None)
            .await
            .unwrap();

        // Decrement by 1 (default)
        let result = decrement("dec_counter", None, Some("dec_ns")).await.unwrap();
        assert_eq!(result.value, 9);

        // Decrement by custom amount
        let result = decrement("dec_counter", Some(3), Some("dec_ns")).await.unwrap();
        assert_eq!(result.value, 6);
    }

    #[tokio::test]
    async fn test_decrement_below_zero() {
        // Start fresh
        let result = decrement("neg_counter", Some(5), Some("neg_ns")).await.unwrap();
        assert_eq!(result.value, -5);
    }

    #[tokio::test]
    async fn test_ttl_no_expiry() {
        set("no_ttl_key", "\"value\"", Some("ttl_ns"), None)
            .await
            .unwrap();

        let result = ttl("no_ttl_key", Some("ttl_ns")).await.unwrap();
        assert_eq!(result.ttl, -2); // -2 indicates no expiry
        assert!(result.expires_at.is_empty());
    }

    #[tokio::test]
    async fn test_ttl_with_expiry() {
        set("ttl_key", "\"value\"", Some("ttl_exp_ns"), Some(3600))
            .await
            .unwrap();

        let result = ttl("ttl_key", Some("ttl_exp_ns")).await.unwrap();
        assert!(result.ttl > 0);
        assert!(result.ttl <= 3600);
        assert!(!result.expires_at.is_empty());
    }

    #[tokio::test]
    async fn test_ttl_nonexistent_key() {
        let result = ttl("nonexistent_ttl_key", Some("ttl_none_ns")).await.unwrap();
        assert_eq!(result.ttl, -1); // -1 indicates key doesn't exist
        assert!(result.expires_at.is_empty());
    }

    #[tokio::test]
    async fn test_default_namespace() {
        // Test with default namespace (None)
        set("default_ns_key", "\"default value\"", None, None)
            .await
            .unwrap();

        let result = get("default_ns_key", None).await.unwrap();
        assert!(result.found);
        assert_eq!(result.value, "\"default value\"");
    }

    #[tokio::test]
    async fn test_namespace_isolation() {
        // Set same key in different namespaces
        set("isolated_key", "\"ns1_value\"", Some("ns1"), None)
            .await
            .unwrap();
        set("isolated_key", "\"ns2_value\"", Some("ns2"), None)
            .await
            .unwrap();

        // Verify they are isolated
        let ns1_result = get("isolated_key", Some("ns1")).await.unwrap();
        let ns2_result = get("isolated_key", Some("ns2")).await.unwrap();

        assert_eq!(ns1_result.value, "\"ns1_value\"");
        assert_eq!(ns2_result.value, "\"ns2_value\"");
    }

    #[tokio::test]
    async fn test_set_plain_string() {
        // Set a plain string (not JSON formatted)
        set("plain_key", "plain text value", Some("plain_ns"), None)
            .await
            .unwrap();

        let result = get("plain_key", Some("plain_ns")).await.unwrap();
        assert!(result.found);
        // Plain text gets wrapped as a JSON string
        assert!(result.value.contains("plain text value"));
    }

    #[tokio::test]
    async fn test_overwrite_value() {
        set("overwrite_key", "\"original\"", Some("overwrite_ns"), None)
            .await
            .unwrap();

        let result1 = get("overwrite_key", Some("overwrite_ns")).await.unwrap();
        assert_eq!(result1.value, "\"original\"");

        // Overwrite with new value
        set("overwrite_key", "\"updated\"", Some("overwrite_ns"), None)
            .await
            .unwrap();

        let result2 = get("overwrite_key", Some("overwrite_ns")).await.unwrap();
        assert_eq!(result2.value, "\"updated\"");
    }
}
