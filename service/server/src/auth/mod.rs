//! Authentication module
//!
//! Provides JWT, OAuth 2.0, and Passkey authentication support

pub mod jwt;
pub mod oauth;
pub mod passkey;
pub mod session;

pub use jwt::JwtManager;
pub use oauth::OAuthManager;
pub use session::SessionManager;
