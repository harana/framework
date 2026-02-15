// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// passkey_credential
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyCredential {
    pub aaguid: Option<String>,
    #[serde(default)]
    pub backed_up: bool,
    pub counter: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub credential_id: String,
    pub device_type: Option<String>,
    pub friendly_name: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    pub public_key_fingerprint: Option<String>,
    pub transports: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub user_id: String,
}

impl PasskeyCredential {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyChallenge {
    pub challenge: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub rp_id: String,
    pub type: String,
    #[serde(default)]
    pub used: bool,
    pub user_id: Option<String>,
}

impl PasskeyChallenge {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_auth_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyAuthLog {
    #[serde(default = "chrono::Utc::now")]
    pub authenticated_at: chrono::DateTime<chrono::Utc>,
    pub credential_id: String,
    pub ip_address: Option<String>,
    #[serde(default)]
    pub success: bool,
    pub user_agent: Option<String>,
    /// Reference: user.id
    pub user_id: String,
}

impl PasskeyAuthLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

