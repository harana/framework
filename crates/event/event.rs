// Harana Components - Events Core Event Types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum EventPriority {
    Low = 0,
    #[default]
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl EventPriority {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn from_i32(value: i32) -> Self {
        match value {
            0 => Self::Low,
            2 => Self::High,
            3 => Self::Critical,
            _ => Self::Normal,
        }
    }
}

/// Event status for tracking lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    #[default]
    Pending,
    Delivered,
    Acknowledged,
    Failed,
    Expired,
}

impl EventStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Delivered => "delivered",
            Self::Acknowledged => "acknowledged",
            Self::Failed => "failed",
            Self::Expired => "expired",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "delivered" => Self::Delivered,
            "acknowledged" => Self::Acknowledged,
            "failed" => Self::Failed,
            "expired" => Self::Expired,
            _ => Self::Pending,
        }
    }
}

/// Event metadata for additional context
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub attributes: HashMap<String, Value>,
}

impl EventMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn with_correlation_id(mut self, id: impl Into<String>) -> Self {
        self.correlation_id = Some(id.into());
        self
    }

    pub fn with_causation_id(mut self, id: impl Into<String>) -> Self {
        self.causation_id = Some(id.into());
        self
    }

    pub fn with_user_id(mut self, id: impl Into<String>) -> Self {
        self.user_id = Some(id.into());
        self
    }

    pub fn with_tenant_id(mut self, id: impl Into<String>) -> Self {
        self.tenant_id = Some(id.into());
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn with_tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tags.extend(tags.into_iter().map(Into::into));
        self
    }

    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        if let Ok(v) = serde_json::to_value(value) {
            self.attributes.insert(key.into(), v);
        }
        self
    }
}

/// Core event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub event_type: String,
    pub channel: String,
    pub payload: Value,
    #[serde(default)]
    pub metadata: EventMetadata,
    #[serde(default)]
    pub priority: EventPriority,
    #[serde(default)]
    pub status: EventStatus,
    #[serde(default)]
    pub delivery_attempts: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

impl Event {
    pub fn new(event_type: impl Into<String>, channel: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: event_type.into(),
            channel: channel.into(),
            payload: Value::Null,
            metadata: EventMetadata::default(),
            priority: EventPriority::Normal,
            status: EventStatus::Pending,
            delivery_attempts: 0,
            ttl_seconds: None,
            scheduled_at: None,
            created_at: now,
            updated_at: now,
            expires_at: None,
        }
    }

    pub fn with_id(id: impl Into<String>, event_type: impl Into<String>, channel: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            event_type: event_type.into(),
            channel: channel.into(),
            payload: Value::Null,
            metadata: EventMetadata::default(),
            priority: EventPriority::Normal,
            status: EventStatus::Pending,
            delivery_attempts: 0,
            ttl_seconds: None,
            scheduled_at: None,
            created_at: now,
            updated_at: now,
            expires_at: None,
        }
    }

    /// Set the event payload
    pub fn with_payload(mut self, payload: impl Serialize) -> Self {
        self.payload = serde_json::to_value(payload).unwrap_or(Value::Null);
        self
    }

    /// Set the event metadata
    pub fn with_metadata(mut self, metadata: EventMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Set the event priority
    pub fn with_priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set the time-to-live in seconds
    pub fn with_ttl(mut self, ttl_seconds: u64) -> Self {
        self.ttl_seconds = Some(ttl_seconds);
        self.expires_at = Some(self.created_at + chrono::Duration::seconds(ttl_seconds as i64));
        self
    }

    /// Schedule the event for later delivery
    pub fn scheduled_for(mut self, scheduled_at: DateTime<Utc>) -> Self {
        self.scheduled_at = Some(scheduled_at);
        self
    }

    /// Schedule the event for delivery after a delay
    pub fn delayed(mut self, delay_seconds: u64) -> Self {
        self.scheduled_at = Some(Utc::now() + chrono::Duration::seconds(delay_seconds as i64));
        self
    }

    /// Check if the event has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }

    /// Check if the event is ready for delivery
    pub fn is_ready(&self) -> bool {
        if self.is_expired() {
            return false;
        }
        if let Some(scheduled_at) = self.scheduled_at {
            Utc::now() >= scheduled_at
        } else {
            true
        }
    }

    /// Mark the event as delivered
    pub fn mark_delivered(&mut self) {
        self.status = EventStatus::Delivered;
        self.delivery_attempts += 1;
        self.updated_at = Utc::now();
    }

    /// Mark the event as acknowledged
    pub fn mark_acknowledged(&mut self) {
        self.status = EventStatus::Acknowledged;
        self.updated_at = Utc::now();
    }

    /// Mark the event as failed
    pub fn mark_failed(&mut self) {
        self.status = EventStatus::Failed;
        self.updated_at = Utc::now();
    }

    /// Mark the event as expired
    pub fn mark_expired(&mut self) {
        self.status = EventStatus::Expired;
        self.updated_at = Utc::now();
    }
}

// Implement Entity trait for storage compatibility
impl harana_components_storage::Entity for Event {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type() -> &'static str {
        "event"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = Event::new("user.created", "users");
        assert_eq!(event.event_type, "user.created");
        assert_eq!(event.channel, "users");
        assert_eq!(event.status, EventStatus::Pending);
        assert!(event.is_ready());
        assert!(!event.is_expired());
    }

    #[test]
    fn test_event_with_payload() {
        let event = Event::new("order.completed", "orders")
            .with_payload(serde_json::json!({"order_id": "123", "total": 99.99}));

        assert!(event.payload.is_object());
        assert_eq!(event.payload["order_id"], "123");
    }

    #[test]
    fn test_event_with_metadata() {
        let metadata = EventMetadata::new()
            .with_source("order-service")
            .with_correlation_id("corr-123")
            .with_tag("important");

        let event = Event::new("order.completed", "orders").with_metadata(metadata);

        assert_eq!(event.metadata.source, Some("order-service".to_string()));
        assert_eq!(event.metadata.correlation_id, Some("corr-123".to_string()));
        assert!(event.metadata.tags.contains(&"important".to_string()));
    }

    #[test]
    fn test_event_priority() {
        assert!(EventPriority::Critical > EventPriority::High);
        assert!(EventPriority::High > EventPriority::Normal);
        assert!(EventPriority::Normal > EventPriority::Low);
    }
}
