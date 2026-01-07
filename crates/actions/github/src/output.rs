// Harana Actions - Github Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// close_issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseIssueOutput {
    pub success: bool
}

// create_branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBranchOutput {
    pub r#ref: String,
    pub success: bool
}

// create_issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIssueOutput {
    pub issue_number: i32,
    pub url: String,
    pub success: bool
}

// create_or_update_file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrUpdateFileOutput {
    pub success: bool,
    pub commit: HashMap<String, Value>,
    pub content: HashMap<String, Value>
}

// create_pull_request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePullRequestOutput {
    pub pr_number: i32,
    pub url: String,
    pub success: bool
}

// create_release
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReleaseOutput {
    pub success: bool,
    pub upload_url: String,
    pub release_id: i32,
    pub url: String
}

// create_repo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRepoOutput {
    pub success: bool,
    pub full_name: String,
    pub repo_id: i32,
    pub url: String
}

// create_webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhookOutput {
    pub webhook_id: i32,
    pub success: bool
}

// delete_branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBranchOutput {
    pub success: bool
}

// delete_file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileOutput {
    pub commit: HashMap<String, Value>,
    pub success: bool
}

// delete_repo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRepoOutput {
    pub success: bool
}

// delete_webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWebhookOutput {
    pub success: bool
}

// get_commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCommitOutput {
    pub url: String,
    pub message: String,
    pub committer: HashMap<String, Value>,
    pub author: HashMap<String, Value>,
    pub sha: String,
    pub created_at: i32
}

// get_file_contents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileContentsOutput {
    pub sha: String,
    pub content: String,
    pub encoding: String,
    pub size: i32
}

// get_release
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReleaseOutput {
    pub name: String,
    pub created_at: i32,
    pub tag_name: String,
    pub body: String,
    pub published_at: i32,
    pub draft: bool,
    pub prerelease: bool,
    pub release_id: i32
}

// get_repo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRepoOutput {
    pub repo_id: i32,
    pub updated_at: i32,
    pub url: String,
    pub description: String,
    pub name: String,
    pub fork: bool,
    pub default_branch: String,
    pub created_at: i32,
    pub full_name: String,
    pub private: bool
}

// list_branches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBranchesOutput {
    pub branches: Vec<HashMap<String, Value>>
}

// list_commits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCommitsOutput {
    pub commits: Vec<HashMap<String, Value>>
}

// list_issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListIssuesOutput {
    pub issues: Vec<HashMap<String, Value>>,
    pub total_count: i32
}

// list_pull_requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPullRequestsOutput {
    pub total_count: i32,
    pub pull_requests: Vec<HashMap<String, Value>>
}

// list_releases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReleasesOutput {
    pub releases: Vec<HashMap<String, Value>>
}

// list_repos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReposOutput {
    pub total_count: i32,
    pub repositories: Vec<HashMap<String, Value>>
}

// merge_pull_request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergePullRequestOutput {
    pub sha: String,
    pub merged: bool,
    pub success: bool
}

// update_issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIssueOutput {
    pub success: bool
}

// update_pull_request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePullRequestOutput {
    pub success: bool
}

// update_repo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRepoOutput {
    pub success: bool
}
