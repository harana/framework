// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConnectOutput {
    pub connection_string: String,
    pub host: String,
    pub password: String,
    pub port: i64,
    pub user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfHyperdriveConfig {
    pub account_id: String,
    pub binding: String,
    #[serde(default)]
    pub caching_disabled: bool,
    pub config_id: String,
    pub config_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub database: String,
    pub host: String,
    pub port: i64,
    pub scheme: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user: String,
}

#[async_trait]
pub trait HyperdriveAction {
    async fn connect(&self, binding: String) -> Result<ConnectOutput, Box<dyn std::error::Error>>;
}
