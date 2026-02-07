#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_create_session() {
        let result = create("user123", None, None).await.unwrap();
        assert!(!result.session_id.is_empty());
        assert!(!result.expires_at.is_empty());
    }

    #[tokio::test]
    async fn test_create_session_with_data() {
        let mut data = HashMap::new();
        data.insert("role".to_string(), json!("admin"));
        data.insert("theme".to_string(), json!("dark"));
        
        let result = create("user_with_data", None, Some(data)).await.unwrap();
        assert!(!result.session_id.is_empty());
        
        // Verify data was stored
        let get_result = get(&result.session_id).await.unwrap();
        assert!(get_result.found);
        assert_eq!(get_result.data.get("role"), Some(&json!("admin")));
    }

    #[tokio::test]
    async fn test_create_session_with_custom_ttl() {
        let result = create("user_ttl", Some(7200), None).await.unwrap();
        assert!(!result.session_id.is_empty());
        
        // Verify session is valid
        let validate_result = validate(&result.session_id).await.unwrap();
        assert!(validate_result.valid);
    }

    #[tokio::test]
    async fn test_get_session() {
        let create_result = create("user_get", None, None).await.unwrap();
        
        let result = get(&create_result.session_id).await.unwrap();
        assert!(result.found);
        assert_eq!(result.user_id, "user_get");
        assert!(!result.created_at.is_empty());
        assert!(!result.expires_at.is_empty());
    }

    #[tokio::test]
    async fn test_get_nonexistent_session() {
        let result = get("nonexistent-session-id").await.unwrap();
        assert!(!result.found);
        assert!(result.user_id.is_empty());
    }

    #[tokio::test]
    async fn test_update_session_merge() {
        let mut initial_data = HashMap::new();
        initial_data.insert("key1".to_string(), json!("value1"));
        
        let create_result = create("user_update", None, Some(initial_data)).await.unwrap();
        
        // Update with merge (default)
        let mut new_data = HashMap::new();
        new_data.insert("key2".to_string(), json!("value2"));
        
        let update_result = update(&create_result.session_id, new_data, None).await.unwrap();
        assert!(update_result.success);
        
        // Verify both keys exist
        let get_result = get(&create_result.session_id).await.unwrap();
        assert_eq!(get_result.data.get("key1"), Some(&json!("value1")));
        assert_eq!(get_result.data.get("key2"), Some(&json!("value2")));
    }

    #[tokio::test]
    async fn test_update_session_replace() {
        let mut initial_data = HashMap::new();
        initial_data.insert("key1".to_string(), json!("value1"));
        
        let create_result = create("user_replace", None, Some(initial_data)).await.unwrap();
        
        // Update with replace (merge = false)
        let mut new_data = HashMap::new();
        new_data.insert("key2".to_string(), json!("value2"));
        
        let update_result = update(&create_result.session_id, new_data, Some(false)).await.unwrap();
        assert!(update_result.success);
        
        // Verify only new key exists
        let get_result = get(&create_result.session_id).await.unwrap();
        assert!(get_result.data.get("key1").is_none());
        assert_eq!(get_result.data.get("key2"), Some(&json!("value2")));
    }

    #[tokio::test]
    async fn test_destroy_session() {
        let create_result = create("user_destroy", None, None).await.unwrap();
        
        // Verify session exists
        let get_result = get(&create_result.session_id).await.unwrap();
        assert!(get_result.found);
        
        // Destroy session
        let destroy_result = destroy(&create_result.session_id).await.unwrap();
        assert!(destroy_result.success);
        
        // Verify session is gone
        let get_after = get(&create_result.session_id).await.unwrap();
        assert!(!get_after.found);
    }

    #[tokio::test]
    async fn test_refresh_session() {
        let create_result = create("user_refresh", Some(60), None).await.unwrap();
        
        // Refresh with longer TTL
        let refresh_result = refresh(&create_result.session_id, Some(7200)).await.unwrap();
        assert!(refresh_result.success);
        assert!(!refresh_result.expires_at.is_empty());
    }

    #[tokio::test]
    async fn test_refresh_nonexistent_session() {
        let result = refresh("nonexistent-session", None).await.unwrap();
        assert!(!result.success);
        assert!(result.expires_at.is_empty());
    }

    #[tokio::test]
    async fn test_get_value() {
        let mut data = HashMap::new();
        data.insert("username".to_string(), json!("testuser"));
        
        let create_result = create("user_getvalue", None, Some(data)).await.unwrap();
        
        let result = get_value(&create_result.session_id, "username").await.unwrap();
        assert!(result.found);
        assert!(result.value.contains("testuser"));
    }

    #[tokio::test]
    async fn test_get_value_not_found() {
        let create_result = create("user_getvalue_none", None, None).await.unwrap();
        
        let result = get_value(&create_result.session_id, "nonexistent").await.unwrap();
        assert!(!result.found);
        assert!(result.value.is_empty());
    }

    #[tokio::test]
    async fn test_set_value() {
        let create_result = create("user_setvalue", None, None).await.unwrap();
        
        let set_result = set_value(&create_result.session_id, "newkey", "\"newvalue\"").await.unwrap();
        assert!(set_result.success);
        
        // Verify value was set
        let get_result = get_value(&create_result.session_id, "newkey").await.unwrap();
        assert!(get_result.found);
        assert!(get_result.value.contains("newvalue"));
    }

    #[tokio::test]
    async fn test_set_value_json() {
        let create_result = create("user_setjson", None, None).await.unwrap();
        
        let set_result = set_value(&create_result.session_id, "obj", r#"{"nested": true}"#).await.unwrap();
        assert!(set_result.success);
        
        let get_result = get_value(&create_result.session_id, "obj").await.unwrap();
        assert!(get_result.found);
        assert!(get_result.value.contains("nested"));
    }

    #[tokio::test]
    async fn test_delete_value() {
        let mut data = HashMap::new();
        data.insert("toDelete".to_string(), json!("value"));
        
        let create_result = create("user_delvalue", None, Some(data)).await.unwrap();
        
        // Delete the value
        let delete_result = delete_value(&create_result.session_id, "toDelete").await.unwrap();
        assert!(delete_result.success);
        
        // Verify value is gone
        let get_result = get_value(&create_result.session_id, "toDelete").await.unwrap();
        assert!(!get_result.found);
    }

    #[tokio::test]
    async fn test_list_users_sessions() {
        let user_id = "user_list_sessions";
        
        // Create multiple sessions for the same user
        let session1 = create(user_id, None, None).await.unwrap();
        let session2 = create(user_id, None, None).await.unwrap();
        let session3 = create(user_id, None, None).await.unwrap();
        
        let result = list_users(user_id).await.unwrap();
        assert!(result.total >= 3);
        
        // Clean up
        let _ = destroy(&session1.session_id).await;
        let _ = destroy(&session2.session_id).await;
        let _ = destroy(&session3.session_id).await;
    }

    #[tokio::test]
    async fn test_list_users_no_sessions() {
        let result = list_users("user_with_no_sessions").await.unwrap();
        assert_eq!(result.total, 0);
        assert!(result.sessions.is_empty());
    }

    #[tokio::test]
    async fn test_destroy_users_sessions() {
        let user_id = "user_destroy_all";
        
        // Create multiple sessions
        let _session1 = create(user_id, None, None).await.unwrap();
        let _session2 = create(user_id, None, None).await.unwrap();
        let _session3 = create(user_id, None, None).await.unwrap();
        
        // Destroy all sessions
        let result = destroy_users(user_id, None).await.unwrap();
        assert!(result.success);
        assert_eq!(result.destroyed_count, 3);
        
        // Verify all sessions are gone
        let list_result = list_users(user_id).await.unwrap();
        assert_eq!(list_result.total, 0);
    }

    #[tokio::test]
    async fn test_destroy_users_sessions_except_one() {
        let user_id = "user_destroy_except";
        
        // Create multiple sessions
        let _session1 = create(user_id, None, None).await.unwrap();
        let session2 = create(user_id, None, None).await.unwrap();
        let _session3 = create(user_id, None, None).await.unwrap();
        
        // Destroy all except session2
        let result = destroy_users(user_id, Some(&session2.session_id)).await.unwrap();
        assert!(result.success);
        assert_eq!(result.destroyed_count, 2);
        
        // Verify session2 still exists
        let validate_result = validate(&session2.session_id).await.unwrap();
        assert!(validate_result.valid);
        
        // Clean up
        let _ = destroy(&session2.session_id).await;
    }

    #[tokio::test]
    async fn test_validate_valid_session() {
        let create_result = create("user_validate", None, None).await.unwrap();
        
        let result = validate(&create_result.session_id).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.user_id, "user_validate");
        assert!(!result.expires_at.is_empty());
        
        // Clean up
        let _ = destroy(&create_result.session_id).await;
    }

    #[tokio::test]
    async fn test_validate_nonexistent_session() {
        let result = validate("nonexistent-session").await.unwrap();
        assert!(!result.valid);
        assert!(result.user_id.is_empty());
        assert!(result.expires_at.is_empty());
    }

    #[tokio::test]
    async fn test_session_isolation() {
        // Create sessions for different users
        let session1 = create("user_a", None, None).await.unwrap();
        let session2 = create("user_b", None, None).await.unwrap();
        
        // Verify they are isolated
        let get1 = get(&session1.session_id).await.unwrap();
        let get2 = get(&session2.session_id).await.unwrap();
        
        assert_eq!(get1.user_id, "user_a");
        assert_eq!(get2.user_id, "user_b");
        
        // Clean up
        let _ = destroy(&session1.session_id).await;
        let _ = destroy(&session2.session_id).await;
    }
}
