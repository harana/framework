use async_trait::async_trait;
use chrono::{DateTime, Utc};
use harana_components_storage::{DOEntity, DOBindValue, DOStore, FilterCondition, QueryOptions};

use crate::{Channel, Event, EventError, EventQuery, EventResult, EventStatus, EventService, Subscription};

// ============================================================================
// DOEntity implementations for Event, Channel, Subscription
// ============================================================================

impl DOEntity for Event {
    fn columns() -> &'static [&'static str] {
        &[
            "id",
            "event_type",
            "channel",
            "payload",
            "metadata",
            "priority",
            "status",
            "delivery_attempts",
            "ttl_seconds",
            "scheduled_at",
            "created_at",
            "updated_at",
            "expires_at",
        ]
    }

    fn bind_column(&self, col: &str) -> DOBindValue {
        match col {
            "id" => DOBindValue::Text(self.id.clone()),
            "event_type" => DOBindValue::Text(self.event_type.clone()),
            "channel" => DOBindValue::Text(self.channel.clone()),
            "payload" => DOBindValue::Text(serde_json::to_string(&self.payload).unwrap_or_default()),
            "metadata" => DOBindValue::Text(serde_json::to_string(&self.metadata).unwrap_or_default()),
            "priority" => DOBindValue::Integer(self.priority.as_i32() as i64),
            "status" => DOBindValue::Text(self.status.as_str().to_string()),
            "delivery_attempts" => DOBindValue::Integer(self.delivery_attempts as i64),
            "ttl_seconds" => DOBindValue::OptInteger(self.ttl_seconds.map(|v| v as i64)),
            "scheduled_at" => DOBindValue::OptText(self.scheduled_at.map(|t| t.to_rfc3339())),
            "created_at" => DOBindValue::Text(self.created_at.to_rfc3339()),
            "updated_at" => DOBindValue::Text(self.updated_at.to_rfc3339()),
            "expires_at" => DOBindValue::OptText(self.expires_at.map(|t| t.to_rfc3339())),
            _ => DOBindValue::Null,
        }
    }
}

impl DOEntity for Channel {
    fn columns() -> &'static [&'static str] {
        &[
            "id",
            "name",
            "config",
            "subscriber_count",
            "total_events",
            "pending_events",
            "active",
        ]
    }

    fn bind_column(&self, col: &str) -> DOBindValue {
        match col {
            "id" => DOBindValue::Text(self.name.clone()),
            "name" => DOBindValue::Text(self.name.clone()),
            "config" => DOBindValue::Text(serde_json::to_string(&self.config).unwrap_or_default()),
            "subscriber_count" => DOBindValue::Integer(self.subscriber_count as i64),
            "total_events" => DOBindValue::Integer(self.total_events as i64),
            "pending_events" => DOBindValue::Integer(self.pending_events as i64),
            "active" => DOBindValue::Bool(self.active),
            _ => DOBindValue::Null,
        }
    }
}

impl DOEntity for Subscription {
    fn columns() -> &'static [&'static str] {
        &[
            "id",
            "channel",
            "name",
            "filter",
            "handler",
            "active",
            "durable",
            "acknowledged_events",
            "last_event_id",
            "last_event_time",
            "created_at",
            "last_active_at",
            "custom_data",
        ]
    }

    fn bind_column(&self, col: &str) -> DOBindValue {
        match col {
            "id" => DOBindValue::Text(self.id.clone()),
            "channel" => DOBindValue::Text(self.channel.clone()),
            "name" => DOBindValue::OptText(self.name.clone()),
            "filter" => DOBindValue::Text(serde_json::to_string(&self.filter).unwrap_or_default()),
            "handler" => DOBindValue::Text(serde_json::to_string(&self.handler).unwrap_or_default()),
            "active" => DOBindValue::Bool(self.active),
            "durable" => DOBindValue::Bool(self.durable),
            "acknowledged_events" => {
                DOBindValue::Text(serde_json::to_string(&self.acknowledged_events).unwrap_or_default())
            }
            "last_event_id" => DOBindValue::OptText(self.last_event_id.clone()),
            "last_event_time" => DOBindValue::OptText(self.last_event_time.map(|t| t.to_rfc3339())),
            "created_at" => DOBindValue::Text(self.created_at.to_rfc3339()),
            "last_active_at" => DOBindValue::Text(self.last_active_at.to_rfc3339()),
            "custom_data" => DOBindValue::OptText(self.custom_data.as_ref().map(|v| v.to_string())),
            _ => DOBindValue::Null,
        }
    }
}

// ============================================================================
// DurableObjectEventService
// ============================================================================

pub struct DurableObjectEventService {
    event_store: DOStore<Event>,
    channel_store: DOStore<Channel>,
    subscription_store: DOStore<Subscription>,
}

impl DurableObjectEventService {
    /// Create a new DurableObjectEventService from a Durable Object state.
    pub fn new(state: &worker::durable::State) -> Self {
        let sql = state.storage().sql();
        Self {
            event_store: DOStore::new(sql.clone(), "events"),
            channel_store: DOStore::new(sql.clone(), "channels"),
            subscription_store: DOStore::new(sql, "subscriptions"),
        }
    }

    /// Create a new DurableObjectEventService with custom table names.
    pub fn with_table_names(
        state: &worker::durable::State,
        events_table: &str,
        channels_table: &str,
        subscriptions_table: &str,
    ) -> Self {
        let sql = state.storage().sql();
        Self {
            event_store: DOStore::new(sql.clone(), events_table),
            channel_store: DOStore::new(sql.clone(), channels_table),
            subscription_store: DOStore::new(sql, subscriptions_table),
        }
    }

    /// Create the required tables if they don't exist.
    pub fn create_tables(&self) -> EventResult<()> {
        self.event_store
            .create_table()
            .map_err(|e| EventError::StorageError(e.to_string()))?;
        self.channel_store
            .create_table()
            .map_err(|e| EventError::StorageError(e.to_string()))?;
        self.subscription_store
            .create_table()
            .map_err(|e| EventError::StorageError(e.to_string()))?;
        Ok(())
    }

    /// Returns the database size in bytes.
    pub fn database_size(&self) -> usize {
        self.event_store.database_size()
    }
}

#[async_trait]
impl EventService for DurableObjectEventService {
    async fn store_event(&self, event: &Event) -> EventResult<()> {
        use harana_components_storage::Store;
        self.event_store
            .create(event)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn store_events(&self, events: &[Event]) -> EventResult<usize> {
        use harana_components_storage::Store;
        self.event_store
            .create_many(events)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn get_event(&self, event_id: &str) -> EventResult<Option<Event>> {
        use harana_components_storage::Store;
        self.event_store
            .find_by_id(event_id)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn query_events(&self, query: EventQuery) -> EventResult<Vec<Event>> {
        use harana_components_storage::Store;

        let mut conditions = Vec::new();

        if let Some(channel) = query.channel {
            conditions.push(FilterCondition::eq("channel", channel));
        }
        if let Some(types) = query.event_types {
            conditions.push(FilterCondition::is_in("event_type", types));
        }
        if let Some(status) = query.status {
            conditions.push(FilterCondition::eq("status", status.as_str()));
        }
        if let Some(start_time) = query.start_time {
            conditions.push(FilterCondition::gte("created_at", start_time.to_rfc3339()));
        }
        if let Some(end_time) = query.end_time {
            conditions.push(FilterCondition::lt("created_at", end_time.to_rfc3339()));
        }

        let filter = if conditions.is_empty() {
            None
        } else if conditions.len() == 1 {
            Some(conditions.remove(0))
        } else {
            Some(FilterCondition::And(conditions))
        };

        let mut options = QueryOptions::new().with_sort("created_at", !query.ascending);
        if let Some(limit) = query.limit {
            options = options.with_limit(limit as u32);
        }
        if let Some(offset) = query.offset {
            options = options.with_offset(offset as u32);
        }

        self.event_store
            .find_many(filter, options)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn count_events(&self, query: EventQuery) -> EventResult<u64> {
        use harana_components_storage::Store;

        let mut conditions = Vec::new();

        if let Some(channel) = query.channel {
            conditions.push(FilterCondition::eq("channel", channel));
        }
        if let Some(types) = query.event_types {
            conditions.push(FilterCondition::is_in("event_type", types));
        }

        let filter = if conditions.is_empty() {
            None
        } else if conditions.len() == 1 {
            Some(conditions.remove(0))
        } else {
            Some(FilterCondition::And(conditions))
        };

        self.event_store
            .count(filter)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn update_event_status(&self, event_id: &str, status: EventStatus) -> EventResult<()> {
        use harana_components_storage::Store;

        let mut event = self
            .event_store
            .find_by_id(event_id)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))?
            .ok_or_else(|| EventError::NotFound {
                event_id: event_id.to_string(),
            })?;

        event.status = status;
        event.updated_at = Utc::now();

        self.event_store
            .update(&event)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn delete_event(&self, event_id: &str) -> EventResult<bool> {
        use harana_components_storage::Store;
        self.event_store
            .delete(event_id)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn delete_events_before(&self, before: DateTime<Utc>) -> EventResult<u64> {
        use harana_components_storage::Store;
        self.event_store
            .delete_many(FilterCondition::lt("created_at", before.to_rfc3339()))
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn delete_expired_events(&self) -> EventResult<u64> {
        use harana_components_storage::Store;
        let now = Utc::now().to_rfc3339();
        self.event_store
            .delete_many(FilterCondition::And(vec![
                FilterCondition::IsNotNull("expires_at".to_string()),
                FilterCondition::lt("expires_at", now),
            ]))
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    // ========== Channel Operations ==========

    async fn upsert_channel(&self, channel: &Channel) -> EventResult<()> {
        use harana_components_storage::Store;
        self.channel_store
            .upsert(channel)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn get_channel(&self, name: &str) -> EventResult<Option<Channel>> {
        use harana_components_storage::Store;
        self.channel_store
            .find_by_id(name)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn list_channels(&self) -> EventResult<Vec<Channel>> {
        use harana_components_storage::Store;
        self.channel_store
            .find_all(QueryOptions::new())
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn delete_channel(&self, name: &str) -> EventResult<bool> {
        use harana_components_storage::Store;

        // Delete all events for this channel
        self.event_store
            .delete_many(FilterCondition::eq("channel", name))
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))?;

        // Delete all subscriptions for this channel
        self.subscription_store
            .delete_many(FilterCondition::eq("channel", name))
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))?;

        // Delete the channel
        self.channel_store
            .delete(name)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    // ========== Subscription Operations ==========

    async fn upsert_subscription(&self, subscription: &Subscription) -> EventResult<()> {
        use harana_components_storage::Store;
        self.subscription_store
            .upsert(subscription)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn get_subscription(&self, subscription_id: &str) -> EventResult<Option<Subscription>> {
        use harana_components_storage::Store;
        self.subscription_store
            .find_by_id(subscription_id)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn list_subscriptions(&self, channel: Option<&str>) -> EventResult<Vec<Subscription>> {
        use harana_components_storage::Store;

        let filter = channel.map(|c| FilterCondition::eq("channel", c));
        self.subscription_store
            .find_many(filter, QueryOptions::new())
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn delete_subscription(&self, subscription_id: &str) -> EventResult<bool> {
        use harana_components_storage::Store;
        self.subscription_store
            .delete(subscription_id)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn acknowledge_event(&self, subscription_id: &str, event_id: &str) -> EventResult<()> {
        use harana_components_storage::Store;

        let mut subscription = self
            .subscription_store
            .find_by_id(subscription_id)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))?
            .ok_or_else(|| EventError::SubscriptionNotFound {
                subscription_id: subscription_id.to_string(),
            })?;

        if subscription.acknowledged_events.contains(event_id) {
            return Err(EventError::AlreadyAcknowledged {
                event_id: event_id.to_string(),
            });
        }

        subscription.acknowledged_events.insert(event_id.to_string());
        subscription.last_event_id = Some(event_id.to_string());
        subscription.last_event_time = Some(Utc::now());
        subscription.last_active_at = Utc::now();

        self.subscription_store
            .update(&subscription)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn get_pending_events(&self, subscription_id: &str, limit: Option<usize>) -> EventResult<Vec<Event>> {
        use harana_components_storage::Store;

        let subscription = self
            .subscription_store
            .find_by_id(subscription_id)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))?
            .ok_or_else(|| EventError::SubscriptionNotFound {
                subscription_id: subscription_id.to_string(),
            })?;

        let mut options = QueryOptions::new().with_sort("created_at", false);
        if let Some(limit) = limit {
            options = options.with_limit(limit as u32);
        }

        let events = self
            .event_store
            .find_many(
                Some(FilterCondition::eq("channel", subscription.channel.clone())),
                options,
            )
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))?;

        // Filter out acknowledged events and apply subscription filter
        let pending: Vec<Event> = events
            .into_iter()
            .filter(|e| !subscription.acknowledged_events.contains(&e.id) && subscription.filter.matches(e))
            .collect();

        Ok(pending)
    }
}
