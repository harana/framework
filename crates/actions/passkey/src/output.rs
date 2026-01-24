// Harana Actions - Passkey Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};

// generate_registration_options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRegistrationOptionsOutput {
    pub challenge: String,
    pub options: PasskeyRegistrationOptions,
}

// verify_registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRegistrationOutput {
    pub aaguid: String,
    pub attestation_object: Vec<u8>,
    pub counter: i32,
    pub credential_backed_up: bool,
    pub credential_device_type: String,
    pub credential_id: String,
    pub credential_public_key: Vec<u8>,
    pub verified: bool,
}

// generate_authentication_options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateAuthenticationOptionsOutput {
    pub challenge: String,
    pub options: PasskeyAuthenticationOptions,
}

// verify_authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyAuthenticationOutput {
    pub credential_backed_up: bool,
    pub credential_device_type: String,
    pub new_counter: i32,
    pub verified: bool,
}

// store_credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreCredentialOutput {
    pub passkey_id: String,
    pub success: bool,
}

// get_user_passkeys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPasskeysOutput {
    pub count: i32,
    pub passkeys: Vec<PasskeyInfo>,
}

// get_passkey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPasskeyOutput {
    pub backed_up: bool,
    pub counter: i32,
    pub created_at: DateTime<Utc>,
    pub credential_id: String,
    pub credential_public_key: Vec<u8>,
    pub device_type: String,
    pub friendly_name: Option<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub passkey_id: String,
    pub transports: Vec<String>,
}

// delete_passkey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePasskeyOutput {
    pub success: bool,
}

// update_passkey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePasskeyOutput {
    pub success: bool,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyRegistrationOptions {
    pub challenge: String,
    pub rp: RelyingParty,
    pub user: PasskeyUser,
    pub pub_key_cred_params: Vec<PubKeyCredParam>,
    pub authenticator_selection: Option<PasskeyAuthenticatorSelection>,
    pub attestation: Option<String>,
    pub timeout: Option<i32>,
    pub exclude_credentials: Option<Vec<PasskeyCredentialDescriptor>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyAuthenticationOptions {
    pub challenge: String,
    pub timeout: Option<i32>,
    pub rp_id: String,
    pub allow_credentials: Option<Vec<PasskeyCredentialDescriptor>>,
    pub user_verification: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelyingParty {
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
pub struct PubKeyCredParam {
    #[serde(rename = "type")]
    pub cred_type: String,
    pub alg: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyAuthenticatorSelection {
    pub authenticator_attachment: Option<String>,
    pub resident_key: Option<String>,
    pub user_verification: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyCredentialDescriptor {
    #[serde(rename = "type")]
    pub cred_type: String,
    pub id: String,
    pub transports: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyInfo {
    pub passkey_id: String,
    pub credential_id: String,
    pub friendly_name: Option<String>,
    pub device_type: String,
    pub backed_up: bool,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyRegistrationResponse {
    pub id: String,
    pub raw_id: Vec<u8>,
    pub response: AuthenticatorAttestationResponse,
    #[serde(rename = "type")]
    pub cred_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatorAttestationResponse {
    pub client_data_json: Vec<u8>,
    pub attestation_object: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyAuthenticationResponse {
    pub id: String,
    pub raw_id: Vec<u8>,
    pub response: AuthenticatorAssertionResponse,
    #[serde(rename = "type")]
    pub cred_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatorAssertionResponse {
    pub client_data_json: Vec<u8>,
    pub authenticator_data: Vec<u8>,
    pub signature: Vec<u8>,
    pub user_handle: Option<Vec<u8>>,
}
