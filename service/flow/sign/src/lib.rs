// Harana Actions - Sign Module
// This module provides digital signing and verification actions.

pub mod output;

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::Utc;
use output::*;
use ring::signature::{
    EcdsaKeyPair, Ed25519KeyPair, RsaKeyPair, UnparsedPublicKey, ECDSA_P256_SHA256_ASN1,
    ECDSA_P256_SHA256_ASN1_SIGNING, ED25519, RSA_PKCS1_2048_8192_SHA256, RSA_PKCS1_2048_8192_SHA512,
    RSA_PKCS1_SHA256, RSA_PKCS1_SHA512,
};
use ring::rand::SystemRandom;
use sha2::{Digest, Sha256};



/// Supported signing algorithms
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Algorithm {
    RsaSha256,
    RsaSha512,
    EcdsaSha256,
    Ed25519,
}

impl Algorithm {
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "RSA-SHA256" | "RSA_SHA256" | "RSASHA256" => Ok(Algorithm::RsaSha256),
            "RSA-SHA512" | "RSA_SHA512" | "RSASHA512" => Ok(Algorithm::RsaSha512),
            "ECDSA-SHA256" | "ECDSA_SHA256" | "ECDSASHA256" | "ECDSA-P256-SHA256" => {
                Ok(Algorithm::EcdsaSha256)
            }
            "ED25519" => Ok(Algorithm::Ed25519),
            _ => Err(format!("Unsupported algorithm: {}", s)),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Algorithm::RsaSha256 => "RSA-SHA256".to_string(),
            Algorithm::RsaSha512 => "RSA-SHA512".to_string(),
            Algorithm::EcdsaSha256 => "ECDSA-SHA256".to_string(),
            Algorithm::Ed25519 => "Ed25519".to_string(),
        }
    }
}

/// Decode a PEM-encoded private key
fn decode_private_key(pem_data: &str) -> Result<Vec<u8>, String> {
    // Try to parse as PEM first
    if let Ok(pem) = pem::parse(pem_data) {
        return Ok(pem.contents().to_vec());
    }
    
    // Try as base64-encoded DER
    BASE64
        .decode(pem_data.trim())
        .map_err(|e| format!("Failed to decode private key: {}", e))
}

/// Decode a PEM-encoded public key
fn decode_public_key(pem_data: &str) -> Result<Vec<u8>, String> {
    // Try to parse as PEM first
    if let Ok(pem) = pem::parse(pem_data) {
        return Ok(pem.contents().to_vec());
    }
    
    // Try as base64-encoded DER
    BASE64
        .decode(pem_data.trim())
        .map_err(|e| format!("Failed to decode public key: {}", e))
}

/// Sign Artifact With Private Key
pub async fn sign_artifact(
    artifact: &[u8],
    private_key: &str,
    algorithm: Option<&str>,
    _artifact_path: Option<&str>,
    format: Option<&str>,
    _metadata: Option<SignatureMetadata>,
) -> Result<SignArtifactOutput, String> {
    let algo = Algorithm::from_str(algorithm.unwrap_or("RSA-SHA256"))?;
    let key_bytes = decode_private_key(private_key)?;
    let rng = SystemRandom::new();
    let sig_format = format.unwrap_or("Raw").to_string();

    let signature = match algo {
        Algorithm::RsaSha256 => {
            let key_pair = RsaKeyPair::from_pkcs8(&key_bytes)
                .map_err(|e| format!("Invalid RSA private key: {}", e))?;
            let mut sig = vec![0u8; key_pair.public().modulus_len()];
            key_pair
                .sign(&RSA_PKCS1_SHA256, &rng, artifact, &mut sig)
                .map_err(|e| format!("Signing failed: {}", e))?;
            sig
        }
        Algorithm::RsaSha512 => {
            let key_pair = RsaKeyPair::from_pkcs8(&key_bytes)
                .map_err(|e| format!("Invalid RSA private key: {}", e))?;
            let mut sig = vec![0u8; key_pair.public().modulus_len()];
            key_pair
                .sign(&RSA_PKCS1_SHA512, &rng, artifact, &mut sig)
                .map_err(|e| format!("Signing failed: {}", e))?;
            sig
        }
        Algorithm::EcdsaSha256 => {
            let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, &key_bytes, &rng)
                .map_err(|e| format!("Invalid ECDSA private key: {}", e))?;
            key_pair.sign(&rng, artifact)
                .map_err(|e| format!("Signing failed: {}", e))?
                .as_ref()
                .to_vec()
        }
        Algorithm::Ed25519 => {
            let key_pair = Ed25519KeyPair::from_pkcs8(&key_bytes)
                .map_err(|e| format!("Invalid Ed25519 private key: {}", e))?;
            key_pair.sign(artifact).as_ref().to_vec()
        }
    };

    Ok(SignArtifactOutput {
        signature: BASE64.encode(&signature),
        signature_format: sig_format,
        algorithm: algo.to_string(),
        timestamp: Utc::now(),
        success: true,
    })
}

/// Verify Artifact Signature
pub async fn verify_artifact(
    artifact: &[u8],
    public_key: &str,
    signature: &str,
    algorithm: Option<&str>,
    _artifact_path: Option<&str>,
    _format: Option<&str>,
) -> Result<VerifyArtifactOutput, String> {
    let algo = Algorithm::from_str(algorithm.unwrap_or("RSA-SHA256"))?;
    let key_bytes = decode_public_key(public_key)?;
    let sig_bytes = BASE64
        .decode(signature)
        .map_err(|e| format!("Invalid signature encoding: {}", e))?;

    let result = match algo {
        Algorithm::RsaSha256 => {
            let public_key = UnparsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, &key_bytes);
            public_key.verify(artifact, &sig_bytes)
        }
        Algorithm::RsaSha512 => {
            let public_key = UnparsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA512, &key_bytes);
            public_key.verify(artifact, &sig_bytes)
        }
        Algorithm::EcdsaSha256 => {
            let public_key = UnparsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, &key_bytes);
            public_key.verify(artifact, &sig_bytes)
        }
        Algorithm::Ed25519 => {
            let public_key = UnparsedPublicKey::new(&ED25519, &key_bytes);
            public_key.verify(artifact, &sig_bytes)
        }
    };

    match result {
        Ok(()) => Ok(VerifyArtifactOutput {
            valid: true,
            signer: None,
            timestamp: Some(Utc::now()),
            algorithm: Some(algo.to_string()),
            error: None,
        }),
        Err(e) => Ok(VerifyArtifactOutput {
            valid: false,
            signer: None,
            timestamp: None,
            algorithm: Some(algo.to_string()),
            error: Some(format!("Verification failed: {}", e)),
        }),
    }
}

/// Sign Code With Certificate
/// Note: Full code signing requires platform-specific tooling.
/// This implementation provides a simplified signature mechanism.
pub async fn sign_code(
    artifact: &[u8],
    _certificate: &str,
    private_key: &str,
    _artifact_path: Option<&str>,
    _certificate_chain: Option<Vec<String>>,
    _description: Option<&str>,
    _format: Option<&str>,
    _timestamp_authority: Option<&str>,
) -> Result<SignCodeOutput, String> {
    // For code signing, we create a signature over the artifact hash
    let key_bytes = decode_private_key(private_key)?;
    let rng = SystemRandom::new();
    
    // Hash the artifact first
    let mut hasher = Sha256::new();
    hasher.update(artifact);
    let hash = hasher.finalize();

    // Try to sign with RSA first, fall back to Ed25519
    let signature = if let Ok(key_pair) = RsaKeyPair::from_pkcs8(&key_bytes) {
        let mut sig = vec![0u8; key_pair.public().modulus_len()];
        key_pair
            .sign(&RSA_PKCS1_SHA256, &rng, &hash, &mut sig)
            .map_err(|e| format!("Signing failed: {}", e))?;
        sig
    } else if let Ok(key_pair) = Ed25519KeyPair::from_pkcs8(&key_bytes) {
        key_pair.sign(&hash).as_ref().to_vec()
    } else {
        return Err("Unsupported private key format".to_string());
    };

    Ok(SignCodeOutput {
        signed_artifact: artifact.to_vec(),
        signature: BASE64.encode(&signature),
        success: true,
        timestamp: Some(Utc::now()),
    })
}

/// Create Detached Signature
pub async fn create_detached(
    artifact: &[u8],
    private_key: &str,
    algorithm: Option<&str>,
    armor: Option<bool>,
    _format: Option<&str>,
) -> Result<CreateDetachedOutput, String> {
    let algo = Algorithm::from_str(algorithm.unwrap_or("RSA-SHA256"))?;
    let key_bytes = decode_private_key(private_key)?;
    let rng = SystemRandom::new();
    let use_armor = armor.unwrap_or(true);

    let signature = match algo {
        Algorithm::RsaSha256 => {
            let key_pair = RsaKeyPair::from_pkcs8(&key_bytes)
                .map_err(|e| format!("Invalid RSA private key: {}", e))?;
            let mut sig = vec![0u8; key_pair.public().modulus_len()];
            key_pair
                .sign(&RSA_PKCS1_SHA256, &rng, artifact, &mut sig)
                .map_err(|e| format!("Signing failed: {}", e))?;
            sig
        }
        Algorithm::RsaSha512 => {
            let key_pair = RsaKeyPair::from_pkcs8(&key_bytes)
                .map_err(|e| format!("Invalid RSA private key: {}", e))?;
            let mut sig = vec![0u8; key_pair.public().modulus_len()];
            key_pair
                .sign(&RSA_PKCS1_SHA512, &rng, artifact, &mut sig)
                .map_err(|e| format!("Signing failed: {}", e))?;
            sig
        }
        Algorithm::EcdsaSha256 => {
            let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, &key_bytes, &rng)
                .map_err(|e| format!("Invalid ECDSA private key: {}", e))?;
            key_pair.sign(&rng, artifact)
                .map_err(|e| format!("Signing failed: {}", e))?
                .as_ref()
                .to_vec()
        }
        Algorithm::Ed25519 => {
            let key_pair = Ed25519KeyPair::from_pkcs8(&key_bytes)
                .map_err(|e| format!("Invalid Ed25519 private key: {}", e))?;
            key_pair.sign(artifact).as_ref().to_vec()
        }
    };

    let signature_str = if use_armor {
        format!(
            "-----BEGIN SIGNATURE-----\n{}\n-----END SIGNATURE-----",
            BASE64.encode(&signature)
        )
    } else {
        BASE64.encode(&signature)
    };

    Ok(CreateDetachedOutput {
        signature: signature_str,
        success: true,
    })
}

/// Verify Detached Signature
pub async fn verify_detached(
    artifact: &[u8],
    public_key: &str,
    signature: &str,
    algorithm: Option<&str>,
    _format: Option<&str>,
) -> Result<VerifyDetachedOutput, String> {
    let algo = Algorithm::from_str(algorithm.unwrap_or("RSA-SHA256"))?;
    let key_bytes = decode_public_key(public_key)?;
    
    // Handle armored signatures
    let sig_str = signature
        .replace("-----BEGIN SIGNATURE-----", "")
        .replace("-----END SIGNATURE-----", "")
        .replace('\n', "")
        .replace('\r', "");
    
    let sig_bytes = BASE64
        .decode(sig_str.trim())
        .map_err(|e| format!("Invalid signature encoding: {}", e))?;

    let result = match algo {
        Algorithm::RsaSha256 => {
            let public_key = UnparsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, &key_bytes);
            public_key.verify(artifact, &sig_bytes)
        }
        Algorithm::RsaSha512 => {
            let public_key = UnparsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA512, &key_bytes);
            public_key.verify(artifact, &sig_bytes)
        }
        Algorithm::EcdsaSha256 => {
            let public_key = UnparsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, &key_bytes);
            public_key.verify(artifact, &sig_bytes)
        }
        Algorithm::Ed25519 => {
            let public_key = UnparsedPublicKey::new(&ED25519, &key_bytes);
            public_key.verify(artifact, &sig_bytes)
        }
    };

    match result {
        Ok(()) => Ok(VerifyDetachedOutput {
            valid: true,
            signer: None,
            error: None,
        }),
        Err(e) => Ok(VerifyDetachedOutput {
            valid: false,
            signer: None,
            error: Some(format!("Verification failed: {}", e)),
        }),
    }
}

/// Sign Container Image
/// Note: Full container signing requires integration with container registries.
/// This implementation provides a signature over the image reference/digest.
pub async fn sign_container(
    image: &str,
    private_key: &str,
    digest: Option<&str>,
    _format: Option<&str>,
    registry: Option<&str>,
    tag: Option<&str>,
) -> Result<SignContainerOutput, String> {
    let key_bytes = decode_private_key(private_key)?;
    
    // Build the image reference to sign
    let image_ref = format!(
        "{}{}:{}@{}",
        registry.map(|r| format!("{}/", r)).unwrap_or_default(),
        image,
        tag.unwrap_or("latest"),
        digest.unwrap_or("sha256:unknown")
    );
    
    // Hash the image reference
    let mut hasher = Sha256::new();
    hasher.update(image_ref.as_bytes());
    let hash = hasher.finalize();

    // Sign the hash
    let signature = if let Ok(key_pair) = Ed25519KeyPair::from_pkcs8(&key_bytes) {
        key_pair.sign(&hash).as_ref().to_vec()
    } else if let Ok(key_pair) = RsaKeyPair::from_pkcs8(&key_bytes) {
        let rng = SystemRandom::new();
        let mut sig = vec![0u8; key_pair.public().modulus_len()];
        key_pair
            .sign(&RSA_PKCS1_SHA256, &rng, &hash, &mut sig)
            .map_err(|e| format!("Signing failed: {}", e))?;
        sig
    } else {
        return Err("Unsupported private key format".to_string());
    };

    // Create signature digest
    let mut sig_hasher = Sha256::new();
    sig_hasher.update(&signature);
    let sig_digest = sig_hasher.finalize();

    Ok(SignContainerOutput {
        signature: BASE64.encode(&signature),
        signature_digest: format!("sha256:{}", hex::encode(sig_digest)),
        success: true,
    })
}

/// Verify Container Image Signature
pub async fn verify_container(
    image: &str,
    public_key: &str,
    _format: Option<&str>,
    registry: Option<&str>,
    signature: Option<&str>,
    tag: Option<&str>,
) -> Result<VerifyContainerOutput, String> {
    let signature = signature.ok_or("Signature is required for verification")?;
    let key_bytes = decode_public_key(public_key)?;
    
    // We need the digest to reconstruct the signed message
    // In a real implementation, this would be retrieved from the registry
    let image_ref = format!(
        "{}{}:{}@{}",
        registry.map(|r| format!("{}/", r)).unwrap_or_default(),
        image,
        tag.unwrap_or("latest"),
        "sha256:unknown" // In practice, this would be retrieved
    );
    
    // Hash the image reference
    let mut hasher = Sha256::new();
    hasher.update(image_ref.as_bytes());
    let hash = hasher.finalize();
    
    let sig_bytes = BASE64
        .decode(signature)
        .map_err(|e| format!("Invalid signature encoding: {}", e))?;

    // Try Ed25519 first, then RSA
    let result = {
        let public_key = UnparsedPublicKey::new(&ED25519, &key_bytes);
        match public_key.verify(&hash, &sig_bytes) {
            Ok(()) => Ok(()),
            Err(_) => {
                let public_key = UnparsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, &key_bytes);
                public_key.verify(&hash, &sig_bytes)
            }
        }
    };

    match result {
        Ok(()) => Ok(VerifyContainerOutput {
            valid: true,
            signer: None,
            timestamp: Some(Utc::now()),
            error: None,
        }),
        Err(e) => Ok(VerifyContainerOutput {
            valid: false,
            signer: None,
            timestamp: None,
            error: Some(format!("Verification failed: {}", e)),
        }),
    }
}

/// Sign JAR Archive
/// Note: Full JAR signing requires Java keytool integration.
/// This implementation provides a simplified signature mechanism.
pub async fn sign_jar(
    _jar_path: &str,
    _keystore: &str,
    _alias: &str,
    _keystore_password: &str,
    _private_key_password: Option<&str>,
    _signature_algorithm: Option<&str>,
    _timestamp_authority: Option<&str>,
) -> Result<SignJarOutput, String> {
    // JAR signing requires Java keytool integration
    // This would typically invoke jarsigner or use a pure Rust implementation
    Err("JAR signing requires Java keytool integration. Use jarsigner CLI or implement with zip crate for pure Rust solution.".to_string())
}

/// Verify JAR Signature
pub async fn verify_jar(
    _jar_path: &str,
    _certificate: Option<&str>,
) -> Result<VerifyJarOutput, String> {
    // JAR verification requires Java keytool integration
    Err("JAR verification requires Java keytool integration. Use jarsigner -verify CLI.".to_string())
}

/// Create Timestamp
pub async fn timestamp(
    _authority_url: &str,
    digest: &str,
    hash_algorithm: Option<&str>,
) -> Result<TimestampOutput, String> {
    let _algo = hash_algorithm.unwrap_or("Sha256");
    
    // Validate the digest is valid hex
    let _digest_bytes = hex::decode(digest)
        .map_err(|e| format!("Invalid digest hex encoding: {}", e))?;
    
    // In a real implementation, this would contact a timestamp authority (TSA)
    // and get a signed timestamp token (RFC 3161)
    // For now, we return a local timestamp
    
    Ok(TimestampOutput {
        timestamp: Utc::now(),
        timestamp_token: BASE64.encode(format!("mock-timestamp-token-{}", Utc::now().timestamp())),
        success: true,
    })
}

// Helper module for hex encoding
mod hex {
    pub fn encode(data: impl AsRef<[u8]>) -> String {
        data.as_ref().iter().map(|b| format!("{:02x}", b)).collect()
    }
    
    pub fn decode(s: &str) -> Result<Vec<u8>, String> {
        if s.len() % 2 != 0 {
            return Err("Invalid hex string length".to_string());
        }
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string()))
            .collect()
    }
}

#[cfg(test)]
mod tests;
