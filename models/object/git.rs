// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitBranch {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_protected: bool,
    pub name: String,
    pub repository_id: String,
    pub sha: Option<String>,
}

impl GitBranch {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitCommit {
    pub author_email: Option<String>,
    pub author_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub committed_at: chrono::DateTime<chrono::Utc>,
    pub message: Option<String>,
    pub repository_id: String,
    pub sha: String,
}

impl GitCommit {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitTag {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub message: Option<String>,
    pub name: String,
    pub repository_id: String,
    pub sha: String,
}

impl GitTag {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Author {
    pub name: String,
    pub email: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

impl Author {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Committer {
    pub name: String,
    pub email: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

impl Committer {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebhookConfig {
    pub url: String,
    pub content_type: String,
    pub secret: String,
    pub insecure_ssl: String,
}

impl WebhookConfig {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GithubBranch {
    pub name: String,
    pub sha: String,
    pub protected: bool,
}

impl GithubBranch {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

