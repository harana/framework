// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// deploy_netlify
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployNetlify {
    pub deploy_message: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    pub functions_dir: Option<String>,
    #[serde(default)]
    pub production: bool,
    pub site_id: String,
    pub source_dir: String,
    pub token_env: String,
}

impl DeployNetlify {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

