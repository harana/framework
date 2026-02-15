// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmitEventInput {
    pub channel: String,
    pub event_type: String,
    pub metadata: String,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmitEventOutput {
    pub event_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubscribeChannelInput {
    pub channel: String,
    pub event_types: Vec<String>,
    pub handler: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubscribeChannelOutput {
    pub subscription_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnsubscribeChannelInput {
    pub subscription_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnsubscribeChannelOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BroadcastEventInput {
    pub event_type: String,
    pub exclude_channels: Vec<String>,
    pub metadata: String,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BroadcastEventOutput {
    pub event_id: String,
    pub recipients: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEventInput {
    pub event_id: String,
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
pub struct ListEventsInput {
    pub channel: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub event_types: Vec<String>,
    pub limit: i64,
    pub start_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListEventsOutput {
    pub events: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AcknowledgeEventInput {
    pub event_id: String,
    pub subscription_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AcknowledgeEventOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplayEventsInput {
    pub channel: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub event_types: Vec<String>,
    pub start_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReplayEventsOutput {
    pub events_replayed: i64,
    pub success: bool,
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

#[async_trait]
pub trait EventAction {
    async fn emit_event(&self, input: EmitEventInput) -> Result<EmitEventOutput, Box<dyn std::error::Error>>;
    async fn subscribe_channel(&self, input: SubscribeChannelInput) -> Result<SubscribeChannelOutput, Box<dyn std::error::Error>>;
    async fn unsubscribe_channel(&self, input: UnsubscribeChannelInput) -> Result<UnsubscribeChannelOutput, Box<dyn std::error::Error>>;
    async fn broadcast_event(&self, input: BroadcastEventInput) -> Result<BroadcastEventOutput, Box<dyn std::error::Error>>;
    async fn get_event(&self, input: GetEventInput) -> Result<GetEventOutput, Box<dyn std::error::Error>>;
    async fn list_events(&self, input: ListEventsInput) -> Result<ListEventsOutput, Box<dyn std::error::Error>>;
    async fn acknowledge_event(&self, input: AcknowledgeEventInput) -> Result<AcknowledgeEventOutput, Box<dyn std::error::Error>>;
    async fn replay_events(&self, input: ReplayEventsInput) -> Result<ReplayEventsOutput, Box<dyn std::error::Error>>;
}
