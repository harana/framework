// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// build_github_release
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BuildGithubRelease {
    pub assets: Vec<String>,
    pub body: Option<String>,
    pub discussion_category: Option<String>,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub generate_release_notes: bool,
    #[serde(default)]
    pub overwrite: bool,
    #[serde(default)]
    pub prerelease: bool,
    pub release_name: String,
    pub repository: String,
    pub tag: String,
    pub token_env: String,
}

impl BuildGithubRelease {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// build_github_release_asset
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BuildGithubReleaseAsset {
    pub content_type: String,
    pub label: Option<String>,
    pub path: String,
}

impl BuildGithubReleaseAsset {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

