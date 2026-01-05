use std::collections::HashMap;

use super::client::AsyncResult;
use super::error::{PluginError, PluginErrorKind, Result};
use crate::models::event::Event;

/// HTTP method for route definitions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options,
    Head,
}

/// Route definition for auth plugins
#[derive(Debug, Clone)]
pub struct Route {
    pub path: String,
    pub method: HttpMethod,
    pub name: String,
    pub description: Option<String>,
    pub requires_auth: bool,
}

impl Route {
    pub fn new(path: impl Into<String>, method: HttpMethod, name: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            method,
            name: name.into(),
            description: None,
            requires_auth: false,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn requires_auth(mut self) -> Self {
        self.requires_auth = true;
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct AuthUser {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub email_verified: bool,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub id_token: Option<String>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AuthSession {
    pub session_id: String,
    pub user: AuthUser,
    pub token: Option<AuthToken>,
    pub created_at: u64,
    pub expires_at: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct AuthChallenge {
    pub challenge_id: String,
    pub challenge: String,
    pub timeout: u64,
    pub parameters: HashMap<String, String>,
}

/// Supported authentication features
pub struct AuthFeature;

impl AuthFeature {
    pub const TOKEN_REFRESH: &'static str = "token_refresh";
    pub const TOKEN_REVOCATION: &'static str = "token_revocation";
    pub const REGISTRATION: &'static str = "registration";
    pub const PASSWORDLESS: &'static str = "passwordless";
    pub const MFA: &'static str = "mfa";
    pub const SSO: &'static str = "sso";
    pub const OAUTH2: &'static str = "oauth2";
    pub const OPENID_CONNECT: &'static str = "openid_connect";
    pub const WEBAUTHN: &'static str = "webauthn";
}

/// Generic authentication provider trait supporting OpenID, Passkey, and other auth methods.
///
/// Implementations should provide the required methods and optionally override
/// the default implementations for features they support.
pub trait Auth: Send + Sync {
    type Config;

    // ============ Required Methods ============

    /// Create a new auth provider instance with the given configuration.
    fn new(config: Self::Config) -> Result<Self>
    where
        Self: Sized;

    /// Unique identifier for this auth provider (e.g., "openid", "passkey").
    fn provider_id(&self) -> &str;

    /// Human-readable name for this auth provider.
    fn provider_name(&self) -> &str;

    /// Get the provider configuration.
    fn config(&self) -> &Self::Config;

    /// Validate the provider configuration.
    fn validate_config(&self) -> Result<()>;

    /// Begin the login/authentication flow.
    ///
    /// For OpenID: Returns authorization URL and state.
    /// For Passkey: Returns WebAuthn challenge options.
    fn begin_login(&self, redirect_uri: &str) -> AsyncResult<'_, AuthChallenge>;

    /// Complete the login/authentication flow.
    ///
    /// For OpenID: Exchanges authorization code for tokens.
    /// For Passkey: Verifies the WebAuthn assertion.
    fn complete_login(&self, challenge_id: &str, response: &str) -> AsyncResult<'_, AuthSession>;

    /// End the user session.
    fn logout(&self, session_id: &str) -> AsyncResult<'_, ()>;

    /// List of features supported by this auth provider.
    fn supported_features(&self) -> Vec<&str>;

    /// Check if a specific feature is supported.
    fn supports_feature(&self, feature: &str) -> bool {
        self.supported_features().contains(&feature)
    }

    /// Get the routes that this auth plugin provides.
    fn routes(&self) -> Vec<Route>;

    /// Get the events that this auth plugin can emit.
    fn events(&self) -> Vec<Event>;

    // ============ Optional Methods (Registration) ============

    /// Begin the registration flow (optional - not all providers support registration).
    ///
    /// For OpenID: May redirect to provider's registration page.
    /// For Passkey: Returns WebAuthn registration options.
    fn begin_register(&self, _user_id: &str, _redirect_uri: &str) -> AsyncResult<'_, AuthChallenge> {
        Box::pin(async {
            Err(Box::new(PluginError::new(
                PluginErrorKind::Unsupported,
                "Registration not supported",
            )))
        })
    }

    /// Complete the registration flow (optional).
    fn complete_register(&self, _challenge_id: &str, _response: &str) -> AsyncResult<'_, AuthUser> {
        Box::pin(async {
            Err(Box::new(PluginError::new(
                PluginErrorKind::Unsupported,
                "Registration not supported",
            )))
        })
    }

    // ============ Optional Methods (Token Management) ============

    /// Refresh an access token using a refresh token (optional - token-based auth only).
    fn refresh_token(&self, _refresh_token: &str) -> AsyncResult<'_, AuthToken> {
        Box::pin(async {
            Err(Box::new(PluginError::new(
                PluginErrorKind::Unsupported,
                "Token refresh not supported",
            )))
        })
    }

    /// Validate a token and return the associated user (optional).
    fn validate_token(&self, _token: &str) -> AsyncResult<'_, AuthUser> {
        Box::pin(async {
            Err(Box::new(PluginError::new(
                PluginErrorKind::Unsupported,
                "Token validation not supported",
            )))
        })
    }

    /// Revoke a token (optional).
    fn revoke_token(&self, _token: &str) -> AsyncResult<'_, ()> {
        Box::pin(async {
            Err(Box::new(PluginError::new(
                PluginErrorKind::Unsupported,
                "Token revocation not supported",
            )))
        })
    }

    // ============ Optional Methods (Session Management) ============

    /// Get user info from an active session (optional).
    fn get_user_info(&self, _session_id: &str) -> AsyncResult<'_, AuthUser> {
        Box::pin(async {
            Err(Box::new(PluginError::new(
                PluginErrorKind::Unsupported,
                "User info retrieval not supported",
            )))
        })
    }

    /// Extend/refresh a session (optional).
    fn refresh_session(&self, _session_id: &str) -> AsyncResult<'_, AuthSession> {
        Box::pin(async {
            Err(Box::new(PluginError::new(
                PluginErrorKind::Unsupported,
                "Session refresh not supported",
            )))
        })
    }
}
