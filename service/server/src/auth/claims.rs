use serde::{Deserialize, Serialize};
use crate::auth::token_type::TokenType;

/// JWT claims structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub iss: String,
    pub aud: String,
    pub token_type: TokenType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default)]
    pub roles: Vec<String>,
    #[serde(flatten)]
    pub custom: std::collections::HashMap<String, serde_json::Value>,
}
