use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub jwt: JwtConfig,
    pub oauth: OAuthConfig,
    pub passkey: PasskeyConfig,
    pub session: SessionConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            jwt: JwtConfig::default(),
            oauth: OAuthConfig::default(),
            passkey: PasskeyConfig::default(),
            session: SessionConfig::default(),
        }
    }
}

impl ServerConfig {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            ..Default::default()
        }
    }

    /// Get the full address string
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// JWT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub access_token_expiry: i64,
    pub refresh_token_expiry: i64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "change-me-in-production-use-env-variable".to_string(),
            issuer: "harana".to_string(),
            audience: "harana-app".to_string(),
            access_token_expiry: 3600,    // 1 hour
            refresh_token_expiry: 604800, // 7 days
        }
    }
}

/// OAuth 2.0 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    pub providers: Vec<OAuthProvider>,
    pub auth_code_expiry: i64,
}

impl Default for OAuthConfig {
    fn default() -> Self {
        Self {
            providers: vec![],
            auth_code_expiry: 600, // 10 minutes
        }
    }
}

/// OAuth provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub userinfo_url: String,
    pub scopes: Vec<String>,
}

/// Passkey/WebAuthn configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeyConfig {
    pub rp_id: String,
    pub rp_name: String,
    pub rp_origin: String,
    pub timeout: u32,
}

impl Default for PasskeyConfig {
    fn default() -> Self {
        Self {
            rp_id: "localhost".to_string(),
            rp_name: "Harana".to_string(),
            rp_origin: "http://localhost:3000".to_string(),
            timeout: 60000,
        }
    }
}

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub cookie_name: String,
    pub ttl: i64,
    pub secure: bool,
    pub domain: Option<String>,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            cookie_name: "harana_session".to_string(),
            ttl: 86400, // 24 hours
            secure: false,
            domain: None,
        }
    }
}
