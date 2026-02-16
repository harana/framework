// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRepoOutput {
    pub full_name: String,
    pub repo_id: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetRepoOutput {
    pub created_at: i64,
    pub default_branch: String,
    pub description: String,
    pub fork: bool,
    pub full_name: String,
    pub name: String,
    pub private: bool,
    pub repo_id: i64,
    pub updated_at: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListReposOutput {
    pub repositories: Vec<String>,
    pub total_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIssueOutput {
    pub issue_number: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListIssuesOutput {
    pub issues: Vec<String>,
    pub total_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePullRequestOutput {
    pub pr_number: i64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergePullRequestOutput {
    pub merged: bool,
    pub sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPullRequestsOutput {
    pub pull_requests: Vec<String>,
    pub total_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCommitOutput {
    pub author: String,
    pub committer: String,
    pub created_at: i64,
    pub message: String,
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateReleaseOutput {
    pub release_id: i64,
    pub upload_url: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetReleaseOutput {
    pub body: String,
    pub created_at: i64,
    pub draft: bool,
    pub name: String,
    pub prerelease: bool,
    pub published_at: i64,
    pub release_id: i64,
    pub tag_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFileContentsOutput {
    pub content: String,
    pub encoding: String,
    pub sha: String,
    pub size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOrUpdateFileOutput {
    pub commit: String,
    pub content: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Author {
    pub name: String,
    pub email: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Committer {
    pub name: String,
    pub email: String,
    pub date: chrono::DateTime<chrono::Utc>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebhookConfig {
    pub url: String,
    pub content_type: String,
    pub secret: String,
    pub insecure_ssl: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GithubBranch {
    pub name: String,
    pub sha: String,
    pub protected: bool,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitRepository {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_branch: String,
    pub description: String,
    pub full_name: String,
    #[serde(default)]
    pub is_private: bool,
    pub provider: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
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
    pub sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitCommit {
    pub author_email: String,
    pub author_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub committed_at: chrono::DateTime<chrono::Utc>,
    pub message: String,
    pub repository_id: String,
    pub sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitTag {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub message: String,
    pub name: String,
    pub repository_id: String,
    pub sha: String,
}

#[async_trait]
pub trait GitAction {
    async fn create_repo(&self, auto_init: bool, description: String, gitignore_template: String, homepage: String, license_template: String, name: String, private: bool) -> Result<CreateRepoOutput, Box<dyn std::error::Error>>;
    async fn get_repo(&self, owner: String, repo: String) -> Result<GetRepoOutput, Box<dyn std::error::Error>>;
    async fn update_repo(&self, description: String, has_issues: bool, has_projects: bool, has_wiki: bool, homepage: String, name: String, owner: String, private: bool, repo: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_repo(&self, owner: String, repo: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_repos(&self, direction: String, per_page: i64, sort: String, type: String) -> Result<ListReposOutput, Box<dyn std::error::Error>>;
    async fn create_issue(&self, assignees: Vec<String>, body: String, labels: Vec<String>, milestone: i64, owner: String, repo: String, title: String) -> Result<CreateIssueOutput, Box<dyn std::error::Error>>;
    async fn update_issue(&self, assignees: Vec<String>, body: String, issue_number: i64, labels: Vec<String>, owner: String, repo: String, state: String, title: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn close_issue(&self, issue_number: i64, owner: String, repo: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_issues(&self, direction: String, labels: Vec<String>, owner: String, per_page: i64, repo: String, sort: String, state: String) -> Result<ListIssuesOutput, Box<dyn std::error::Error>>;
    async fn create_pull_request(&self, base: String, body: String, draft: bool, head: String, owner: String, repo: String, title: String) -> Result<CreatePullRequestOutput, Box<dyn std::error::Error>>;
    async fn update_pull_request(&self, body: String, owner: String, pr_number: i64, repo: String, state: String, title: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn merge_pull_request(&self, commit_message: String, commit_title: String, merge_method: String, owner: String, pr_number: i64, repo: String) -> Result<MergePullRequestOutput, Box<dyn std::error::Error>>;
    async fn list_pull_requests(&self, direction: String, owner: String, per_page: i64, repo: String, sort: String, state: String) -> Result<ListPullRequestsOutput, Box<dyn std::error::Error>>;
    async fn create_branch(&self, branch: String, owner: String, repo: String, sha: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_branch(&self, branch: String, owner: String, repo: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_branches(&self, owner: String, per_page: i64, protected: bool, repo: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn get_commit(&self, owner: String, repo: String, sha: String) -> Result<GetCommitOutput, Box<dyn std::error::Error>>;
    async fn list_commits(&self, author: String, owner: String, path: String, per_page: i64, repo: String, sha: String, since: i64, until: i64) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_release(&self, body: String, draft: bool, name: String, owner: String, prerelease: bool, repo: String, tag_name: String, target_commitish: String) -> Result<CreateReleaseOutput, Box<dyn std::error::Error>>;
    async fn get_release(&self, owner: String, release_id: i64, repo: String) -> Result<GetReleaseOutput, Box<dyn std::error::Error>>;
    async fn list_releases(&self, owner: String, per_page: i64, repo: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn create_webhook(&self, active: bool, config: String, events: Vec<String>, owner: String, repo: String, webhook_id: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_webhook(&self, owner: String, repo: String, webhook_id: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_file_contents(&self, owner: String, path: String, ref: String, repo: String) -> Result<GetFileContentsOutput, Box<dyn std::error::Error>>;
    async fn create_or_update_file(&self, branch: String, content: String, message: String, owner: String, path: String, repo: String, sha: String) -> Result<CreateOrUpdateFileOutput, Box<dyn std::error::Error>>;
    async fn delete_file(&self, branch: String, message: String, owner: String, path: String, repo: String, sha: String) -> Result<String, Box<dyn std::error::Error>>;
}
