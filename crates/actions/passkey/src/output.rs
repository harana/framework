// Harana Actions - Passkey Module Output Types
// Output structs for WebAuthn passkey action methods using webauthn-rs.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ============================================================================
// Action Output Types
// ============================================================================

/// Output for generate_registration_options action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRegistrationOptionsOutput {
    pub challenge: String,
    pub options: PasskeyRegistrationOptions,
}

/// Output for verify_registration action
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

/// Output for generate_authentication_options action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateAuthenticationOptionsOutput {
    pub challenge: String,
    pub options: PasskeyAuthenticationOptions,
}

/// Output for verify_authentication action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyAuthenticationOutput {
    pub verified: bool,
    pub new_counter: i32,
    pub credential_id: String,
    pub user_verified: bool,
    pub credential_backed_up: bool,
}

/// Output for store_credential action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreCredentialOutput {
    pub success: bool,
    pub credential_id: String,
}

/// Output for get_user_passkeys action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPasskeysOutput {
    pub passkeys: Vec<PasskeyInfo>,
}

/// Output for get_passkey action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPasskeyOutput {
    pub found: bool,
    pub passkey: Option<PasskeyInfo>,
}

/// Output for update_counter action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCounterOutput {
    pub success: bool,
    pub new_counter: i32,
}

/// Output for delete_passkey action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePasskeyOutput {
    pub success: bool,
}

/// Output for update_passkey_name action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePasskeyNameOutput {
    pub success: bool,
}

/// Output for check_support action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckSupportOutput {
    pub supported: bool,
    pub platform_authenticator_available: bool,
    pub conditional_mediation_available: bool,
}

// ============================================================================
// Supporting Types - WebAuthn Options
// ============================================================================

/// Authenticator selection criteria for registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyAuthenticatorSelection {
    pub authenticator_attachment: Option<String>,
    pub resident_key: Option<String>,
    pub require_resident_key: Option<bool>,
    pub user_verification: Option<String>,
}

/// Credential descriptor for allow/exclude lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyCredentialDescriptor {
    pub id: String,
    #[serde(rename = "type")]
    pub credential_type: String,
    pub transports: Option<Vec<String>>,
}

/// Registration options sent to client
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

/// Relying party information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyRelyingParty {
    pub id: String,
    pub name: String,
}

/// User information for registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyUser {
    pub id: String,
    pub name: String,
    pub display_name: String,
}

/// Public key credential parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyCredParam {
    #[serde(rename = "type")]
    pub credential_type: String,
    pub alg: i32,
}

/// Authentication options sent to client
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

/// Registration response from client authenticator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyRegistrationResponse {
    pub id: String,
    pub raw_id: String,
    pub response: PasskeyAuthenticatorAttestationResponse,
    #[serde(rename = "type")]
    pub credential_type: String,
}

/// Attestation response data from registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyAuthenticatorAttestationResponse {
    pub client_data_json: String,
    pub attestation_object: String,
    pub transports: Option<Vec<String>>,
}

/// Authentication response from client authenticator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyAuthenticationResponse {
    pub id: String,
    pub raw_id: String,
    pub response: PasskeyAuthenticatorAssertionResponse,
    #[serde(rename = "type")]
    pub credential_type: String,
}

/// Assertion response data from authentication
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

/// Passkey information for listing/display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyInfo {
    pub credential_id: String,
    pub name: Option<String>,
    pub created_at: String,
    pub last_used_at: Option<String>,
    pub counter: i32,
}

/// Stored passkey data (in-memory or database)
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
