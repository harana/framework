#[cfg(test)]
mod tests {
    use crate::*;
    use base64::{Engine, engine::general_purpose::STANDARD as BASE64};

    #[tokio::test]
    async fn test_hash_sha256() {
        let result = hash("hello world", Some("sha256"), Some("hex")).await.unwrap();
        assert_eq!(
            result.digest,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[tokio::test]
    async fn test_hash_sha512() {
        let result = hash("hello world", Some("sha512"), Some("hex")).await.unwrap();
        assert!(!result.digest.is_empty());
        assert_eq!(result.digest.len(), 128); // 512 bits = 64 bytes = 128 hex chars
    }

    #[tokio::test]
    async fn test_hash_blake3() {
        let result = hash("hello world", Some("blake3"), Some("hex")).await.unwrap();
        assert!(!result.digest.is_empty());
    }

    #[tokio::test]
    async fn test_hash_base64() {
        let result = hash("hello world", Some("sha256"), Some("base64")).await.unwrap();
        assert!(result.digest.contains('=')); // Base64 padding
    }

    #[tokio::test]
    async fn test_hmac_sha256() {
        let result = hmac("hello world", "secret-key", Some("sha256"), Some("hex"))
            .await
            .unwrap();
        assert!(!result.digest.is_empty());
        assert_eq!(result.digest.len(), 64); // 256 bits = 32 bytes = 64 hex chars
    }

    #[tokio::test]
    async fn test_generate_key() {
        let result = generate_key(Some("base64"), Some(32)).await.unwrap();
        let decoded = BASE64.decode(&result.key).unwrap();
        assert_eq!(decoded.len(), 32);
    }

    #[tokio::test]
    async fn test_generate_key_hex() {
        let result = generate_key(Some("hex"), Some(16)).await.unwrap();
        assert_eq!(result.key.len(), 32); // 16 bytes = 32 hex chars
    }

    #[tokio::test]
    async fn test_encrypt_decrypt() {
        // Generate a key
        let key_result = generate_key(Some("base64"), Some(32)).await.unwrap();

        // Encrypt
        let encrypt_result = encrypt(&key_result.key, "secret message", None, None).await.unwrap();
        assert!(!encrypt_result.ciphertext.is_empty());
        assert!(!encrypt_result.iv.is_empty());
        assert!(!encrypt_result.auth_tag.is_empty());

        // Decrypt
        let decrypt_result = decrypt(
            &encrypt_result.ciphertext,
            &encrypt_result.iv,
            &key_result.key,
            Some(&encrypt_result.auth_tag),
            None,
        )
        .await
        .unwrap();

        assert_eq!(decrypt_result.plaintext, "secret message");
    }

    #[tokio::test]
    async fn test_generate_keypair_ed25519() {
        let result = generate_keypair(Some("ed25519"), None).await.unwrap();

        // Verify keys are base64 encoded and correct length
        let private_key_bytes = BASE64.decode(&result.private_key).unwrap();
        let public_key_bytes = BASE64.decode(&result.public_key).unwrap();

        assert_eq!(private_key_bytes.len(), 32); // Ed25519 private key is 32 bytes
        assert_eq!(public_key_bytes.len(), 32); // Ed25519 public key is 32 bytes
    }

    #[tokio::test]
    async fn test_sign_and_verify() {
        // Generate a key pair
        let keypair = generate_keypair(Some("ed25519"), None).await.unwrap();

        let message = "Hello, World!";

        // Sign the message
        let sign_result = sign(&keypair.private_key, message, Some("ed25519")).await.unwrap();
        assert!(!sign_result.signature.is_empty());

        // Verify signature is 64 bytes (512 bits) when decoded
        let sig_bytes = BASE64.decode(&sign_result.signature).unwrap();
        assert_eq!(sig_bytes.len(), 64);

        // Verify the signature with the correct public key
        let verify_result = verify(&sign_result.signature, &keypair.public_key, message, Some("ed25519"))
            .await
            .unwrap();
        assert!(verify_result.valid);
    }

    #[tokio::test]
    async fn test_verify_invalid_signature() {
        // Generate a key pair
        let keypair = generate_keypair(Some("ed25519"), None).await.unwrap();

        let message = "Hello, World!";

        // Sign the message
        let sign_result = sign(&keypair.private_key, message, Some("ed25519")).await.unwrap();

        // Verify with wrong message should fail
        let verify_result = verify(
            &sign_result.signature,
            &keypair.public_key,
            "Wrong message",
            Some("ed25519"),
        )
        .await
        .unwrap();
        assert!(!verify_result.valid);
    }

    #[tokio::test]
    async fn test_verify_wrong_public_key() {
        // Generate two key pairs
        let keypair1 = generate_keypair(Some("ed25519"), None).await.unwrap();
        let keypair2 = generate_keypair(Some("ed25519"), None).await.unwrap();

        let message = "Hello, World!";

        // Sign with first key pair
        let sign_result = sign(&keypair1.private_key, message, Some("ed25519")).await.unwrap();

        // Verify with second key pair's public key should fail
        let verify_result = verify(&sign_result.signature, &keypair2.public_key, message, Some("ed25519"))
            .await
            .unwrap();
        assert!(!verify_result.valid);
    }
}
