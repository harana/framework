// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateRegistrationOptionsOutput {
    pub challenge: String,
    pub options: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyRegistrationOutput {
    pub aaguid: String,
    pub attestation_object: String,
    pub counter: i64,
    pub credential_backed_up: bool,
    pub credential_device_type: String,
    pub credential_id: String,
    pub credential_public_key: String,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateAuthenticationOptionsOutput {
    pub challenge: String,
    pub options: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyAuthenticationOutput {
    pub credential_backed_up: bool,
    pub credential_device_type: String,
    pub new_counter: i64,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetUserPasskeysOutput {
    pub count: i64,
    pub passkeys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPasskeyOutput {
    pub backed_up: bool,
    pub counter: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub credential_id: String,
    pub credential_public_key: String,
    pub device_type: String,
    pub friendly_name: String,
    pub last_used_at: chrono::DateTime<chrono::Utc>,
    pub passkey_id: String,
    pub transports: Vec<String>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckSupportOutput {
    pub conditional_mediation: bool,
    pub platform_authenticator: bool,
    pub user_verifying_platform_authenticator: bool,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyAuthenticatorSelection {
    pub authenticator_attachment: String,
    pub resident_key: String,
    pub require_resident_key: bool,
    pub user_verification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyCredentialDescriptor {
    pub id: String,
    pub type: String,
    pub transports: Vec<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyRelyingParty {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyUser {
    pub id: String,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyCredParam {
    pub type: String,
    pub alg: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyRegistrationResponse {
    pub id: String,
    pub raw_id: String,
    pub response: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyAuthenticatorAttestationResponse {
    pub client_data_json: String,
    pub attestation_object: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyAuthenticationOptions {
    pub challenge: String,
    pub timeout: i64,
    pub rp_id: String,
    pub allow_credentials: Vec<String>,
    pub user_verification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyAuthenticationResponse {
    pub id: String,
    pub raw_id: String,
    pub response: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyAuthenticatorAssertionResponse {
    pub client_data_json: String,
    pub authenticator_data: String,
    pub signature: String,
    pub user_handle: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyCredential {
    pub aaguid: String,
    #[serde(default)]
    pub backed_up: bool,
    pub counter: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub credential_id: String,
    pub device_type: String,
    pub friendly_name: String,
    #[serde(default)]
    pub is_active: bool,
    pub last_used_at: chrono::DateTime<chrono::Utc>,
    pub public_key_fingerprint: String,
    pub transports: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

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
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyAuthLog {
    #[serde(default = "chrono::Utc::now")]
    pub authenticated_at: chrono::DateTime<chrono::Utc>,
    pub credential_id: String,
    pub ip_address: String,
    #[serde(default)]
    pub success: bool,
    pub user_agent: String,
    pub user_id: String,
}

#[async_trait]
pub trait PasskeyAction {
    async fn generate_registration_options(&self, attestation: String, authenticator_selection: String, exclude_credentials: Vec<String>, relying_party_id: String, relying_party_name: String, timeout: i64, user_display_name: String, user_id: String, user_name: String) -> Result<GenerateRegistrationOptionsOutput, Box<dyn std::error::Error>>;
    async fn verify_registration(&self, expected_challenge: String, expected_origin: String, expected_rp_id: String, require_user_verification: bool, response: String) -> Result<VerifyRegistrationOutput, Box<dyn std::error::Error>>;
    async fn generate_authentication_options(&self, allow_credentials: Vec<String>, relying_party_id: String, timeout: i64, user_verification: String) -> Result<GenerateAuthenticationOptionsOutput, Box<dyn std::error::Error>>;
    async fn verify_authentication(&self, credential_current_counter: i64, credential_public_key: String, expected_challenge: String, expected_origin: String, expected_rp_id: String, require_user_verification: bool, response: String) -> Result<VerifyAuthenticationOutput, Box<dyn std::error::Error>>;
    async fn store_credential(&self, aaguid: String, backed_up: bool, counter: i64, credential_id: String, credential_public_key: String, device_type: String, friendly_name: String, transports: Vec<String>, user_id: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_user_passkeys(&self, user_id: String) -> Result<GetUserPasskeysOutput, Box<dyn std::error::Error>>;
    async fn get_passkey(&self, credential_id: String) -> Result<GetPasskeyOutput, Box<dyn std::error::Error>>;
    async fn update_counter(&self, credential_id: String, new_counter: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_passkey(&self, passkey_id: String, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn update_passkey_name(&self, friendly_name: String, passkey_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn check_support(&self, user_agent: String) -> Result<CheckSupportOutput, Box<dyn std::error::Error>>;
}
