// Tests for the secret module

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_and_get_secret() {
        let name = "test-secret-1";
        let value = "my-secret-value";

        // Set a secret
        let set_result = set_secret(name, value, Some("Test secret"), None).await;
        assert!(set_result.is_ok());
        let set_output = set_result.unwrap();
        assert!(set_output.success);
        assert!(!set_output.version.is_empty());

        // Get the secret
        let get_result = get_secret(name, None).await;
        assert!(get_result.is_ok());
        let get_output = get_result.unwrap();
        assert_eq!(get_output.value, value);

        // Clean up
        let _ = delete_secret(name, Some(true)).await;
    }

    #[tokio::test]
    async fn test_secret_exists() {
        let name = "test-secret-exists";
        
        // Should not exist initially
        let exists_result = secret_exists(name).await;
        assert!(exists_result.is_ok());
        assert!(!exists_result.unwrap().exists);

        // Create the secret
        let _ = set_secret(name, "value", None, None).await;

        // Should exist now
        let exists_result = secret_exists(name).await;
        assert!(exists_result.is_ok());
        assert!(exists_result.unwrap().exists);

        // Clean up
        let _ = delete_secret(name, Some(true)).await;
    }

    #[tokio::test]
    async fn test_rotate_secret() {
        let name = "test-secret-rotate";
        
        // Create initial secret
        let set_result = set_secret(name, "initial-value", None, None).await;
        assert!(set_result.is_ok());
        let initial_version = set_result.unwrap().version;

        // Rotate the secret
        let rotate_result = rotate_secret(name, "new-value").await;
        assert!(rotate_result.is_ok());
        let rotate_output = rotate_result.unwrap();
        assert!(rotate_output.success);
        assert_eq!(rotate_output.old_version, initial_version);
        assert_ne!(rotate_output.new_version, initial_version);

        // Verify new value
        let get_result = get_secret(name, None).await;
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap().value, "new-value");

        // Clean up
        let _ = delete_secret(name, Some(true)).await;
    }

    #[tokio::test]
    async fn test_generate_secret() {
        let name = "test-secret-generate";
        
        // Generate alphanumeric secret
        let gen_result = generate_secret(name, Some(16), Some("Alphanumeric"), None).await;
        assert!(gen_result.is_ok());
        let gen_output = gen_result.unwrap();
        assert!(gen_output.success);
        assert_eq!(gen_output.value.len(), 16);

        // Clean up
        let _ = delete_secret(name, Some(true)).await;
    }

    #[tokio::test]
    async fn test_list_secrets() {
        let prefix = "test-list-";
        
        // Create some secrets
        let _ = set_secret(&format!("{}1", prefix), "value1", None, None).await;
        let _ = set_secret(&format!("{}2", prefix), "value2", None, None).await;

        // List secrets with prefix
        let list_result = list_secrets(Some(prefix), None).await;
        assert!(list_result.is_ok());
        let list_output = list_result.unwrap();
        assert!(list_output.total >= 2);

        // Clean up
        let _ = delete_secret(&format!("{}1", prefix), Some(true)).await;
        let _ = delete_secret(&format!("{}2", prefix), Some(true)).await;
    }
}
