// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployHeroku {
    pub addons: Vec<String>,
    pub app_name: String,
    pub buildpacks: Vec<String>,
    pub config_vars: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub enabled: bool,
    pub post_deploy: Vec<String>,
    pub pre_deploy: Vec<String>,
    pub region: String,
    #[serde(default)]
    pub rollback_on_failure: bool,
    #[serde(default)]
    pub run_migrations: bool,
    pub scale: std::collections::HashMap<String, String>,
    pub stack: String,
}

