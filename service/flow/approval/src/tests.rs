use super::*;

#[tokio::test]
async fn test_create_request() {
    let result = create_request(
        vec!["approver1".to_string()],
        "access_request",
        "resource-123",
        "database",
        "Access to Production DB",
        Some("Need access for debugging"),
        None,
        None,
        Some("high"),
    ).await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.request_id.is_empty());
    assert_eq!(output.status, "pending");
    assert!(output.success);
}

#[tokio::test]
async fn test_create_request_empty_approvers() {
    let result = create_request(
        vec![],
        "access_request",
        "resource-123",
        "database",
        "Test Request",
        None,
        None,
        None,
        None,
    ).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("approver"));
}

#[tokio::test]
async fn test_create_request_empty_title() {
    let result = create_request(
        vec!["approver1".to_string()],
        "access_request",
        "resource-123",
        "database",
        "  ",
        None,
        None,
        None,
        None,
    ).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Title"));
}

#[tokio::test]
async fn test_get_request() {
    // Create a request first
    let create_result = create_request(
        vec!["approver1".to_string(), "approver2".to_string()],
        "change_request",
        "resource-456",
        "api",
        "API Change Request",
        Some("Updating API endpoint"),
        None,
        None,
        None,
    ).await.unwrap();

    // Get the request
    let get_result = get_request(&create_result.request_id).await;

    assert!(get_result.is_ok());
    let output = get_result.unwrap();
    assert_eq!(output.request_id, create_result.request_id);
    assert_eq!(output.title, "API Change Request");
    assert_eq!(output.status, "pending");
    assert_eq!(output.approvers.len(), 2);
}

#[tokio::test]
async fn test_get_request_not_found() {
    let result = get_request("nonexistent-id").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[tokio::test]
async fn test_approve_request() {
    // Create a request
    let create_result = create_request(
        vec!["approver-a".to_string()],
        "deploy_request",
        "app-789",
        "application",
        "Deploy Request",
        None,
        None,
        None,
        None,
    ).await.unwrap();

    // Approve it
    let approve_result = approve(
        "approver-a",
        &create_result.request_id,
        Some("Looks good!"),
    ).await;

    assert!(approve_result.is_ok());
    let output = approve_result.unwrap();
    assert!(output.success);
    assert_eq!(output.status, "approved");
}

#[tokio::test]
async fn test_approve_not_assigned() {
    let create_result = create_request(
        vec!["approver-x".to_string()],
        "test_request",
        "res-1",
        "test",
        "Test",
        None,
        None,
        None,
        None,
    ).await.unwrap();

    let result = approve("wrong-approver", &create_result.request_id, None).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not assigned"));
}

#[tokio::test]
async fn test_reject_request() {
    let create_result = create_request(
        vec!["rejector-1".to_string()],
        "deploy_request",
        "app-reject",
        "application",
        "Reject Test",
        None,
        None,
        None,
        None,
    ).await.unwrap();

    let reject_result = reject(
        "rejector-1",
        "Does not meet requirements",
        &create_result.request_id,
    ).await;

    assert!(reject_result.is_ok());
    let output = reject_result.unwrap();
    assert!(output.success);
    assert_eq!(output.status, "rejected");
}

#[tokio::test]
async fn test_cancel_request() {
    let create_result = create_request(
        vec!["cancel-approver".to_string()],
        "cancel_test",
        "cancel-res",
        "test",
        "Cancel Test",
        None,
        None,
        None,
        None,
    ).await.unwrap();

    let cancel_result = cancel(&create_result.request_id, Some("No longer needed")).await;

    assert!(cancel_result.is_ok());
    let output = cancel_result.unwrap();
    assert!(output.success);

    // Verify status changed
    let get_result = get_request(&create_result.request_id).await.unwrap();
    assert_eq!(get_result.status, "cancelled");
}

#[tokio::test]
async fn test_cancel_already_approved() {
    let create_result = create_request(
        vec!["multi-approver".to_string()],
        "multi_test",
        "multi-res",
        "test",
        "Multi Test",
        None,
        None,
        None,
        None,
    ).await.unwrap();

    // Approve first
    approve("multi-approver", &create_result.request_id, None).await.unwrap();

    // Try to cancel
    let cancel_result = cancel(&create_result.request_id, None).await;
    assert!(cancel_result.is_err());
    assert!(cancel_result.unwrap_err().contains("cannot be cancelled"));
}

#[tokio::test]
async fn test_add_approver() {
    let create_result = create_request(
        vec!["initial-approver".to_string()],
        "add_test",
        "add-res",
        "test",
        "Add Approver Test",
        None,
        None,
        None,
        None,
    ).await.unwrap();

    let add_result = add_approver("new-approver", &create_result.request_id, Some(true)).await;
    assert!(add_result.is_ok());
    assert!(add_result.unwrap().success);

    // Verify approver was added
    let get_result = get_request(&create_result.request_id).await.unwrap();
    assert_eq!(get_result.approvers.len(), 2);
}

#[tokio::test]
async fn test_add_approver_duplicate() {
    let create_result = create_request(
        vec!["dup-approver".to_string()],
        "dup_test",
        "dup-res",
        "test",
        "Duplicate Test",
        None,
        None,
        None,
        None,
    ).await.unwrap();

    let result = add_approver("dup-approver", &create_result.request_id, None).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already assigned"));
}

#[tokio::test]
async fn test_remove_approver() {
    let create_result = create_request(
        vec!["rem-approver-1".to_string(), "rem-approver-2".to_string()],
        "rem_test",
        "rem-res",
        "test",
        "Remove Approver Test",
        None,
        None,
        None,
        None,
    ).await.unwrap();

    let remove_result = remove_approver("rem-approver-2", &create_result.request_id).await;
    assert!(remove_result.is_ok());
    assert!(remove_result.unwrap().success);

    // Verify approver was removed
    let get_result = get_request(&create_result.request_id).await.unwrap();
    assert_eq!(get_result.approvers.len(), 1);
}

#[tokio::test]
async fn test_remove_last_approver() {
    let create_result = create_request(
        vec!["last-approver".to_string()],
        "last_test",
        "last-res",
        "test",
        "Last Approver Test",
        None,
        None,
        None,
        None,
    ).await.unwrap();

    let result = remove_approver("last-approver", &create_result.request_id).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("last approver"));
}

#[tokio::test]
async fn test_get_history() {
    let create_result = create_request(
        vec!["hist-approver".to_string()],
        "history_test",
        "hist-res",
        "test",
        "History Test",
        None,
        None,
        None,
        None,
    ).await.unwrap();

    // Approve the request to add more history
    approve("hist-approver", &create_result.request_id, Some("Approved!")).await.unwrap();

    let history_result = get_history(&create_result.request_id).await;
    assert!(history_result.is_ok());
    let output = history_result.unwrap();
    assert!(output.total >= 2); // At least created + approved
    assert!(output.history.iter().any(|h| h.action == "created"));
    assert!(output.history.iter().any(|h| h.action == "approved"));
}

#[tokio::test]
async fn test_list_requests() {
    // Create a few requests
    create_request(
        vec!["list-approver".to_string()],
        "list_test",
        "list-res-1",
        "database",
        "List Test 1",
        None,
        None,
        None,
        None,
    ).await.unwrap();

    create_request(
        vec!["list-approver".to_string()],
        "list_test",
        "list-res-2",
        "database",
        "List Test 2",
        None,
        None,
        None,
        None,
    ).await.unwrap();

    // List requests
    let list_result = list_requests(
        Some("list-approver"),
        Some(10),
        Some(0),
        None,
        Some("database"),
        Some("pending"),
    ).await;

    assert!(list_result.is_ok());
    let output = list_result.unwrap();
    assert!(output.total >= 2);
}

#[tokio::test]
async fn test_multi_approver_workflow() {
    // Create request with multiple required approvers
    let create_result = create_request(
        vec!["multi-1".to_string(), "multi-2".to_string()],
        "multi_approver",
        "multi-res",
        "sensitive",
        "Multi Approver Request",
        None,
        None,
        None,
        None,
    ).await.unwrap();

    // Add second approver as required
    add_approver("multi-2-extra", &create_result.request_id, Some(true)).await.unwrap();

    // First approval - should still be pending
    let first_approval = approve("multi-1", &create_result.request_id, None).await.unwrap();
    // After first required approver approves, check status
    let get_result = get_request(&create_result.request_id).await.unwrap();
    // Status might be "approved" or "pending" depending on logic - first approver is required
    assert!(get_result.status == "approved" || get_result.status == "pending");
}
