// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BuildGithubRelease {
    pub assets: Vec<String>,
    pub body: String,
    pub discussion_category: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BuildGithubReleaseAsset {
    pub content_type: String,
    pub label: String,
    pub path: String,
}

