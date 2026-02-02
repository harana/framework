// Harana Actions - Crypto Module
// This module provides crypto actions and functionality.

pub mod output;

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hmac::{Hmac, Mac};
use output::*;
use rand::prelude::*;
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
        return Err(format!(
            "Unsupported algorithm: {}. Only aes-256-gcm is supported.",
            algorithm
        ));
    }

    let key_bytes = BASE64.decode(key).map_err(|e| format!("Invalid base64 key: {}", e))?;

    if key_bytes.len() != 32 {
        return Err("Key must be 32 bytes (256 bits) for AES-256".to_string());
    }

    let iv_bytes = BASE64.decode(iv).map_err(|e| format!("Invalid base64 IV: {}", e))?;

    if iv_bytes.len() != 12 {
        return Err("IV must be 12 bytes for AES-GCM".to_string());
    }

    let mut ciphertext_bytes = BASE64
        .decode(ciphertext)
        .map_err(|e| format!("Invalid base64 ciphertext: {}", e))?;

    // Append auth tag if provided separately
    if let Some(tag) = auth_tag {
        let tag_bytes = BASE64
            .decode(tag)
            .map_err(|e| format!("Invalid base64 auth tag: {}", e))?;
        ciphertext_bytes.extend(tag_bytes);
    }

    let cipher = Aes256Gcm::new_from_slice(&key_bytes).map_err(|e| format!("Failed to create cipher: {}", e))?;

    let nonce = Nonce::from_slice(&iv_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext_bytes.as_ref())
        .map_err(|_| "Decryption failed - invalid key, IV, or corrupted data".to_string())?;

    let plaintext_str =
        String::from_utf8(plaintext).map_err(|e| format!("Decrypted data is not valid UTF-8: {}", e))?;

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
        return Err(format!(
            "Unsupported algorithm: {}. Only aes-256-gcm is supported.",
            algorithm
        ));
    }

    let key_bytes = BASE64.decode(key).map_err(|e| format!("Invalid base64 key: {}", e))?;

    if key_bytes.len() != 32 {
        return Err("Key must be 32 bytes (256 bits) for AES-256".to_string());
    }

    let iv_bytes = if let Some(iv_str) = iv {
        let bytes = BASE64.decode(iv_str).map_err(|e| format!("Invalid base64 IV: {}", e))?;
        if bytes.len() != 12 {
            return Err("IV must be 12 bytes for AES-GCM".to_string());
        }
        bytes
    } else {
        let mut bytes = [0u8; 12];
        rand::rng().fill_bytes(&mut bytes);
        bytes.to_vec()
    };

    let cipher = Aes256Gcm::new_from_slice(&key_bytes).map_err(|e| format!("Failed to create cipher: {}", e))?;

    let nonce = Nonce::from_slice(&iv_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data.as_bytes())
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
pub async fn generate_key(encoding: Option<&str>, length: Option<i32>) -> Result<GenerateKeyOutput, String> {
    let length = length.unwrap_or(32) as usize; // Default 256-bit key
    let encoding = encoding.unwrap_or("base64");

    let mut key = vec![0u8; length];
    rand::rng().fill_bytes(&mut key);

    let key_str = match encoding {
        "base64" => BASE64.encode(&key),
        "hex" => hex::encode(&key),
        _ => return Err(format!("Unsupported encoding: {}. Use 'base64' or 'hex'.", encoding)),
    };

    Ok(GenerateKeyOutput { key: key_str })
}

/// Generate Key Pair
///
/// Generates an Ed25519 key pair for digital signatures.
pub async fn generate_keypair(
    algorithm: Option<&str>,
    _key_size: Option<i32>,
) -> Result<GenerateKeypairOutput, String> {
    let algorithm = algorithm.unwrap_or("ed25519");

    match algorithm.to_lowercase().as_str() {
        "ed25519" => {
            // Generate 32 random bytes for the private key
            let mut key_bytes = [0u8; 32];
            rand::rng().fill_bytes(&mut key_bytes);

            let signing_key = SigningKey::from_bytes(&key_bytes);
            let verifying_key = signing_key.verifying_key();

            // Encode keys as base64
            let private_key = BASE64.encode(signing_key.to_bytes());
            let public_key = BASE64.encode(verifying_key.to_bytes());

            Ok(GenerateKeypairOutput {
                private_key,
                public_key,
            })
        }
        _ => Err(format!(
            "Unsupported algorithm: {}. Only 'ed25519' is currently supported.",
            algorithm
        )),
    }
}

/// Hash Data With Algorithm
///
/// Computes a cryptographic hash of the input data.
pub async fn hash(data: &str, algorithm: Option<&str>, encoding: Option<&str>) -> Result<HashOutput, String> {
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
        "blake3" => blake3::hash(data.as_bytes()).as_bytes().to_vec(),
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
            let mut mac =
                <Hmac<Sha256> as Mac>::new_from_slice(key_bytes).map_err(|e| format!("Invalid key: {}", e))?;
            mac.update(data.as_bytes());
            mac.finalize().into_bytes().to_vec()
        }
        "sha384" | "sha-384" => {
            let mut mac =
                <Hmac<Sha384> as Mac>::new_from_slice(key_bytes).map_err(|e| format!("Invalid key: {}", e))?;
            mac.update(data.as_bytes());
            mac.finalize().into_bytes().to_vec()
        }
        "sha512" | "sha-512" => {
            let mut mac =
                <Hmac<Sha512> as Mac>::new_from_slice(key_bytes).map_err(|e| format!("Invalid key: {}", e))?;
            mac.update(data.as_bytes());
            mac.finalize().into_bytes().to_vec()
        }
        _ => {
            return Err(format!(
                "Unsupported algorithm: {}. Use sha256, sha384, or sha512.",
                algorithm
            ));
        }
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
/// Signs data using Ed25519 digital signature algorithm.
pub async fn sign(private_key: &str, data: &str, algorithm: Option<&str>) -> Result<SignOutput, String> {
    let algorithm = algorithm.unwrap_or("ed25519");

    match algorithm.to_lowercase().as_str() {
        "ed25519" => {
            // Decode the base64-encoded private key
            let key_bytes = BASE64
                .decode(private_key)
                .map_err(|e| format!("Invalid base64 private key: {}", e))?;

            // Ed25519 private keys are 32 bytes
            let key_array: [u8; 32] = key_bytes
                .try_into()
                .map_err(|_| "Private key must be 32 bytes for Ed25519".to_string())?;

            let signing_key = SigningKey::from_bytes(&key_array);

            // Sign the data
            let signature = signing_key.sign(data.as_bytes());

            // Return base64-encoded signature
            Ok(SignOutput {
                signature: BASE64.encode(signature.to_bytes()),
            })
        }
        _ => Err(format!(
            "Unsupported algorithm: {}. Only 'ed25519' is currently supported.",
            algorithm
        )),
    }
}

/// Verify Signature With Public Key
///
/// Verifies a digital signature using Ed25519.
pub async fn verify(
    signature: &str,
    public_key: &str,
    data: &str,
    algorithm: Option<&str>,
) -> Result<VerifyOutput, String> {
    let algorithm = algorithm.unwrap_or("ed25519");

    match algorithm.to_lowercase().as_str() {
        "ed25519" => {
            // Decode the base64-encoded public key
            let key_bytes = BASE64
                .decode(public_key)
                .map_err(|e| format!("Invalid base64 public key: {}", e))?;

            // Ed25519 public keys are 32 bytes
            let key_array: [u8; 32] = key_bytes
                .try_into()
                .map_err(|_| "Public key must be 32 bytes for Ed25519".to_string())?;

            let verifying_key =
                VerifyingKey::from_bytes(&key_array).map_err(|e| format!("Invalid public key: {}", e))?;

            // Decode the base64-encoded signature
            let sig_bytes = BASE64
                .decode(signature)
                .map_err(|e| format!("Invalid base64 signature: {}", e))?;

            // Ed25519 signatures are 64 bytes
            let sig_array: [u8; 64] = sig_bytes
                .try_into()
                .map_err(|_| "Signature must be 64 bytes for Ed25519".to_string())?;

            let sig = Signature::from_bytes(&sig_array);

            // Verify the signature
            let valid = verifying_key.verify(data.as_bytes(), &sig).is_ok();

            Ok(VerifyOutput { valid })
        }
        _ => Err(format!(
            "Unsupported algorithm: {}. Only 'ed25519' is currently supported.",
            algorithm
        )),
    }
}

#[cfg(test)]
mod tests;
