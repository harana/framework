mod config;

pub use config::ServerConfig;

use {
    asynk_strim::{Yielder, stream_fn},
    axum::{
        Router,
        response::{Html, IntoResponse},
        routing::{get, post},
    },
    core::time::Duration,
    datastar::prelude::{MergeFragments, ReadSignals, Sse},
    serde::Deserialize,
    tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

/// HTTP Server for handling web requests
pub struct HttpServer {
    config: ServerConfig,
    router: Router,
}

impl HttpServer {
    pub fn new(config: ServerConfig) -> Self {
        let router = Router::new()
            // General routes
            .route("/", get(index))
            .route("/health", get(health))
            .route("/about", get(about))
            // OpenID Connect routes
            .route("/auth/openid/login", get(openid_login))
            .route("/auth/openid/callback", get(openid_callback))
            .route("/auth/openid/logout", get(openid_logout))
            .route("/.well-known/openid-configuration", get(openid_configuration))
            // OAuth 2.0 routes
            .route("/oauth/authorize", get(oauth_authorize))
            .route("/oauth/token", post(oauth_token))
            .route("/oauth/revoke", post(oauth_revoke))
            .route("/oauth/introspect", post(oauth_introspect))
            .route("/oauth/userinfo", get(oauth_userinfo))
            // Passkey/WebAuthn routes
            .route("/auth/passkey/register/options", post(passkey_register_options))
            .route("/auth/passkey/register/verify", post(passkey_register_verify))
            .route("/auth/passkey/login/options", post(passkey_login_options))
            .route("/auth/passkey/login/verify", post(passkey_login_verify))
            // Session management
            .route("/auth/session", get(session_info))
            .route("/auth/session/refresh", post(session_refresh))
            .route("/auth/logout", post(logout))
            // SSE/Events
            .route("/events", get(events));
        Self { config, router }
    }

    /// Initialize tracing/logging
    pub fn init_tracing() {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "harana_core=debug,tower_http=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    /// Start the HTTP server
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = tokio::net::TcpListener::bind(self.config.address()).await?;

        tracing::debug!("listening on {}", listener.local_addr()?);

        axum::serve(listener, self.router).await?;

        Ok(())
    }

    /// Get the server's address
    pub fn address(&self) -> String {
        self.config.address()
    }
}

/// Default index route handler
async fn index() -> Html<&'static str> {
    Html(include_str!("../assets/hello-world.html"))
}

const MESSAGE: &str = "Hello, world!";

/// Signals received from the client
#[derive(Deserialize)]
pub struct Signals {
    pub delay: u64,
}

/// Hello world SSE endpoint with datastar integration
async fn hello_world(ReadSignals(signals): ReadSignals<Signals>) -> impl IntoResponse {
    Sse(stream_fn(move |mut yielder: Yielder<MergeFragments>| async move {
        for i in 0..MESSAGE.len() {
            let elements = format!("<div id='message'>{}</div>", &MESSAGE[0..i + 1]);
            let fragment = MergeFragments::new(elements);

            yielder.yield_item(fragment).await;

            tokio::time::sleep(Duration::from_millis(signals.delay)).await;
        }
    }))
}

// =============================================================================
// General Routes
// =============================================================================

/// Health check endpoint
async fn health() -> impl IntoResponse {
    axum::Json(serde_json::json!({ "status": "ok" }))
}

/// About page
async fn about() -> impl IntoResponse {
    axum::Json(serde_json::json!({ "name": "Harana", "version": "0.1.0" }))
}

/// Server-sent events endpoint
async fn events(ReadSignals(signals): ReadSignals<Signals>) -> impl IntoResponse {
    hello_world(ReadSignals(signals)).await
}

// =============================================================================
// OpenID Connect Routes
// =============================================================================

/// Initiate OpenID Connect login flow
async fn openid_login() -> impl IntoResponse {
    // TODO: Implement OpenID login - redirect to identity provider
    axum::Json(serde_json::json!({ "error": "not_implemented", "message": "OpenID login not yet implemented" }))
}

/// OpenID Connect callback after authentication
async fn openid_callback() -> impl IntoResponse {
    // TODO: Implement OpenID callback - exchange code for tokens
    axum::Json(serde_json::json!({ "error": "not_implemented", "message": "OpenID callback not yet implemented" }))
}

/// OpenID Connect logout
async fn openid_logout() -> impl IntoResponse {
    // TODO: Implement OpenID logout - revoke tokens and redirect
    axum::Json(serde_json::json!({ "error": "not_implemented", "message": "OpenID logout not yet implemented" }))
}

/// OpenID Connect discovery document
async fn openid_configuration() -> impl IntoResponse {
    // TODO: Return proper OpenID configuration
    axum::Json(serde_json::json!({
        "issuer": "https://example.com",
        "authorization_endpoint": "/oauth/authorize",
        "token_endpoint": "/oauth/token",
        "userinfo_endpoint": "/oauth/userinfo",
        "revocation_endpoint": "/oauth/revoke",
        "introspection_endpoint": "/oauth/introspect",
        "jwks_uri": "/.well-known/jwks.json",
        "response_types_supported": ["code", "token", "id_token"],
        "subject_types_supported": ["public"],
        "id_token_signing_alg_values_supported": ["RS256"],
        "scopes_supported": ["openid", "profile", "email"],
        "token_endpoint_auth_methods_supported": ["client_secret_basic", "client_secret_post"],
        "claims_supported": ["sub", "name", "email", "email_verified"]
    }))
}

// =============================================================================
// OAuth 2.0 Routes
// =============================================================================

/// OAuth 2.0 authorization endpoint
async fn oauth_authorize() -> impl IntoResponse {
    // TODO: Implement OAuth authorization - show consent screen
    axum::Json(serde_json::json!({ "error": "not_implemented", "message": "OAuth authorize not yet implemented" }))
}

/// OAuth 2.0 token endpoint
async fn oauth_token() -> impl IntoResponse {
    // TODO: Implement OAuth token - exchange code/credentials for tokens
    axum::Json(serde_json::json!({ "error": "not_implemented", "message": "OAuth token not yet implemented" }))
}

/// OAuth 2.0 token revocation endpoint
async fn oauth_revoke() -> impl IntoResponse {
    // TODO: Implement OAuth revoke - invalidate access/refresh tokens
    axum::Json(serde_json::json!({ "active": false }))
}

/// OAuth 2.0 token introspection endpoint
async fn oauth_introspect() -> impl IntoResponse {
    // TODO: Implement OAuth introspect - validate and return token info
    axum::Json(serde_json::json!({ "active": false }))
}

/// OAuth 2.0 userinfo endpoint
async fn oauth_userinfo() -> impl IntoResponse {
    // TODO: Implement OAuth userinfo - return authenticated user details
    axum::Json(serde_json::json!({ "error": "not_implemented", "message": "OAuth userinfo not yet implemented" }))
}

// =============================================================================
// Passkey/WebAuthn Routes
// =============================================================================

/// Get options for passkey registration
async fn passkey_register_options() -> impl IntoResponse {
    // TODO: Implement passkey registration options - return PublicKeyCredentialCreationOptions
    axum::Json(
        serde_json::json!({ "error": "not_implemented", "message": "Passkey registration options not yet implemented" }),
    )
}

/// Verify passkey registration
async fn passkey_register_verify() -> impl IntoResponse {
    // TODO: Implement passkey registration verification - validate and store credential
    axum::Json(
        serde_json::json!({ "error": "not_implemented", "message": "Passkey registration verify not yet implemented" }),
    )
}

/// Get options for passkey login
async fn passkey_login_options() -> impl IntoResponse {
    // TODO: Implement passkey login options - return PublicKeyCredentialRequestOptions
    axum::Json(
        serde_json::json!({ "error": "not_implemented", "message": "Passkey login options not yet implemented" }),
    )
}

/// Verify passkey login
async fn passkey_login_verify() -> impl IntoResponse {
    // TODO: Implement passkey login verification - validate assertion and create session
    axum::Json(serde_json::json!({ "error": "not_implemented", "message": "Passkey login verify not yet implemented" }))
}

// =============================================================================
// Session Management Routes
// =============================================================================

/// Get current session info
async fn session_info() -> impl IntoResponse {
    // TODO: Implement session info - return current user session details
    axum::Json(serde_json::json!({ "authenticated": false }))
}

/// Refresh session/tokens
async fn session_refresh() -> impl IntoResponse {
    // TODO: Implement session refresh - extend session or refresh tokens
    axum::Json(serde_json::json!({ "error": "not_implemented", "message": "Session refresh not yet implemented" }))
}

/// Logout and invalidate session
async fn logout() -> impl IntoResponse {
    // TODO: Implement logout - clear session and revoke tokens
    axum::Json(serde_json::json!({ "success": true }))
}
