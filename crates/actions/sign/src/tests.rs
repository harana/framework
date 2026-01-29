#[cfg(test)]
mod tests {
    use crate::*;
    use ring::signature::{Ed25519KeyPair, KeyPair};
    use ring::rand::SystemRandom;

    // Helper function to generate Ed25519 key pair for testing
    fn generate_ed25519_keypair() -> (String, String) {
        let rng = SystemRandom::new();
        let pkcs8_doc = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_doc.as_ref()).unwrap();
        
        let private_key = BASE64.encode(pkcs8_doc.as_ref());
        let public_key = BASE64.encode(key_pair.public_key().as_ref());
        
        (private_key, public_key)
    }

    #[tokio::test]
    async fn test_sign_artifact_ed25519() {
        let (private_key, _public_key) = generate_ed25519_keypair();
        let artifact = b"Hello, World!";
        
        let result = sign_artifact(
            artifact,
            &private_key,
            Some("Ed25519"),
            None,
            None,
            None,
        )
        .await;
        
        assert!(result.is_ok(), "Sign failed: {:?}", result.err());
        let output = result.unwrap();
        assert!(output.success);
        assert!(!output.signature.is_empty());
        assert_eq!(output.algorithm, "Ed25519");
    }

    #[tokio::test]
    async fn test_verify_artifact_ed25519() {
        let (private_key, public_key) = generate_ed25519_keypair();
        let artifact = b"Hello, World!";
        
        // First sign
        let sign_result = sign_artifact(
            artifact,
            &private_key,
            Some("Ed25519"),
            None,
            None,
            None,
        )
        .await
        .unwrap();
        
        // Then verify
        let verify_result = verify_artifact(
            artifact,
            &public_key,
            &sign_result.signature,
            Some("Ed25519"),
            None,
            None,
        )
        .await;
        
        assert!(verify_result.is_ok());
        let output = verify_result.unwrap();
        assert!(output.valid, "Verification failed: {:?}", output.error);
        assert!(output.error.is_none());
    }

    #[tokio::test]
    async fn test_verify_artifact_invalid_signature() {
        let (_private_key, public_key) = generate_ed25519_keypair();
        let artifact = b"Hello, World!";
        let wrong_signature = "aW52YWxpZCBzaWduYXR1cmU="; // "invalid signature" in base64
        
        let result = verify_artifact(
            artifact,
            &public_key,
            wrong_signature,
            Some("Ed25519"),
            None,
            None,
        )
        .await;
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.valid);
        assert!(output.error.is_some());
    }

    #[tokio::test]
    async fn test_verify_artifact_tampered_data() {
        let (private_key, public_key) = generate_ed25519_keypair();
        let artifact = b"Hello, World!";
        let tampered_artifact = b"Hello, World?"; // Tampered data
        
        // Sign original
        let sign_result = sign_artifact(
            artifact,
            &private_key,
            Some("Ed25519"),
            None,
            None,
            None,
        )
        .await
        .unwrap();
        
        // Verify with tampered data
        let verify_result = verify_artifact(
            tampered_artifact,
            &public_key,
            &sign_result.signature,
            Some("Ed25519"),
            None,
            None,
        )
        .await;
        
        assert!(verify_result.is_ok());
        let output = verify_result.unwrap();
        assert!(!output.valid);
    }

    #[tokio::test]
    async fn test_create_detached_signature() {
        let (private_key, _public_key) = generate_ed25519_keypair();
        let artifact = b"Document content";
        
        let result = create_detached(
            artifact,
            &private_key,
            Some("Ed25519"),
            Some(true), // armored
            None,
        )
        .await;
        
        assert!(result.is_ok(), "Create detached failed: {:?}", result.err());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.signature.contains("-----BEGIN SIGNATURE-----"));
        assert!(output.signature.contains("-----END SIGNATURE-----"));
    }

    #[tokio::test]
    async fn test_create_detached_unarmored() {
        let (private_key, _public_key) = generate_ed25519_keypair();
        let artifact = b"Document content";
        
        let result = create_detached(
            artifact,
            &private_key,
            Some("Ed25519"),
            Some(false), // not armored
            None,
        )
        .await;
        
        assert!(result.is_ok(), "Create detached failed: {:?}", result.err());
        let output = result.unwrap();
        assert!(output.success);
        assert!(!output.signature.contains("-----BEGIN"));
    }

    #[tokio::test]
    async fn test_verify_detached_signature() {
        let (private_key, public_key) = generate_ed25519_keypair();
        let artifact = b"Document content";
        
        // Create signature
        let create_result = create_detached(
            artifact,
            &private_key,
            Some("Ed25519"),
            Some(true),
            None,
        )
        .await
        .unwrap();
        
        // Verify signature
        let verify_result = verify_detached(
            artifact,
            &public_key,
            &create_result.signature,
            Some("Ed25519"),
            None,
        )
        .await;
        
        assert!(verify_result.is_ok());
        let output = verify_result.unwrap();
        assert!(output.valid, "Verification failed: {:?}", output.error);
    }

    #[tokio::test]
    async fn test_sign_code() {
        let (private_key, _public_key) = generate_ed25519_keypair();
        let artifact = b"binary code content";
        let certificate = "mock-certificate"; // Simplified for testing
        
        let result = sign_code(
            artifact,
            certificate,
            &private_key,
            None,
            None,
            None,
            None,
            None,
        )
        .await;
        
        assert!(result.is_ok(), "Sign code failed: {:?}", result.err());
        let output = result.unwrap();
        assert!(output.success);
        assert!(!output.signature.is_empty());
        assert_eq!(output.signed_artifact, artifact.to_vec());
    }

    #[tokio::test]
    async fn test_sign_container() {
        let (private_key, _public_key) = generate_ed25519_keypair();
        
        let result = sign_container(
            "myapp",
            &private_key,
            Some("sha256:abc123"),
            None,
            Some("docker.io"),
            Some("v1.0"),
        )
        .await;
        
        assert!(result.is_ok(), "Sign container failed: {:?}", result.err());
        let output = result.unwrap();
        assert!(output.success);
        assert!(!output.signature.is_empty());
        assert!(output.signature_digest.starts_with("sha256:"));
    }

    #[tokio::test]
    async fn test_timestamp() {
        let digest = "a948904f2f0f479b8f8564cbf12dac6b18b5e1af4f2c1f0e0f3e8e8d8f8a8b8c";
        
        let result = timestamp(
            "http://timestamp.example.com",
            digest,
            Some("Sha256"),
        )
        .await;
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(!output.timestamp_token.is_empty());
    }

    #[tokio::test]
    async fn test_algorithm_parsing() {
        // Test various algorithm string formats
        assert!(Algorithm::from_str("RSA-SHA256").is_ok());
        assert!(Algorithm::from_str("rsa-sha256").is_ok());
        assert!(Algorithm::from_str("RSA_SHA256").is_ok());
        assert!(Algorithm::from_str("Ed25519").is_ok());
        assert!(Algorithm::from_str("ECDSA-SHA256").is_ok());
        assert!(Algorithm::from_str("ECDSA-P256-SHA256").is_ok());
        assert!(Algorithm::from_str("unknown-algo").is_err());
    }

    #[tokio::test]
    async fn test_invalid_private_key() {
        let artifact = b"test data";
        let invalid_key = "not-a-valid-key";
        
        let result = sign_artifact(
            artifact,
            invalid_key,
            Some("Ed25519"),
            None,
            None,
            None,
        )
        .await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_sign_jar_not_implemented() {
        let result = sign_jar(
            "/path/to/file.jar",
            "/path/to/keystore.jks",
            "myalias",
            "password",
            None,
            None,
            None,
        )
        .await;
        
        // JAR signing requires Java tooling
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_jar_not_implemented() {
        let result = verify_jar(
            "/path/to/file.jar",
            None,
        )
        .await;
        
        // JAR verification requires Java tooling
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_roundtrip_sign_verify() {
        let (private_key, public_key) = generate_ed25519_keypair();
        
        // Test complete roundtrip for multiple data sizes
        let test_data = vec![
            b"small".to_vec(),
            b"medium size data for testing".to_vec(),
            vec![0u8; 1000], // 1KB of zeros
        ];
        
        for data in test_data {
            let sign_result = sign_artifact(
                &data,
                &private_key,
                Some("Ed25519"),
                None,
                None,
                None,
            )
            .await
            .unwrap();
            
            let verify_result = verify_artifact(
                &data,
                &public_key,
                &sign_result.signature,
                Some("Ed25519"),
                None,
                None,
            )
            .await
            .unwrap();
            
            assert!(verify_result.valid, "Verification failed for data of size {}", data.len());
        }
    }
}

