#[cfg(test)]
mod tests {
    use crate::*;
    use data_encoding::BASE64;

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
        let result = hmac("hello world", "secret-key", Some("sha256"), Some("hex")).await.unwrap();
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
        ).await.unwrap();
        
        assert_eq!(decrypt_result.plaintext, "secret message");
    }
}
