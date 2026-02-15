// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// deploy_github_pages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployGithubPages {
    pub branch: String,
    pub cname: Option<String>,
    pub commit_message: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub force: bool,
    #[serde(default)]
    pub keep_history: bool,
    pub repository: String,
    pub source_dir: String,
    pub token_env: String,
    pub user: Option<String>,
}

impl DeployGithubPages {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// deploy_github_pages_user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployGithubPagesUser {
    pub email: String,
    pub name: String,
}

impl DeployGithubPagesUser {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

