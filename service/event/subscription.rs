// Harana Components - Events Subscription Types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;

use crate::{Event, EventPriority};

/// Filter for subscriptions to select specific events
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionFilter {
    #[serde(default)]
    pub event_types: HashSet<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_priority: Option<EventPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default)]
    pub required_tags: HashSet<String>,
    #[serde(default)]
    pub any_tags: HashSet<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_filter: Option<String>,
}

impl SubscriptionFilter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a filter for specific event types
    pub fn for_types(types: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            event_types: types.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }

    /// Create a filter for event type prefix
    pub fn for_prefix(prefix: impl Into<String>) -> Self {
        Self {
            event_type_prefix: Some(prefix.into()),
            ..Default::default()
        }
    }

    pub fn with_types(mut self, types: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.event_types = types.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.event_type_prefix = Some(prefix.into());
        self
    }

    pub fn with_min_priority(mut self, priority: EventPriority) -> Self {
        self.min_priority = Some(priority);
        self
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn with_required_tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.required_tags = tags.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_any_tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.any_tags = tags.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_tenant_id(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Check if an event matches this filter
    pub fn matches(&self, event: &Event) -> bool {
        // Check event types
        if !self.event_types.is_empty() && !self.event_types.contains(&event.event_type) {
            return false;
        }

        // Check event type prefix
        if let Some(ref prefix) = self.event_type_prefix {
            if !event.event_type.starts_with(prefix) {
                return false;
            }
        }

        // Check minimum priority
        if let Some(min_priority) = self.min_priority {
            if event.priority < min_priority {
                return false;
            }
        }

        // Check source
        if let Some(ref source) = self.source {
            if event.metadata.source.as_ref() != Some(source) {
                return false;
            }
        }

        // Check required tags (ALL must be present)
        if !self.required_tags.is_empty() {
            for tag in &self.required_tags {
                if !event.metadata.tags.contains(tag) {
                    return false;
                }
            }
        }

        // Check any tags (at least ONE must be present)
        if !self.any_tags.is_empty() {
            let has_any = self.any_tags.iter().any(|tag| event.metadata.tags.contains(tag));
            if !has_any {
                return false;
            }
        }

        // Check tenant ID
        if let Some(ref tenant_id) = self.tenant_id {
            if event.metadata.tenant_id.as_ref() != Some(tenant_id) {
                return false;
            }
        }

        // Check user ID
        if let Some(ref user_id) = self.user_id {
            if event.metadata.user_id.as_ref() != Some(user_id) {
                return false;
            }
        }

        true
    }

    /// Check if this filter accepts all events (no restrictions)
    pub fn is_empty(&self) -> bool {
        self.event_types.is_empty()
            && self.event_type_prefix.is_none()
            && self.min_priority.is_none()
            && self.source.is_none()
            && self.required_tags.is_empty()
            && self.any_tags.is_empty()
            && self.tenant_id.is_none()
            && self.user_id.is_none()
            && self.custom_filter.is_none()
    }
}

/// Handler configuration for subscription callbacks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionHandler {
    pub handler_id: String,
    pub handler_type: HandlerType,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub timeout_ms: u64,
    pub ordered: bool,
    pub max_concurrency: usize,
}

/// Types of event handlers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HandlerType {
    /// In-process callback function
    Callback,
    /// HTTP webhook
    Webhook,
    /// Message queue
    Queue,
    /// Custom handler
    Custom(String),
}

impl Default for SubscriptionHandler {
    fn default() -> Self {
        Self {
            handler_id: String::new(),
            handler_type: HandlerType::Callback,
            max_retries: 3,
            retry_delay_ms: 1000,
            timeout_ms: 30000,
            ordered: false,
            max_concurrency: 10,
        }
    }
}

impl SubscriptionHandler {
    pub fn callback(handler_id: impl Into<String>) -> Self {
        Self {
            handler_id: handler_id.into(),
            handler_type: HandlerType::Callback,
            ..Default::default()
        }
    }

    pub fn webhook(url: impl Into<String>) -> Self {
        Self {
            handler_id: url.into(),
            handler_type: HandlerType::Webhook,
            ..Default::default()
        }
    }

    pub fn with_retries(mut self, max_retries: u32, delay_ms: u64) -> Self {
        self.max_retries = max_retries;
        self.retry_delay_ms = delay_ms;
        self
    }

    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    pub fn ordered(mut self) -> Self {
        self.ordered = true;
        self.max_concurrency = 1;
        self
    }

    pub fn with_concurrency(mut self, max: usize) -> Self {
        self.max_concurrency = max;
        self
    }
}

/// Subscription information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub channel: String,
    pub name: Option<String>,
    pub filter: SubscriptionFilter,
    pub handler: SubscriptionHandler,
    pub active: bool,
    pub durable: bool,
    #[serde(default)]
    pub acknowledged_events: HashSet<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_event_time: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<Value>,
}

impl Subscription {
    pub fn new(channel: impl Into<String>, handler: SubscriptionHandler) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            channel: channel.into(),
            name: None,
            filter: SubscriptionFilter::default(),
            handler,
            active: true,
            durable: false,
            acknowledged_events: HashSet::new(),
            last_event_id: None,
            last_event_time: None,
            created_at: now,
            last_active_at: now,
            custom_data: None,
        }
    }

    pub fn with_id(id: impl Into<String>, channel: impl Into<String>, handler: SubscriptionHandler) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            channel: channel.into(),
            name: None,
            filter: SubscriptionFilter::default(),
            handler,
            active: true,
            durable: false,
            acknowledged_events: HashSet::new(),
            last_event_id: None,
            last_event_time: None,
            created_at: now,
            last_active_at: now,
            custom_data: None,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_filter(mut self, filter: SubscriptionFilter) -> Self {
        self.filter = filter;
        self
    }

    pub fn durable(mut self) -> Self {
        self.durable = true;
        self
    }

    /// Check if an event should be delivered to this subscription
    pub fn should_receive(&self, event: &Event) -> bool {
        if !self.active {
            return false;
        }

        // Check channel
        if event.channel != self.channel {
            return false;
        }

        // Check if already acknowledged
        if self.acknowledged_events.contains(&event.id) {
            return false;
        }

        // Check filter
        self.filter.matches(event)
    }

    /// Mark an event as acknowledged
    pub fn acknowledge(&mut self, event_id: &str) {
        self.acknowledged_events.insert(event_id.to_string());
        self.last_event_id = Some(event_id.to_string());
        self.last_event_time = Some(Utc::now());
        self.last_active_at = Utc::now();
    }

    /// Deactivate the subscription
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Reactivate the subscription
    pub fn activate(&mut self) {
        self.active = true;
        self.last_active_at = Utc::now();
    }
}

// Implement Entity trait for storage compatibility
impl harana_components_storage::Entity for Subscription {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type() -> &'static str {
        "subscription"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EventMetadata;

    fn create_test_event(event_type: &str, channel: &str) -> Event {
        Event::new(event_type, channel)
    }

    #[test]
    fn test_filter_empty_matches_all() {
        let filter = SubscriptionFilter::new();
        let event = create_test_event("user.created", "users");
        assert!(filter.matches(&event));
    }

    #[test]
    fn test_filter_by_event_types() {
        let filter = SubscriptionFilter::for_types(vec!["user.created", "user.updated"]);

        assert!(filter.matches(&create_test_event("user.created", "users")));
        assert!(filter.matches(&create_test_event("user.updated", "users")));
        assert!(!filter.matches(&create_test_event("user.deleted", "users")));
    }

    #[test]
    fn test_filter_by_prefix() {
        let filter = SubscriptionFilter::for_prefix("user.");

        assert!(filter.matches(&create_test_event("user.created", "users")));
        assert!(filter.matches(&create_test_event("user.updated", "users")));
        assert!(!filter.matches(&create_test_event("order.created", "orders")));
    }

    #[test]
    fn test_filter_by_priority() {
        let filter = SubscriptionFilter::new().with_min_priority(EventPriority::High);

        let normal_event = create_test_event("test", "channel");
        let high_event = create_test_event("test", "channel").with_priority(EventPriority::High);
        let critical_event = create_test_event("test", "channel").with_priority(EventPriority::Critical);

        assert!(!filter.matches(&normal_event));
        assert!(filter.matches(&high_event));
        assert!(filter.matches(&critical_event));
    }

    #[test]
    fn test_filter_by_tags() {
        let filter = SubscriptionFilter::new().with_required_tags(vec!["important"]);

        let event_with_tag =
            create_test_event("test", "channel").with_metadata(EventMetadata::new().with_tag("important"));
        let event_without_tag = create_test_event("test", "channel");

        assert!(filter.matches(&event_with_tag));
        assert!(!filter.matches(&event_without_tag));
    }

    #[test]
    fn test_subscription_should_receive() {
        let handler = SubscriptionHandler::callback("test_handler");
        let mut subscription =
            Subscription::new("users", handler).with_filter(SubscriptionFilter::for_types(vec!["user.created"]));

        let event1 = create_test_event("user.created", "users");
        let event2 = create_test_event("user.deleted", "users");
        let event3 = create_test_event("user.created", "orders");

        assert!(subscription.should_receive(&event1));
        assert!(!subscription.should_receive(&event2)); // wrong type
        assert!(!subscription.should_receive(&event3)); // wrong channel

        // Acknowledge event1
        subscription.acknowledge(&event1.id);
        assert!(!subscription.should_receive(&event1)); // already acknowledged
    }
}
