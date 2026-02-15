// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// deploy_heroku
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployHeroku {
    pub addons: Vec<String>,
    pub app_name: String,
    pub buildpacks: Vec<String>,
    pub config_vars: String,
    #[serde(default)]
    pub enabled: bool,
    pub post_deploy: Vec<String>,
    pub pre_deploy: Vec<String>,
    pub region: String,
    #[serde(default)]
    pub rollback_on_failure: bool,
    #[serde(default)]
    pub run_migrations: bool,
    pub scale: String,
    pub stack: Option<String>,
}

impl DeployHeroku {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

