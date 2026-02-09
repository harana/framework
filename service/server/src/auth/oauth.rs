use crate::config::{OAuthConfig, OAuthProvider, OAuthProviderKind};
use crate::error::ServerError;
use harana_components_cache::{CacheStore, PutOptions};
use openidconnect::core::{
    CoreClient, CoreGenderClaim, CoreIdTokenClaims, CoreIdTokenVerifier, CoreProviderMetadata, CoreResponseType,
};
use openidconnect::{
    AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, OAuth2TokenResponse,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse as OidcTokenResponse,
};
use openidconnect::{EndpointMaybeSet, EndpointNotSet, EndpointSet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Type alias for the OIDC client as returned by `CoreClient::from_provider_metadata`.
/// Auth URL is always set; token and userinfo URLs may or may not be present in
/// the discovery document, so they are `EndpointMaybeSet`.
type DiscoveredClient =
    CoreClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointMaybeSet, EndpointMaybeSet>;

// =============================================================================
// Public types
// =============================================================================

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OAuthStateData {
    provider: String,
    nonce: String,
    pkce_verifier: String,
    redirect_uri: String,
}

// =============================================================================
// Cache key helpers
// =============================================================================

fn state_cache_key(state: &str) -> String {
    format!("oauth:state:{state}")
}

fn discovery_cache_key(provider: &str) -> String {
    format!("oauth:discovery:{provider}")
}

// =============================================================================
// OAuthManager
// =============================================================================

pub struct OAuthManager {
    config: OAuthConfig,
    providers: HashMap<String, OAuthProvider>,
    cache: Arc<dyn CacheStore>,
}

impl OAuthManager {
    pub fn new(config: OAuthConfig, cache: Arc<dyn CacheStore>) -> Self {
        let providers: HashMap<String, OAuthProvider> =
            config.providers.iter().map(|p| (p.name.clone(), p.clone())).collect();

        Self {
            config,
            providers,
            cache,
        }
    }

    // -------------------------------------------------------------------------
    // Helpers
    // -------------------------------------------------------------------------

    /// TTL for state/nonce entries.
    fn state_ttl(&self) -> u64 {
        self.config.state_ttl.unwrap_or(600)
    }

    /// TTL for cached discovery documents.
    fn discovery_ttl(&self) -> u64 {
        self.config.discovery_cache_ttl.unwrap_or(3600)
    }

    /// Resolve the OIDC discovery URL for a provider.
    fn discovery_url(provider: &OAuthProvider) -> Result<String, ServerError> {
        if let Some(ref url) = provider.discovery_url {
            return Ok(url.clone());
        }
        match provider.kind {
            OAuthProviderKind::Cloudflare => {
                let team = provider.cloudflare_team_domain.as_deref().ok_or_else(|| {
                    ServerError::OAuthError("Cloudflare provider requires `cloudflare_team_domain`".into())
                })?;
                Ok(format!(
                    "https://{team}/cdn-cgi/access/sso/oidc/{}/.well-known/openid-configuration",
                    provider.client_id
                ))
            }
            OAuthProviderKind::Google => Ok("https://accounts.google.com/.well-known/openid-configuration".into()),
            OAuthProviderKind::Microsoft => {
                Ok("https://login.microsoftonline.com/common/v2.0/.well-known/openid-configuration".into())
            }
            _ => Err(ServerError::OAuthError(
                "Provider requires either `discovery_url` or explicit auth/token/userinfo URLs".into(),
            )),
        }
    }

    /// Build an HTTP client from `openidconnect::reqwest` (which re-exports the
    /// version of reqwest that openidconnect itself depends on, avoiding version
    /// mismatches with the workspace's reqwest).
    #[cfg(feature = "standalone")]
    fn oidc_http_client() -> Result<openidconnect::reqwest::Client, ServerError> {
        let client = openidconnect::reqwest::ClientBuilder::new()
            .redirect(openidconnect::reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| ServerError::OAuthError(format!("HTTP client error: {e}")))?;
        Ok(client)
    }

    // -------------------------------------------------------------------------
    // Discovery & client construction
    // -------------------------------------------------------------------------

    /// Fetch (or retrieve from cache) the OIDC provider metadata and build a
    /// [`CoreClient`].
    #[cfg(feature = "standalone")]
    async fn build_client(
        &self,
        provider: &OAuthProvider,
        redirect_uri: &str,
    ) -> Result<DiscoveredClient, ServerError> {
        let metadata = self.get_provider_metadata(provider).await?;

        let client = CoreClient::from_provider_metadata(
            metadata,
            ClientId::new(provider.client_id.clone()),
            Some(ClientSecret::new(provider.client_secret.clone())),
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_uri.to_string())
                .map_err(|e| ServerError::OAuthError(format!("Invalid redirect URI: {e}")))?,
        );

        Ok(client)
    }

    /// Fetch provider metadata, using the cache to avoid repeated HTTP calls.
    #[cfg(feature = "standalone")]
    async fn get_provider_metadata(&self, provider: &OAuthProvider) -> Result<CoreProviderMetadata, ServerError> {
        let cache_key = discovery_cache_key(&provider.name);

        // Try cache first
        if let Ok(Some(cached)) = self.cache.get_text(&cache_key).await {
            if let Ok(meta) = serde_json::from_str::<CoreProviderMetadata>(&cached) {
                return Ok(meta);
            }
        }

        let discovery_url_str = Self::discovery_url(provider)?;
        let http_client = Self::oidc_http_client()?;

        let metadata = if provider.kind == OAuthProviderKind::Cloudflare {
            // Cloudflare Access uses a non-standard discovery path
            let resp = http_client
                .get(&discovery_url_str)
                .send()
                .await
                .map_err(|e| ServerError::OAuthError(format!("Discovery fetch failed: {e}")))?;
            let body = resp
                .text()
                .await
                .map_err(|e| ServerError::OAuthError(format!("Discovery body read failed: {e}")))?;
            serde_json::from_str::<CoreProviderMetadata>(&body)
                .map_err(|e| ServerError::OAuthError(format!("Discovery parse failed: {e}")))?
        } else {
            let issuer = IssuerUrl::new(discovery_url_str)
                .map_err(|e| ServerError::OAuthError(format!("Invalid issuer URL: {e}")))?;
            CoreProviderMetadata::discover_async(issuer, &http_client)
                .await
                .map_err(|e| ServerError::OAuthError(format!("OIDC discovery failed: {e}")))?
        };

        // Cache the metadata
        if let Ok(serialized) = serde_json::to_string(&metadata) {
            let _ = self
                .cache
                .put(
                    &cache_key,
                    &serialized,
                    PutOptions::new().with_ttl(self.discovery_ttl()),
                )
                .await;
        }

        Ok(metadata)
    }

    // -------------------------------------------------------------------------
    // Authorization flow - step 1: build the authorization URL
    // -------------------------------------------------------------------------

    /// Generate the authorization URL that the user-agent should be redirected
    /// to. The CSRF state, PKCE verifier and nonce are stored in the cache.
    #[cfg(feature = "standalone")]
    pub async fn get_authorization_url(&self, provider_name: &str, redirect_uri: &str) -> Result<String, ServerError> {
        let provider = self
            .providers
            .get(provider_name)
            .ok_or_else(|| ServerError::OAuthError(format!("Unknown provider: {provider_name}")))?;

        let client = self.build_client(provider, redirect_uri).await?;

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let mut auth_req = client.authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        );

        for scope in &provider.scopes {
            auth_req = auth_req.add_scope(Scope::new(scope.clone()));
        }

        auth_req = auth_req.set_pkce_challenge(pkce_challenge);

        let (auth_url, csrf_state, nonce) = auth_req.url();

        let state_data = OAuthStateData {
            provider: provider_name.to_string(),
            nonce: nonce.secret().clone(),
            pkce_verifier: pkce_verifier.secret().clone(),
            redirect_uri: redirect_uri.to_string(),
        };
        let state_json =
            serde_json::to_string(&state_data).map_err(|e| ServerError::Internal(format!("serialize state: {e}")))?;

        self.cache
            .put(
                &state_cache_key(csrf_state.secret()),
                &state_json,
                PutOptions::new().with_ttl(self.state_ttl()),
            )
            .await
            .map_err(|e| ServerError::Internal(format!("cache put state: {e}")))?;

        Ok(auth_url.to_string())
    }

    #[cfg(not(feature = "standalone"))]
    pub async fn get_authorization_url(
        &self,
        _provider_name: &str,
        _redirect_uri: &str,
    ) -> Result<String, ServerError> {
        Err(ServerError::OAuthError(
            "Authorization URL generation not yet implemented for Cloudflare Workers".into(),
        ))
    }

    // -------------------------------------------------------------------------
    // Authorization flow - step 2: handle the callback
    // -------------------------------------------------------------------------

    /// Handle the authorization callback: validate the CSRF state, exchange the
    /// code for tokens, verify the ID token (including nonce), and return the
    /// authenticated user's information together with the raw token response.
    #[cfg(feature = "standalone")]
    pub async fn handle_callback(&self, code: &str, state: &str) -> Result<(UserInfo, TokenResponse), ServerError> {
        // 1. Retrieve & remove state from cache (one-time use)
        let state_json = self
            .cache
            .get_text(&state_cache_key(state))
            .await
            .map_err(|e| ServerError::Internal(format!("cache get state: {e}")))?
            .ok_or_else(|| ServerError::OAuthError("Invalid or expired state parameter".into()))?;

        let _ = self.cache.delete(&state_cache_key(state)).await;

        let state_data: OAuthStateData = serde_json::from_str(&state_json)
            .map_err(|e| ServerError::OAuthError(format!("Corrupt state data: {e}")))?;

        // 2. Look up provider
        let provider = self
            .providers
            .get(&state_data.provider)
            .ok_or_else(|| ServerError::OAuthError(format!("Unknown provider in state: {}", state_data.provider)))?;

        // 3. Build client & HTTP transport
        let client = self.build_client(provider, &state_data.redirect_uri).await?;
        let http_client = Self::oidc_http_client()?;

        // 4. Exchange code for tokens (with PKCE verifier).
        //    exchange_code returns Result when the token endpoint comes from
        //    discovery (EndpointMaybeSet).
        let pkce_verifier = PkceCodeVerifier::new(state_data.pkce_verifier);
        let token_response = client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .map_err(|e| ServerError::OAuthError(format!("No token endpoint: {e}")))?
            .set_pkce_verifier(pkce_verifier)
            .request_async(&http_client)
            .await
            .map_err(|e| ServerError::OAuthError(format!("Token exchange failed: {e}")))?;

        // 5. Extract & verify the ID token
        let id_token = token_response
            .id_token()
            .ok_or_else(|| ServerError::OAuthError("Provider did not return an ID token".into()))?;

        let nonce = Nonce::new(state_data.nonce);
        let id_token_verifier: CoreIdTokenVerifier<'_> = client.id_token_verifier();
        let claims: &CoreIdTokenClaims = id_token
            .claims(&id_token_verifier, &nonce)
            .map_err(|e| ServerError::OAuthError(format!("ID token verification failed: {e}")))?;

        // 6. Build UserInfo from ID token claims
        let user_info = Self::user_info_from_claims(claims);

        // 7. Build our TokenResponse
        let expires_in = token_response.expires_in().map(|d| d.as_secs() as i64).unwrap_or(3600);

        let token_resp = TokenResponse {
            access_token: token_response.access_token().secret().clone(),
            token_type: "Bearer".to_string(),
            expires_in,
            refresh_token: token_response.refresh_token().map(|t| t.secret().clone()),
            scope: token_response
                .scopes()
                .map(|s| s.iter().map(|scope| scope.to_string()).collect::<Vec<_>>().join(" ")),
            id_token: Some(id_token.to_string()),
        };

        // 8. Cache user info keyed by the access token
        if let Ok(ui_json) = serde_json::to_string(&user_info) {
            let _ = self
                .cache
                .put(
                    &format!("oauth:userinfo:{}", token_response.access_token().secret()),
                    &ui_json,
                    PutOptions::new().with_ttl(expires_in.max(0) as u64),
                )
                .await;
        }

        Ok((user_info, token_resp))
    }

    #[cfg(not(feature = "standalone"))]
    pub async fn handle_callback(&self, _code: &str, _state: &str) -> Result<(UserInfo, TokenResponse), ServerError> {
        Err(ServerError::OAuthError(
            "OAuth callback not yet implemented for Cloudflare Workers".into(),
        ))
    }

    // -------------------------------------------------------------------------
    // User info helpers
    // -------------------------------------------------------------------------

    /// Extract [`UserInfo`] from verified ID token claims.
    fn user_info_from_claims(claims: &CoreIdTokenClaims) -> UserInfo {
        UserInfo {
            sub: claims.subject().to_string(),
            name: claims.name().and_then(|n| n.get(None)).map(|n| n.as_str().to_owned()),
            email: claims.email().map(|e| e.as_str().to_owned()),
            email_verified: claims.email_verified(),
            picture: claims
                .picture()
                .and_then(|p| p.get(None))
                .map(|p| p.as_str().to_owned()),
        }
    }

    /// Retrieve cached user info for a given access token.
    pub async fn get_cached_user_info(&self, access_token: &str) -> Result<Option<UserInfo>, ServerError> {
        let key = format!("oauth:userinfo:{access_token}");
        match self.cache.get_text(&key).await {
            Ok(Some(json)) => {
                let info: UserInfo = serde_json::from_str(&json)
                    .map_err(|e| ServerError::Internal(format!("deserialize user info: {e}")))?;
                Ok(Some(info))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(ServerError::Internal(format!("cache get userinfo: {e}"))),
        }
    }

    /// Fetch user info from the provider's userinfo endpoint (standalone only).
    #[cfg(feature = "standalone")]
    pub async fn get_external_user_info(
        &self,
        provider_name: &str,
        access_token: &str,
    ) -> Result<UserInfo, ServerError> {
        if let Some(cached) = self.get_cached_user_info(access_token).await? {
            return Ok(cached);
        }

        let provider = self
            .providers
            .get(provider_name)
            .ok_or_else(|| ServerError::OAuthError(format!("Unknown provider: {provider_name}")))?;

        let client = self.build_client(provider, "https://unused").await?;
        let http_client = Self::oidc_http_client()?;

        let userinfo_claims: openidconnect::UserInfoClaims<openidconnect::EmptyAdditionalClaims, CoreGenderClaim> =
            client
                .user_info(openidconnect::AccessToken::new(access_token.to_string()), None)
                .map_err(|e| ServerError::OAuthError(format!("Userinfo not supported: {e}")))?
                .request_async(&http_client)
                .await
                .map_err(|e| ServerError::OAuthError(format!("Userinfo request failed: {e}")))?;

        let info = UserInfo {
            sub: userinfo_claims.subject().to_string(),
            name: userinfo_claims
                .name()
                .and_then(|n| n.get(None))
                .map(|n| n.as_str().to_owned()),
            email: userinfo_claims.email().map(|e| e.as_str().to_owned()),
            email_verified: userinfo_claims.email_verified(),
            picture: userinfo_claims
                .picture()
                .and_then(|p| p.get(None))
                .map(|p| p.as_str().to_owned()),
        };

        if let Ok(ui_json) = serde_json::to_string(&info) {
            let _ = self
                .cache
                .put(
                    &format!("oauth:userinfo:{access_token}"),
                    &ui_json,
                    PutOptions::new().with_ttl(300),
                )
                .await;
        }

        Ok(info)
    }

    #[cfg(not(feature = "standalone"))]
    pub async fn get_external_user_info(
        &self,
        _provider_name: &str,
        _access_token: &str,
    ) -> Result<UserInfo, ServerError> {
        Err(ServerError::OAuthError(
            "External user info fetch not yet implemented for Cloudflare Workers".into(),
        ))
    }

    // -------------------------------------------------------------------------
    // OIDC discovery document (for this server acting as an IdP)
    // -------------------------------------------------------------------------

    /// Return the OpenID Connect discovery document for this server when it
    /// acts as its own OIDC provider.
    pub fn get_openid_configuration(&self, issuer: &str) -> serde_json::Value {
        serde_json::json!({
            "issuer": issuer,
            "authorization_endpoint": format!("{issuer}/oauth/authorize"),
            "token_endpoint": format!("{issuer}/oauth/token"),
            "userinfo_endpoint": format!("{issuer}/oauth/userinfo"),
            "revocation_endpoint": format!("{issuer}/oauth/revoke"),
            "introspection_endpoint": format!("{issuer}/oauth/introspect"),
            "jwks_uri": format!("{issuer}/.well-known/jwks.json"),
            "response_types_supported": ["code", "id_token", "code id_token"],
            "subject_types_supported": ["public"],
            "id_token_signing_alg_values_supported": ["RS256"],
            "scopes_supported": ["openid", "profile", "email", "offline_access"],
            "token_endpoint_auth_methods_supported": ["client_secret_basic", "client_secret_post"],
            "claims_supported": ["sub", "name", "email", "email_verified", "picture", "locale", "updated_at"],
            "code_challenge_methods_supported": ["S256"],
            "grant_types_supported": ["authorization_code", "refresh_token"]
        })
    }

    // -------------------------------------------------------------------------
    // Provider listing (useful for UI)
    // -------------------------------------------------------------------------

    /// Return the list of configured provider names.
    pub fn provider_names(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }
}
