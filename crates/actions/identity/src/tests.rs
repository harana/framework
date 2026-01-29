use super::*;
use serial_test::serial;
use serde_json::json;

fn clear_storage() {
    USERS.clear();
    USERNAME_INDEX.clear();
    EMAIL_INDEX.clear();
    TOKENS.clear();
    REFRESH_TOKENS.clear();
    RESET_TOKENS.clear();
}

#[tokio::test]
#[serial]
async fn test_create_user() {
    clear_storage();
    
    let result = create_user(
        "testuser",
        "password123",
        "test@example.com",
        Some(vec!["user".to_string()]),
        None,
    )
    .await
    .unwrap();
    
    assert!(result.success);
    assert!(!result.user_id.is_empty());
    
    // Verify user is stored
    assert!(USERS.contains_key(&result.user_id));
}

#[tokio::test]
#[serial]
async fn test_create_user_duplicate_username() {
    clear_storage();
    
    create_user("dupuser", "pass1", "dup1@example.com", None, None)
        .await
        .unwrap();
    
    let result = create_user("dupuser", "pass2", "dup2@example.com", None, None).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Username already exists"));
}

#[tokio::test]
#[serial]
async fn test_create_user_duplicate_email() {
    clear_storage();
    
    create_user("user1", "pass1", "same@example.com", None, None)
        .await
        .unwrap();
    
    let result = create_user("user2", "pass2", "same@example.com", None, None).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Email already exists"));
}

#[tokio::test]
#[serial]
async fn test_get_user() {
    clear_storage();
    
    let created = create_user(
        "getuser",
        "password",
        "getuser@example.com",
        Some(vec!["admin".to_string()]),
        None,
    )
    .await
    .unwrap();
    
    let result = get_user(&created.user_id).await.unwrap();
    
    assert_eq!(result.user_id, created.user_id);
    assert_eq!(result.username, "getuser");
    assert_eq!(result.email, "getuser@example.com");
    assert!(result.roles.contains(&"admin".to_string()));
}

#[tokio::test]
#[serial]
async fn test_get_user_not_found() {
    clear_storage();
    
    let result = get_user("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn test_authenticate_success() {
    clear_storage();
    
    create_user("authuser", "secretpass", "auth@example.com", None, None)
        .await
        .unwrap();
    
    let result = authenticate("authuser", "secretpass", None).await.unwrap();
    
    assert!(result.success);
    assert!(!result.access_token.is_empty());
    assert!(!result.refresh_token.is_empty());
    assert!(!result.user_id.is_empty());
}

#[tokio::test]
#[serial]
async fn test_authenticate_wrong_password() {
    clear_storage();
    
    create_user("authuser2", "rightpass", "auth2@example.com", None, None)
        .await
        .unwrap();
    
    let result = authenticate("authuser2", "wrongpass", None).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid username or password"));
}

#[tokio::test]
#[serial]
async fn test_authenticate_wrong_username() {
    clear_storage();
    
    let result = authenticate("nouser", "anypass", None).await;
    
    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn test_verify_token() {
    clear_storage();
    
    create_user("verifyuser", "password", "verify@example.com", None, None)
        .await
        .unwrap();
    
    let auth = authenticate("verifyuser", "password", None).await.unwrap();
    
    let result = verify_token(&auth.access_token).await.unwrap();
    
    assert!(result.valid);
    assert_eq!(result.user_id, auth.user_id);
    assert!(result.claims.contains_key("username"));
}

#[tokio::test]
#[serial]
async fn test_verify_token_revoked() {
    clear_storage();
    
    create_user("revokeuser", "password", "revoke@example.com", None, None)
        .await
        .unwrap();
    
    let auth = authenticate("revokeuser", "password", None).await.unwrap();
    
    // Revoke the token
    revoke_token(&auth.access_token).await.unwrap();
    
    // Verify it's no longer valid
    let result = verify_token(&auth.access_token).await.unwrap();
    assert!(!result.valid);
}

#[tokio::test]
#[serial]
async fn test_refresh_token() {
    clear_storage();
    
    create_user("refreshuser", "password", "refresh@example.com", None, None)
        .await
        .unwrap();
    
    let auth = authenticate("refreshuser", "password", None).await.unwrap();
    let old_access_token = auth.access_token.clone();
    
    let result = refresh_token(&auth.refresh_token).await.unwrap();
    
    assert!(!result.access_token.is_empty());
    assert!(!result.refresh_token.is_empty());
    assert_ne!(result.access_token, old_access_token);
    
    // Old token should be revoked
    let old_verify = verify_token(&old_access_token).await.unwrap();
    assert!(!old_verify.valid);
    
    // New token should be valid
    let new_verify = verify_token(&result.access_token).await.unwrap();
    assert!(new_verify.valid);
}

#[tokio::test]
#[serial]
async fn test_change_password() {
    clear_storage();
    
    let created = create_user("pwduser", "oldpass", "pwd@example.com", None, None)
        .await
        .unwrap();
    
    let result = change_password("newpass", "oldpass", &created.user_id)
        .await
        .unwrap();
    
    assert!(result.success);
    
    // Old password should no longer work
    let auth_old = authenticate("pwduser", "oldpass", None).await;
    assert!(auth_old.is_err());
    
    // New password should work
    let auth_new = authenticate("pwduser", "newpass", None).await;
    assert!(auth_new.is_ok());
}

#[tokio::test]
#[serial]
async fn test_change_password_wrong_current() {
    clear_storage();
    
    let created = create_user("pwduser2", "realpass", "pwd2@example.com", None, None)
        .await
        .unwrap();
    
    let result = change_password("newpass", "wrongpass", &created.user_id).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Current password is incorrect"));
}

#[tokio::test]
#[serial]
async fn test_request_password_reset() {
    clear_storage();
    
    create_user("resetuser", "password", "reset@example.com", None, None)
        .await
        .unwrap();
    
    let result = request_password_reset("reset@example.com").await.unwrap();
    
    assert!(result.success);
    assert!(!result.reset_token.is_empty());
}

#[tokio::test]
#[serial]
async fn test_request_password_reset_email_not_found() {
    clear_storage();
    
    let result = request_password_reset("noone@example.com").await;
    
    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn test_reset_password() {
    clear_storage();
    
    create_user("resetuser2", "oldpass", "reset2@example.com", None, None)
        .await
        .unwrap();
    
    let reset_req = request_password_reset("reset2@example.com").await.unwrap();
    
    let result = reset_password(&reset_req.reset_token, "newpass123").await.unwrap();
    
    assert!(result.success);
    
    // New password should work
    let auth = authenticate("resetuser2", "newpass123", None).await;
    assert!(auth.is_ok());
}

#[tokio::test]
#[serial]
async fn test_reset_password_token_already_used() {
    clear_storage();
    
    create_user("resetuser3", "oldpass", "reset3@example.com", None, None)
        .await
        .unwrap();
    
    let reset_req = request_password_reset("reset3@example.com").await.unwrap();
    
    // Use the token once
    reset_password(&reset_req.reset_token, "newpass1").await.unwrap();
    
    // Try to use it again
    let result = reset_password(&reset_req.reset_token, "newpass2").await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already used"));
}

#[tokio::test]
#[serial]
async fn test_update_user() {
    clear_storage();
    
    let created = create_user(
        "updateuser",
        "password",
        "update@example.com",
        Some(vec!["user".to_string()]),
        None,
    )
    .await
    .unwrap();
    
    let mut metadata = HashMap::new();
    metadata.insert("theme".to_string(), json!("dark"));
    
    let result = update_user(
        &created.user_id,
        Some(metadata),
        Some(vec!["user".to_string(), "admin".to_string()]),
        Some("updated@example.com"),
    )
    .await
    .unwrap();
    
    assert!(result.success);
    
    // Verify changes
    let user = get_user(&created.user_id).await.unwrap();
    assert_eq!(user.email, "updated@example.com");
    assert!(user.roles.contains(&"admin".to_string()));
    assert_eq!(user.metadata.get("theme"), Some(&json!("dark")));
}

#[tokio::test]
#[serial]
async fn test_update_user_email_conflict() {
    clear_storage();
    
    create_user("user1", "pass", "email1@example.com", None, None)
        .await
        .unwrap();
    
    let created2 = create_user("user2", "pass", "email2@example.com", None, None)
        .await
        .unwrap();
    
    // Try to change user2's email to user1's email
    let result = update_user(
        &created2.user_id,
        None,
        None,
        Some("email1@example.com"),
    )
    .await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Email already in use"));
}

#[tokio::test]
#[serial]
async fn test_delete_user_soft() {
    clear_storage();
    
    let created = create_user("deleteuser", "pass", "delete@example.com", None, None)
        .await
        .unwrap();
    
    let result = delete_user(&created.user_id, Some(false)).await.unwrap();
    assert!(result.success);
    
    // User should not be retrievable
    let get_result = get_user(&created.user_id).await;
    assert!(get_result.is_err());
    
    // But user should still exist in storage (soft delete)
    assert!(USERS.contains_key(&created.user_id));
}

#[tokio::test]
#[serial]
async fn test_delete_user_hard() {
    clear_storage();
    
    let created = create_user("harddelete", "pass", "hard@example.com", None, None)
        .await
        .unwrap();
    
    let result = delete_user(&created.user_id, Some(true)).await.unwrap();
    assert!(result.success);
    
    // User should be completely removed
    assert!(!USERS.contains_key(&created.user_id));
    assert!(!USERNAME_INDEX.contains_key("harddelete"));
    assert!(!EMAIL_INDEX.contains_key("hard@example.com"));
}

#[tokio::test]
#[serial]
async fn test_list_users() {
    clear_storage();
    
    // Create multiple users
    for i in 0..5 {
        create_user(
            &format!("listuser{}", i),
            "pass",
            &format!("list{}@example.com", i),
            Some(vec![if i % 2 == 0 { "admin" } else { "user" }.to_string()]),
            None,
        )
        .await
        .unwrap();
    }
    
    let result = list_users(None, None, None, None).await.unwrap();
    assert_eq!(result.total, 5);
    assert_eq!(result.users.len(), 5);
}

#[tokio::test]
#[serial]
async fn test_list_users_with_role_filter() {
    clear_storage();
    
    create_user("admin1", "pass", "admin1@ex.com", Some(vec!["admin".to_string()]), None).await.unwrap();
    create_user("admin2", "pass", "admin2@ex.com", Some(vec!["admin".to_string()]), None).await.unwrap();
    create_user("user1", "pass", "user1@ex.com", Some(vec!["user".to_string()]), None).await.unwrap();
    
    let result = list_users(None, Some(vec!["admin".to_string()]), None, None)
        .await
        .unwrap();
    
    assert_eq!(result.total, 2);
}

#[tokio::test]
#[serial]
async fn test_list_users_with_search() {
    clear_storage();
    
    create_user("alice", "pass", "alice@company.com", None, None).await.unwrap();
    create_user("bob", "pass", "bob@company.com", None, None).await.unwrap();
    create_user("charlie", "pass", "charlie@other.com", None, None).await.unwrap();
    
    let result = list_users(None, None, None, Some("company")).await.unwrap();
    
    assert_eq!(result.total, 2);
}

#[tokio::test]
#[serial]
async fn test_list_users_pagination() {
    clear_storage();
    
    for i in 0..10 {
        create_user(
            &format!("pageuser{}", i),
            "pass",
            &format!("page{}@example.com", i),
            None,
            None,
        )
        .await
        .unwrap();
    }
    
    // First page
    let page1 = list_users(Some(5), None, Some(0), None).await.unwrap();
    assert_eq!(page1.users.len(), 5);
    assert_eq!(page1.total, 10);
    
    // Second page
    let page2 = list_users(Some(5), None, Some(5), None).await.unwrap();
    assert_eq!(page2.users.len(), 5);
}

#[tokio::test]
#[serial]
async fn test_authenticate_deleted_user() {
    clear_storage();
    
    let created = create_user("deletedauth", "pass", "delauth@example.com", None, None)
        .await
        .unwrap();
    
    delete_user(&created.user_id, Some(false)).await.unwrap();
    
    let result = authenticate("deletedauth", "pass", None).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("deleted"));
}

#[tokio::test]
#[serial]
async fn test_full_auth_flow() {
    clear_storage();
    
    // 1. Create user
    let created = create_user(
        "fullflow",
        "initialpass",
        "fullflow@example.com",
        Some(vec!["user".to_string()]),
        None,
    )
    .await
    .unwrap();
    
    // 2. Authenticate
    let auth = authenticate("fullflow", "initialpass", None).await.unwrap();
    assert!(auth.success);
    
    // 3. Verify token
    let verify = verify_token(&auth.access_token).await.unwrap();
    assert!(verify.valid);
    
    // 4. Update user
    update_user(
        &created.user_id,
        None,
        Some(vec!["user".to_string(), "premium".to_string()]),
        None,
    )
    .await
    .unwrap();
    
    // 5. Refresh token
    let refreshed = refresh_token(&auth.refresh_token).await.unwrap();
    
    // 6. Verify new token
    let verify_new = verify_token(&refreshed.access_token).await.unwrap();
    assert!(verify_new.valid);
    
    // 7. Old token should be revoked
    let verify_old = verify_token(&auth.access_token).await.unwrap();
    assert!(!verify_old.valid);
    
    // 8. Change password
    change_password("newpass", "initialpass", &created.user_id)
        .await
        .unwrap();
    
    // 9. Authenticate with new password
    let auth_new = authenticate("fullflow", "newpass", None).await.unwrap();
    assert!(auth_new.success);
}
