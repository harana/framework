// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait HerokuAction {
    async fn deploy_heroku(&self) -> Result<(), Box<dyn std::error::Error>>;
}
