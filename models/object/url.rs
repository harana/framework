// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShortUrl {
    pub alias: String,
    pub click_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub is_active: bool,
    pub original_url: String,
    pub short_url: String,
}

impl ShortUrl {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShortUrlClick {
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
    pub ip_address: Option<String>,
    pub referrer: Option<String>,
    pub short_url_id: String,
    pub user_agent: Option<String>,
}

impl ShortUrlClick {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl Url {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

