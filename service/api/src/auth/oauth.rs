//! OAuth 2.0 support

use crate::config::{OAuthConfig, OAuthProvider};
use crate::error::ServerError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{Duration, Utc};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OAuth authorization code store
static AUTH_CODE_STORE: Lazy<DashMap<String, AuthCodeData>> = Lazy::new(DashMap::new);

/// OAuth state parameter store (for CSRF protection)
static STATE_STORE: Lazy<DashMap<String, StateData>> = Lazy::new(DashMap::new);

/// Authorization code data
#[derive(Debug, Clone)]
struct AuthCodeData {
    user_id: String,
    client_id: String,
    redirect_uri: String,
    scope: String,
    expires_at: chrono::DateTime<Utc>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
}

/// State parameter data for CSRF protection
#[derive(Debug, Clone)]
struct StateData {
    provider: String,
    redirect_uri: String,
    expires_at: chrono::DateTime<Utc>,
}

/// OAuth authorization request
#[derive(Debug, Clone, Deserialize)]
pub struct AuthorizationRequest {
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub state: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
}

/// OAuth token request
#[derive(Debug, Clone, Deserialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub code: Option<String>,
    pub redirect_uri: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub refresh_token: Option<String>,
    pub code_verifier: Option<String>,
}

/// OAuth token response
#[derive(Debug, Clone, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
}

/// OAuth user info response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub sub: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
}

/// OAuth introspection response
#[derive(Debug, Clone, Serialize)]
pub struct IntrospectionResponse {
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
}

/// External OAuth provider response
#[derive(Debug, Clone, Deserialize)]
pub struct ExternalTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<i64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
}

/// OAuth Manager for handling OAuth 2.0 flows
pub struct OAuthManager {
    config: OAuthConfig,
    providers: HashMap<String, OAuthProvider>,
}

impl OAuthManager {
    /// Create a new OAuth manager
    pub fn new(config: OAuthConfig) -> Self {
        let providers: HashMap<String, OAuthProvider> = config
            .providers
            .iter()
            .map(|p| (p.name.clone(), p.clone()))
            .collect();

        Self { config, providers }
    }

    /// Generate a secure random string
    fn generate_random_string(len: usize) -> String {
        let mut rng = rand::thread_rng();
        let bytes: Vec<u8> = (0..len).map(|_| rng.gen()).collect();
        URL_SAFE_NO_PAD.encode(&bytes)
    }

    /// Generate an authorization code
    pub fn generate_auth_code(
        &self,
        user_id: &str,
        client_id: &str,
        redirect_uri: &str,
        scope: &str,
        code_challenge: Option<&str>,
        code_challenge_method: Option<&str>,
    ) -> String {
        let code = Self::generate_random_string(32);
        let expires_at = Utc::now() + Duration::seconds(self.config.auth_code_expiry);

        AUTH_CODE_STORE.insert(
            code.clone(),
            AuthCodeData {
                user_id: user_id.to_string(),
                client_id: client_id.to_string(),
                redirect_uri: redirect_uri.to_string(),
                scope: scope.to_string(),
                expires_at,
                code_challenge: code_challenge.map(String::from),
                code_challenge_method: code_challenge_method.map(String::from),
            },
        );

        code
    }

    /// Exchange authorization code for user ID (validates code)
    pub fn exchange_auth_code(
        &self,
        code: &str,
        client_id: &str,
        redirect_uri: &str,
        code_verifier: Option<&str>,
    ) -> Result<(String, String), ServerError> {
        let auth_data = AUTH_CODE_STORE
            .remove(code)
            .map(|(_, data)| data)
            .ok_or_else(|| ServerError::OAuthError("Invalid or expired authorization code".to_string()))?;

        // Verify expiration
        if Utc::now() > auth_data.expires_at {
            return Err(ServerError::OAuthError("Authorization code expired".to_string()));
        }

        // Verify client_id
        if auth_data.client_id != client_id {
            return Err(ServerError::OAuthError("Client ID mismatch".to_string()));
        }

        // Verify redirect_uri
        if auth_data.redirect_uri != redirect_uri {
            return Err(ServerError::OAuthError("Redirect URI mismatch".to_string()));
        }

        // Verify PKCE if code_challenge was provided
        if let Some(challenge) = &auth_data.code_challenge {
            let verifier = code_verifier
                .ok_or_else(|| ServerError::OAuthError("Code verifier required".to_string()))?;

            let method = auth_data.code_challenge_method.as_deref().unwrap_or("plain");
            let computed_challenge = if method == "S256" {
                use sha2::{Sha256, Digest};
                let hash = Sha256::digest(verifier.as_bytes());
                URL_SAFE_NO_PAD.encode(hash)
            } else {
                verifier.to_string()
            };

            if &computed_challenge != challenge {
                return Err(ServerError::OAuthError("Invalid code verifier".to_string()));
            }
        }

        Ok((auth_data.user_id, auth_data.scope))
    }

    /// Generate state parameter for external OAuth flow
    pub fn generate_state(&self, provider: &str, redirect_uri: &str) -> String {
        let state = Self::generate_random_string(32);
        let expires_at = Utc::now() + Duration::seconds(600); // 10 minutes

        STATE_STORE.insert(
            state.clone(),
            StateData {
                provider: provider.to_string(),
                redirect_uri: redirect_uri.to_string(),
                expires_at,
            },
        );

        state
    }

    /// Validate state parameter
    pub fn validate_state(&self, state: &str) -> Result<(String, String), ServerError> {
        let state_data = STATE_STORE
            .remove(state)
            .map(|(_, data)| data)
            .ok_or_else(|| ServerError::OAuthError("Invalid or expired state".to_string()))?;

        if Utc::now() > state_data.expires_at {
            return Err(ServerError::OAuthError("State expired".to_string()));
        }

        Ok((state_data.provider, state_data.redirect_uri))
    }

    /// Get authorization URL for external OAuth provider
    pub fn get_authorization_url(&self, provider_name: &str, redirect_uri: &str) -> Result<String, ServerError> {
        let provider = self
            .providers
            .get(provider_name)
            .ok_or_else(|| ServerError::OAuthError(format!("Unknown provider: {}", provider_name)))?;

        let state = self.generate_state(provider_name, redirect_uri);
        let scope = provider.scopes.join(" ");

        let url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
            provider.auth_url,
            urlencoding::encode(&provider.client_id),
            urlencoding::encode(redirect_uri),
            urlencoding::encode(&scope),
            urlencoding::encode(&state)
        );

        Ok(url)
    }

    /// Exchange code from external provider for tokens
    pub async fn exchange_external_code(
        &self,
        provider_name: &str,
        code: &str,
        redirect_uri: &str,
    ) -> Result<ExternalTokenResponse, ServerError> {
        let provider = self
            .providers
            .get(provider_name)
            .ok_or_else(|| ServerError::OAuthError(format!("Unknown provider: {}", provider_name)))?;

        let client = reqwest::Client::new();
        let response = client
            .post(&provider.token_url)
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", redirect_uri),
                ("client_id", &provider.client_id),
                ("client_secret", &provider.client_secret),
            ])
            .send()
            .await
            .map_err(|e| ServerError::OAuthError(format!("Token request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ServerError::OAuthError(format!("Token exchange failed: {}", error_text)));
        }

        response
            .json::<ExternalTokenResponse>()
            .await
            .map_err(|e| ServerError::OAuthError(format!("Failed to parse token response: {}", e)))
    }

    /// Get user info from external provider
    pub async fn get_external_user_info(
        &self,
        provider_name: &str,
        access_token: &str,
    ) -> Result<UserInfo, ServerError> {
        let provider = self
            .providers
            .get(provider_name)
            .ok_or_else(|| ServerError::OAuthError(format!("Unknown provider: {}", provider_name)))?;

        let client = reqwest::Client::new();
        let response = client
            .get(&provider.userinfo_url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| ServerError::OAuthError(format!("User info request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(ServerError::OAuthError("Failed to fetch user info".to_string()));
        }

        response
            .json::<UserInfo>()
            .await
            .map_err(|e| ServerError::OAuthError(format!("Failed to parse user info: {}", e)))
    }

    /// Get OpenID Connect discovery document
    pub fn get_openid_configuration(&self, issuer: &str) -> serde_json::Value {
        serde_json::json!({
            "issuer": issuer,
            "authorization_endpoint": format!("{}/oauth/authorize", issuer),
            "token_endpoint": format!("{}/oauth/token", issuer),
            "userinfo_endpoint": format!("{}/oauth/userinfo", issuer),
            "revocation_endpoint": format!("{}/oauth/revoke", issuer),
            "introspection_endpoint": format!("{}/oauth/introspect", issuer),
            "jwks_uri": format!("{}/.well-known/jwks.json", issuer),
            "response_types_supported": ["code", "token", "id_token", "code token", "code id_token", "token id_token", "code token id_token"],
            "subject_types_supported": ["public"],
            "id_token_signing_alg_values_supported": ["RS256", "HS256"],
            "scopes_supported": ["openid", "profile", "email", "offline_access"],
            "token_endpoint_auth_methods_supported": ["client_secret_basic", "client_secret_post"],
            "claims_supported": ["sub", "name", "email", "email_verified", "picture", "locale", "updated_at"],
            "code_challenge_methods_supported": ["plain", "S256"],
            "grant_types_supported": ["authorization_code", "refresh_token", "client_credentials"]
        })
    }
}
