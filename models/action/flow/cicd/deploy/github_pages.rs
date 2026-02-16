// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployGithubPages {
    pub branch: String,
    pub cname: String,
    pub commit_message: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub force: bool,
    #[serde(default)]
    pub keep_history: bool,
    pub repository: String,
    pub source_dir: String,
    pub token_env: String,
    pub user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployGithubPagesUser {
    pub email: String,
    pub name: String,
}

