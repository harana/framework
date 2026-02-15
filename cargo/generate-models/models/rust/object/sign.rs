// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// signing_key
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SigningKey {
    pub algorithm: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub created_by: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub is_active: bool,
    pub key_id: String,
    pub public_key_fingerprint: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub usage: String,
}

impl SigningKey {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// signature
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Signature {
    pub algorithm: String,
    pub artifact_hash: String,
    pub artifact_path: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub format: String,
    #[serde(default)]
    pub is_valid: bool,
    pub key_id: String,
    pub signer: Option<String>,
}

impl Signature {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// signature_verification_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignatureVerificationLog {
    pub error_message: Option<String>,
    pub ip_address: Option<String>,
    #[serde(default)]
    pub is_valid: bool,
    /// Reference: signature.id
    pub signature_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub verified_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub verified_by: Option<String>,
}

impl SignatureVerificationLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

