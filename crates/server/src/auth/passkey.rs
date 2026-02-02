//! Passkey/WebAuthn authentication handlers

use crate::error::ServerError;
use crate::config::PasskeyConfig;
use harana_actions_passkey::{
    generate_registration_options, verify_registration,
    generate_authentication_options, verify_authentication,
    store_credential, get_user_passkeys,
    output::{
        PasskeyRegistrationResponse, PasskeyAuthenticationResponse,
        PasskeyCredentialDescriptor, GenerateRegistrationOptionsOutput,
        GenerateAuthenticationOptionsOutput, VerifyRegistrationOutput,
        VerifyAuthenticationOutput,
    },
};
use serde::{Deserialize, Serialize};

/// Passkey registration request
#[derive(Debug, Clone, Deserialize)]
pub struct PasskeyRegisterRequest {
    pub user_id: String,
    pub user_name: String,
    pub user_display_name: String,
}

/// Passkey registration verify request
#[derive(Debug, Clone, Deserialize)]
pub struct PasskeyRegisterVerifyRequest {
    pub challenge: String,
    pub response: PasskeyRegistrationResponse,
    pub user_id: String,
    pub credential_name: Option<String>,
}

/// Passkey login request
#[derive(Debug, Clone, Deserialize)]
pub struct PasskeyLoginRequest {
    pub user_id: Option<String>,
}

/// Passkey login verify request
#[derive(Debug, Clone, Deserialize)]
pub struct PasskeyLoginVerifyRequest {
    pub challenge: String,
    pub response: PasskeyAuthenticationResponse,
}

/// Response for passkey registration options
#[derive(Debug, Clone, Serialize)]
pub struct PasskeyRegisterOptionsResponse {
    pub challenge: String,
    pub options: serde_json::Value,
}

/// Response for passkey login options
#[derive(Debug, Clone, Serialize)]
pub struct PasskeyLoginOptionsResponse {
    pub challenge: String,
    pub options: serde_json::Value,
}

/// Response for successful passkey verification
#[derive(Debug, Clone, Serialize)]
pub struct PasskeyVerifyResponse {
    pub verified: bool,
    pub user_id: Option<String>,
    pub credential_id: String,
}

/// Generate passkey registration options
pub async fn generate_register_options(
    config: &PasskeyConfig,
    request: &PasskeyRegisterRequest,
) -> Result<GenerateRegistrationOptionsOutput, ServerError> {
    // Get existing passkeys for the user to exclude
    let existing = get_user_passkeys(&request.user_id).await.ok();
    let exclude_credentials: Option<Vec<PasskeyCredentialDescriptor>> = existing.map(|p| {
        p.passkeys
            .into_iter()
            .map(|pk| PasskeyCredentialDescriptor {
                id: pk.credential_id,
                credential_type: "public-key".to_string(),
                transports: None,
            })
            .collect()
    });

    generate_registration_options(
        &config.rp_id,
        &config.rp_origin,
        &request.user_id,
        &request.user_name,
        &request.user_display_name,
        Some(config.timeout as i32),
        Some("none"),
        None,
        exclude_credentials,
    )
    .await
    .map_err(|e| ServerError::PasskeyError(e))
}

/// Verify passkey registration
pub async fn verify_register(
    config: &PasskeyConfig,
    request: &PasskeyRegisterVerifyRequest,
) -> Result<VerifyRegistrationOutput, ServerError> {
    let result = verify_registration(
        request.response.clone(),
        &request.challenge,
        &config.rp_origin,
        &config.rp_id,
        Some(true),
    )
    .await
    .map_err(|e| ServerError::PasskeyError(e))?;

    if result.verified {
        // Store the credential
        let _ = store_credential(
            &request.user_id,
            &result.credential_id,
            result.credential_public_key.clone(),
            result.counter,
            request.credential_name.as_deref(),
        )
        .await;
    }

    Ok(result)
}

/// Generate passkey authentication options
pub async fn generate_login_options(
    config: &PasskeyConfig,
    user_id: Option<&str>,
) -> Result<GenerateAuthenticationOptionsOutput, ServerError> {
    // Get allow credentials for the user if provided
    let allow_credentials: Option<Vec<PasskeyCredentialDescriptor>> = if let Some(uid) = user_id {
        get_user_passkeys(uid).await.ok().map(|p| {
            p.passkeys
                .into_iter()
                .map(|pk| PasskeyCredentialDescriptor {
                    id: pk.credential_id,
                    credential_type: "public-key".to_string(),
                    transports: None,
                })
                .collect()
        })
    } else {
        None
    };

    generate_authentication_options(
        &config.rp_id,
        &config.rp_origin,
        Some(config.timeout as i32),
        Some("preferred"),
        allow_credentials,
    )
    .await
    .map_err(|e| ServerError::PasskeyError(e))
}

/// Verify passkey authentication
pub async fn verify_login(
    config: &PasskeyConfig,
    request: &PasskeyLoginVerifyRequest,
) -> Result<VerifyAuthenticationOutput, ServerError> {
    verify_authentication(
        request.response.clone(),
        &request.challenge,
        &config.rp_origin,
        &config.rp_id,
        None,
        None,
        Some(true),
    )
    .await
    .map_err(|e| ServerError::PasskeyError(e))
}
