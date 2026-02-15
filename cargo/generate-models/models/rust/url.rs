// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// short_url
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShortUrl {
    pub alias: String,
    pub click_count: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub created_by: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub is_active: bool,
    pub original_url: String,
    pub short_url: String,
}

impl ShortUrl {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// short_url_click
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShortUrlClick {
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
    pub ip_address: Option<String>,
    pub referrer: Option<String>,
    /// Reference: short_url.id
    pub short_url_id: String,
    pub user_agent: Option<String>,
}

impl ShortUrlClick {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// url
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Url {
    pub url: String,
    pub protocol: String,
    pub host: String,
    pub port: i64,
    pub path: String,
    pub query: String,
    pub fragment: String,
}

impl Url {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

