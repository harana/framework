// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployCloudflare {
    pub account_id_env: String,
    pub api_token_env: String,
    pub directory: String,
    #[serde(default)]
    pub enabled: bool,
    pub environment: String,
    pub project_name: String,
    pub route: String,
    pub workers: Vec<String>,
    pub zone_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployCloudflareWorker {
    pub compatibility_date: String,
    pub entry_point: String,
    pub name: String,
    pub routes: Vec<String>,
    pub vars: std::collections::HashMap<String, String>,
}

