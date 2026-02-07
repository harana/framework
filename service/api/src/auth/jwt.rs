//! JWT token management

use crate::config::JwtConfig;
use crate::error::ServerError;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
        pub sub: String,
        pub iat: i64,
        pub exp: i64,
        pub iss: String,
        pub aud: String,
        pub token_type: TokenType,
    /// User email (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User roles
    #[serde(default)]
    pub roles: Vec<String>,
    /// Custom claims
    #[serde(flatten)]
    pub custom: std::collections::HashMap<String, serde_json::Value>,
}

/// Token type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    /// Access token
    Access,
    /// Refresh token
    Refresh,
}

/// Token pair response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
        pub access_token: String,
        pub refresh_token: String,
        pub token_type: String,
        pub expires_in: i64,
}

/// JWT Manager for creating and validating tokens
pub struct JwtManager {
    config: JwtConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtManager {
    /// Create a new JWT manager
    pub fn new(config: JwtConfig) -> Self {
        let encoding_key = EncodingKey::from_secret(config.secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(config.secret.as_bytes());

        Self {
            config,
            encoding_key,
            decoding_key,
        }
    }

    /// Create a token pair for a user
    pub fn create_token_pair(
        &self,
        user_id: &str,
        email: Option<&str>,
        roles: Vec<String>,
    ) -> Result<TokenPair, ServerError> {
        let access_token = self.create_access_token(user_id, email, roles.clone())?;
        let refresh_token = self.create_refresh_token(user_id)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.config.access_token_expiry,
        })
    }

    /// Create an access token
    pub fn create_access_token(
        &self,
        user_id: &str,
        email: Option<&str>,
        roles: Vec<String>,
    ) -> Result<String, ServerError> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.config.access_token_expiry);

        let claims = Claims {
            sub: user_id.to_string(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            token_type: TokenType::Access,
            email: email.map(String::from),
            roles,
            custom: std::collections::HashMap::new(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| ServerError::Internal(format!("Failed to create token: {}", e)))
    }

    /// Create a refresh token
    pub fn create_refresh_token(&self, user_id: &str) -> Result<String, ServerError> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.config.refresh_token_expiry);

        let claims = Claims {
            sub: user_id.to_string(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            token_type: TokenType::Refresh,
            email: None,
            roles: vec![],
            custom: std::collections::HashMap::new(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| ServerError::Internal(format!("Failed to create refresh token: {}", e)))
    }

    /// Validate and decode a token
    pub fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, ServerError> {
        let mut validation = Validation::default();
        validation.set_audience(&[&self.config.audience]);
        validation.set_issuer(&[&self.config.issuer]);

        decode::<Claims>(token, &self.decoding_key, &validation).map_err(ServerError::from)
    }

    /// Validate an access token specifically
    pub fn validate_access_token(&self, token: &str) -> Result<Claims, ServerError> {
        let token_data = self.validate_token(token)?;

        if token_data.claims.token_type != TokenType::Access {
            return Err(ServerError::InvalidToken("Expected access token".to_string()));
        }

        Ok(token_data.claims)
    }

    /// Validate a refresh token and create new token pair
    pub fn refresh_tokens(&self, refresh_token: &str) -> Result<TokenPair, ServerError> {
        let token_data = self.validate_token(refresh_token)?;

        if token_data.claims.token_type != TokenType::Refresh {
            return Err(ServerError::InvalidToken("Expected refresh token".to_string()));
        }

        // Create new token pair with same user info
        self.create_token_pair(
            &token_data.claims.sub,
            token_data.claims.email.as_deref(),
            token_data.claims.roles,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_validate_token() {
        let config = JwtConfig::default();
        let manager = JwtManager::new(config);

        let token_pair = manager
            .create_token_pair("user123", Some("test@example.com"), vec!["user".to_string()])
            .unwrap();

        let claims = manager.validate_access_token(&token_pair.access_token).unwrap();
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.email, Some("test@example.com".to_string()));
    }

    #[test]
    fn test_refresh_token() {
        let config = JwtConfig::default();
        let manager = JwtManager::new(config);

        let token_pair = manager.create_token_pair("user123", None, vec![]).unwrap();

        let new_pair = manager.refresh_tokens(&token_pair.refresh_token).unwrap();
        assert!(!new_pair.access_token.is_empty());
    }
}
