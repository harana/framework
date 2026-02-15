// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait AppengineAction {
    async fn deploy_appengine(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_appengine_scaling(&self) -> Result<(), Box<dyn std::error::Error>>;
}
