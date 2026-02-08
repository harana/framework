// Harana Actions - Secret Module
// This module provides secret management actions and functionality.

pub mod output;

#[cfg(test)]
mod tests;

use chrono::Utc;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use output::*;
use rand::Rng;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct StoredSecret {
    name: String,
    value: String,
    version: String,
    description: Option<String>,
    created_at: String,
    updated_at: String,
    expires_at: Option<String>,
    versions: Vec<VersionEntry>,
}

#[derive(Debug, Clone)]
struct VersionEntry {
    version: String,
    value: String,
    created_at: String,
}

/// In-memory secret storage
static SECRETS: Lazy<DashMap<String, StoredSecret>> = Lazy::new(DashMap::new);

/// Get Secret Value
///
/// Retrieves a secret value by name, optionally specifying a version.
///
pub async fn get_secret(
    name: &str,
    version: Option<&str>,
) -> Result<GetSecretOutput, String> {
    let secret = SECRETS
        .get(name)
        .ok_or_else(|| format!("Secret not found: {}", name))?;

    // Check expiration
    if let Some(ref expires_at) = secret.expires_at {
        if let Ok(expiry) = chrono::DateTime::parse_from_rfc3339(expires_at) {
            if expiry < Utc::now() {
                return Err(format!("Secret has expired: {}", name));
            }
        }
    }

    let (value, ver, created_at) = if let Some(requested_version) = version {
        // Find specific version
        let version_entry = secret
            .versions
            .iter()
            .find(|v| v.version == requested_version)
            .ok_or_else(|| format!("Version not found: {}", requested_version))?;
        (
            version_entry.value.clone(),
            version_entry.version.clone(),
            Some(version_entry.created_at.clone()),
        )
    } else {
        // Return current version
        (
            secret.value.clone(),
            secret.version.clone(),
            Some(secret.created_at.clone()),
        )
    };

    Ok(GetSecretOutput {
        value,
        version: ver,
        created_at,
    })
}

/// Set Secret Value
///
/// Creates or updates a secret with the specified value.
///
pub async fn set_secret(
    name: &str,
    value: &str,
    description: Option<&str>,
    expires_at: Option<&str>,
) -> Result<SetSecretOutput, String> {
    let now = Utc::now().to_rfc3339();
    let new_version = Uuid::new_v4().to_string();

    if let Some(mut existing) = SECRETS.get_mut(name) {
        // Update existing secret
        let version_entry = VersionEntry {
            version: existing.version.clone(),
            value: existing.value.clone(),
            created_at: existing.updated_at.clone(),
        };
        existing.versions.push(version_entry);
        existing.value = value.to_string();
        existing.version = new_version.clone();
        existing.updated_at = now;
        if let Some(desc) = description {
            existing.description = Some(desc.to_string());
        }
        if let Some(exp) = expires_at {
            existing.expires_at = Some(exp.to_string());
        }
    } else {
        // Create new secret
        let secret = StoredSecret {
            name: name.to_string(),
            value: value.to_string(),
            version: new_version.clone(),
            description: description.map(|s| s.to_string()),
            created_at: now.clone(),
            updated_at: now,
            expires_at: expires_at.map(|s| s.to_string()),
            versions: Vec::new(),
        };
        SECRETS.insert(name.to_string(), secret);
    }

    Ok(SetSecretOutput {
        success: true,
        version: new_version,
    })
}

/// Delete Secret
///
/// Deletes a secret by name.
///
pub async fn delete_secret(
    name: &str,
    force: Option<bool>,
) -> Result<DeleteSecretOutput, String> {
    let _force = force.unwrap_or(false);
    
    if SECRETS.remove(name).is_some() {
        Ok(DeleteSecretOutput { success: true })
    } else {
        Err(format!("Secret not found: {}", name))
    }
}

/// List Available Secrets
///
/// Lists all secrets, optionally filtered by prefix.
///
pub async fn list_secrets(
    prefix: Option<&str>,
    limit: Option<i32>,
) -> Result<ListSecretsOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    
    let secrets: Vec<SecretInfo> = SECRETS
        .iter()
        .filter(|entry| {
            if let Some(p) = prefix {
                entry.key().starts_with(p)
            } else {
                true
            }
        })
        .take(limit)
        .map(|entry| SecretInfo {
            name: entry.name.clone(),
            description: entry.description.clone(),
            created_at: Some(entry.created_at.clone()),
            updated_at: Some(entry.updated_at.clone()),
        })
        .collect();

    let total = secrets.len() as i32;

    Ok(ListSecretsOutput { secrets, total })
}

/// Rotate Secret Value
///
/// Rotates a secret by setting a new value while preserving the old version.
///
pub async fn rotate_secret(
    name: &str,
    new_value: &str,
) -> Result<RotateSecretOutput, String> {
    let mut secret = SECRETS
        .get_mut(name)
        .ok_or_else(|| format!("Secret not found: {}", name))?;

    let old_version = secret.version.clone();
    let new_version = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // Store old version
    let version_entry = VersionEntry {
        version: old_version.clone(),
        value: secret.value.clone(),
        created_at: secret.updated_at.clone(),
    };
    secret.versions.push(version_entry);

    // Update to new value
    secret.value = new_value.to_string();
    secret.version = new_version.clone();
    secret.updated_at = now;

    Ok(RotateSecretOutput {
        success: true,
        old_version,
        new_version,
    })
}

/// Get Secret Metadata
///
/// Retrieves metadata about a secret without exposing its value.
///
pub async fn get_secret_metadata(
    name: &str,
) -> Result<GetSecretMetadataOutput, String> {
    let secret = SECRETS
        .get(name)
        .ok_or_else(|| format!("Secret not found: {}", name))?;

    Ok(GetSecretMetadataOutput {
        description: secret.description.clone(),
        created_at: Some(secret.created_at.clone()),
        updated_at: Some(secret.updated_at.clone()),
        expires_at: secret.expires_at.clone(),
        version_count: (secret.versions.len() + 1) as i32,
    })
}

/// Check If Secret Exists
///
/// Checks whether a secret with the given name exists.
///
pub async fn secret_exists(
    name: &str,
) -> Result<SecretExistsOutput, String> {
    Ok(SecretExistsOutput {
        exists: SECRETS.contains_key(name),
    })
}

/// Get Secret Versions
///
/// Lists all versions of a secret.
///
pub async fn list_secret_versions(
    name: &str,
    limit: Option<i32>,
) -> Result<ListSecretVersionsOutput, String> {
    let secret = SECRETS
        .get(name)
        .ok_or_else(|| format!("Secret not found: {}", name))?;

    let limit = limit.unwrap_or(10) as usize;

    let mut versions: Vec<SecretVersion> = secret
        .versions
        .iter()
        .rev()
        .take(limit.saturating_sub(1))
        .map(|v| SecretVersion {
            version: v.version.clone(),
            created_at: v.created_at.clone(),
            is_current: false,
        })
        .collect();

    // Add current version at the beginning
    versions.insert(0, SecretVersion {
        version: secret.version.clone(),
        created_at: secret.updated_at.clone(),
        is_current: true,
    });

    let total = versions.len() as i32;

    Ok(ListSecretVersionsOutput { versions, total })
}

/// Restore Secret Version
///
/// Restores a previous version of a secret as the current version.
///
pub async fn restore_secret_version(
    name: &str,
    version: &str,
) -> Result<RestoreSecretVersionOutput, String> {
    let mut secret = SECRETS
        .get_mut(name)
        .ok_or_else(|| format!("Secret not found: {}", name))?;

    // Find the version to restore
    let version_entry = secret
        .versions
        .iter()
        .find(|v| v.version == version)
        .ok_or_else(|| format!("Version not found: {}", version))?
        .clone();

    let new_version = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // Store current as old version
    let current_entry = VersionEntry {
        version: secret.version.clone(),
        value: secret.value.clone(),
        created_at: secret.updated_at.clone(),
    };
    secret.versions.push(current_entry);

    // Restore the old value
    secret.value = version_entry.value;
    secret.version = new_version.clone();
    secret.updated_at = now;

    Ok(RestoreSecretVersionOutput {
        success: true,
        new_version,
    })
}

/// Generate Random Secret
///
/// Generates a random secret value and stores it.
///
pub async fn generate_secret(
    name: &str,
    length: Option<i32>,
    charset: Option<&str>,
    description: Option<&str>,
) -> Result<GenerateSecretOutput, String> {
    let length = length.unwrap_or(32) as usize;
    let charset = charset.unwrap_or("Alphanumeric");

    let value = match charset.to_lowercase().as_str() {
        "hex" => generate_hex(length),
        "base64" => generate_base64(length),
        _ => generate_alphanumeric(length), // Default: Alphanumeric
    };

    let result = set_secret(name, &value, description, None).await?;

    Ok(GenerateSecretOutput {
        success: true,
        value,
        version: result.version,
    })
}

fn generate_alphanumeric(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn generate_hex(length: usize) -> String {
    const CHARSET: &[u8] = b"0123456789abcdef";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn generate_base64(length: usize) -> String {
    use base64::{Engine as _, engine::general_purpose};
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..length).map(|_| rng.gen()).collect();
    general_purpose::STANDARD.encode(&bytes)[..length].to_string()
}
