// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// deploy_cloudflare
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployCloudflare {
    pub account_id_env: String,
    pub api_token_env: String,
    pub directory: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    pub environment: String,
    pub project_name: String,
    pub route: Option<String>,
    pub workers: Vec<String>,
    pub zone_id: Option<String>,
}

impl DeployCloudflare {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_cloudflare_worker
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployCloudflareWorker {
    pub compatibility_date: Option<String>,
    pub entry_point: String,
    pub name: String,
    pub routes: Vec<String>,
    pub vars: String,
}

impl DeployCloudflareWorker {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

