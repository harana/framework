// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Environment {
    pub blob: Option<String>,
    pub cache: Option<String>,
    pub deployment: Option<String>,
    pub environment: String,
    pub http: Option<String>,
    pub jwt: Option<String>,
    pub log: Option<String>,
    pub oauth: Option<String>,
    pub passkey: Option<String>,
    pub session: String,
    pub storage: Option<String>,
    pub tls: Option<String>,
    pub tracing: Option<String>,
}

impl Environment {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpOptions {
    pub allowed_origins: Option<Vec<String>>,
    #[serde(default)]
    pub enable_cors: bool,
    pub graceful_shutdown_seconds: i64,
    pub host: String,
    pub max_concurrent_requests: Option<i64>,
    pub port: i64,
    pub read_timeout_seconds: i64,
    pub write_timeout_seconds: i64,
}

impl HttpOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TlsOptions {
    pub alpn_protocols: Option<Vec<String>>,
    pub cert_path: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    pub key_path: Option<String>,
}

impl TlsOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LogOptions {
    pub level: String,
    pub format: String,
    #[serde(default)]
    pub metrics_enabled: bool,
    pub metrics_listen: Option<String>,
    pub sentry_dsn: Option<String>,
}

impl LogOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TracingOptions {
    #[serde(default)]
    pub enabled: bool,
    pub provider: String,
    pub endpoint: Option<String>,
    pub sample_rate: f64,
}

impl TracingOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageOptions {
    pub backend: String,
    pub mongodb_url: Option<String>,
    pub mongodb_database: Option<String>,
    pub d1_binding: Option<String>,
    pub durable_object: Option<String>,
}

impl StorageOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CacheOptions {
    pub backend: String,
    pub kv_binding: Option<String>,
    pub mongodb_url: Option<String>,
    pub mongodb_database: Option<String>,
}

impl CacheOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlobOptions {
    pub backend: String,
    pub file_base_path: Option<String>,
    pub r2_binding: Option<String>,
}

impl BlobOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JwtOptions {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub access_token_expiry: i64,
    pub refresh_token_expiry: i64,
}

impl JwtOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OauthOptions {
    pub providers: Vec<String>,
    pub auth_code_expiry: i64,
    pub issuer: Option<String>,
    pub discovery_cache_ttl: i64,
    pub state_ttl: i64,
}

impl OauthOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OauthProvider {
    pub name: String,
    pub kind: String,
    pub client_id: String,
    pub client_secret: String,
    pub discovery_url: Option<String>,
    pub auth_url: Option<String>,
    pub token_url: Option<String>,
    pub userinfo_url: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub cloudflare_team_domain: Option<String>,
}

impl OauthProvider {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasskeyOptions {
    pub relying_party_id: String,
    pub relying_party_name: String,
    pub relying_party_origin: String,
    pub timeout: i64,
}

impl PasskeyOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SessionOptions {
    pub cookie_name: String,
    pub ttl: i64,
    #[serde(default)]
    pub secure: bool,
    pub domain: Option<String>,
}

impl SessionOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DurableObjectOptions {
    pub binding: String,
    pub namespace: String,
    pub alarm_interval_seconds: Option<i64>,
    pub max_stored_bytes: Option<i64>,
}

impl DurableObjectOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeploymentOptions {
    pub target: String,
    pub cloudflare_worker: Option<String>,
    pub standalone: Option<String>,
}

impl DeploymentOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkerOptions {
    pub worker_name: String,
    pub compatibility_date: String,
    pub account_id: Option<String>,
    pub route: Option<String>,
    pub zone_id: Option<String>,
    pub durable_object_binding: Option<String>,
}

impl CloudflareWorkerOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StandaloneOptions {
    pub workers: i64,
    pub max_blocking_threads: Option<i64>,
    #[serde(default)]
    pub enable_http2: bool,
    pub pid_file: Option<String>,
}

impl StandaloneOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

