// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParseOutput {
    pub fragment: String,
    pub host: String,
    pub path: String,
    pub port: i64,
    pub protocol: String,
    pub query: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ValidateOutput {
    pub errors: Vec<String>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Url {
    pub url: String,
    pub protocol: String,
    pub host: String,
    pub port: i64,
    pub path: String,
    pub query: std::collections::HashMap<String, String>,
    pub fragment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShortUrl {
    pub alias: String,
    pub click_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    pub original_url: String,
    pub short_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShortUrlClick {
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
    pub ip_address: String,
    pub referrer: String,
    pub short_url_id: String,
    pub user_agent: String,
}

#[async_trait]
pub trait UrlAction {
    async fn parse(&self, url: String) -> Result<ParseOutput, Box<dyn std::error::Error>>;
    async fn build(&self, fragment: String, host: String, path: String, port: i64, protocol: String, query: std::collections::HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn encode(&self, component: bool, data: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn decode(&self, component: bool, data: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn shorten(&self, custom_alias: String, expiration: String, url: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn expand(&self, url: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn validate(&self, allowed_protocols: Vec<String>, require_tld: bool, url: String) -> Result<ValidateOutput, Box<dyn std::error::Error>>;
}
