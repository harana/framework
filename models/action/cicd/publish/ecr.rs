// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait EcrAction {
    async fn publish_ecr(&self) -> Result<(), Box<dyn std::error::Error>>;
}
