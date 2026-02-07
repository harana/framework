use super::*;

#[tokio::test]
async fn test_record_consent() {
    clear_all_data();
    
    let result = record_consent(
        "user123".to_string(),
        ConsentType::Marketing,
        true,
        "1.0".to_string(),
        Some("192.168.1.1".to_string()),
        Some("Mozilla/5.0".to_string()),
        None,
    ).await;

    assert!(result.consent_id.starts_with("con_"));
    assert_eq!(result.user_id, "user123");
    assert!(result.success);
    assert!(result.timestamp > 0);
}

#[tokio::test]
async fn test_revoke_consent() {
    clear_all_data();
    
    // First record consent
    record_consent(
        "user123".to_string(),
        ConsentType::Analytics,
        true,
        "1.0".to_string(),
        None,
        None,
        None,
    ).await;

    // Then revoke it
    let result = revoke_consent(
        "user123".to_string(),
        ConsentType::Analytics,
        Some("User requested".to_string()),
    ).await;

    assert!(result.success);
    assert!(result.revoked_at > 0);
}

#[tokio::test]
async fn test_get_consent_status() {
    clear_all_data();
    
    // Record multiple consents
    record_consent(
        "user456".to_string(),
        ConsentType::Marketing,
        true,
        "1.0".to_string(),
        None,
        None,
        None,
    ).await;

    record_consent(
        "user456".to_string(),
        ConsentType::Analytics,
        false,
        "1.0".to_string(),
        None,
        None,
        None,
    ).await;

    // Get all consents
    let result = get_consent_status("user456".to_string(), None).await;

    assert_eq!(result.user_id, "user456");
    assert_eq!(result.consents.len(), 2);
}

#[tokio::test]
async fn test_update_consent() {
    clear_all_data();
    
    // Record consent
    let consent = record_consent(
        "user789".to_string(),
        ConsentType::Functional,
        false,
        "1.0".to_string(),
        None,
        None,
        None,
    ).await;

    // Update it
    let result = update_consent(
        consent.consent_id,
        true,
        Some("2.0".to_string()),
    ).await;

    assert!(result.success);
}

#[tokio::test]
async fn test_export_data_lifecycle() {
    clear_all_data();
    
    // Request export
    let export = export_data(
        "user_export".to_string(),
        Some(ExportFormat::Json),
        Some(vec!["profile".to_string(), "orders".to_string()]),
        None,
    ).await;

    assert!(export.export_id.starts_with("exp_"));
    assert_eq!(export.status, "pending");

    // Check status
    let status = get_export_status(export.export_id).await;
    // Status should be pending or completed depending on timing
    assert!(status.status == ExportStatus::Pending || status.status == ExportStatus::Completed);
}

#[tokio::test]
async fn test_delete_data_lifecycle() {
    clear_all_data();
    
    // Request deletion
    let deletion = delete_data(
        "user_delete".to_string(),
        "User requested account deletion".to_string(),
        Some(DeleteType::Soft),
        Some(vec!["legal".to_string()]),
    ).await;

    assert!(deletion.deletion_id.starts_with("del_"));
    assert!(deletion.success);

    // Check status
    let status = get_deletion_status(deletion.deletion_id.clone()).await;
    assert_eq!(status.status, DeletionStatus::Pending);

    // Cancel deletion
    let cancel = cancel_deletion(deletion.deletion_id).await;
    assert!(cancel.cancelled);
    assert!(cancel.success);
}

#[tokio::test]
async fn test_anonymize_data() {
    clear_all_data();
    
    let result = anonymize_data(
        "user_anon".to_string(),
        Some(vec!["audit_logs".to_string()]),
    ).await;

    assert!(result.anonymization_id.starts_with("anon_"));
    assert!(result.success);
}

#[tokio::test]
async fn test_consent_history() {
    clear_all_data();
    
    // Record consent changes
    record_consent(
        "user_history".to_string(),
        ConsentType::Marketing,
        true,
        "1.0".to_string(),
        None,
        None,
        None,
    ).await;

    revoke_consent(
        "user_history".to_string(),
        ConsentType::Marketing,
        None,
    ).await;

    // Get history
    let result = list_consent_history(
        "user_history".to_string(),
        Some(ConsentType::Marketing),
        None,
        None,
        None,
    ).await;

    assert_eq!(result.history.len(), 2);
    assert_eq!(result.history[0].action, "granted");
    assert_eq!(result.history[1].action, "revoked");
}

#[tokio::test]
async fn test_data_access_log() {
    clear_all_data();
    
    // Record accesses
    record_data_access(
        "user_access".to_string(),
        "admin_001".to_string(),
        AccessType::Read,
        "user_profile".to_string(),
        Some("Customer support request".to_string()),
    ).await;

    record_data_access(
        "user_access".to_string(),
        "system".to_string(),
        AccessType::Write,
        "user_preferences".to_string(),
        None,
    ).await;

    // Get log
    let result = get_access_log(
        "user_access".to_string(),
        None,
        None,
        None,
    ).await;

    assert_eq!(result.accesses.len(), 2);
    assert_eq!(result.total, 2);
}

#[tokio::test]
async fn test_policy_lifecycle() {
    clear_all_data();
    
    // Create policy
    let policy = create_policy_version(
        "Privacy Policy".to_string(),
        "This is our privacy policy content...".to_string(),
        "1.0.0".to_string(),
        Utc::now().timestamp(),
    ).await;

    assert!(policy.policy_id.starts_with("pol_"));
    assert!(policy.success);

    // Get active policy
    let active = get_active_policy(Some(PolicyType::Privacy)).await;
    assert_eq!(active.version, "1.0.0");
    assert!(!active.content.is_empty());
}

#[tokio::test]
async fn test_policy_acceptance() {
    clear_all_data();
    
    // Create a policy first
    let policy = create_policy_version(
        "Terms of Service".to_string(),
        "Terms content...".to_string(),
        "2.0.0".to_string(),
        Utc::now().timestamp(),
    ).await;

    // Record acceptance
    let result = record_policy_acceptance(
        "user_accept".to_string(),
        policy.policy_id,
        Some("10.0.0.1".to_string()),
        Some("Chrome/100".to_string()),
    ).await;

    assert!(result.acceptance_id.starts_with("acc_"));
    assert!(result.success);
}

#[tokio::test]
async fn test_rtbf_eligibility() {
    clear_all_data();
    
    let result = verify_rtbf_eligibility("user_rtbf".to_string()).await;

    assert_eq!(result.user_id, "user_rtbf");
    assert!(result.eligible);
    assert!(!result.reasons.is_empty());
    assert!(result.blocking_factors.is_empty());
}
