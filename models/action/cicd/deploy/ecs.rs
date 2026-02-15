// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait EcsAction {
    async fn deploy_ecs(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_ecs_deployment(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn deploy_ecs_load_balancer(&self) -> Result<(), Box<dyn std::error::Error>>;
}
