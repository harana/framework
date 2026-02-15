// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegisterInput {
    #[serde(default)]
    pub active: bool,
    pub description: String,
    pub events: Vec<String>,
    pub secret: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegisterOutput {
    pub secret: String,
    pub success: bool,
    pub webhook_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateInput {
    pub active: bool,
    pub description: String,
    pub events: Vec<String>,
    pub secret: String,
    pub url: String,
    pub webhook_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnregisterInput {
    pub webhook_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnregisterOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TriggerInput {
    pub event: String,
    pub payload: String,
    pub webhook_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TriggerOutput {
    pub success: bool,
    pub triggered_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetInput {
    pub webhook_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetOutput {
    pub active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub events: Vec<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListsInput {
    #[serde(default)]
    pub active_only: bool,
    pub event: String,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListsOutput {
    pub total: i64,
    pub webhooks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestInput {
    pub payload: String,
    pub webhook_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestOutput {
    pub error: String,
    pub response_time_ms: i64,
    pub status_code: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDeliveriesInput {
    pub limit: i64,
    pub offset: i64,
    pub status: String,
    pub webhook_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDeliveriesOutput {
    pub deliveries: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetryDeliveryInput {
    pub delivery_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetryDeliveryOutput {
    pub status_code: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifySignatureInput {
    pub algorithm: String,
    pub payload: String,
    pub secret: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifySignatureOutput {
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotateSecretInput {
    pub webhook_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RotateSecretOutput {
    pub new_secret: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Webhook {
    pub webhook_id: String,
    pub url: String,
    pub events: Vec<String>,
    pub secret: String,
    pub active: bool,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebhookDelivery {
    pub delivery_id: String,
    pub webhook_id: String,
    pub event: String,
    pub status: String,
    pub status_code: i64,
    pub response_time_ms: i64,
    pub error: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait WebhookAction {
    async fn register(&self, input: RegisterInput) -> Result<RegisterOutput, Box<dyn std::error::Error>>;
    async fn update(&self, input: UpdateInput) -> Result<UpdateOutput, Box<dyn std::error::Error>>;
    async fn unregister(&self, input: UnregisterInput) -> Result<UnregisterOutput, Box<dyn std::error::Error>>;
    async fn trigger(&self, input: TriggerInput) -> Result<TriggerOutput, Box<dyn std::error::Error>>;
    async fn get(&self, input: GetInput) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn lists(&self, input: ListsInput) -> Result<ListsOutput, Box<dyn std::error::Error>>;
    async fn test(&self, input: TestInput) -> Result<TestOutput, Box<dyn std::error::Error>>;
    async fn get_deliveries(&self, input: GetDeliveriesInput) -> Result<GetDeliveriesOutput, Box<dyn std::error::Error>>;
    async fn retry_delivery(&self, input: RetryDeliveryInput) -> Result<RetryDeliveryOutput, Box<dyn std::error::Error>>;
    async fn verify_signature(&self, input: VerifySignatureInput) -> Result<VerifySignatureOutput, Box<dyn std::error::Error>>;
    async fn rotate_secret(&self, input: RotateSecretInput) -> Result<RotateSecretOutput, Box<dyn std::error::Error>>;
}
