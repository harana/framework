//! Route definitions

use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers;
use crate::state::AppState;

/// General application routes
pub fn general_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::general::index))
        .route("/health", get(handlers::general::health))
        .route("/about", get(handlers::general::about))
        .route("/api/protected", get(handlers::general::protected))
        .route("/api/maybe-protected", get(handlers::general::maybe_protected))
}

/// Authentication routes
pub fn auth_routes() -> Router<AppState> {
    Router::new()
        // Passkey/WebAuthn routes
        .route(
            "/auth/passkey/register/options",
            post(handlers::auth::passkey_register_options),
        )
        .route(
            "/auth/passkey/register/verify",
            post(handlers::auth::passkey_register_verify),
        )
        .route(
            "/auth/passkey/login/options",
            post(handlers::auth::passkey_login_options),
        )
        .route("/auth/passkey/login/verify", post(handlers::auth::passkey_login_verify))
        // OAuth 2.0 routes
        .route("/oauth/authorize", get(handlers::auth::oauth_authorize))
        .route("/oauth/token", post(handlers::auth::oauth_token))
        .route("/oauth/revoke", post(handlers::auth::oauth_revoke))
        .route("/oauth/introspect", post(handlers::auth::oauth_introspect))
        .route("/oauth/userinfo", get(handlers::auth::oauth_userinfo))
        // OpenID Connect routes
        .route(
            "/.well-known/openid-configuration",
            get(handlers::auth::openid_configuration),
        )
        .route("/auth/openid/login", get(handlers::auth::openid_login))
        .route("/auth/openid/callback", get(handlers::auth::openid_callback))
        .route("/auth/openid/logout", get(handlers::auth::openid_logout))
        // Session management routes
        .route("/auth/session", get(handlers::auth::session_info))
        .route("/auth/session/refresh", post(handlers::auth::session_refresh))
        .route("/auth/logout", post(handlers::auth::logout))
        // Token management
        .route("/auth/token/refresh", post(handlers::auth::refresh_tokens))
}

/// Event/SSE routes (standalone only — requires tokio + event bus)
#[cfg(feature = "standalone")]
pub fn event_routes() -> Router<AppState> {
    use axum::routing::delete;

    Router::new()
        .route("/api/event", get(handlers::event::event_stream))
        .route("/api/event/notification", get(handlers::event::notifications))
        .route("/api/event/publish", post(handlers::event::publish_event))
        .route("/api/event/channel", post(handlers::event::create_channel))
        .route("/api/event/channel", get(handlers::event::list_channels))
        .route("/api/event/channel/delete", post(handlers::event::delete_channel))
        .route("/api/event/channel/stream", get(handlers::event::channel_stream))
        .route("/api/event/channel/share", post(handlers::event::share_channel))
        .route("/api/event/channel/unshare", delete(handlers::event::unshare_channel))
}

/// No-op event routes for Cloudflare Workers (SSE not supported in workers the same way)
#[cfg(not(feature = "standalone"))]
pub fn event_routes() -> Router<AppState> {
    Router::new()
}
