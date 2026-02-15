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

/// passkey
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Passkey {
    pub passkey_id: String,
    pub credential_id: String,
    pub user_id: String,
    pub credential_public_key: String,
    pub counter: i64,
    pub device_type: String,
    pub backed_up: bool,
    pub friendly_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: chrono::DateTime<chrono::Utc>,
}

impl Passkey {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_authenticator_selection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyAuthenticatorSelection {
    pub authenticator_attachment: String,
    pub resident_key: String,
    pub require_resident_key: bool,
    pub user_verification: String,
}

impl PasskeyAuthenticatorSelection {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_credential_descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyCredentialDescriptor {
    pub id: String,
    pub type: String,
    pub transports: Vec<String>,
}

impl PasskeyCredentialDescriptor {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_registration_options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyRegistrationOptions {
    pub rp: String,
    pub user: String,
    pub challenge: String,
    pub pub_key_cred_params: Vec<String>,
    pub timeout: i64,
    pub attestation: String,
}

impl PasskeyRegistrationOptions {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_relying_party
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyRelyingParty {
    pub id: String,
    pub name: String,
}

impl PasskeyRelyingParty {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyUser {
    pub id: String,
    pub name: String,
    pub display_name: String,
}

impl PasskeyUser {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_cred_param
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyCredParam {
    pub type: String,
    pub alg: i64,
}

impl PasskeyCredParam {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_registration_response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyRegistrationResponse {
    pub id: String,
    pub raw_id: String,
    pub response: String,
    pub type: String,
}

impl PasskeyRegistrationResponse {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_authenticator_attestation_response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyAuthenticatorAttestationResponse {
    pub client_data_json: String,
    pub attestation_object: String,
}

impl PasskeyAuthenticatorAttestationResponse {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_authentication_options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyAuthenticationOptions {
    pub challenge: String,
    pub timeout: i64,
    pub rp_id: String,
    pub allow_credentials: Vec<String>,
    pub user_verification: String,
}

impl PasskeyAuthenticationOptions {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_authentication_response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyAuthenticationResponse {
    pub id: String,
    pub raw_id: String,
    pub response: String,
    pub type: String,
}

impl PasskeyAuthenticationResponse {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_authenticator_assertion_response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyAuthenticatorAssertionResponse {
    pub client_data_json: String,
    pub authenticator_data: String,
    pub signature: String,
    pub user_handle: String,
}

impl PasskeyAuthenticatorAssertionResponse {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// passkey_info
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyInfo {
    pub passkey_id: String,
    pub credential_id: String,
    pub friendly_name: String,
    pub device_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: chrono::DateTime<chrono::Utc>,
}

impl PasskeyInfo {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

