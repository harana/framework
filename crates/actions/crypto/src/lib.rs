// Harana Actions - Crypto Module
// This module provides crypto actions and functionality.

#![warn(missing_docs)]

pub mod output;

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hmac::{Hmac, Mac};
use output::*;
use rand::RngCore;
use sha2::{Digest, Sha256, Sha384, Sha512};
use sha3::{Sha3_256, Sha3_384, Sha3_512};

/// Decrypt Data
/// 
/// Decrypts data using AES-GCM encryption.
pub async fn decrypt(
    ciphertext: &str,
    iv: &str,
    key: &str,
    auth_tag: Option<&str>,
    algorithm: Option<&str>,
) -> Result<DecryptOutput, String> {
    let algorithm = algorithm.unwrap_or("aes-256-gcm");
    
    if algorithm != "aes-256-gcm" {
        return Err(format!("Unsupported algorithm: {}. Only aes-256-gcm is supported.", algorithm));
    }
    
    let key_bytes = BASE64.decode(key)
        .map_err(|e| format!("Invalid base64 key: {}", e))?;
    
    if key_bytes.len() != 32 {
        return Err("Key must be 32 bytes (256 bits) for AES-256".to_string());
    }
    
    let iv_bytes = BASE64.decode(iv)
        .map_err(|e| format!("Invalid base64 IV: {}", e))?;
    
    if iv_bytes.len() != 12 {
        return Err("IV must be 12 bytes for AES-GCM".to_string());
    }
    
    let mut ciphertext_bytes = BASE64.decode(ciphertext)
        .map_err(|e| format!("Invalid base64 ciphertext: {}", e))?;
    
    // Append auth tag if provided separately
    if let Some(tag) = auth_tag {
        let tag_bytes = BASE64.decode(tag)
            .map_err(|e| format!("Invalid base64 auth tag: {}", e))?;
        ciphertext_bytes.extend(tag_bytes);
    }
    
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)
        .map_err(|e| format!("Failed to create cipher: {}", e))?;
    
    let nonce = Nonce::from_slice(&iv_bytes);
    
    let plaintext = cipher.decrypt(nonce, ciphertext_bytes.as_ref())
        .map_err(|_| "Decryption failed - invalid key, IV, or corrupted data".to_string())?;
    
    let plaintext_str = String::from_utf8(plaintext)
        .map_err(|e| format!("Decrypted data is not valid UTF-8: {}", e))?;
    
    Ok(DecryptOutput {
        plaintext: plaintext_str,
    })
}

/// Encrypt Data With AES
/// 
/// Encrypts data using AES-GCM encryption.
pub async fn encrypt(
    key: &str,
    data: &str,
    algorithm: Option<&str>,
    iv: Option<&str>,
) -> Result<EncryptOutput, String> {
    let algorithm = algorithm.unwrap_or("aes-256-gcm");
    
    if algorithm != "aes-256-gcm" {
        return Err(format!("Unsupported algorithm: {}. Only aes-256-gcm is supported.", algorithm));
    }
    
    let key_bytes = BASE64.decode(key)
        .map_err(|e| format!("Invalid base64 key: {}", e))?;
    
    if key_bytes.len() != 32 {
        return Err("Key must be 32 bytes (256 bits) for AES-256".to_string());
    }
    
    let iv_bytes = if let Some(iv_str) = iv {
        let bytes = BASE64.decode(iv_str)
            .map_err(|e| format!("Invalid base64 IV: {}", e))?;
        if bytes.len() != 12 {
            return Err("IV must be 12 bytes for AES-GCM".to_string());
        }
        bytes
    } else {
        let mut bytes = [0u8; 12];
        OsRng.fill_bytes(&mut bytes);
        bytes.to_vec()
    };
    
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)
        .map_err(|e| format!("Failed to create cipher: {}", e))?;
    
    let nonce = Nonce::from_slice(&iv_bytes);
    
    let ciphertext = cipher.encrypt(nonce, data.as_bytes())
        .map_err(|e| format!("Encryption failed: {}", e))?;
    
    // AES-GCM appends the 16-byte auth tag to the ciphertext
    let (encrypted, tag) = ciphertext.split_at(ciphertext.len() - 16);
    
    Ok(EncryptOutput {
        ciphertext: BASE64.encode(encrypted),
        iv: BASE64.encode(&iv_bytes),
        auth_tag: BASE64.encode(tag),
    })
}

/// Generate Encryption Key
/// 
/// Generates a random encryption key.
pub async fn generate_key(
    encoding: Option<&str>,
    length: Option<i32>,
) -> Result<GenerateKeyOutput, String> {
    let length = length.unwrap_or(32) as usize; // Default 256-bit key
    let encoding = encoding.unwrap_or("base64");
    
    let mut key = vec![0u8; length];
    OsRng.fill_bytes(&mut key);
    
    let key_str = match encoding {
        "base64" => BASE64.encode(&key),
        "hex" => hex::encode(&key),
        _ => return Err(format!("Unsupported encoding: {}. Use 'base64' or 'hex'.", encoding)),
    };
    
    Ok(GenerateKeyOutput { key: key_str })
}

/// Generate Key Pair
/// 
/// Note: This is a placeholder - full RSA/EC key generation requires additional dependencies.
pub async fn generate_keypair(
    algorithm: Option<&str>,
    _key_size: Option<i32>,
) -> Result<GenerateKeypairOutput, String> {
    let algorithm = algorithm.unwrap_or("ed25519");
    
    // Placeholder - actual implementation would use ed25519-dalek or rsa crate
    Err(format!(
        "Key pair generation for {} requires additional implementation. \
        Use a dedicated cryptography library for production use.",
        algorithm
    ))
}

/// Hash Data With Algorithm
/// 
/// Computes a cryptographic hash of the input data.
pub async fn hash(
    data: &str,
    algorithm: Option<&str>,
    encoding: Option<&str>,
) -> Result<HashOutput, String> {
    let algorithm = algorithm.unwrap_or("sha256");
    let encoding = encoding.unwrap_or("hex");
    
    let hash_bytes: Vec<u8> = match algorithm.to_lowercase().as_str() {
        "sha256" | "sha-256" => {
            let mut hasher = Sha256::new();
            hasher.update(data.as_bytes());
            hasher.finalize().to_vec()
        }
        "sha384" | "sha-384" => {
            let mut hasher = Sha384::new();
            hasher.update(data.as_bytes());
            hasher.finalize().to_vec()
        }
        "sha512" | "sha-512" => {
            let mut hasher = Sha512::new();
            hasher.update(data.as_bytes());
            hasher.finalize().to_vec()
        }
        "sha3-256" => {
            let mut hasher = Sha3_256::new();
            hasher.update(data.as_bytes());
            hasher.finalize().to_vec()
        }
        "sha3-384" => {
            let mut hasher = Sha3_384::new();
            hasher.update(data.as_bytes());
            hasher.finalize().to_vec()
        }
        "sha3-512" => {
            let mut hasher = Sha3_512::new();
            hasher.update(data.as_bytes());
            hasher.finalize().to_vec()
        }
        "blake3" => {
            blake3::hash(data.as_bytes()).as_bytes().to_vec()
        }
        _ => return Err(format!("Unsupported algorithm: {}", algorithm)),
    };
    
    let digest = match encoding {
        "hex" => hex::encode(&hash_bytes),
        "base64" => BASE64.encode(&hash_bytes),
        _ => return Err(format!("Unsupported encoding: {}. Use 'hex' or 'base64'.", encoding)),
    };
    
    Ok(HashOutput { digest })
}

/// Generate HMAC Digest
/// 
/// Computes an HMAC (Hash-based Message Authentication Code).
pub async fn hmac(
    data: &str,
    key: &str,
    algorithm: Option<&str>,
    encoding: Option<&str>,
) -> Result<HmacOutput, String> {
    let algorithm = algorithm.unwrap_or("sha256");
    let encoding = encoding.unwrap_or("hex");
    
    let key_bytes = key.as_bytes();
    
    let mac_bytes: Vec<u8> = match algorithm.to_lowercase().as_str() {
        "sha256" | "sha-256" => {
            let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(key_bytes)
                .map_err(|e| format!("Invalid key: {}", e))?;
            mac.update(data.as_bytes());
            mac.finalize().into_bytes().to_vec()
        }
        "sha384" | "sha-384" => {
            let mut mac = <Hmac<Sha384> as Mac>::new_from_slice(key_bytes)
                .map_err(|e| format!("Invalid key: {}", e))?;
            mac.update(data.as_bytes());
            mac.finalize().into_bytes().to_vec()
        }
        "sha512" | "sha-512" => {
            let mut mac = <Hmac<Sha512> as Mac>::new_from_slice(key_bytes)
                .map_err(|e| format!("Invalid key: {}", e))?;
            mac.update(data.as_bytes());
            mac.finalize().into_bytes().to_vec()
        }
        _ => return Err(format!("Unsupported algorithm: {}. Use sha256, sha384, or sha512.", algorithm)),
    };
    
    let digest = match encoding {
        "hex" => hex::encode(&mac_bytes),
        "base64" => BASE64.encode(&mac_bytes),
        _ => return Err(format!("Unsupported encoding: {}. Use 'hex' or 'base64'.", encoding)),
    };
    
    Ok(HmacOutput { digest })
}

/// Sign Data With Private Key
/// 
/// Note: This is a placeholder - full signing requires additional dependencies.
pub async fn sign(
    _private_key: &str,
    _data: &str,
    algorithm: Option<&str>,
) -> Result<SignOutput, String> {
    let algorithm = algorithm.unwrap_or("ed25519");
    
    Err(format!(
        "Digital signing with {} requires additional implementation. \
        Use a dedicated cryptography library for production use.",
        algorithm
    ))
}

/// Verify Signature With Public Key
/// 
/// Note: This is a placeholder - full verification requires additional dependencies.
pub async fn verify(
    _signature: &str,
    _public_key: &str,
    _data: &str,
    algorithm: Option<&str>,
) -> Result<VerifyOutput, String> {
    let algorithm = algorithm.unwrap_or("ed25519");
    
    Err(format!(
        "Signature verification with {} requires additional implementation. \
        Use a dedicated cryptography library for production use.",
        algorithm
    ))
}

#[cfg(test)]
mod tests;
