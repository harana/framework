// Harana Actions - Passkey Module Output Types
// Output structs for WebAuthn passkey action methods using webauthn-rs.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ============================================================================
// Action Output Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRegistrationOptionsOutput {
    pub challenge: String,
    pub options: PasskeyRegistrationOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRegistrationOutput {
    pub verified: bool,
    pub credential_id: String,
    pub credential_public_key: Vec<u8>,
    pub counter: i32,
    pub aaguid: String,
    pub attestation_object: Vec<u8>,
    pub credential_device_type: String,
    pub credential_backed_up: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateAuthenticationOptionsOutput {
    pub challenge: String,
    pub options: PasskeyAuthenticationOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyAuthenticationOutput {
    pub verified: bool,
    pub new_counter: i32,
    pub credential_id: String,
    pub user_verified: bool,
    pub credential_backed_up: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreCredentialOutput {
    pub success: bool,
    pub credential_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPasskeysOutput {
    pub passkeys: Vec<PasskeyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPasskeyOutput {
    pub found: bool,
    pub passkey: Option<PasskeyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCounterOutput {
    pub success: bool,
    pub new_counter: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePasskeyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePasskeyNameOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckSupportOutput {
    pub supported: bool,
    pub platform_authenticator_available: bool,
    pub conditional_mediation_available: bool,
}

// ============================================================================
// Supporting Types - WebAuthn Options
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyAuthenticatorSelection {
    pub authenticator_attachment: Option<String>,
    pub resident_key: Option<String>,
    pub require_resident_key: Option<bool>,
    pub user_verification: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyCredentialDescriptor {
    pub id: String,
    #[serde(rename = "type")]
    pub credential_type: String,
    pub transports: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyRegistrationOptions {
    pub rp: PasskeyRelyingParty,
    pub user: PasskeyUser,
    pub challenge: String,
    pub pub_key_cred_params: Vec<PasskeyCredParam>,
    pub timeout: i32,
    pub attestation: String,
    pub exclude_credentials: Option<Vec<PasskeyCredentialDescriptor>>,
    pub authenticator_selection: Option<PasskeyAuthenticatorSelection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyRelyingParty {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyUser {
    pub id: String,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyCredParam {
    #[serde(rename = "type")]
    pub credential_type: String,
    pub alg: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyAuthenticationOptions {
    pub challenge: String,
    pub timeout: i32,
    pub rp_id: String,
    pub allow_credentials: Option<Vec<PasskeyCredentialDescriptor>>,
    pub user_verification: String,
}

// ============================================================================
// Supporting Types - Client Responses
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyRegistrationResponse {
    pub id: String,
    pub raw_id: String,
    pub response: PasskeyAuthenticatorAttestationResponse,
    #[serde(rename = "type")]
    pub credential_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyAuthenticatorAttestationResponse {
    pub client_data_json: String,
    pub attestation_object: String,
    pub transports: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyAuthenticationResponse {
    pub id: String,
    pub raw_id: String,
    pub response: PasskeyAuthenticatorAssertionResponse,
    #[serde(rename = "type")]
    pub credential_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyAuthenticatorAssertionResponse {
    pub client_data_json: String,
    pub authenticator_data: String,
    pub signature: String,
    pub user_handle: Option<String>,
}

// ============================================================================
// Supporting Types - Storage
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyInfo {
    pub credential_id: String,
    pub name: Option<String>,
    pub created_at: String,
    pub last_used_at: Option<String>,
    pub counter: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredPasskey {
    pub credential_id: String,
    pub user_id: String,
    pub credential_public_key: Vec<u8>,
    pub counter: i32,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}
