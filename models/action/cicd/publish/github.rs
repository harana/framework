// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait GithubAction {
    async fn build_github_release(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn build_github_release_asset(&self) -> Result<(), Box<dyn std::error::Error>>;
}
