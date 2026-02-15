// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait NetlifyAction {
    async fn deploy_netlify(&self) -> Result<(), Box<dyn std::error::Error>>;
}
