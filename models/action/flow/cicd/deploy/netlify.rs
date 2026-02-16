// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployNetlify {
    pub deploy_message: String,
    #[serde(default)]
    pub enabled: bool,
    pub functions_dir: String,
    #[serde(default)]
    pub production: bool,
    pub site_id: String,
    pub source_dir: String,
    pub token_env: String,
}

