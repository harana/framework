use async_trait::async_trait;
use chrono::{DateTime, Utc};
use harana_components_storage::{FilterCondition, MongoStore, QueryOptions, Store};
use mongodb::{Client, Database};

use crate::{Channel, Event, EventError, EventQuery, EventResult, EventStatus, EventService, Subscription};

// ============================================================================
// MongoEventService
// ============================================================================

pub struct MongoEventService {
    event_store: MongoStore<Event>,
    channel_store: MongoStore<Channel>,
    subscription_store: MongoStore<Subscription>,
}

impl MongoEventService {
    /// Create a new MongoEventService from a MongoDB client and database.
    pub fn new(client: Client, database: &Database) -> Self {
        Self {
            event_store: MongoStore::new(client.clone(), database, "events"),
            channel_store: MongoStore::new(client.clone(), database, "channels"),
            subscription_store: MongoStore::new(client, database, "subscriptions"),
        }
    }

    /// Create a new MongoEventService with custom collection names.
    pub fn with_collection_names(
        client: Client,
        database: &Database,
        events_collection: &str,
        channels_collection: &str,
        subscriptions_collection: &str,
    ) -> Self {
        Self {
            event_store: MongoStore::new(client.clone(), database, events_collection),
            channel_store: MongoStore::new(client.clone(), database, channels_collection),
            subscription_store: MongoStore::new(client, database, subscriptions_collection),
        }
    }

    /// Connect to a MongoDB instance and create the store.
    pub async fn connect(url: &str, database_name: &str) -> EventResult<Self> {
        let client_options = mongodb::options::ClientOptions::parse(url)
            .await
            .map_err(|e| EventError::StorageError(format!("Failed to parse MongoDB URL: {}", e)))?;

        let client = Client::with_options(client_options)
            .map_err(|e| EventError::StorageError(format!("Failed to create MongoDB client: {}", e)))?;

        let database = client.database(database_name);
        Ok(Self::new(client, &database))
    }

    /// Get a reference to the underlying event store.
    pub fn event_store(&self) -> &MongoStore<Event> {
        &self.event_store
    }

    /// Get a reference to the underlying channel store.
    pub fn channel_store(&self) -> &MongoStore<Channel> {
        &self.channel_store
    }

    /// Get a reference to the underlying subscription store.
    pub fn subscription_store(&self) -> &MongoStore<Subscription> {
        &self.subscription_store
    }
}

#[async_trait]
impl EventService for MongoEventService {
    async fn store_event(&self, event: &Event) -> EventResult<()> {
        self.event_store
            .create(event)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn store_events(&self, events: &[Event]) -> EventResult<usize> {
        self.event_store
            .create_many(events)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn get_event(&self, event_id: &str) -> EventResult<Option<Event>> {
        self.event_store
            .find_by_id(event_id)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn query_events(&self, query: EventQuery) -> EventResult<Vec<Event>> {
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
        self.event_store
            .delete(event_id)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn delete_events_before(&self, before: DateTime<Utc>) -> EventResult<u64> {
        self.event_store
            .delete_many(FilterCondition::lt("created_at", before.to_rfc3339()))
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn delete_expired_events(&self) -> EventResult<u64> {
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
        self.channel_store
            .upsert(channel)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn get_channel(&self, name: &str) -> EventResult<Option<Channel>> {
        self.channel_store
            .find_by_id(name)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn list_channels(&self) -> EventResult<Vec<Channel>> {
        self.channel_store
            .find_all(QueryOptions::new())
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn delete_channel(&self, name: &str) -> EventResult<bool> {
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
        self.subscription_store
            .upsert(subscription)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn get_subscription(&self, subscription_id: &str) -> EventResult<Option<Subscription>> {
        self.subscription_store
            .find_by_id(subscription_id)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn list_subscriptions(&self, channel: Option<&str>) -> EventResult<Vec<Subscription>> {
        let filter = channel.map(|c| FilterCondition::eq("channel", c));
        self.subscription_store
            .find_many(filter, QueryOptions::new())
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn delete_subscription(&self, subscription_id: &str) -> EventResult<bool> {
        self.subscription_store
            .delete(subscription_id)
            .await
            .map_err(|e| EventError::StorageError(e.to_string()))
    }

    async fn acknowledge_event(&self, subscription_id: &str, event_id: &str) -> EventResult<()> {
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
