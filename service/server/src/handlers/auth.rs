use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, header},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use serde_json::json;

use crate::auth::{
    oauth::{AuthorizationRequest, IntrospectionResponse, TokenRequest, TokenResponse, UserInfo},
    passkey::{PasskeyLoginRequest, PasskeyLoginVerifyRequest, PasskeyRegisterRequest, PasskeyRegisterVerifyRequest},
    session::AuthMethod,
};
use crate::error::ServerError;
use crate::extractors::{Auth, AuthSession, OptionalAuthSession};
use crate::state::AppState;

// =============================================================================
// Passkey/WebAuthn Routes
// =============================================================================

/// Get passkey registration options
pub async fn passkey_register_options(
    State(state): State<AppState>,
    Json(request): Json<PasskeyRegisterRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let options = crate::auth::passkey::generate_register_options(&state.config.passkey, &request).await?;

    Ok(Json(json!({
        "challenge": options.challenge,
        "options": options.options
    })))
}

/// Verify passkey registration
pub async fn passkey_register_verify(
    State(state): State<AppState>,
    Json(request): Json<PasskeyRegisterVerifyRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let result = crate::auth::passkey::verify_register(&state.config.passkey, &request).await?;

    if result.verified {
        // Create session for the user
        let session = state
            .sessions
            .create_session(&request.user_id, None, vec!["user".to_string()], AuthMethod::Passkey)
            .await
            .map_err(|e| ServerError::Internal(format!("create session: {e}")))?;

        // Create JWT tokens
        let tokens = state
            .jwt
            .create_token_pair(&request.user_id, None, vec!["user".to_string()])?;

        // Set session cookie
        let cookie = format!(
            "{}={}; HttpOnly; Path=/; Max-Age={}; SameSite=Lax{}",
            state.sessions.cookie_name(),
            session.id,
            state.sessions.ttl(),
            if state.sessions.is_secure() { "; Secure" } else { "" }
        );

        let mut headers = HeaderMap::new();
        headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

        Ok((
            headers,
            Json(json!({
                "verified": true,
                "credential_id": result.credential_id,
                "session_id": session.id,
                "tokens": tokens
            })),
        ))
    } else {
        Ok((
            HeaderMap::new(),
            Json(json!({
                "verified": false,
                "error": "Registration verification failed"
            })),
        ))
    }
}

/// Get passkey login options
pub async fn passkey_login_options(
    State(state): State<AppState>,
    Json(request): Json<PasskeyLoginRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let options =
        crate::auth::passkey::generate_login_options(&state.config.passkey, request.user_id.as_deref()).await?;

    Ok(Json(json!({
        "challenge": options.challenge,
        "options": options.options
    })))
}

/// Verify passkey login
pub async fn passkey_login_verify(
    State(state): State<AppState>,
    Json(request): Json<PasskeyLoginVerifyRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let result = crate::auth::passkey::verify_login(&state.config.passkey, &request).await?;

    if result.verified {
        // Get user ID from credential (you'd look this up in your database)
        let user_id = &result.credential_id; // Placeholder - should look up actual user

        // Create session
        let session = state
            .sessions
            .create_session(user_id, None, vec!["user".to_string()], AuthMethod::Passkey)
            .await
            .map_err(|e| ServerError::Internal(format!("create session: {e}")))?;

        // Create JWT tokens
        let tokens = state.jwt.create_token_pair(user_id, None, vec!["user".to_string()])?;

        // Set session cookie
        let cookie = format!(
            "{}={}; HttpOnly; Path=/; Max-Age={}; SameSite=Lax{}",
            state.sessions.cookie_name(),
            session.id,
            state.sessions.ttl(),
            if state.sessions.is_secure() { "; Secure" } else { "" }
        );

        let mut headers = HeaderMap::new();
        headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

        Ok((
            headers,
            Json(json!({
                "verified": true,
                "user_id": user_id,
                "session_id": session.id,
                "tokens": tokens
            })),
        ))
    } else {
        Ok((
            HeaderMap::new(),
            Json(json!({
                "verified": false,
                "error": "Authentication failed"
            })),
        ))
    }
}

// =============================================================================
// OAuth 2.0 Routes
// =============================================================================

/// OAuth authorization endpoint
pub async fn oauth_authorize(
    State(_state): State<AppState>,
    Query(request): Query<AuthorizationRequest>,
) -> impl IntoResponse {
    // In a real implementation, you'd:
    // 1. Verify the client_id is registered
    // 2. Show a consent screen to the user
    // 3. After user consents, redirect with an authorization code

    // For now, return an error indicating this needs implementation
    Json(json!({
        "error": "consent_required",
        "message": "User consent required. In production, this would show a consent screen.",
        "client_id": request.client_id,
        "redirect_uri": request.redirect_uri,
        "scope": request.scope,
        "state": request.state
    }))
}

/// OAuth token endpoint
///
/// Currently supports `refresh_token` grant type only.  Authorization code
/// exchange is handled via the `/auth/openid/callback` OIDC flow which uses
/// the `openidconnect` crate directly.
pub async fn oauth_token(
    State(state): State<AppState>,
    Json(request): Json<TokenRequest>,
) -> Result<impl IntoResponse, ServerError> {
    match request.grant_type.as_str() {
        "refresh_token" => {
            let refresh_token = request
                .refresh_token
                .as_deref()
                .ok_or(ServerError::BadRequest("refresh_token required".to_string()))?;

            let tokens = state.jwt.refresh_tokens(refresh_token)?;

            Ok(Json(TokenResponse {
                access_token: tokens.access_token,
                token_type: "Bearer".to_string(),
                expires_in: tokens.expires_in,
                refresh_token: Some(tokens.refresh_token),
                scope: None,
                id_token: None,
            }))
        }
        _ => Err(ServerError::BadRequest(format!(
            "Unsupported grant_type: {}",
            request.grant_type
        ))),
    }
}

/// OAuth token revocation endpoint
pub async fn oauth_revoke(State(_state): State<AppState>) -> impl IntoResponse {
    // In production, you'd invalidate the token in a blocklist
    Json(json!({ "revoked": true }))
}

/// OAuth token introspection endpoint
pub async fn oauth_introspect(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ServerError> {
    // Extract token from request body or Authorization header
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    match token {
        Some(t) => match state.jwt.validate_access_token(t) {
            Ok(claims) => Ok(Json(IntrospectionResponse {
                active: true,
                scope: Some("openid profile email".to_string()),
                client_id: Some(claims.aud.clone()),
                username: claims.email.clone(),
                token_type: Some("Bearer".to_string()),
                exp: Some(claims.exp),
                iat: Some(claims.iat),
                sub: Some(claims.sub),
            })),
            Err(_) => Ok(Json(IntrospectionResponse {
                active: false,
                scope: None,
                client_id: None,
                username: None,
                token_type: None,
                exp: None,
                iat: None,
                sub: None,
            })),
        },
        None => Ok(Json(IntrospectionResponse {
            active: false,
            scope: None,
            client_id: None,
            username: None,
            token_type: None,
            exp: None,
            iat: None,
            sub: None,
        })),
    }
}

/// OAuth userinfo endpoint
pub async fn oauth_userinfo(auth: Auth) -> impl IntoResponse {
    Json(UserInfo {
        sub: auth.user_id,
        name: None,
        email: auth.email,
        email_verified: Some(true),
        picture: None,
    })
}

// =============================================================================
// OpenID Connect Routes
// =============================================================================

/// OpenID Connect discovery document
pub async fn openid_configuration(State(state): State<AppState>) -> impl IntoResponse {
    let issuer = format!("http://{}", state.config.address());
    Json(state.oauth.get_openid_configuration(&issuer))
}

#[derive(Debug, Deserialize)]
pub struct ExternalLoginQuery {
    pub provider: String,
    pub redirect_uri: Option<String>,
}

pub async fn openid_login(
    State(state): State<AppState>,
    Query(query): Query<ExternalLoginQuery>,
) -> Result<impl IntoResponse, ServerError> {
    let redirect_uri = query
        .redirect_uri
        .unwrap_or_else(|| format!("http://{}/auth/openid/callback", state.config.address()));

    let auth_url = state
        .oauth
        .get_authorization_url(&query.provider, &redirect_uri)
        .await?;

    Ok(Redirect::temporary(&auth_url))
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

pub async fn openid_callback(
    State(state): State<AppState>,
    Query(query): Query<OAuthCallbackQuery>,
) -> Result<impl IntoResponse, ServerError> {
    // Check for errors from the IdP
    if let Some(error) = query.error {
        return Err(ServerError::OAuthError(format!(
            "{}: {}",
            error,
            query.error_description.unwrap_or_default()
        )));
    }

    let code = query
        .code
        .ok_or(ServerError::OAuthError("Missing authorization code".to_string()))?;
    let state_param = query
        .state
        .ok_or(ServerError::OAuthError("Missing state parameter".to_string()))?;

    // The new OAuthManager handles state validation, code exchange, ID token
    // verification and user info extraction in a single call.
    let (user_info, _token_resp) = state.oauth.handle_callback(&code, &state_param).await?;

    // Create local session
    let session = state
        .sessions
        .create_session(
            &user_info.sub,
            user_info.email.as_deref(),
            vec!["user".to_string()],
            AuthMethod::Oidc,
        )
        .await
        .map_err(|e| ServerError::Internal(format!("create session: {e}")))?;

    // Create local JWT tokens
    let local_tokens =
        state
            .jwt
            .create_token_pair(&user_info.sub, user_info.email.as_deref(), vec!["user".to_string()])?;

    // Set session cookie
    let cookie = format!(
        "{}={}; HttpOnly; Path=/; Max-Age={}; SameSite=Lax{}",
        state.sessions.cookie_name(),
        session.id,
        state.sessions.ttl(),
        if state.sessions.is_secure() { "; Secure" } else { "" }
    );

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    Ok((
        headers,
        Json(json!({
            "success": true,
            "user": user_info,
            "session_id": session.id,
            "tokens": local_tokens
        })),
    ))
}

/// OpenID Connect logout
pub async fn openid_logout(
    State(state): State<AppState>,
    OptionalAuthSession(session): OptionalAuthSession,
) -> impl IntoResponse {
    // Destroy session if exists
    if let Some(auth_session) = session {
        state.sessions.destroy_session(&auth_session.session.id).await;
    }

    // Clear session cookie
    let cookie = format!(
        "{}=; HttpOnly; Path=/; Max-Age=0; SameSite=Lax",
        state.sessions.cookie_name()
    );

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    (headers, Json(json!({ "success": true })))
}

// =============================================================================
// Session Management Routes
// =============================================================================

/// Get current session info
pub async fn session_info(
    State(state): State<AppState>,
    OptionalAuthSession(session): OptionalAuthSession,
) -> impl IntoResponse {
    let session_id = session.as_ref().map(|s| s.session.id.as_str());
    Json(state.sessions.get_session_info(session_id).await)
}

/// Refresh session/tokens
pub async fn session_refresh(
    State(state): State<AppState>,
    auth: AuthSession,
) -> Result<impl IntoResponse, ServerError> {
    // Extend session
    let session = state.sessions.extend_session(&auth.session.id).await?;

    // Create new tokens
    let tokens = state
        .jwt
        .create_token_pair(&session.user_id, session.email.as_deref(), session.roles.clone())?;

    Ok(Json(json!({
        "success": true,
        "session": state.sessions.get_session_info(Some(&session.id)).await,
        "tokens": tokens
    })))
}

/// Logout and invalidate session
pub async fn logout(
    State(state): State<AppState>,
    OptionalAuthSession(session): OptionalAuthSession,
) -> impl IntoResponse {
    if let Some(auth_session) = session {
        state.sessions.destroy_session(&auth_session.session.id).await;
    }

    // Clear session cookie
    let cookie = format!(
        "{}=; HttpOnly; Path=/; Max-Age=0; SameSite=Lax",
        state.sessions.cookie_name()
    );

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    (headers, Json(json!({ "success": true })))
}

// =============================================================================
// Token Management Routes
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

pub async fn refresh_tokens(
    State(state): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let tokens = state.jwt.refresh_tokens(&request.refresh_token)?;
    Ok(Json(tokens))
}
