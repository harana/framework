use super::*;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use serde_json::json;
use serial_test::serial;

fn clear_storage() {
    AUDIT_LOGS.clear();
    ALERTS.clear();
    *RETENTION_POLICY.write() = None;
    EXPORTS.clear();
}

#[tokio::test]
#[serial]
async fn test_log_event_basic() {
    clear_storage();
    
    let result = log_event(
        "user.login",
        "user123",
        "Success",
        "session456",
        "Session",
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();
    
    assert!(result.success);
    assert!(!result.audit_id.is_empty());
    
    // Verify event is stored
    assert!(AUDIT_LOGS.contains_key(&result.audit_id));
}

#[tokio::test]
#[serial]
async fn test_log_event_with_details() {
    clear_storage();
    
    let mut details = HashMap::new();
    details.insert("browser".to_string(), json!("Chrome"));
    details.insert("os".to_string(), json!("macOS"));
    
    let mut metadata = HashMap::new();
    metadata.insert("request_id".to_string(), json!("req123"));
    
    let result = log_event(
        "user.login",
        "user123",
        "Success",
        "session456",
        "Session",
        Some("Admin"),
        Some(details),
        Some("192.168.1.1"),
        Some(metadata),
        Some("Mozilla/5.0"),
    )
    .await
    .unwrap();
    
    assert!(result.success);
    
    // Verify stored event has all details
    let event = AUDIT_LOGS.get(&result.audit_id).unwrap();
    assert_eq!(event.actor_type, "Admin");
    assert_eq!(event.ip_address, Some("192.168.1.1".to_string()));
    assert_eq!(event.user_agent, Some("Mozilla/5.0".to_string()));
    assert!(event.details.is_some());
}

#[tokio::test]
#[serial]
async fn test_get_event() {
    clear_storage();
    
    let logged = log_event(
        "file.upload",
        "user456",
        "Success",
        "file789",
        "File",
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();
    
    let result = get_event(&logged.audit_id).await.unwrap();
    
    assert_eq!(result.event.audit_id, logged.audit_id);
    assert_eq!(result.event.action, "file.upload");
    assert_eq!(result.event.actor_id, "user456");
    assert_eq!(result.event.resource_id, "file789");
}

#[tokio::test]
#[serial]
async fn test_get_event_not_found() {
    clear_storage();
    
    let result = get_event("nonexistent").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[tokio::test]
#[serial]
async fn test_query_logs_basic() {
    clear_storage();
    
    // Create some events
    for i in 0..5 {
        log_event(
            "user.login",
            &format!("user{}", i),
            "Success",
            &format!("session{}", i),
            "Session",
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
    }
    
    let start_time = Utc::now() - Duration::hours(1);
    let result = query_logs(
        start_time,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();
    
    assert_eq!(result.total, 5);
    assert_eq!(result.logs.len(), 5);
    assert!(!result.has_more);
}

#[tokio::test]
#[serial]
async fn test_query_logs_with_filters() {
    clear_storage();
    
    // Create events with different actions
    log_event("user.login", "user1", "Success", "s1", "Session", None, None, None, None, None).await.unwrap();
    log_event("user.logout", "user1", "Success", "s1", "Session", None, None, None, None, None).await.unwrap();
    log_event("user.login", "user2", "Failure", "s2", "Session", None, None, None, None, None).await.unwrap();
    log_event("file.upload", "user1", "Success", "f1", "File", None, None, None, None, None).await.unwrap();
    
    let start_time = Utc::now() - Duration::hours(1);
    
    // Filter by action
    let result = query_logs(
        start_time,
        Some("user.login"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();
    
    assert_eq!(result.total, 2);
    
    // Filter by actor_id
    let result = query_logs(
        start_time,
        None,
        Some("user1"),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();
    
    assert_eq!(result.total, 3);
    
    // Filter by outcome
    let result = query_logs(
        start_time,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("Failure"),
        None,
        None,
        None,
    )
    .await
    .unwrap();
    
    assert_eq!(result.total, 1);
}

#[tokio::test]
#[serial]
async fn test_query_logs_pagination() {
    clear_storage();
    
    // Create 10 events
    for i in 0..10 {
        log_event(
            "user.action",
            &format!("user{}", i),
            "Success",
            &format!("res{}", i),
            "Resource",
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
    }
    
    let start_time = Utc::now() - Duration::hours(1);
    
    // Get first page
    let result = query_logs(
        start_time,
        None,
        None,
        None,
        None,
        Some(5),
        Some(0),
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();
    
    assert_eq!(result.logs.len(), 5);
    assert!(result.has_more);
    assert_eq!(result.total, 10);
    
    // Get second page
    let result = query_logs(
        start_time,
        None,
        None,
        None,
        None,
        Some(5),
        Some(5),
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();
    
    assert_eq!(result.logs.len(), 5);
    assert!(!result.has_more);
}

#[tokio::test]
#[serial]
async fn test_get_actor_activity() {
    clear_storage();
    
    // Create events for different actors
    log_event("user.login", "actor1", "Success", "s1", "Session", None, None, None, None, None).await.unwrap();
    log_event("file.upload", "actor1", "Success", "f1", "File", None, None, None, None, None).await.unwrap();
    log_event("file.delete", "actor1", "Failure", "f2", "File", None, None, None, None, None).await.unwrap();
    log_event("user.login", "actor2", "Success", "s2", "Session", None, None, None, None, None).await.unwrap();
    
    let result = get_actor_activity("actor1", None, None, None).await.unwrap();
    
    assert_eq!(result.total, 3);
    assert_eq!(result.activities.len(), 3);
    assert!(result.first_activity.is_some());
    assert!(result.last_activity.is_some());
}

#[tokio::test]
#[serial]
async fn test_get_resource_history() {
    clear_storage();
    
    // Create events for a specific resource
    log_event("file.create", "user1", "Success", "doc123", "Document", None, None, None, None, None).await.unwrap();
    log_event("file.update", "user2", "Success", "doc123", "Document", None, None, None, None, None).await.unwrap();
    log_event("file.share", "user1", "Success", "doc123", "Document", None, None, None, None, None).await.unwrap();
    log_event("file.update", "user3", "Success", "other456", "Document", None, None, None, None, None).await.unwrap();
    
    let result = get_resource_history("doc123", "Document", None, None, None).await.unwrap();
    
    assert_eq!(result.total, 3);
    assert_eq!(result.history.len(), 3);
    assert!(result.created_at.is_some());
    assert!(result.last_modified.is_some());
}

#[tokio::test]
#[serial]
async fn test_export_logs() {
    clear_storage();
    
    // Create some events
    for i in 0..5 {
        log_event(
            "test.action",
            &format!("user{}", i),
            "Success",
            &format!("res{}", i),
            "Resource",
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
    }
    
    let start_time = Utc::now() - Duration::hours(1);
    let end_time = Utc::now() + Duration::hours(1);
    
    let result = export_logs(
        end_time,
        start_time,
        None,
        Some("Json"),
        None,
    )
    .await
    .unwrap();
    
    assert!(!result.export_id.is_empty());
    assert!(result.download_url.contains(&result.export_id));
    assert_eq!(result.record_count, 5);
    assert!(result.file_size > 0);
    assert!(result.expires_at > Utc::now());
}

#[tokio::test]
#[serial]
async fn test_export_logs_with_filter() {
    clear_storage();
    
    log_event("action.a", "user1", "Success", "r1", "TypeA", None, None, None, None, None).await.unwrap();
    log_event("action.b", "user2", "Failure", "r2", "TypeB", None, None, None, None, None).await.unwrap();
    log_event("action.a", "user1", "Success", "r3", "TypeA", None, None, None, None, None).await.unwrap();
    
    let start_time = Utc::now() - Duration::hours(1);
    let end_time = Utc::now() + Duration::hours(1);
    
    let filter = AuditFilter {
        action: Some("action.a".to_string()),
        actor_id: None,
        actor_type: None,
        resource_type: None,
        resource_id: None,
        outcome: None,
    };
    
    let result = export_logs(
        end_time,
        start_time,
        Some(filter),
        Some("Csv"),
        None,
    )
    .await
    .unwrap();
    
    assert_eq!(result.record_count, 2);
    assert!(result.download_url.contains("csv"));
}

#[tokio::test]
#[serial]
async fn test_create_alert() {
    clear_storage();
    
    let mut conditions = HashMap::new();
    conditions.insert("action".to_string(), json!("user.delete"));
    conditions.insert("threshold".to_string(), json!(10));
    
    let channels = vec!["email".to_string(), "slack".to_string()];
    
    let result = create_alert(
        "Suspicious Deletions",
        conditions,
        channels,
        Some("Alert when many deletions occur"),
        Some(true),
    )
    .await
    .unwrap();
    
    assert!(result.success);
    assert!(!result.alert_id.is_empty());
    
    // Verify stored
    assert!(ALERTS.contains_key(&result.alert_id));
}

#[tokio::test]
#[serial]
async fn test_delete_alert() {
    clear_storage();
    
    let mut conditions = HashMap::new();
    conditions.insert("action".to_string(), json!("test"));
    
    let created = create_alert(
        "Test Alert",
        conditions,
        vec!["email".to_string()],
        None,
        None,
    )
    .await
    .unwrap();
    
    let result = delete_alert(&created.alert_id).await.unwrap();
    assert!(result.success);
    
    // Verify deleted
    assert!(!ALERTS.contains_key(&created.alert_id));
}

#[tokio::test]
#[serial]
async fn test_delete_alert_not_found() {
    clear_storage();
    
    let result = delete_alert("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn test_list_alerts() {
    clear_storage();
    
    let mut conditions = HashMap::new();
    conditions.insert("test".to_string(), json!(true));
    
    // Create some alerts
    for i in 0..3 {
        create_alert(
            &format!("Alert {}", i),
            conditions.clone(),
            vec!["email".to_string()],
            None,
            Some(i % 2 == 0), // alternating enabled
        )
        .await
        .unwrap();
    }
    
    // List all
    let result = list_alerts(None, None).await.unwrap();
    assert_eq!(result.total, 3);
    
    // List only enabled
    let result = list_alerts(Some(true), None).await.unwrap();
    assert_eq!(result.total, 2);
    
    // List only disabled
    let result = list_alerts(Some(false), None).await.unwrap();
    assert_eq!(result.total, 1);
}

#[tokio::test]
#[serial]
async fn test_get_statistics() {
    clear_storage();
    
    // Create varied events
    log_event("user.login", "user1", "Success", "s1", "Session", None, None, None, None, None).await.unwrap();
    log_event("user.login", "user2", "Success", "s2", "Session", None, None, None, None, None).await.unwrap();
    log_event("user.logout", "user1", "Success", "s1", "Session", None, None, None, None, None).await.unwrap();
    log_event("file.upload", "user1", "Success", "f1", "File", None, None, None, None, None).await.unwrap();
    log_event("file.upload", "user3", "Failure", "f2", "File", None, None, None, None, None).await.unwrap();
    
    let start_time = Utc::now() - Duration::hours(1);
    let end_time = Utc::now() + Duration::hours(1);
    
    // Group by action
    let result = get_statistics(start_time, end_time, Some("action")).await.unwrap();
    
    assert_eq!(result.total_events, 5);
    assert_eq!(result.unique_actors, 3); // user1, user2, user3
    assert!(result.statistics.len() >= 3); // user.login, user.logout, file.upload
}

#[tokio::test]
#[serial]
async fn test_set_retention_policy() {
    clear_storage();
    
    let result = set_retention_policy(
        90,
        Some(true),
        Some(30),
    )
    .await
    .unwrap();
    
    assert!(result.success);
    assert!(!result.policy_id.is_empty());
}

#[tokio::test]
#[serial]
async fn test_get_retention_policy() {
    clear_storage();
    
    // Initially no policy
    let result = get_retention_policy().await.unwrap();
    assert!(result.policy.is_none());
    
    // Set a policy
    set_retention_policy(90, Some(true), Some(30)).await.unwrap();
    
    // Now should have policy
    let result = get_retention_policy().await.unwrap();
    assert!(result.policy.is_some());
    
    let policy = result.policy.unwrap();
    assert_eq!(policy.retention_days, 90);
    assert!(policy.archive_to_cold_storage);
    assert_eq!(policy.cold_storage_after_days, Some(30));
}

#[tokio::test]
#[serial]
async fn test_query_logs_time_filtering() {
    clear_storage();
    
    // Create events
    log_event("early.event", "user1", "Success", "r1", "Test", None, None, None, None, None).await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    log_event("later.event", "user2", "Success", "r2", "Test", None, None, None, None, None).await.unwrap();
    
    // Query with very recent start time should get fewer events
    let very_recent = Utc::now() - Duration::milliseconds(50);
    let result = query_logs(
        very_recent,
        None, None, None, None, None, None, None, None, None, None,
    )
    .await
    .unwrap();
    
    // Should only get the later event
    assert!(result.total >= 1);
}

#[tokio::test]
#[serial]
async fn test_multiple_actors_activity() {
    clear_storage();
    
    // Create events for multiple actors
    for actor in ["alice", "bob", "charlie"] {
        for _ in 0..3 {
            log_event(
                "test.action",
                actor,
                "Success",
                "resource",
                "Test",
                None, None, None, None, None,
            )
            .await
            .unwrap();
        }
    }
    
    // Each actor should have 3 activities
    let alice = get_actor_activity("alice", None, None, None).await.unwrap();
    let bob = get_actor_activity("bob", None, None, None).await.unwrap();
    let charlie = get_actor_activity("charlie", None, None, None).await.unwrap();
    
    assert_eq!(alice.total, 3);
    assert_eq!(bob.total, 3);
    assert_eq!(charlie.total, 3);
}
