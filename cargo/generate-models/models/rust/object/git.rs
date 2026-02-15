// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// git_repository
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitRepository {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_branch: String,
    pub description: Option<String>,
    pub full_name: String,
    #[serde(default)]
    pub is_private: bool,
    pub provider: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: Option<String>,
}

impl GitRepository {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// git_branch
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitBranch {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_protected: bool,
    pub name: String,
    /// Reference: git_repository.id
    pub repository_id: String,
    pub sha: Option<String>,
}

impl GitBranch {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// git_commit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitCommit {
    pub author_email: Option<String>,
    pub author_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub committed_at: chrono::DateTime<chrono::Utc>,
    pub message: Option<String>,
    /// Reference: git_repository.id
    pub repository_id: String,
    pub sha: String,
}

impl GitCommit {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// git_tag
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitTag {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub message: Option<String>,
    pub name: String,
    /// Reference: git_repository.id
    pub repository_id: String,
    pub sha: String,
}

impl GitTag {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

