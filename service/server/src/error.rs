//! Server error types

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Authentication required")]
    Unauthorized,

    #[error("Access forbidden")]
    Forbidden,

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Session not found or expired")]
    SessionNotFound,

    #[error("Passkey error: {0}")]
    PasskeyError(String),

    #[error("OAuth error: {0}")]
    OAuthError(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match &self {
            ServerError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized", self.to_string()),
            ServerError::Forbidden => (StatusCode::FORBIDDEN, "forbidden", self.to_string()),
            ServerError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            ServerError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.clone()),
            ServerError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid_credentials", self.to_string()),
            ServerError::TokenExpired => (StatusCode::UNAUTHORIZED, "token_expired", self.to_string()),
            ServerError::InvalidToken(msg) => (StatusCode::UNAUTHORIZED, "invalid_token", msg.clone()),
            ServerError::SessionNotFound => (StatusCode::UNAUTHORIZED, "session_not_found", self.to_string()),
            ServerError::PasskeyError(msg) => (StatusCode::BAD_REQUEST, "passkey_error", msg.clone()),
            ServerError::OAuthError(msg) => (StatusCode::BAD_REQUEST, "oauth_error", msg.clone()),
            ServerError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "An internal error occurred".to_string(),
                )
            }
            ServerError::Database(msg) => {
                tracing::error!("Database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_error",
                    "A database error occurred".to_string(),
                )
            }
            ServerError::Validation(msg) => (StatusCode::BAD_REQUEST, "validation_error", msg.clone()),
        };

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": message,
            }
        }));

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for ServerError {
    fn from(err: anyhow::Error) -> Self {
        ServerError::Internal(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for ServerError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => ServerError::TokenExpired,
            _ => ServerError::InvalidToken(err.to_string()),
        }
    }
}
