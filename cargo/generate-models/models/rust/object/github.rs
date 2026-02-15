// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// github_repository
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GithubRepository {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_branch: String,
    pub description: Option<String>,
    #[serde(default)]
    pub fork: bool,
    pub full_name: String,
    #[serde(default)]
    pub has_issues: bool,
    #[serde(default)]
    pub has_wiki: bool,
    pub homepage: Option<String>,
    #[serde(default)]
    pub is_private: bool,
    pub repo_id: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: Option<String>,
}

impl GithubRepository {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// github_issue
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GithubIssue {
    pub assignees: Option<String>,
    pub body: Option<String>,
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub issue_number: i64,
    pub labels: Option<String>,
    /// Reference: git_hub_repository.id
    pub repository_id: String,
    pub state: String,
    pub title: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: Option<String>,
}

impl GithubIssue {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// github_pull_request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GithubPullRequest {
    pub base_branch: String,
    pub body: Option<String>,
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub head_branch: String,
    #[serde(default)]
    pub is_draft: bool,
    pub merged_at: Option<chrono::DateTime<chrono::Utc>>,
    pub pr_number: i64,
    /// Reference: git_hub_repository.id
    pub repository_id: String,
    pub state: String,
    pub title: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: Option<String>,
}

impl GithubPullRequest {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// github_webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GithubWebhook {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub events: String,
    pub hook_id: i64,
    #[serde(default)]
    pub is_active: bool,
    /// Reference: git_hub_repository.id
    pub repository_id: String,
    pub secret: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
}

impl GithubWebhook {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

