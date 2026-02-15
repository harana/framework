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

/// github_repository
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GithubRepository {
    pub repo_id: i64,
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub url: String,
    pub private: bool,
    pub default_branch: String,
    pub fork: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl GithubRepository {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// author
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Author {
    pub name: String,
    pub email: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

impl Author {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// committer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Committer {
    pub name: String,
    pub email: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

impl Committer {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// commit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Commit {
    pub sha: String,
    pub message: String,
    pub url: String,
    pub author: String,
    pub committer: String,
}

impl Commit {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// file_content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: i64,
    pub url: String,
    pub content: String,
    pub encoding: String,
}

impl FileContent {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// webhook_config
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebhookConfig {
    pub url: String,
    pub content_type: String,
    pub secret: String,
    pub insecure_ssl: String,
}

impl WebhookConfig {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// github_issue
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GithubIssue {
    pub issue_number: i64,
    pub title: String,
    pub body: String,
    pub state: String,
    pub labels: Vec<String>,
    pub assignees: Vec<String>,
    pub milestone: i64,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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
    pub pr_number: i64,
    pub title: String,
    pub body: String,
    pub state: String,
    pub head: String,
    pub base: String,
    pub draft: bool,
    pub merged: bool,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl GithubPullRequest {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// github_branch
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GithubBranch {
    pub name: String,
    pub sha: String,
    pub protected: bool,
}

impl GithubBranch {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// github_commit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GithubCommit {
    pub sha: String,
    pub message: String,
    pub url: String,
    pub author: String,
    pub committer: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl GithubCommit {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// github_release
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GithubRelease {
    pub release_id: i64,
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub published_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
}

impl GithubRelease {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

