// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FetchInput {
    pub body: String,
    pub headers: String,
    pub method: String,
    pub service_binding: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FetchOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetVarInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetVarOutput {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSecretInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSecretOutput {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduledInput {
    pub cron: String,
    pub service_binding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ScheduledOutput {
    pub scheduled_time: chrono::DateTime<chrono::Utc>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WaitUntilInput {
    pub promise: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WaitUntilOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PassThroughInput {
    pub request: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PassThroughOutput {
    pub response: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ServiceBindingFetchInput {
    pub body: String,
    pub headers: String,
    pub method: String,
    pub service: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ServiceBindingFetchOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetVersionInput {
    pub binding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetVersionOutput {
    pub id: String,
    pub message: String,
    pub tag: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait WorkerAction {
    async fn fetch(&self, input: FetchInput) -> Result<FetchOutput, Box<dyn std::error::Error>>;
    async fn get_var(&self, input: GetVarInput) -> Result<GetVarOutput, Box<dyn std::error::Error>>;
    async fn get_secret(&self, input: GetSecretInput) -> Result<GetSecretOutput, Box<dyn std::error::Error>>;
    async fn scheduled(&self, input: ScheduledInput) -> Result<ScheduledOutput, Box<dyn std::error::Error>>;
    async fn wait_until(&self, input: WaitUntilInput) -> Result<WaitUntilOutput, Box<dyn std::error::Error>>;
    async fn pass_through(&self, input: PassThroughInput) -> Result<PassThroughOutput, Box<dyn std::error::Error>>;
    async fn service_binding_fetch(&self, input: ServiceBindingFetchInput) -> Result<ServiceBindingFetchOutput, Box<dyn std::error::Error>>;
    async fn get_version(&self, input: GetVersionInput) -> Result<GetVersionOutput, Box<dyn std::error::Error>>;
}
