// Harana Actions - Git Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// create_repo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRepoOutput {
    pub full_name: String,
    pub repo_id: i64,
    pub success: bool,
    pub url: String,
}

// get_repo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRepoOutput {
    pub created_at: i64,
    pub default_branch: String,
    pub description: Option<String>,
    pub fork: bool,
    pub full_name: String,
    pub name: String,
    pub private: bool,
    pub repo_id: i64,
    pub updated_at: i64,
    pub url: String,
}

// update_repo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRepoOutput {
    pub success: bool,
}

// delete_repo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRepoOutput {
    pub success: bool,
}

// list_repos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReposOutput {
    pub repositories: Vec<GithubRepository>,
    pub total_count: i32,
}

// create_issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIssueOutput {
    pub issue_number: i32,
    pub success: bool,
    pub url: String,
}

// update_issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIssueOutput {
    pub success: bool,
}

// close_issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseIssueOutput {
    pub success: bool,
}

// create_pull_request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePullRequestOutput {
    pub number: i32,
    pub success: bool,
    pub url: String,
}

// merge_pull_request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergePullRequestOutput {
    pub merged: bool,
    pub sha: Option<String>,
    pub success: bool,
}

// create_branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBranchOutput {
    pub ref_name: String,
    pub sha: String,
    pub success: bool,
}

// delete_branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBranchOutput {
    pub success: bool,
}

// list_branches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBranchesOutput {
    pub branches: Vec<GitBranch>,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubRepository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub private: bool,
    pub fork: bool,
    pub url: String,
    pub default_branch: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitBranch {
    pub name: String,
    pub sha: String,
    pub protected: bool,
}
