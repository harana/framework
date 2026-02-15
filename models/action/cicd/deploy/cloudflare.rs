// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait CloudflareAction {
    async fn deploy_cloudflare(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_cloudflare_worker(&self) -> Result<(), Box<dyn std::error::Error>>;
}
