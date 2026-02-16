// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BroadcastEventOutput {
    pub event_id: String,
    pub recipients: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEventOutput {
    pub channel: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub event_type: String,
    pub metadata: String,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListEventsOutput {
    pub events: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Event {
    pub event_id: String,
    pub event_type: String,
    pub channel: String,
    pub payload: String,
    pub metadata: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventMetadata {
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventRecord {
    pub event_id: String,
    pub event_type: String,
    pub channel: String,
    pub payload: String,
    pub metadata: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub acknowledged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventSubscription {
    pub callback_url: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub event_type: String,
    pub filter: String,
    pub id: String,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventLog {
    pub error_message: String,
    pub event_id: String,
    pub id: String,
    #[serde(default = "chrono::Utc::now")]
    pub processed_at: chrono::DateTime<chrono::Utc>,
    pub retry_count: i64,
    pub status: String,
    pub subscription_id: String,
}

#[async_trait]
pub trait EventAction {
    async fn emit_event(&self, channel: String, event_type: String, metadata: String, payload: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn subscribe_channel(&self, channel: String, event_types: Vec<String>, handler: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn unsubscribe_channel(&self, subscription_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn broadcast_event(&self, event_type: String, exclude_channels: Vec<String>, metadata: String, payload: String) -> Result<BroadcastEventOutput, Box<dyn std::error::Error>>;
    async fn get_event(&self, event_id: String) -> Result<GetEventOutput, Box<dyn std::error::Error>>;
    async fn list_events(&self, channel: String, end_time: chrono::DateTime<chrono::Utc>, event_types: Vec<String>, limit: i64, start_time: chrono::DateTime<chrono::Utc>) -> Result<ListEventsOutput, Box<dyn std::error::Error>>;
    async fn acknowledge_event(&self, event_id: String, subscription_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn replay_events(&self, channel: String, end_time: chrono::DateTime<chrono::Utc>, event_types: Vec<String>, start_time: chrono::DateTime<chrono::Utc>) -> Result<i64, Box<dyn std::error::Error>>;
}
