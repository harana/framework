// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FetchInput {
    pub body: String,
    pub certificate_binding: String,
    pub headers: String,
    pub method: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FetchOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[async_trait]
pub trait MtlsAction {
    async fn fetch(&self, input: FetchInput) -> Result<FetchOutput, Box<dyn std::error::Error>>;
}
