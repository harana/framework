// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait DockerhubAction {
    async fn publish_dockerhub(&self) -> Result<(), Box<dyn std::error::Error>>;
}
