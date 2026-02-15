// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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

impl Event {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventMetadata {
    pub values: std::collections::HashMap<String, String>,
}

impl EventMetadata {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl EventRecord {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventPriority {
    pub value: String,
}

impl EventPriority {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventStatus {
    pub value: String,
}

impl EventStatus {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventFullMetadata {
    pub attributes: std::collections::HashMap<String, String>,
    pub causation_id: Option<String>,
    pub correlation_id: Option<String>,
    pub source: Option<String>,
    pub tags: Vec<String>,
    pub tenant_id: Option<String>,
    pub user_id: Option<String>,
}

impl EventFullMetadata {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventFull {
    pub channel: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub delivery_attempts: i64,
    pub event_type: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub id: String,
    pub metadata: String,
    pub payload: String,
    pub priority: String,
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub ttl_seconds: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl EventFull {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventChannelConfig {
    #[serde(default)]
    pub allow_duplicates: bool,
    pub allowed_event_types: Vec<String>,
    pub buffer_size: i64,
    pub default_ttl_seconds: Option<i64>,
    #[serde(default)]
    pub durable: bool,
    pub max_events: Option<i64>,
    #[serde(default)]
    pub persistent: bool,
}

impl EventChannelConfig {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl EventChannel {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventHandlerType {
    pub value: String,
}

impl EventHandlerType {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl EventSubscriptionHandler {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventSubscriptionFilter {
    pub any_tags: Vec<String>,
    pub custom_filter: Option<String>,
    pub event_type_prefix: Option<String>,
    pub event_types: Vec<String>,
    pub min_priority: String,
    pub required_tags: Vec<String>,
    pub source: Option<String>,
    pub tenant_id: Option<String>,
    pub user_id: Option<String>,
}

impl EventSubscriptionFilter {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventSubscription {
    #[serde(default)]
    pub active: bool,
    pub channel: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub custom_data: Option<String>,
    #[serde(default)]
    pub durable: bool,
    pub filter: String,
    pub handler: String,
    pub id: String,
    #[serde(default = "chrono::Utc::now")]
    pub last_active_at: chrono::DateTime<chrono::Utc>,
    pub last_event_id: Option<String>,
    pub last_event_time: Option<chrono::DateTime<chrono::Utc>>,
    pub name: Option<String>,
}

impl EventSubscription {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventQuery {
    #[serde(default)]
    pub ascending: bool,
    pub channel: Option<String>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub event_types: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
}

impl EventQuery {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

