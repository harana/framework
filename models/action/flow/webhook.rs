// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegisterOutput {
    pub secret: String,
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
pub struct ListsOutput {
    pub total: i64,
    pub webhooks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TestOutput {
    pub error: String,
    pub response_time_ms: i64,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetDeliveriesOutput {
    pub deliveries: Vec<String>,
    pub total: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebhookEvent {
    pub description: String,
    #[serde(default)]
    pub is_active: bool,
    pub payload_schema: String,
}

#[async_trait]
pub trait WebhookAction {
    async fn register(&self, active: bool, description: String, events: Vec<String>, secret: String, url: String) -> Result<RegisterOutput, Box<dyn std::error::Error>>;
    async fn update(&self, active: bool, description: String, events: Vec<String>, secret: String, url: String, webhook_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn unregister(&self, webhook_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn trigger(&self, event: String, payload: String, webhook_ids: Vec<String>, triggered_count: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn get(&self, webhook_id: String) -> Result<GetOutput, Box<dyn std::error::Error>>;
    async fn lists(&self, active_only: bool, event: String, limit: i64, offset: i64) -> Result<ListsOutput, Box<dyn std::error::Error>>;
    async fn test(&self, payload: String, webhook_id: String) -> Result<TestOutput, Box<dyn std::error::Error>>;
    async fn get_deliveries(&self, limit: i64, offset: i64, status: String, webhook_id: String) -> Result<GetDeliveriesOutput, Box<dyn std::error::Error>>;
    async fn retry_delivery(&self, delivery_id: String) -> Result<i64, Box<dyn std::error::Error>>;
    async fn verify_signature(&self, algorithm: String, payload: String, secret: String, signature: String) -> Result<bool, Box<dyn std::error::Error>>;
    async fn rotate_secret(&self, webhook_id: String) -> Result<String, Box<dyn std::error::Error>>;
}
