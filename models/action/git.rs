// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRepoInput {
    #[serde(default)]
    pub auto_init: bool,
    pub description: String,
    pub gitignore_template: String,
    pub homepage: String,
    pub license_template: String,
    pub name: String,
    #[serde(default)]
    pub private: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRepoOutput {
    pub full_name: String,
    pub repo_id: i64,
    pub success: bool,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetRepoInput {
    pub owner: String,
    pub repo: String,
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
pub struct UpdateRepoInput {
    pub description: String,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub homepage: String,
    pub name: String,
    pub owner: String,
    pub private: bool,
    pub repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateRepoOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRepoInput {
    pub owner: String,
    pub repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteRepoOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListReposInput {
    pub direction: String,
    pub per_page: i64,
    pub sort: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListReposOutput {
    pub repositories: Vec<String>,
    pub total_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIssueInput {
    pub assignees: Vec<String>,
    pub body: String,
    pub labels: Vec<String>,
    pub milestone: i64,
    pub owner: String,
    pub repo: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIssueOutput {
    pub issue_number: i64,
    pub success: bool,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateIssueInput {
    pub assignees: Vec<String>,
    pub body: String,
    pub issue_number: i64,
    pub labels: Vec<String>,
    pub owner: String,
    pub repo: String,
    pub state: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateIssueOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloseIssueInput {
    pub issue_number: i64,
    pub owner: String,
    pub repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloseIssueOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListIssuesInput {
    pub direction: String,
    pub labels: Vec<String>,
    pub owner: String,
    pub per_page: i64,
    pub repo: String,
    pub sort: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListIssuesOutput {
    pub issues: Vec<String>,
    pub total_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePullRequestInput {
    pub base: String,
    pub body: String,
    #[serde(default)]
    pub draft: bool,
    pub head: String,
    pub owner: String,
    pub repo: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePullRequestOutput {
    pub pr_number: i64,
    pub success: bool,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdatePullRequestInput {
    pub body: String,
    pub owner: String,
    pub pr_number: i64,
    pub repo: String,
    pub state: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdatePullRequestOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergePullRequestInput {
    pub commit_message: String,
    pub commit_title: String,
    pub merge_method: String,
    pub owner: String,
    pub pr_number: i64,
    pub repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergePullRequestOutput {
    pub merged: bool,
    pub sha: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPullRequestsInput {
    pub direction: String,
    pub owner: String,
    pub per_page: i64,
    pub repo: String,
    pub sort: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListPullRequestsOutput {
    pub pull_requests: Vec<String>,
    pub total_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateBranchInput {
    pub branch: String,
    pub owner: String,
    pub repo: String,
    pub sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateBranchOutput {
    pub ref: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteBranchInput {
    pub branch: String,
    pub owner: String,
    pub repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteBranchOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListBranchesInput {
    pub owner: String,
    pub per_page: i64,
    pub protected: bool,
    pub repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListBranchesOutput {
    pub branches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCommitInput {
    pub owner: String,
    pub repo: String,
    pub sha: String,
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
pub struct ListCommitsInput {
    pub author: String,
    pub owner: String,
    pub path: String,
    pub per_page: i64,
    pub repo: String,
    pub sha: String,
    pub since: i64,
    pub until: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCommitsOutput {
    pub commits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateReleaseInput {
    pub body: String,
    #[serde(default)]
    pub draft: bool,
    pub name: String,
    pub owner: String,
    #[serde(default)]
    pub prerelease: bool,
    pub repo: String,
    pub tag_name: String,
    pub target_commitish: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateReleaseOutput {
    pub release_id: i64,
    pub success: bool,
    pub upload_url: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetReleaseInput {
    pub owner: String,
    pub release_id: i64,
    pub repo: String,
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
pub struct ListReleasesInput {
    pub owner: String,
    pub per_page: i64,
    pub repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListReleasesOutput {
    pub releases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateWebhookInput {
    #[serde(default)]
    pub active: bool,
    pub config: String,
    pub events: Vec<String>,
    pub owner: String,
    pub repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateWebhookOutput {
    pub success: bool,
    pub webhook_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteWebhookInput {
    pub owner: String,
    pub repo: String,
    pub webhook_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteWebhookOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFileContentsInput {
    pub owner: String,
    pub path: String,
    pub ref: String,
    pub repo: String,
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
pub struct CreateOrUpdateFileInput {
    pub branch: String,
    pub content: String,
    pub message: String,
    pub owner: String,
    pub path: String,
    pub repo: String,
    pub sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateOrUpdateFileOutput {
    pub commit: String,
    pub content: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteFileInput {
    pub branch: String,
    pub message: String,
    pub owner: String,
    pub path: String,
    pub repo: String,
    pub sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteFileOutput {
    pub commit: String,
    pub success: bool,
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

#[async_trait]
pub trait GitAction {
    async fn create_repo(&self, input: CreateRepoInput) -> Result<CreateRepoOutput, Box<dyn std::error::Error>>;
    async fn get_repo(&self, input: GetRepoInput) -> Result<GetRepoOutput, Box<dyn std::error::Error>>;
    async fn update_repo(&self, input: UpdateRepoInput) -> Result<UpdateRepoOutput, Box<dyn std::error::Error>>;
    async fn delete_repo(&self, input: DeleteRepoInput) -> Result<DeleteRepoOutput, Box<dyn std::error::Error>>;
    async fn list_repos(&self, input: ListReposInput) -> Result<ListReposOutput, Box<dyn std::error::Error>>;
    async fn create_issue(&self, input: CreateIssueInput) -> Result<CreateIssueOutput, Box<dyn std::error::Error>>;
    async fn update_issue(&self, input: UpdateIssueInput) -> Result<UpdateIssueOutput, Box<dyn std::error::Error>>;
    async fn close_issue(&self, input: CloseIssueInput) -> Result<CloseIssueOutput, Box<dyn std::error::Error>>;
    async fn list_issues(&self, input: ListIssuesInput) -> Result<ListIssuesOutput, Box<dyn std::error::Error>>;
    async fn create_pull_request(&self, input: CreatePullRequestInput) -> Result<CreatePullRequestOutput, Box<dyn std::error::Error>>;
    async fn update_pull_request(&self, input: UpdatePullRequestInput) -> Result<UpdatePullRequestOutput, Box<dyn std::error::Error>>;
    async fn merge_pull_request(&self, input: MergePullRequestInput) -> Result<MergePullRequestOutput, Box<dyn std::error::Error>>;
    async fn list_pull_requests(&self, input: ListPullRequestsInput) -> Result<ListPullRequestsOutput, Box<dyn std::error::Error>>;
    async fn create_branch(&self, input: CreateBranchInput) -> Result<CreateBranchOutput, Box<dyn std::error::Error>>;
    async fn delete_branch(&self, input: DeleteBranchInput) -> Result<DeleteBranchOutput, Box<dyn std::error::Error>>;
    async fn list_branches(&self, input: ListBranchesInput) -> Result<ListBranchesOutput, Box<dyn std::error::Error>>;
    async fn get_commit(&self, input: GetCommitInput) -> Result<GetCommitOutput, Box<dyn std::error::Error>>;
    async fn list_commits(&self, input: ListCommitsInput) -> Result<ListCommitsOutput, Box<dyn std::error::Error>>;
    async fn create_release(&self, input: CreateReleaseInput) -> Result<CreateReleaseOutput, Box<dyn std::error::Error>>;
    async fn get_release(&self, input: GetReleaseInput) -> Result<GetReleaseOutput, Box<dyn std::error::Error>>;
    async fn list_releases(&self, input: ListReleasesInput) -> Result<ListReleasesOutput, Box<dyn std::error::Error>>;
    async fn create_webhook(&self, input: CreateWebhookInput) -> Result<CreateWebhookOutput, Box<dyn std::error::Error>>;
    async fn delete_webhook(&self, input: DeleteWebhookInput) -> Result<DeleteWebhookOutput, Box<dyn std::error::Error>>;
    async fn get_file_contents(&self, input: GetFileContentsInput) -> Result<GetFileContentsOutput, Box<dyn std::error::Error>>;
    async fn create_or_update_file(&self, input: CreateOrUpdateFileInput) -> Result<CreateOrUpdateFileOutput, Box<dyn std::error::Error>>;
    async fn delete_file(&self, input: DeleteFileInput) -> Result<DeleteFileOutput, Box<dyn std::error::Error>>;
}
