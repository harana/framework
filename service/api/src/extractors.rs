//! Request extractors for authentication

use crate::auth::jwt::Claims;
use crate::auth::session::Session;
use crate::error::ServerError;
use crate::state::AppState;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts},
};

/// Extractor for authenticated user from JWT token
#[derive(Debug, Clone)]
pub struct AuthUser {
        pub claims: Claims,
}

impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        // Extract token from Authorization header
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(ServerError::Unauthorized)?;

        // Check for Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(ServerError::Unauthorized)?;

        // Validate token
        let claims = app_state.jwt.validate_access_token(token)?;

        Ok(AuthUser { claims })
    }
}

/// Optional authenticated user extractor
#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<AuthUser>);

impl<S> FromRequestParts<S> for OptionalAuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match AuthUser::from_request_parts(parts, state).await {
            Ok(user) => Ok(OptionalAuthUser(Some(user))),
            Err(_) => Ok(OptionalAuthUser(None)),
        }
    }
}

/// Extractor for session-based authentication
#[derive(Debug, Clone)]
pub struct AuthSession {
        pub session: Session,
}

impl<S> FromRequestParts<S> for AuthSession
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let cookie_name = app_state.sessions.cookie_name();

        // Extract session ID from cookie
        let cookies = parts
            .headers
            .get(header::COOKIE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");

        let session_id = cookies
            .split(';')
            .find_map(|cookie| {
                let cookie = cookie.trim();
                if cookie.starts_with(cookie_name) {
                    cookie.split('=').nth(1).map(|v| v.trim())
                } else {
                    None
                }
            })
            .ok_or(ServerError::SessionNotFound)?;

        // Get session
        let session = app_state.sessions.get_session(session_id)?;

        // Touch session to update last activity
        let _ = app_state.sessions.touch_session(session_id);

        Ok(AuthSession { session })
    }
}

/// Optional session extractor
#[derive(Debug, Clone)]
pub struct OptionalAuthSession(pub Option<AuthSession>);

impl<S> FromRequestParts<S> for OptionalAuthSession
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match AuthSession::from_request_parts(parts, state).await {
            Ok(session) => Ok(OptionalAuthSession(Some(session))),
            Err(_) => Ok(OptionalAuthSession(None)),
        }
    }
}

/// Extractor that accepts either JWT or Session authentication
#[derive(Debug, Clone)]
pub struct Auth {
        pub user_id: String,
        pub email: Option<String>,
        pub roles: Vec<String>,
        pub source: AuthSource,
}

/// Source of authentication
#[derive(Debug, Clone)]
pub enum AuthSource {
    /// JWT token
    Jwt(Claims),
    /// Session
    Session(Session),
}

impl<S> FromRequestParts<S> for Auth
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Try JWT first
        if let Ok(auth_user) = AuthUser::from_request_parts(parts, state).await {
            return Ok(Auth {
                user_id: auth_user.claims.sub.clone(),
                email: auth_user.claims.email.clone(),
                roles: auth_user.claims.roles.clone(),
                source: AuthSource::Jwt(auth_user.claims),
            });
        }

        // Fall back to session
        if let Ok(auth_session) = AuthSession::from_request_parts(parts, state).await {
            return Ok(Auth {
                user_id: auth_session.session.user_id.clone(),
                email: auth_session.session.email.clone(),
                roles: auth_session.session.roles.clone(),
                source: AuthSource::Session(auth_session.session),
            });
        }

        Err(ServerError::Unauthorized)
    }
}

/// Optional auth extractor
#[derive(Debug, Clone)]
pub struct OptionalAuth(pub Option<Auth>);

impl<S> FromRequestParts<S> for OptionalAuth
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Auth::from_request_parts(parts, state).await {
            Ok(auth) => Ok(OptionalAuth(Some(auth))),
            Err(_) => Ok(OptionalAuth(None)),
        }
    }
}
