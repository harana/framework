// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait GithubPagesAction {
    async fn deploy_github_pages(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_github_pages_user(&self) -> Result<(), Box<dyn std::error::Error>>;
}
