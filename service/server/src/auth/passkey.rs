//! Passkey/WebAuthn authentication handlers

use crate::error::ServerError;
use crate::config::PasskeyConfig;
#[cfg(feature = "standalone")]
use harana_actions_passkey::{
    generate_registration_options, verify_registration,
    generate_authentication_options, verify_authentication,
    store_credential, get_user_passkeys,
    output::{
        PasskeyCredentialDescriptor, GenerateRegistrationOptionsOutput,
        GenerateAuthenticationOptionsOutput, VerifyRegistrationOutput,
        VerifyAuthenticationOutput,
    },
};
#[cfg(feature = "standalone")]
use harana_actions_passkey::output::{
    PasskeyRegistrationResponse, PasskeyAuthenticationResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct PasskeyRegisterRequest {
    pub user_id: String,
    pub user_name: String,
    pub user_display_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PasskeyRegisterVerifyRequest {
    pub challenge: String,
    pub response: serde_json::Value,
    pub user_id: String,
    pub credential_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PasskeyLoginRequest {
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PasskeyLoginVerifyRequest {
    pub challenge: String,
    pub response: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct PasskeyRegisterOptionsResponse {
    pub challenge: String,
    pub options: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct PasskeyLoginOptionsResponse {
    pub challenge: String,
    pub options: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct PasskeyVerifyResponse {
    pub verified: bool,
    pub user_id: Option<String>,
    pub credential_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisterOptionsResult {
    pub challenge: String,
    pub options: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginOptionsResult {
    pub challenge: String,
    pub options: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct VerifyResult {
    pub verified: bool,
    pub credential_id: String,
}

/// Generate passkey registration options
#[cfg(feature = "standalone")]
pub async fn generate_register_options(
    config: &PasskeyConfig,
    request: &PasskeyRegisterRequest,
) -> Result<RegisterOptionsResult, ServerError> {
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

    let result = generate_registration_options(
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
    .map_err(|e| ServerError::PasskeyError(e))?;

    Ok(RegisterOptionsResult {
        challenge: result.challenge,
        options: result.options,
    })
}

#[cfg(not(feature = "standalone"))]
pub async fn generate_register_options(
    _config: &PasskeyConfig,
    _request: &PasskeyRegisterRequest,
) -> Result<RegisterOptionsResult, ServerError> {
    Err(ServerError::PasskeyError(
        "Passkey registration not yet supported on Cloudflare Workers".to_string(),
    ))
}

/// Verify passkey registration
#[cfg(feature = "standalone")]
pub async fn verify_register(
    config: &PasskeyConfig,
    request: &PasskeyRegisterVerifyRequest,
) -> Result<VerifyResult, ServerError> {
    let response: PasskeyRegistrationResponse = serde_json::from_value(request.response.clone())
        .map_err(|e| ServerError::PasskeyError(format!("Invalid response: {}", e)))?;

    let result = verify_registration(
        response,
        &request.challenge,
        &config.rp_origin,
        &config.rp_id,
        Some(true),
    )
    .await
    .map_err(|e| ServerError::PasskeyError(e))?;

    if result.verified {
        let _ = store_credential(
            &request.user_id,
            &result.credential_id,
            result.credential_public_key.clone(),
            result.counter,
            request.credential_name.as_deref(),
        )
        .await;
    }

    Ok(VerifyResult {
        verified: result.verified,
        credential_id: result.credential_id,
    })
}

#[cfg(not(feature = "standalone"))]
pub async fn verify_register(
    _config: &PasskeyConfig,
    _request: &PasskeyRegisterVerifyRequest,
) -> Result<VerifyResult, ServerError> {
    Err(ServerError::PasskeyError(
        "Passkey verification not yet supported on Cloudflare Workers".to_string(),
    ))
}

/// Generate passkey authentication options
#[cfg(feature = "standalone")]
pub async fn generate_login_options(
    config: &PasskeyConfig,
    user_id: Option<&str>,
) -> Result<LoginOptionsResult, ServerError> {
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

    let result = generate_authentication_options(
        &config.rp_id,
        &config.rp_origin,
        Some(config.timeout as i32),
        Some("preferred"),
        allow_credentials,
    )
    .await
    .map_err(|e| ServerError::PasskeyError(e))?;

    Ok(LoginOptionsResult {
        challenge: result.challenge,
        options: result.options,
    })
}

#[cfg(not(feature = "standalone"))]
pub async fn generate_login_options(
    _config: &PasskeyConfig,
    _user_id: Option<&str>,
) -> Result<LoginOptionsResult, ServerError> {
    Err(ServerError::PasskeyError(
        "Passkey login not yet supported on Cloudflare Workers".to_string(),
    ))
}

/// Verify passkey authentication
#[cfg(feature = "standalone")]
pub async fn verify_login(
    config: &PasskeyConfig,
    request: &PasskeyLoginVerifyRequest,
) -> Result<VerifyResult, ServerError> {
    let response: PasskeyAuthenticationResponse = serde_json::from_value(request.response.clone())
        .map_err(|e| ServerError::PasskeyError(format!("Invalid response: {}", e)))?;

    let result = verify_authentication(
        response,
        &request.challenge,
        &config.rp_origin,
        &config.rp_id,
        None,
        None,
        Some(true),
    )
    .await
    .map_err(|e| ServerError::PasskeyError(e))?;

    Ok(VerifyResult {
        verified: result.verified,
        credential_id: result.credential_id,
    })
}

#[cfg(not(feature = "standalone"))]
pub async fn verify_login(
    _config: &PasskeyConfig,
    _request: &PasskeyLoginVerifyRequest,
) -> Result<VerifyResult, ServerError> {
    Err(ServerError::PasskeyError(
        "Passkey authentication not yet supported on Cloudflare Workers".to_string(),
    ))
}
