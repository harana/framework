// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// encryption_key
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EncryptionKey {
    pub algorithm: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub created_by: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub is_active: bool,
    pub key_id: String,
    pub key_size: i64,
    pub key_type: String,
    pub last_rotated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub usage: String,
}

impl EncryptionKey {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// key_pair
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeyPair {
    pub algorithm: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub created_by: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub is_active: bool,
    pub key_id: String,
    pub key_size: i64,
    pub public_key_fingerprint: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl KeyPair {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// crypto_operation_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CryptoOperationLog {
    pub algorithm: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub key_id: String,
    pub operation: String,
    pub status: String,
    /// Reference: user.id
    pub user_id: Option<String>,
}

impl CryptoOperationLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

