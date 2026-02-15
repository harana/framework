// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SigningKey {
    pub algorithm: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignatureVerificationLog {
    pub error_message: Option<String>,
    pub ip_address: Option<String>,
    #[serde(default)]
    pub is_valid: bool,
    pub signature_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub verified_at: chrono::DateTime<chrono::Utc>,
    pub verified_by: Option<String>,
}

impl SignatureVerificationLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignatureMetadata {
    pub signer: String,
    pub organization: String,
    pub email: String,
    pub description: String,
    pub url: String,
    pub custom_attributes: std::collections::HashMap<String, String>,
}

impl SignatureMetadata {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignedArtifact {
    pub artifact: String,
    pub signature: String,
    pub algorithm: String,
    pub format: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub signer: String,
    pub certificate_chain: Vec<String>,
}

impl SignedArtifact {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

