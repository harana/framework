use super::*;
use serde_json::json;

fn clear_storage() {
    CONSENTS.clear();
    USER_CONSENTS.clear();
    CONSENT_HISTORY.clear();
    ACCESS_LOGS.clear();
    POLICIES.clear();
    POLICY_ACCEPTANCES.clear();
    EXPORTS.clear();
    DELETIONS.clear();
    ANONYMIZATIONS.clear();
}

#[tokio::test]
async fn test_record_consent() {
    clear_storage();
    
    let result = record_consent(
        "user123",
        "marketing",
        true,
        "1.0",
        None,
        Some("192.168.1.1"),
        Some("Mozilla/5.0"),
    )
    .await
    .unwrap();
    
    assert!(result.success);
    assert!(!result.consent_id.is_empty());
    assert_eq!(result.user_id, "user123");
}

#[tokio::test]
async fn test_record_consent_with_metadata() {
    clear_storage();
    
    let mut metadata = HashMap::new();
    metadata.insert("source".to_string(), json!("signup_form"));
    
    let result = record_consent(
        "user456",
        "analytics",
        true,
        "2.0",
        Some(metadata),
        None,
        None,
    )
    .await
    .unwrap();
    
    assert!(result.success);
}

#[tokio::test]
async fn test_get_consent_status() {
    clear_storage();
    
    record_consent("user789", "marketing", true, "1.0", None, None, None)
        .await
        .unwrap();
    record_consent("user789", "analytics", false, "1.0", None, None, None)
        .await
        .unwrap();
    
    let result = get_consent_status("user789", None).await.unwrap();
    
    assert_eq!(result.user_id, "user789");
    assert_eq!(result.consents.len(), 2);
}

#[tokio::test]
async fn test_get_consent_status_filtered() {
    clear_storage();
    
    record_consent("user_filter", "marketing", true, "1.0", None, None, None)
        .await
        .unwrap();
    record_consent("user_filter", "analytics", false, "1.0", None, None, None)
        .await
        .unwrap();
    
    let result = get_consent_status("user_filter", Some("marketing"))
        .await
        .unwrap();
    
    assert_eq!(result.consents.len(), 1);
}

#[tokio::test]
async fn test_revoke_consent() {
    clear_storage();
    
    record_consent("user_revoke", "marketing", true, "1.0", None, None, None)
        .await
        .unwrap();
    
    let result = revoke_consent("marketing", "user_revoke", Some("User request"))
        .await
        .unwrap();
    
    assert!(result.success);
    assert!(!result.consent_id.is_empty());
    
    // Verify consent is revoked
    let status = get_consent_status("user_revoke", Some("marketing"))
        .await
        .unwrap();
    
    let consent = &status.consents[0];
    assert_eq!(consent.get("revoked"), Some(&json!(true)));
}

#[tokio::test]
async fn test_update_consent() {
    clear_storage();
    
    let recorded = record_consent("user_update", "marketing", true, "1.0", None, None, None)
        .await
        .unwrap();
    
    let result = update_consent(&recorded.consent_id, false, Some("2.0"))
        .await
        .unwrap();
    
    assert!(result.success);
    
    // Verify update
    let status = get_consent_status("user_update", Some("marketing"))
        .await
        .unwrap();
    
    let consent = &status.consents[0];
    assert_eq!(consent.get("granted"), Some(&json!(false)));
    assert_eq!(consent.get("version"), Some(&json!("2.0")));
}

#[tokio::test]
async fn test_list_consent_history() {
    clear_storage();
    
    record_consent("user_history", "marketing", true, "1.0", None, None, None)
        .await
        .unwrap();
    revoke_consent("marketing", "user_history", None)
        .await
        .unwrap();
    record_consent("user_history", "marketing", true, "2.0", None, None, None)
        .await
        .unwrap();
    
    let result = list_consent_history("user_history", None, None, None, None)
        .await
        .unwrap();
    
    assert_eq!(result.user_id, "user_history");
    assert!(result.total >= 3); // granted, revoked, granted
}

#[tokio::test]
async fn test_record_data_access() {
    clear_storage();
    
    let result = record_data_access(
        "admin123",
        "user_profile",
        "read",
        "user_accessed",
        Some("Customer support"),
    )
    .await
    .unwrap();
    
    assert!(result.success);
    assert!(!result.access_id.is_empty());
}

#[tokio::test]
async fn test_get_access_log() {
    clear_storage();
    
    for i in 0..5 {
        record_data_access(
            &format!("accessor{}", i),
            "profile",
            "read",
            "user_log",
            None,
        )
        .await
        .unwrap();
    }
    
    let result = get_access_log("user_log", None, None, None).await.unwrap();
    
    assert_eq!(result.user_id, "user_log");
    assert_eq!(result.total, 5);
    assert_eq!(result.accesses.len(), 5);
}

#[tokio::test]
async fn test_create_policy_version() {
    clear_storage();
    
    let result = create_policy_version(
        "Privacy Policy",
        1700000000,
        "This is the privacy policy content...",
        "1.0.0",
    )
    .await
    .unwrap();
    
    assert!(result.success);
    assert!(!result.policy_id.is_empty());
    assert_eq!(result.version, "1.0.0");
}

#[tokio::test]
async fn test_get_active_policy() {
    clear_storage();
    
    create_policy_version(
        "Privacy Policy",
        1700000000,
        "Policy content here",
        "1.0.0",
    )
    .await
    .unwrap();
    
    let result = get_active_policy(Some("privacy")).await.unwrap();
    
    assert_eq!(result.version, "1.0.0");
    assert_eq!(result.content, "Policy content here");
}

#[tokio::test]
async fn test_get_active_policy_not_found() {
    clear_storage();
    
    let result = get_active_policy(Some("nonexistent")).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_policy_version_supersedes_previous() {
    clear_storage();
    
    create_policy_version("Policy v1", 1700000000, "Content v1", "1.0")
        .await
        .unwrap();
    create_policy_version("Policy v2", 1700100000, "Content v2", "2.0")
        .await
        .unwrap();
    
    let active = get_active_policy(None).await.unwrap();
    assert_eq!(active.version, "2.0");
}

#[tokio::test]
async fn test_record_policy_acceptance() {
    clear_storage();
    
    let policy = create_policy_version(
        "Privacy Policy",
        1700000000,
        "Content",
        "1.0",
    )
    .await
    .unwrap();
    
    let result = record_policy_acceptance(
        &policy.policy_id,
        "user_accept",
        Some("Mozilla/5.0"),
        Some("192.168.1.1"),
    )
    .await
    .unwrap();
    
    assert!(result.success);
    assert!(!result.acceptance_id.is_empty());
}

#[tokio::test]
async fn test_record_policy_acceptance_not_found() {
    clear_storage();
    
    let result = record_policy_acceptance("nonexistent", "user", None, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_export_data() {
    clear_storage();
    
    let result = export_data("user_export", None, None, Some("json"))
        .await
        .unwrap();
    
    assert!(!result.export_id.is_empty());
    assert_eq!(result.user_id, "user_export");
    assert_eq!(result.status, "completed");
    assert!(result.download_url.contains(&result.export_id));
}

#[tokio::test]
async fn test_get_export_status() {
    clear_storage();
    
    let exported = export_data("user_status", None, None, None)
        .await
        .unwrap();
    
    let result = get_export_status(&exported.export_id).await.unwrap();
    
    assert_eq!(result.export_id, exported.export_id);
    assert_eq!(result.status, "completed");
}

#[tokio::test]
async fn test_delete_data() {
    clear_storage();
    
    let result = delete_data(
        "user_delete",
        "User requested account deletion",
        Some("soft"),
        Some(vec!["billing".to_string()]),
    )
    .await
    .unwrap();
    
    assert!(result.success);
    assert!(!result.deletion_id.is_empty());
    assert_eq!(result.status, "scheduled");
}

#[tokio::test]
async fn test_get_deletion_status() {
    clear_storage();
    
    let deleted = delete_data("user_del_status", "Test", None, None)
        .await
        .unwrap();
    
    let result = get_deletion_status(&deleted.deletion_id).await.unwrap();
    
    assert_eq!(result.deletion_id, deleted.deletion_id);
    assert_eq!(result.user_id, "user_del_status");
    assert_eq!(result.status, "scheduled");
}

#[tokio::test]
async fn test_cancel_deletion() {
    clear_storage();
    
    let deleted = delete_data("user_cancel", "Test", None, None)
        .await
        .unwrap();
    
    let result = cancel_deletion(&deleted.deletion_id).await.unwrap();
    
    assert!(result.success);
    assert!(result.cancelled);
    
    // Verify cancelled
    let status = get_deletion_status(&deleted.deletion_id).await.unwrap();
    assert_eq!(status.status, "cancelled");
}

#[tokio::test]
async fn test_cancel_deletion_already_cancelled() {
    clear_storage();
    
    let deleted = delete_data("user_double_cancel", "Test", None, None)
        .await
        .unwrap();
    
    cancel_deletion(&deleted.deletion_id).await.unwrap();
    
    // Try to cancel again
    let result = cancel_deletion(&deleted.deletion_id).await.unwrap();
    assert!(result.success);
    assert!(!result.cancelled); // Already cancelled, so cancelled=false
}

#[tokio::test]
async fn test_anonymize_data() {
    clear_storage();
    
    let result = anonymize_data(
        "user_anon",
        Some(vec!["billing".to_string()]),
    )
    .await
    .unwrap();
    
    assert!(result.success);
    assert!(!result.anonymization_id.is_empty());
    assert_eq!(result.user_id, "user_anon");
}

#[tokio::test]
async fn test_verify_rtbf_eligibility() {
    clear_storage();
    
    let result = verify_rtbf_eligibility("user_rtbf").await.unwrap();
    
    assert_eq!(result.user_id, "user_rtbf");
    assert!(result.eligible);
    assert!(result.blocking_factors.is_empty());
    assert!(!result.reasons.is_empty());
}

#[tokio::test]
async fn test_verify_rtbf_with_pending_deletion() {
    clear_storage();
    
    delete_data("user_rtbf_pending", "Test", None, None)
        .await
        .unwrap();
    
    let result = verify_rtbf_eligibility("user_rtbf_pending").await.unwrap();
    
    assert!(!result.eligible);
    assert!(!result.blocking_factors.is_empty());
}

#[tokio::test]
async fn test_full_consent_flow() {
    clear_storage();
    
    // 1. Record initial consent
    let consent = record_consent(
        "user_flow",
        "marketing",
        true,
        "1.0",
        None,
        Some("192.168.1.1"),
        None,
    )
    .await
    .unwrap();
    
    assert!(consent.success);
    
    // 2. Check consent status
    let status = get_consent_status("user_flow", Some("marketing"))
        .await
        .unwrap();
    
    assert_eq!(status.consents.len(), 1);
    
    // 3. Update consent
    update_consent(&consent.consent_id, false, Some("1.1"))
        .await
        .unwrap();
    
    // 4. Revoke consent
    revoke_consent("marketing", "user_flow", Some("Changed mind"))
        .await
        .unwrap();
    
    // 5. Check history
    let history = list_consent_history("user_flow", None, None, None, None)
        .await
        .unwrap();
    
    assert!(history.total >= 3); // granted, updated, revoked
    
    // 6. Verify status shows revoked
    let final_status = get_consent_status("user_flow", Some("marketing"))
        .await
        .unwrap();
    
    let consent_record = &final_status.consents[0];
    assert_eq!(consent_record.get("revoked"), Some(&json!(true)));
}

#[tokio::test]
async fn test_full_gdpr_flow() {
    clear_storage();
    
    let user_id = "gdpr_user";
    
    // 1. Create privacy policy
    let policy = create_policy_version(
        "Privacy Policy",
        1700000000,
        "GDPR compliant policy...",
        "1.0",
    )
    .await
    .unwrap();
    
    // 2. User accepts policy
    record_policy_acceptance(&policy.policy_id, user_id, None, None)
        .await
        .unwrap();
    
    // 3. User gives consent
    record_consent(user_id, "marketing", true, "1.0", None, None, None)
        .await
        .unwrap();
    record_consent(user_id, "analytics", true, "1.0", None, None, None)
        .await
        .unwrap();
    
    // 4. Track data access
    record_data_access("system", "profile", "read", user_id, Some("Service delivery"))
        .await
        .unwrap();
    
    // 5. User requests data export
    let export = export_data(user_id, None, None, Some("json"))
        .await
        .unwrap();
    
    assert_eq!(export.status, "completed");
    
    // 6. User checks RTBF eligibility
    let rtbf = verify_rtbf_eligibility(user_id).await.unwrap();
    assert!(rtbf.eligible);
    
    // 7. User requests deletion
    let deletion = delete_data(user_id, "GDPR Article 17 request", None, None)
        .await
        .unwrap();
    
    assert_eq!(deletion.status, "scheduled");
}
