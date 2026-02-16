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
pub struct EventPriority {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventStatus {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventFullMetadata {
    pub attributes: std::collections::HashMap<String, String>,
    pub causation_id: String,
    pub correlation_id: String,
    pub source: String,
    pub tags: Vec<String>,
    pub tenant_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventFull {
    pub channel: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub delivery_attempts: i64,
    pub event_type: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub id: String,
    pub metadata: String,
    pub payload: String,
    pub priority: String,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub ttl_seconds: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventChannelConfig {
    #[serde(default)]
    pub allow_duplicates: bool,
    pub allowed_event_types: Vec<String>,
    pub buffer_size: i64,
    pub default_ttl_seconds: i64,
    #[serde(default)]
    pub durable: bool,
    pub max_events: i64,
    #[serde(default)]
    pub persistent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventChannel {
    #[serde(default)]
    pub active: bool,
    pub config: String,
    pub name: String,
    pub pending_events: i64,
    pub subscriber_count: i64,
    pub total_events: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventHandlerType {
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventSubscriptionHandler {
    pub handler_id: String,
    pub handler_type: String,
    pub max_concurrency: i64,
    pub max_retries: i64,
    #[serde(default)]
    pub ordered: bool,
    pub retry_delay_ms: i64,
    pub timeout_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventSubscriptionFilter {
    pub any_tags: Vec<String>,
    pub custom_filter: String,
    pub event_type_prefix: String,
    pub event_types: Vec<String>,
    pub min_priority: String,
    pub required_tags: Vec<String>,
    pub source: String,
    pub tenant_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventSubscription {
    #[serde(default)]
    pub active: bool,
    pub channel: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub custom_data: String,
    #[serde(default)]
    pub durable: bool,
    pub filter: String,
    pub handler: String,
    pub id: String,
    #[serde(default = "chrono::Utc::now")]
    pub last_active_at: chrono::DateTime<chrono::Utc>,
    pub last_event_id: String,
    pub last_event_time: chrono::DateTime<chrono::Utc>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventQuery {
    #[serde(default)]
    pub ascending: bool,
    pub channel: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub event_types: Vec<String>,
    pub limit: i64,
    pub offset: i64,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub status: String,
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
