use crate::*;

#[tokio::test]
async fn test_generate_registration_options() {
    let result = generate_registration_options(
        "example.com",
        "https://example.com",
        "user123",
        "john.doe@example.com",
        "John Doe",
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();

    assert!(!result.challenge.is_empty());
    assert_eq!(result.options.rp.id, "example.com");
    assert_eq!(result.options.user.name, "john.doe@example.com");
    assert_eq!(result.options.user.display_name, "John Doe");
    assert!(result.options.timeout > 0); // webauthn-rs default is 300000
    assert_eq!(result.options.attestation, "none");
    assert!(!result.options.pub_key_cred_params.is_empty());
}

#[tokio::test]
async fn test_store_and_get_passkey() {
    let credential_id = format!("test_cred_{}", uuid::Uuid::new_v4());
    let user_id = "user456";
    let public_key = vec![0x04; 65];

    let store_result = store_credential(
        user_id,
        &credential_id,
        public_key.clone(),
        0,
        Some("My Passkey"),
    )
    .await
    .unwrap();

    assert!(store_result.success);
    assert_eq!(store_result.credential_id, credential_id);

    let get_result = get_passkey(&credential_id).await.unwrap();
    assert!(get_result.found);
    assert!(get_result.passkey.is_some());
    let passkey = get_result.passkey.unwrap();
    assert_eq!(passkey.credential_id, credential_id);
    assert_eq!(passkey.name, Some("My Passkey".to_string()));
    assert_eq!(passkey.counter, 0);
}

#[tokio::test]
async fn test_get_user_passkeys() {
    let user_id = format!("user_{}", uuid::Uuid::new_v4());
    let public_key = vec![0x04; 65];

    for i in 0..3 {
        let credential_id = format!("cred_{}_{}", user_id, i);
        store_credential(
            &user_id,
            &credential_id,
            public_key.clone(),
            0,
            Some(&format!("Passkey {}", i)),
        )
        .await
        .unwrap();
    }

    let result = get_user_passkeys(&user_id).await.unwrap();
    assert_eq!(result.passkeys.len(), 3);
}

#[tokio::test]
async fn test_update_counter() {
    let credential_id = format!("counter_test_{}", uuid::Uuid::new_v4());
    let public_key = vec![0x04; 65];

    store_credential("user789", &credential_id, public_key, 5, None)
        .await
        .unwrap();

    let update_result = update_counter(&credential_id, 10).await.unwrap();
    assert!(update_result.success);
    assert_eq!(update_result.new_counter, 10);

    let get_result = get_passkey(&credential_id).await.unwrap();
    assert!(get_result.found);
    assert_eq!(get_result.passkey.unwrap().counter, 10);
}

#[tokio::test]
async fn test_update_passkey_name() {
    let credential_id = format!("name_test_{}", uuid::Uuid::new_v4());
    let public_key = vec![0x04; 65];

    store_credential("user_name_test", &credential_id, public_key, 0, Some("Old Name"))
        .await
        .unwrap();

    let update_result = update_passkey_name(&credential_id, "New Name")
        .await
        .unwrap();
    assert!(update_result.success);

    let get_result = get_passkey(&credential_id).await.unwrap();
    assert!(get_result.found);
    assert_eq!(get_result.passkey.unwrap().name, Some("New Name".to_string()));
}

#[tokio::test]
async fn test_delete_passkey() {
    let credential_id = format!("delete_test_{}", uuid::Uuid::new_v4());
    let public_key = vec![0x04; 65];

    store_credential("delete_user", &credential_id, public_key, 0, None)
        .await
        .unwrap();

    let delete_result = delete_passkey(&credential_id).await.unwrap();
    assert!(delete_result.success);

    let get_result = get_passkey(&credential_id).await.unwrap();
    assert!(!get_result.found);
    assert!(get_result.passkey.is_none());
}

#[tokio::test]
async fn test_delete_nonexistent_passkey() {
    let delete_result = delete_passkey("nonexistent_credential").await.unwrap();
    assert!(!delete_result.success);
}

#[tokio::test]
async fn test_check_support() {
    let result = check_support().await.unwrap();
    assert!(result.supported);
    assert!(!result.platform_authenticator_available);
    assert!(!result.conditional_mediation_available);
}

#[tokio::test]
async fn test_challenges_are_unique() {
    let result1 = generate_registration_options(
        "example.com",
        "https://example.com",
        "user1",
        "user1@example.com",
        "User 1",
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();

    let result2 = generate_registration_options(
        "example.com",
        "https://example.com",
        "user2",
        "user2@example.com",
        "User 2",
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();

    assert_ne!(result1.challenge, result2.challenge);
}

#[tokio::test]
async fn test_passkey_not_found() {
    let result = get_passkey("nonexistent_credential").await.unwrap();
    assert!(!result.found);
    assert!(result.passkey.is_none());
}

#[tokio::test]
async fn test_update_counter_not_found() {
    let result = update_counter("nonexistent_credential", 10).await;
    assert!(result.is_err());
}
