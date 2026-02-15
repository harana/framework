// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub repository_id: String,
    pub state: String,
    pub title: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: Option<String>,
}

impl GithubIssue {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub repository_id: String,
    pub state: String,
    pub title: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: Option<String>,
}

impl GithubPullRequest {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GithubWebhook {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub events: String,
    pub hook_id: i64,
    #[serde(default)]
    pub is_active: bool,
    pub repository_id: String,
    pub secret: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
}

impl GithubWebhook {
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

