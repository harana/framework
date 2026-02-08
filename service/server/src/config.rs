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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    pub providers: Vec<OAuthProvider>,
    pub auth_code_expiry: i64,
    pub issuer: Option<String>,
    pub discovery_cache_ttl: Option<u64>,
    pub state_ttl: Option<u64>,
}

impl Default for OAuthConfig {
    fn default() -> Self {
        Self {
            providers: vec![],
            auth_code_expiry: 600,
            issuer: None,
            discovery_cache_ttl: Some(3600),
            state_ttl: Some(600),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OAuthProviderKind {
    Cloudflare,
    Google,
    Github,
    Microsoft,
    Generic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub name: String,
    pub kind: OAuthProviderKind,
    pub client_id: String,
    pub client_secret: String,
    pub discovery_url: Option<String>,
    pub auth_url: Option<String>,
    pub token_url: Option<String>,
    pub userinfo_url: Option<String>,
    pub scopes: Vec<String>,
    pub cloudflare_team_domain: Option<String>,
}

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
