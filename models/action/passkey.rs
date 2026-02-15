// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateRegistrationOptionsInput {
    pub attestation: String,
    pub authenticator_selection: String,
    pub exclude_credentials: Vec<String>,
    pub relying_party_id: String,
    pub relying_party_name: String,
    pub timeout: i64,
    pub user_display_name: String,
    pub user_id: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateRegistrationOptionsOutput {
    pub challenge: String,
    pub options: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyRegistrationInput {
    pub expected_challenge: String,
    pub expected_origin: String,
    pub expected_rp_id: String,
    #[serde(default)]
    pub require_user_verification: bool,
    pub response: String,
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
pub struct GenerateAuthenticationOptionsInput {
    pub allow_credentials: Vec<String>,
    pub relying_party_id: String,
    pub timeout: i64,
    pub user_verification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateAuthenticationOptionsOutput {
    pub challenge: String,
    pub options: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyAuthenticationInput {
    pub credential_current_counter: i64,
    pub credential_public_key: String,
    pub expected_challenge: String,
    pub expected_origin: String,
    pub expected_rp_id: String,
    #[serde(default)]
    pub require_user_verification: bool,
    pub response: String,
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
pub struct StoreCredentialInput {
    pub aaguid: String,
    pub backed_up: bool,
    pub counter: i64,
    pub credential_id: String,
    pub credential_public_key: String,
    pub device_type: String,
    pub friendly_name: String,
    pub transports: Vec<String>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StoreCredentialOutput {
    pub passkey_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetUserPasskeysInput {
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetUserPasskeysOutput {
    pub count: i64,
    pub passkeys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetPasskeyInput {
    pub credential_id: String,
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
pub struct UpdateCounterInput {
    pub credential_id: String,
    pub new_counter: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateCounterOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeletePasskeyInput {
    pub passkey_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeletePasskeyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdatePasskeyNameInput {
    pub friendly_name: String,
    pub passkey_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdatePasskeyNameOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckSupportInput {
    pub user_agent: String,
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

#[async_trait]
pub trait PasskeyAction {
    async fn generate_registration_options(&self, input: GenerateRegistrationOptionsInput) -> Result<GenerateRegistrationOptionsOutput, Box<dyn std::error::Error>>;
    async fn verify_registration(&self, input: VerifyRegistrationInput) -> Result<VerifyRegistrationOutput, Box<dyn std::error::Error>>;
    async fn generate_authentication_options(&self, input: GenerateAuthenticationOptionsInput) -> Result<GenerateAuthenticationOptionsOutput, Box<dyn std::error::Error>>;
    async fn verify_authentication(&self, input: VerifyAuthenticationInput) -> Result<VerifyAuthenticationOutput, Box<dyn std::error::Error>>;
    async fn store_credential(&self, input: StoreCredentialInput) -> Result<StoreCredentialOutput, Box<dyn std::error::Error>>;
    async fn get_user_passkeys(&self, input: GetUserPasskeysInput) -> Result<GetUserPasskeysOutput, Box<dyn std::error::Error>>;
    async fn get_passkey(&self, input: GetPasskeyInput) -> Result<GetPasskeyOutput, Box<dyn std::error::Error>>;
    async fn update_counter(&self, input: UpdateCounterInput) -> Result<UpdateCounterOutput, Box<dyn std::error::Error>>;
    async fn delete_passkey(&self, input: DeletePasskeyInput) -> Result<DeletePasskeyOutput, Box<dyn std::error::Error>>;
    async fn update_passkey_name(&self, input: UpdatePasskeyNameInput) -> Result<UpdatePasskeyNameOutput, Box<dyn std::error::Error>>;
    async fn check_support(&self, input: CheckSupportInput) -> Result<CheckSupportOutput, Box<dyn std::error::Error>>;
}
