// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConnectInput {
    pub binding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConnectOutput {
    pub connection_string: String,
    pub host: String,
    pub password: String,
    pub port: i64,
    pub user: String,
}

#[async_trait]
pub trait HyperdriveAction {
    async fn connect(&self, input: ConnectInput) -> Result<ConnectOutput, Box<dyn std::error::Error>>;
}
