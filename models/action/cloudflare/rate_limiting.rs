// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LimitInput {
    pub key: String,
    pub rate_limiter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LimitOutput {
    pub success: bool,
}

#[async_trait]
pub trait RateLimitingAction {
    async fn limit(&self, input: LimitInput) -> Result<LimitOutput, Box<dyn std::error::Error>>;
}
