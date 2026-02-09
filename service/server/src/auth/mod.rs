// JWT authentication module - now split into separate types
mod claims;
mod jwt_manager;
mod token_pair;
mod token_type;

// Legacy module for backward compatibility
pub mod jwt;

pub mod oauth;
pub mod passkey;
pub mod session;

// Re-export JWT types from their individual modules
pub use claims::Claims;
pub use jwt_manager::JwtManager;
pub use token_pair::TokenPair;
pub use token_type::TokenType;

// Re-export other auth managers
pub use oauth::OAuthManager;
pub use session::SessionManager;
