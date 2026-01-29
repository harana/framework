pub mod output;

use output::*;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::Utc;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use uuid::Uuid;
use url::Url;
use webauthn_rs::prelude::*;
use webauthn_rs_proto::{
    AuthenticatorTransport, UserVerificationPolicy,
    AuthenticatorAttestationResponseRaw, AuthenticatorAssertionResponseRaw,
    RegistrationExtensionsClientOutputs, AuthenticationExtensionsClientOutputs,
};

// In-memory storage for passkeys and registration/authentication states
static PASSKEY_STORE: Lazy<DashMap<String, StoredPasskey>> = Lazy::new(DashMap::new);
static REGISTRATION_STATE_STORE: Lazy<DashMap<String, PasskeyRegistration>> = Lazy::new(DashMap::new);
static AUTHENTICATION_STATE_STORE: Lazy<DashMap<String, PasskeyAuthentication>> = Lazy::new(DashMap::new);

fn create_webauthn(rp_id: &str, rp_origin: &str) -> Result<Webauthn, String> {
    let rp_origin = Url::parse(rp_origin)
        .map_err(|e| format!("Invalid RP origin URL: {}", e))?;
    
    WebauthnBuilder::new(rp_id, &rp_origin)
        .map_err(|e| format!("Failed to create WebauthnBuilder: {}", e))?
        .build()
        .map_err(|e| format!("Failed to build Webauthn: {}", e))
}

/// Generates registration options for WebAuthn passkey registration using webauthn-rs.
pub async fn generate_registration_options(
    relying_party_id: &str,
    relying_party_origin: &str,
    user_id: &str,
    user_name: &str,
    user_display_name: &str,
    _timeout: Option<i32>,
    _attestation: Option<&str>,
    _authenticator_selection: Option<PasskeyAuthenticatorSelection>,
    exclude_credentials: Option<Vec<PasskeyCredentialDescriptor>>,
) -> Result<GenerateRegistrationOptionsOutput, String> {
    let webauthn = create_webauthn(relying_party_id, relying_party_origin)?;
    
    let user_uuid = Uuid::parse_str(user_id).unwrap_or_else(|_| Uuid::new_v4());
    
    let exclude_creds: Option<Vec<CredentialID>> = exclude_credentials.as_ref().map(|creds| {
        creds.iter()
            .filter_map(|c| URL_SAFE_NO_PAD.decode(&c.id).ok())
            .map(CredentialID::from)
            .collect()
    });
    
    let (ccr, reg_state) = webauthn
        .start_passkey_registration(user_uuid, user_name, user_display_name, exclude_creds)
        .map_err(|e| format!("Failed to start registration: {}", e))?;
    
    let challenge_str = URL_SAFE_NO_PAD.encode(ccr.public_key.challenge.as_ref());
    REGISTRATION_STATE_STORE.insert(challenge_str.clone(), reg_state);
    
    let options = PasskeyRegistrationOptions {
        rp: PasskeyRelyingParty {
            id: ccr.public_key.rp.id.clone(),
            name: ccr.public_key.rp.name.clone(),
        },
        user: PasskeyUser {
            id: URL_SAFE_NO_PAD.encode(ccr.public_key.user.id.as_ref()),
            name: ccr.public_key.user.name.clone(),
            display_name: ccr.public_key.user.display_name.clone(),
        },
        challenge: challenge_str.clone(),
        pub_key_cred_params: ccr.public_key.pub_key_cred_params.iter().map(|p| {
            PasskeyCredParam {
                credential_type: "public-key".to_string(),
                alg: p.alg as i32,
            }
        }).collect(),
        timeout: ccr.public_key.timeout.unwrap_or(60000) as i32,
        attestation: "none".to_string(),
        exclude_credentials: None,
        authenticator_selection: None,
    };

    Ok(GenerateRegistrationOptionsOutput { challenge: challenge_str, options })
}

/// Verifies a WebAuthn registration response using webauthn-rs.
pub async fn verify_registration(
    response: PasskeyRegistrationResponse,
    expected_challenge: &str,
    expected_origin: &str,
    expected_rp_id: &str,
    _require_user_verification: Option<bool>,
) -> Result<VerifyRegistrationOutput, String> {
    let webauthn = create_webauthn(expected_rp_id, expected_origin)?;
    
    let reg_state = REGISTRATION_STATE_STORE
        .remove(expected_challenge)
        .map(|(_, v)| v)
        .ok_or("Registration state not found or expired")?;
    
    let reg_response = build_register_credential(&response)?;

    let passkey = webauthn
        .finish_passkey_registration(&reg_response, &reg_state)
        .map_err(|e| format!("Registration verification failed: {}", e))?;
    
    let cred_id = URL_SAFE_NO_PAD.encode(passkey.cred_id().as_ref());
    let passkey_json = serde_json::to_vec(&passkey).unwrap_or_default();

    Ok(VerifyRegistrationOutput {
        verified: true,
        credential_id: cred_id,
        credential_public_key: passkey_json,
        counter: 0,
        aaguid: String::new(),
        attestation_object: vec![],
        credential_device_type: "single_device".to_string(),
        credential_backed_up: false,
    })
}

/// Generates authentication options for WebAuthn passkey authentication using webauthn-rs.
pub async fn generate_authentication_options(
    relying_party_id: &str,
    relying_party_origin: &str,
    _timeout: Option<i32>,
    _user_verification: Option<&str>,
    allow_credentials: Option<Vec<PasskeyCredentialDescriptor>>,
) -> Result<GenerateAuthenticationOptionsOutput, String> {
    let webauthn = create_webauthn(relying_party_id, relying_party_origin)?;

    let passkeys: Vec<Passkey> = if let Some(creds) = &allow_credentials {
        creds.iter()
            .filter_map(|c| {
                PASSKEY_STORE.get(&c.id).and_then(|stored| {
                    serde_json::from_slice(&stored.credential_public_key).ok()
                })
            })
            .collect()
    } else {
        vec![]
    };
    
    if passkeys.is_empty() {
        return Err("No valid passkeys found for authentication".to_string());
    }
    
    let (rcr, auth_state) = webauthn
        .start_passkey_authentication(&passkeys)
        .map_err(|e| format!("Failed to start authentication: {}", e))?;

    let challenge_str = URL_SAFE_NO_PAD.encode(rcr.public_key.challenge.as_ref());
    AUTHENTICATION_STATE_STORE.insert(challenge_str.clone(), auth_state);

    let rp_id = rcr.public_key.rp_id.clone();
    
    let allow_creds: Option<Vec<PasskeyCredentialDescriptor>> = Some(
        rcr.public_key.allow_credentials.iter().map(|c| PasskeyCredentialDescriptor {
            id: URL_SAFE_NO_PAD.encode(c.id.as_ref()),
            credential_type: "public-key".to_string(),
            transports: c.transports.as_ref().map(|t| {
                t.iter().map(|tr| format!("{:?}", tr).to_lowercase()).collect()
            }),
        }).collect()
    );

    let user_verification = match rcr.public_key.user_verification {
        UserVerificationPolicy::Required => "required".to_string(),
        UserVerificationPolicy::Preferred => "preferred".to_string(),
        UserVerificationPolicy::Discouraged_DO_NOT_USE => "discouraged".to_string(),
    };

    let options = PasskeyAuthenticationOptions {
        challenge: challenge_str.clone(),
        timeout: rcr.public_key.timeout.unwrap_or(60000) as i32,
        rp_id,
        allow_credentials: allow_creds,
        user_verification,
    };

    Ok(GenerateAuthenticationOptionsOutput { challenge: challenge_str, options })
}

/// Verifies a WebAuthn authentication response using webauthn-rs.
pub async fn verify_authentication(
    response: PasskeyAuthenticationResponse,
    expected_challenge: &str,
    expected_origin: &str,
    expected_rp_id: &str,
    _credential_public_key: Option<&[u8]>,
    _credential_current_counter: Option<i32>,
    _require_user_verification: Option<bool>,
) -> Result<VerifyAuthenticationOutput, String> {
    let webauthn = create_webauthn(expected_rp_id, expected_origin)?;
    
    let auth_state = AUTHENTICATION_STATE_STORE
        .remove(expected_challenge)
        .map(|(_, v)| v)
        .ok_or("Authentication state not found or expired")?;
    
    let pub_cred = build_auth_credential(&response)?;
    
    let auth_result = webauthn
        .finish_passkey_authentication(&pub_cred, &auth_state)
        .map_err(|e| format!("Authentication verification failed: {}", e))?;
    
    // Update stored passkey counter
    if let Some(mut stored) = PASSKEY_STORE.get_mut(&response.id) {
        if let Ok(mut passkey) = serde_json::from_slice::<Passkey>(&stored.credential_public_key) {
            passkey.update_credential(&auth_result);
            if let Ok(updated_json) = serde_json::to_vec(&passkey) {
                stored.credential_public_key = updated_json;
            }
        }
        stored.counter = auth_result.counter() as i32;
        stored.last_used_at = Some(Utc::now());
    }

    Ok(VerifyAuthenticationOutput {
        verified: true,
        new_counter: auth_result.counter() as i32,
        credential_id: response.id.clone(),
        user_verified: auth_result.user_verified(),
        credential_backed_up: auth_result.backup_state(),
    })
}

/// Stores a credential in the simulated store.
pub async fn store_credential(
    user_id: &str,
    credential_id: &str,
    credential_public_key: Vec<u8>,
    counter: i32,
    name: Option<&str>,
) -> Result<StoreCredentialOutput, String> {
    let stored = StoredPasskey {
        credential_id: credential_id.to_string(),
        user_id: user_id.to_string(),
        credential_public_key,
        counter,
        name: name.map(|s| s.to_string()),
        created_at: Utc::now(),
        last_used_at: None,
    };
    PASSKEY_STORE.insert(credential_id.to_string(), stored);
    Ok(StoreCredentialOutput {
        success: true,
        credential_id: credential_id.to_string(),
    })
}

/// Gets all passkeys for a user.
pub async fn get_user_passkeys(user_id: &str) -> Result<GetUserPasskeysOutput, String> {
    let passkeys: Vec<PasskeyInfo> = PASSKEY_STORE
        .iter()
        .filter(|entry| entry.value().user_id == user_id)
        .map(|entry| {
            let stored = entry.value();
            PasskeyInfo {
                credential_id: stored.credential_id.clone(),
                name: stored.name.clone(),
                created_at: stored.created_at.to_rfc3339(),
                last_used_at: stored.last_used_at.map(|d| d.to_rfc3339()),
                counter: stored.counter,
            }
        })
        .collect();
    Ok(GetUserPasskeysOutput { passkeys })
}

/// Gets a specific passkey by credential ID.
pub async fn get_passkey(credential_id: &str) -> Result<GetPasskeyOutput, String> {
    match PASSKEY_STORE.get(credential_id) {
        Some(entry) => {
            let stored = entry.value();
            Ok(GetPasskeyOutput {
                found: true,
                passkey: Some(PasskeyInfo {
                    credential_id: stored.credential_id.clone(),
                    name: stored.name.clone(),
                    created_at: stored.created_at.to_rfc3339(),
                    last_used_at: stored.last_used_at.map(|d| d.to_rfc3339()),
                    counter: stored.counter,
                }),
            })
        }
        None => Ok(GetPasskeyOutput {
            found: false,
            passkey: None,
        }),
    }
}

/// Updates the counter for a credential.
pub async fn update_counter(credential_id: &str, new_counter: i32) -> Result<UpdateCounterOutput, String> {
    match PASSKEY_STORE.get_mut(credential_id) {
        Some(mut entry) => {
            entry.counter = new_counter;
            entry.last_used_at = Some(Utc::now());
            Ok(UpdateCounterOutput {
                success: true,
                new_counter,
            })
        }
        None => Err(format!("Credential not found: {}", credential_id)),
    }
}

/// Deletes a passkey by credential ID.
pub async fn delete_passkey(credential_id: &str) -> Result<DeletePasskeyOutput, String> {
    match PASSKEY_STORE.remove(credential_id) {
        Some(_) => Ok(DeletePasskeyOutput { success: true }),
        None => Ok(DeletePasskeyOutput { success: false }),
    }
}

/// Updates the name of a passkey.
pub async fn update_passkey_name(credential_id: &str, name: &str) -> Result<UpdatePasskeyNameOutput, String> {
    match PASSKEY_STORE.get_mut(credential_id) {
        Some(mut entry) => {
            entry.name = Some(name.to_string());
            Ok(UpdatePasskeyNameOutput { success: true })
        }
        None => Ok(UpdatePasskeyNameOutput { success: false }),
    }
}

/// Checks if passkey/WebAuthn is supported (always true on server-side).
pub async fn check_support() -> Result<CheckSupportOutput, String> {
    Ok(CheckSupportOutput {
        supported: true,
        platform_authenticator_available: false,
        conditional_mediation_available: false,
    })
}

/// Build a RegisterPublicKeyCredential from PasskeyRegistrationResponse
fn build_register_credential(response: &PasskeyRegistrationResponse) -> Result<RegisterPublicKeyCredential, String> {
    let raw_id = URL_SAFE_NO_PAD.decode(&response.raw_id)
        .map_err(|e| format!("Invalid raw_id: {}", e))?;
    
    let client_data_json = URL_SAFE_NO_PAD.decode(&response.response.client_data_json)
        .map_err(|e| format!("Invalid client_data_json: {}", e))?;
    
    let attestation_object = URL_SAFE_NO_PAD.decode(&response.response.attestation_object)
        .map_err(|e| format!("Invalid attestation_object: {}", e))?;
    
    let transports: Option<Vec<AuthenticatorTransport>> = response.response.transports.as_ref().map(|ts| {
        ts.iter().filter_map(|t| match t.as_str() {
            "usb" => Some(AuthenticatorTransport::Usb),
            "nfc" => Some(AuthenticatorTransport::Nfc),
            "ble" => Some(AuthenticatorTransport::Ble),
            "internal" => Some(AuthenticatorTransport::Internal),
            "hybrid" => Some(AuthenticatorTransport::Hybrid),
            _ => None,
        }).collect()
    });
    
    Ok(RegisterPublicKeyCredential {
        id: response.id.clone(),
        raw_id: Base64UrlSafeData::from(raw_id),
        response: AuthenticatorAttestationResponseRaw {
            attestation_object: Base64UrlSafeData::from(attestation_object),
            client_data_json: Base64UrlSafeData::from(client_data_json),
            transports,
        },
        type_: "public-key".to_string(),
        extensions: RegistrationExtensionsClientOutputs::default(),
    })
}

/// Build a PublicKeyCredential from PasskeyAuthenticationResponse
fn build_auth_credential(response: &PasskeyAuthenticationResponse) -> Result<PublicKeyCredential, String> {
    let raw_id = URL_SAFE_NO_PAD.decode(&response.raw_id)
        .map_err(|e| format!("Invalid raw_id: {}", e))?;
    
    let client_data_json = URL_SAFE_NO_PAD.decode(&response.response.client_data_json)
        .map_err(|e| format!("Invalid client_data_json: {}", e))?;
    
    let authenticator_data = URL_SAFE_NO_PAD.decode(&response.response.authenticator_data)
        .map_err(|e| format!("Invalid authenticator_data: {}", e))?;
    
    let signature = URL_SAFE_NO_PAD.decode(&response.response.signature)
        .map_err(|e| format!("Invalid signature: {}", e))?;
    
    let user_handle = response.response.user_handle.as_ref()
        .map(|h| URL_SAFE_NO_PAD.decode(h))
        .transpose()
        .map_err(|e| format!("Invalid user_handle: {}", e))?;
    
    Ok(PublicKeyCredential {
        id: response.id.clone(),
        raw_id: Base64UrlSafeData::from(raw_id),
        response: AuthenticatorAssertionResponseRaw {
            authenticator_data: Base64UrlSafeData::from(authenticator_data),
            client_data_json: Base64UrlSafeData::from(client_data_json),
            signature: Base64UrlSafeData::from(signature),
            user_handle: user_handle.map(Base64UrlSafeData::from),
        },
        type_: "public-key".to_string(),
        extensions: AuthenticationExtensionsClientOutputs::default(),
    })
}

#[cfg(test)]
mod tests;
