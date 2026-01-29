use super::*;
use serial_test::serial;
use serde_json::json;

fn clear_storage() {
    POLICIES.clear();
    ROLE_POLICIES.clear();
    USER_POLICIES.clear();
}

#[tokio::test]
#[serial]
async fn test_create_policy() {
    clear_storage();
    
    let result = create(
        vec!["read".to_string(), "write".to_string()],
        "allow",
        vec!["documents/*".to_string()],
        "DocumentAccess",
        HashMap::new(),
        Some("Allow access to documents"),
    )
    .await
    .unwrap();
    
    assert!(result.success);
    assert!(!result.policy_id.is_empty());
}

#[tokio::test]
#[serial]
async fn test_create_policy_invalid_effect() {
    clear_storage();
    
    let result = create(
        vec!["read".to_string()],
        "invalid",
        vec!["*".to_string()],
        "Test",
        HashMap::new(),
        None,
    )
    .await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Effect must be"));
}

#[tokio::test]
#[serial]
async fn test_get_policy() {
    clear_storage();
    
    let created = create(
        vec!["read".to_string()],
        "allow",
        vec!["*".to_string()],
        "ReadAll",
        HashMap::new(),
        Some("Read all resources"),
    )
    .await
    .unwrap();
    
    let result = get(&created.policy_id).await.unwrap();
    
    assert_eq!(result.name, "ReadAll");
    assert_eq!(result.effect, "allow");
    assert_eq!(result.actions, vec!["read"]);
    assert_eq!(result.resources, vec!["*"]);
    assert_eq!(result.description, "Read all resources");
}

#[tokio::test]
#[serial]
async fn test_get_policy_not_found() {
    clear_storage();
    
    let result = get("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn test_update_policy() {
    clear_storage();
    
    let created = create(
        vec!["read".to_string()],
        "allow",
        vec!["*".to_string()],
        "OriginalName",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    let result = update(
        &created.policy_id,
        Some("UpdatedName"),
        None,
        Some("New description"),
        Some(vec!["specific/*".to_string()]),
        Some(vec!["read".to_string(), "write".to_string()]),
        Some("deny"),
    )
    .await
    .unwrap();
    
    assert!(result.success);
    
    // Verify changes
    let policy = get(&created.policy_id).await.unwrap();
    assert_eq!(policy.name, "UpdatedName");
    assert_eq!(policy.description, "New description");
    assert_eq!(policy.effect, "deny");
    assert_eq!(policy.actions, vec!["read", "write"]);
    assert_eq!(policy.resources, vec!["specific/*"]);
}

#[tokio::test]
#[serial]
async fn test_delete_policy() {
    clear_storage();
    
    let created = create(
        vec!["read".to_string()],
        "allow",
        vec!["*".to_string()],
        "ToDelete",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    let result = delete(&created.policy_id).await.unwrap();
    assert!(result.success);
    
    // Verify deleted
    let get_result = get(&created.policy_id).await;
    assert!(get_result.is_err());
}

#[tokio::test]
#[serial]
async fn test_list_policies() {
    clear_storage();
    
    for i in 0..5 {
        create(
            vec!["action".to_string()],
            "allow",
            vec!["*".to_string()],
            &format!("Policy{}", i),
            HashMap::new(),
            None,
        )
        .await
        .unwrap();
    }
    
    let result = list_policies(None, None).await.unwrap();
    assert_eq!(result.total, 5);
    assert_eq!(result.policies.len(), 5);
}

#[tokio::test]
#[serial]
async fn test_list_policies_pagination() {
    clear_storage();
    
    for i in 0..10 {
        create(
            vec!["action".to_string()],
            "allow",
            vec!["*".to_string()],
            &format!("Policy{}", i),
            HashMap::new(),
            None,
        )
        .await
        .unwrap();
    }
    
    let page1 = list_policies(Some(3), Some(0)).await.unwrap();
    assert_eq!(page1.policies.len(), 3);
    assert_eq!(page1.total, 10);
    
    let page2 = list_policies(Some(3), Some(3)).await.unwrap();
    assert_eq!(page2.policies.len(), 3);
}

#[tokio::test]
#[serial]
async fn test_attach_to_role() {
    clear_storage();
    
    let created = create(
        vec!["read".to_string()],
        "allow",
        vec!["*".to_string()],
        "Test",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    let result = attach_to_role(&created.policy_id, "admin").await.unwrap();
    assert!(result.success);
    
    // Verify attached
    let policies = ROLE_POLICIES.get("admin").unwrap();
    assert!(policies.contains(&created.policy_id));
}

#[tokio::test]
#[serial]
async fn test_attach_to_role_policy_not_found() {
    clear_storage();
    
    let result = attach_to_role("nonexistent", "admin").await;
    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn test_detach_from_role() {
    clear_storage();
    
    let created = create(
        vec!["read".to_string()],
        "allow",
        vec!["*".to_string()],
        "Test",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    attach_to_role(&created.policy_id, "admin").await.unwrap();
    
    let result = detach_from_role(&created.policy_id, "admin").await.unwrap();
    assert!(result.success);
    
    // Verify detached
    let policies = ROLE_POLICIES.get("admin").unwrap();
    assert!(!policies.contains(&created.policy_id));
}

#[tokio::test]
#[serial]
async fn test_attach_to_user() {
    clear_storage();
    
    let created = create(
        vec!["read".to_string()],
        "allow",
        vec!["*".to_string()],
        "Test",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    let result = attach_to_user(&created.policy_id, "user123").await.unwrap();
    assert!(result.success);
}

#[tokio::test]
#[serial]
async fn test_detach_from_user() {
    clear_storage();
    
    let created = create(
        vec!["read".to_string()],
        "allow",
        vec!["*".to_string()],
        "Test",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    attach_to_user(&created.policy_id, "user123").await.unwrap();
    
    let result = detach_from_user(&created.policy_id, "user123").await.unwrap();
    assert!(result.success);
}

#[tokio::test]
#[serial]
async fn test_evaluate_allow() {
    clear_storage();
    
    let created = create(
        vec!["read".to_string(), "write".to_string()],
        "allow",
        vec!["documents/*".to_string()],
        "DocAccess",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    let result = evaluate("read", "documents/file.txt", &created.policy_id, None)
        .await
        .unwrap();
    
    assert!(result.allowed);
    assert!(result.reason.contains("allows"));
}

#[tokio::test]
#[serial]
async fn test_evaluate_deny() {
    clear_storage();
    
    let created = create(
        vec!["delete".to_string()],
        "deny",
        vec!["*".to_string()],
        "NoDelete",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    let result = evaluate("delete", "anything", &created.policy_id, None)
        .await
        .unwrap();
    
    assert!(!result.allowed);
    assert!(result.reason.contains("denies"));
}

#[tokio::test]
#[serial]
async fn test_evaluate_action_not_covered() {
    clear_storage();
    
    let created = create(
        vec!["read".to_string()],
        "allow",
        vec!["*".to_string()],
        "ReadOnly",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    let result = evaluate("write", "anything", &created.policy_id, None)
        .await
        .unwrap();
    
    assert!(!result.allowed);
    assert!(result.reason.contains("Action not covered"));
}

#[tokio::test]
#[serial]
async fn test_evaluate_resource_not_covered() {
    clear_storage();
    
    let created = create(
        vec!["read".to_string()],
        "allow",
        vec!["documents/*".to_string()],
        "DocsOnly",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    let result = evaluate("read", "images/photo.jpg", &created.policy_id, None)
        .await
        .unwrap();
    
    assert!(!result.allowed);
    assert!(result.reason.contains("Resource not covered"));
}

#[tokio::test]
#[serial]
async fn test_evaluate_with_conditions_met() {
    clear_storage();
    
    let mut conditions = HashMap::new();
    conditions.insert("department".to_string(), json!({"equals": "engineering"}));
    
    let created = create(
        vec!["*".to_string()],
        "allow",
        vec!["*".to_string()],
        "EngOnly",
        conditions,
        None,
    )
    .await
    .unwrap();
    
    let mut context = HashMap::new();
    context.insert("department".to_string(), json!("engineering"));
    
    let result = evaluate("read", "anything", &created.policy_id, Some(context))
        .await
        .unwrap();
    
    assert!(result.allowed);
    assert_eq!(result.evaluated_conditions.get("department"), Some(&json!(true)));
}

#[tokio::test]
#[serial]
async fn test_evaluate_with_conditions_not_met() {
    clear_storage();
    
    let mut conditions = HashMap::new();
    conditions.insert("department".to_string(), json!({"equals": "engineering"}));
    
    let created = create(
        vec!["*".to_string()],
        "allow",
        vec!["*".to_string()],
        "EngOnly",
        conditions,
        None,
    )
    .await
    .unwrap();
    
    let mut context = HashMap::new();
    context.insert("department".to_string(), json!("sales"));
    
    let result = evaluate("read", "anything", &created.policy_id, Some(context))
        .await
        .unwrap();
    
    assert!(!result.allowed);
    assert!(result.reason.contains("Conditions not met"));
}

#[tokio::test]
#[serial]
async fn test_evaluate_with_in_condition() {
    clear_storage();
    
    let mut conditions = HashMap::new();
    conditions.insert("role".to_string(), json!({"in": ["admin", "manager"]}));
    
    let created = create(
        vec!["*".to_string()],
        "allow",
        vec!["*".to_string()],
        "AdminOrManager",
        conditions,
        None,
    )
    .await
    .unwrap();
    
    let mut context = HashMap::new();
    context.insert("role".to_string(), json!("manager"));
    
    let result = evaluate("anything", "anything", &created.policy_id, Some(context))
        .await
        .unwrap();
    
    assert!(result.allowed);
}

#[tokio::test]
#[serial]
async fn test_evaluate_wildcard_action() {
    clear_storage();
    
    let created = create(
        vec!["s3:*".to_string()],
        "allow",
        vec!["*".to_string()],
        "S3All",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    let result = evaluate("s3:GetObject", "bucket", &created.policy_id, None)
        .await
        .unwrap();
    
    assert!(result.allowed);
}

#[tokio::test]
#[serial]
async fn test_delete_removes_from_attachments() {
    clear_storage();
    
    let created = create(
        vec!["read".to_string()],
        "allow",
        vec!["*".to_string()],
        "Test",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    // Attach to role and user
    attach_to_role(&created.policy_id, "admin").await.unwrap();
    attach_to_user(&created.policy_id, "user123").await.unwrap();
    
    // Delete policy
    delete(&created.policy_id).await.unwrap();
    
    // Verify removed from attachments
    let role_policies = ROLE_POLICIES.get("admin").unwrap();
    assert!(!role_policies.contains(&created.policy_id));
    
    let user_policies = USER_POLICIES.get("user123").unwrap();
    assert!(!user_policies.contains(&created.policy_id));
}

#[tokio::test]
#[serial]
async fn test_matches_pattern_exact() {
    assert!(matches_pattern("read", "read"));
    assert!(!matches_pattern("read", "write"));
}

#[tokio::test]
#[serial]
async fn test_matches_pattern_wildcard() {
    assert!(matches_pattern("*", "anything"));
    assert!(matches_pattern("s3:*", "s3:GetObject"));
    assert!(matches_pattern("documents/*", "documents/file.txt"));
    assert!(!matches_pattern("documents/*", "images/file.txt"));
}

#[tokio::test]
#[serial]
async fn test_multiple_policies_same_role() {
    clear_storage();
    
    let policy1 = create(
        vec!["read".to_string()],
        "allow",
        vec!["*".to_string()],
        "Policy1",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    let policy2 = create(
        vec!["write".to_string()],
        "allow",
        vec!["*".to_string()],
        "Policy2",
        HashMap::new(),
        None,
    )
    .await
    .unwrap();
    
    attach_to_role(&policy1.policy_id, "admin").await.unwrap();
    attach_to_role(&policy2.policy_id, "admin").await.unwrap();
    
    let policies = ROLE_POLICIES.get("admin").unwrap();
    assert_eq!(policies.len(), 2);
}
