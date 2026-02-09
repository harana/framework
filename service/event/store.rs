use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::VecDeque;
use std::sync::Arc;

use crate::{Channel, Event, EventError, EventQuery, EventResult, EventStatus, EventStore, Subscription};

pub struct InMemoryEventStore {
    events: DashMap<String, Event>,
    channels: DashMap<String, Channel>,
    subscriptions: DashMap<String, Subscription>,
    // Channel -> Event IDs (ordered by creation time)
    channel_events: DashMap<String, Arc<RwLock<VecDeque<String>>>>,
    // Subscription -> Acknowledged Event IDs
    subscription_acks: DashMap<String, Arc<RwLock<std::collections::HashSet<String>>>>,
    // Configuration
    max_events_per_channel: Option<usize>,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self {
            events: DashMap::new(),
            channels: DashMap::new(),
            subscriptions: DashMap::new(),
            channel_events: DashMap::new(),
            subscription_acks: DashMap::new(),
            max_events_per_channel: None,
        }
    }

    pub fn with_max_events(mut self, max: usize) -> Self {
        self.max_events_per_channel = Some(max);
        self
    }

    fn get_or_create_channel_events(&self, channel: &str) -> Arc<RwLock<VecDeque<String>>> {
        self.channel_events
            .entry(channel.to_string())
            .or_insert_with(|| Arc::new(RwLock::new(VecDeque::new())))
            .clone()
    }

    fn get_or_create_subscription_acks(&self, subscription_id: &str) -> Arc<RwLock<std::collections::HashSet<String>>> {
        self.subscription_acks
            .entry(subscription_id.to_string())
            .or_insert_with(|| Arc::new(RwLock::new(std::collections::HashSet::new())))
            .clone()
    }

    fn enforce_max_events(&self, channel: &str) {
        if let Some(max) = self.max_events_per_channel {
            let events = self.get_or_create_channel_events(channel);
            let mut events = events.write();
            while events.len() > max {
                if let Some(event_id) = events.pop_front() {
                    self.events.remove(&event_id);
                }
            }
        }
    }
}

impl Default for InMemoryEventStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventStore for InMemoryEventStore {
    async fn store_event(&self, event: &Event) -> EventResult<()> {
        // Check for duplicate
        if self.events.contains_key(&event.id) {
            return Err(EventError::DuplicateEvent {
                event_id: event.id.clone(),
            });
        }

        // Store the event
        self.events.insert(event.id.clone(), event.clone());

        // Add to channel events
        let channel_events = self.get_or_create_channel_events(&event.channel);
        {
            let mut events = channel_events.write();
            events.push_back(event.id.clone());
        }

        // Update channel stats
        if let Some(mut channel) = self.channels.get_mut(&event.channel) {
            channel.total_events += 1;
            channel.pending_events += 1;
        }

        // Enforce max events limit
        self.enforce_max_events(&event.channel);

        Ok(())
    }

    async fn store_events(&self, events: &[Event]) -> EventResult<usize> {
        let mut count = 0;
        for event in events {
            if self.store_event(event).await.is_ok() {
                count += 1;
            }
        }
        Ok(count)
    }

    async fn get_event(&self, event_id: &str) -> EventResult<Option<Event>> {
        Ok(self.events.get(event_id).map(|e| e.clone()))
    }

    async fn query_events(&self, query: EventQuery) -> EventResult<Vec<Event>> {
        let mut events: Vec<Event> = self
            .events
            .iter()
            .map(|e| e.clone())
            .filter(|e| {
                // Filter by channel
                if let Some(ref channel) = query.channel {
                    if &e.channel != channel {
                        return false;
                    }
                }

                // Filter by event types
                if let Some(ref types) = query.event_types {
                    if !types.contains(&e.event_type) {
                        return false;
                    }
                }

                // Filter by status
                if let Some(status) = query.status {
                    if e.status != status {
                        return false;
                    }
                }

                // Filter by time range
                if let Some(start) = query.start_time {
                    if e.created_at < start {
                        return false;
                    }
                }
                if let Some(end) = query.end_time {
                    if e.created_at >= end {
                        return false;
                    }
                }

                true
            })
            .collect();

        // Sort by created_at
        if query.ascending {
            events.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        } else {
            events.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        }

        // Apply offset and limit
        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(usize::MAX);

        Ok(events.into_iter().skip(offset).take(limit).collect())
    }

    async fn count_events(&self, query: EventQuery) -> EventResult<u64> {
        let events = self
            .query_events(EventQuery {
                limit: None,
                offset: None,
                ..query
            })
            .await?;
        Ok(events.len() as u64)
    }

    async fn update_event_status(&self, event_id: &str, status: EventStatus) -> EventResult<()> {
        let mut event = self.events.get_mut(event_id).ok_or_else(|| EventError::NotFound {
            event_id: event_id.to_string(),
        })?;

        event.status = status;
        event.updated_at = Utc::now();
        Ok(())
    }

    async fn delete_event(&self, event_id: &str) -> EventResult<bool> {
        if let Some((_, event)) = self.events.remove(event_id) {
            // Remove from channel events
            if let Some(channel_events) = self.channel_events.get(&event.channel) {
                let mut events = channel_events.write();
                events.retain(|id| id != event_id);
            }

            // Update channel stats
            if let Some(mut channel) = self.channels.get_mut(&event.channel) {
                channel.pending_events = channel.pending_events.saturating_sub(1);
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn delete_events_before(&self, before: DateTime<Utc>) -> EventResult<u64> {
        let to_delete: Vec<String> = self
            .events
            .iter()
            .filter(|e| e.created_at < before)
            .map(|e| e.id.clone())
            .collect();

        let count = to_delete.len() as u64;
        for id in to_delete {
            self.delete_event(&id).await?;
        }
        Ok(count)
    }

    async fn delete_expired_events(&self) -> EventResult<u64> {
        let now = Utc::now();
        let to_delete: Vec<String> = self
            .events
            .iter()
            .filter(|e| e.expires_at.map(|exp| exp < now).unwrap_or(false))
            .map(|e| e.id.clone())
            .collect();

        let count = to_delete.len() as u64;
        for id in to_delete {
            self.delete_event(&id).await?;
        }
        Ok(count)
    }

    async fn upsert_channel(&self, channel: &Channel) -> EventResult<()> {
        self.channels.insert(channel.name.clone(), channel.clone());
        Ok(())
    }

    async fn get_channel(&self, name: &str) -> EventResult<Option<Channel>> {
        Ok(self.channels.get(name).map(|c| c.clone()))
    }

    async fn list_channels(&self) -> EventResult<Vec<Channel>> {
        Ok(self.channels.iter().map(|c| c.clone()).collect())
    }

    async fn delete_channel(&self, name: &str) -> EventResult<bool> {
        // Delete all events for this channel
        if let Some(channel_events) = self.channel_events.remove(name) {
            let events = channel_events.1.read();
            for event_id in events.iter() {
                self.events.remove(event_id);
            }
        }

        // Delete subscriptions for this channel
        let sub_ids: Vec<String> = self
            .subscriptions
            .iter()
            .filter(|s| s.channel == name)
            .map(|s| s.id.clone())
            .collect();

        for sub_id in sub_ids {
            self.subscriptions.remove(&sub_id);
            self.subscription_acks.remove(&sub_id);
        }

        // Delete the channel
        Ok(self.channels.remove(name).is_some())
    }

    async fn upsert_subscription(&self, subscription: &Subscription) -> EventResult<()> {
        self.subscriptions.insert(subscription.id.clone(), subscription.clone());
        Ok(())
    }

    async fn get_subscription(&self, subscription_id: &str) -> EventResult<Option<Subscription>> {
        Ok(self.subscriptions.get(subscription_id).map(|s| s.clone()))
    }

    async fn list_subscriptions(&self, channel: Option<&str>) -> EventResult<Vec<Subscription>> {
        let subs: Vec<Subscription> = self
            .subscriptions
            .iter()
            .filter(|s| channel.map(|c| s.channel == c).unwrap_or(true))
            .map(|s| s.clone())
            .collect();
        Ok(subs)
    }

    async fn delete_subscription(&self, subscription_id: &str) -> EventResult<bool> {
        self.subscription_acks.remove(subscription_id);
        Ok(self.subscriptions.remove(subscription_id).is_some())
    }

    async fn acknowledge_event(&self, subscription_id: &str, event_id: &str) -> EventResult<()> {
        // Verify subscription exists
        if !self.subscriptions.contains_key(subscription_id) {
            return Err(EventError::SubscriptionNotFound {
                subscription_id: subscription_id.to_string(),
            });
        }

        // Verify event exists
        if !self.events.contains_key(event_id) {
            return Err(EventError::NotFound {
                event_id: event_id.to_string(),
            });
        }

        // Add to acknowledged events
        let acks = self.get_or_create_subscription_acks(subscription_id);
        {
            let mut acks = acks.write();
            if acks.contains(event_id) {
                return Err(EventError::AlreadyAcknowledged {
                    event_id: event_id.to_string(),
                });
            }
            acks.insert(event_id.to_string());
        }

        // Update subscription
        if let Some(mut sub) = self.subscriptions.get_mut(subscription_id) {
            sub.acknowledged_events.insert(event_id.to_string());
            sub.last_event_id = Some(event_id.to_string());
            sub.last_event_time = Some(Utc::now());
            sub.last_active_at = Utc::now();
        }

        Ok(())
    }

    async fn get_pending_events(&self, subscription_id: &str, limit: Option<usize>) -> EventResult<Vec<Event>> {
        let subscription = self
            .subscriptions
            .get(subscription_id)
            .ok_or_else(|| EventError::SubscriptionNotFound {
                subscription_id: subscription_id.to_string(),
            })?;

        let acks = self.get_or_create_subscription_acks(subscription_id);
        let acks = acks.read();

        let channel_events = self.channel_events.get(&subscription.channel).map(|e| e.clone());

        let mut events = Vec::new();

        if let Some(channel_events) = channel_events {
            let event_ids = channel_events.read();
            for event_id in event_ids.iter() {
                // Skip acknowledged events
                if acks.contains(event_id) {
                    continue;
                }

                if let Some(event) = self.events.get(event_id) {
                    // Check if event matches subscription filter
                    if subscription.should_receive(&event) {
                        events.push(event.clone());
                        if let Some(limit) = limit {
                            if events.len() >= limit {
                                break;
                            }
                        }
                    }
                }
            }
        }

        Ok(events)
    }
}

// ============================================================================
// Persistent Event Store (requires storage feature)
// ============================================================================

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite", feature = "mongodb"))]
mod persistent {
    use super::*;
    use harana_components_storage::{Entity, FilterCondition, QueryOptions, StorageResult, Store};
    use std::marker::PhantomData;

    pub struct PersistentEventStore<S>
    where
        S: Store<Event> + Store<Channel> + Store<Subscription>,
    {
        store: S,
        _phantom: PhantomData<(Event, Channel, Subscription)>,
    }

    impl<S> PersistentEventStore<S>
    where
        S: Store<Event> + Store<Channel> + Store<Subscription>,
    {
        pub fn new(store: S) -> Self {
            Self {
                store,
                _phantom: PhantomData,
            }
        }
    }

    #[async_trait]
    impl<S> EventStore for PersistentEventStore<S>
    where
        S: Store<Event> + Store<Channel> + Store<Subscription> + Send + Sync,
    {
        async fn store_event(&self, event: &Event) -> EventResult<()> {
            let store: &dyn Store<Event, Filter = <S as Store<Event>>::Filter> = &self.store;
            store
                .create(event)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn store_events(&self, events: &[Event]) -> EventResult<usize> {
            let store: &dyn Store<Event, Filter = <S as Store<Event>>::Filter> = &self.store;
            store
                .create_many(events)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn get_event(&self, event_id: &str) -> EventResult<Option<Event>> {
            let store: &dyn Store<Event, Filter = <S as Store<Event>>::Filter> = &self.store;
            store
                .find_by_id(event_id)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn query_events(&self, query: EventQuery) -> EventResult<Vec<Event>> {
            let store: &dyn Store<Event, Filter = <S as Store<Event>>::Filter> = &self.store;

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

            store
                .find_many(filter, options)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn count_events(&self, query: EventQuery) -> EventResult<u64> {
            let store: &dyn Store<Event, Filter = <S as Store<Event>>::Filter> = &self.store;

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

            store
                .count(filter)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn update_event_status(&self, event_id: &str, status: EventStatus) -> EventResult<()> {
            let store: &dyn Store<Event, Filter = <S as Store<Event>>::Filter> = &self.store;

            let mut event = store
                .find_by_id(event_id)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))?
                .ok_or_else(|| EventError::NotFound {
                    event_id: event_id.to_string(),
                })?;

            event.status = status;
            event.updated_at = Utc::now();

            store
                .update(&event)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn delete_event(&self, event_id: &str) -> EventResult<bool> {
            let store: &dyn Store<Event, Filter = <S as Store<Event>>::Filter> = &self.store;
            store
                .delete(event_id)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn delete_events_before(&self, before: DateTime<Utc>) -> EventResult<u64> {
            let store: &dyn Store<Event, Filter = <S as Store<Event>>::Filter> = &self.store;
            store
                .delete_many(FilterCondition::lt("created_at", before.to_rfc3339()))
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn delete_expired_events(&self) -> EventResult<u64> {
            let store: &dyn Store<Event, Filter = <S as Store<Event>>::Filter> = &self.store;
            let now = Utc::now().to_rfc3339();
            store
                .delete_many(FilterCondition::And(vec![
                    FilterCondition::IsNotNull("expires_at".to_string()),
                    FilterCondition::lt("expires_at", now),
                ]))
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn upsert_channel(&self, channel: &Channel) -> EventResult<()> {
            let store: &dyn Store<Channel, Filter = <S as Store<Channel>>::Filter> = &self.store;
            store
                .upsert(channel)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn get_channel(&self, name: &str) -> EventResult<Option<Channel>> {
            let store: &dyn Store<Channel, Filter = <S as Store<Channel>>::Filter> = &self.store;
            store
                .find_by_id(name)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn list_channels(&self) -> EventResult<Vec<Channel>> {
            let store: &dyn Store<Channel, Filter = <S as Store<Channel>>::Filter> = &self.store;
            store
                .find_all(QueryOptions::new())
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn delete_channel(&self, name: &str) -> EventResult<bool> {
            // First delete all events for this channel
            let event_store: &dyn Store<Event, Filter = <S as Store<Event>>::Filter> = &self.store;
            event_store
                .delete_many(FilterCondition::eq("channel", name))
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))?;

            // Delete all subscriptions for this channel
            let sub_store: &dyn Store<Subscription, Filter = <S as Store<Subscription>>::Filter> = &self.store;
            sub_store
                .delete_many(FilterCondition::eq("channel", name))
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))?;

            // Delete the channel
            let channel_store: &dyn Store<Channel, Filter = <S as Store<Channel>>::Filter> = &self.store;
            channel_store
                .delete(name)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn upsert_subscription(&self, subscription: &Subscription) -> EventResult<()> {
            let store: &dyn Store<Subscription, Filter = <S as Store<Subscription>>::Filter> = &self.store;
            store
                .upsert(subscription)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn get_subscription(&self, subscription_id: &str) -> EventResult<Option<Subscription>> {
            let store: &dyn Store<Subscription, Filter = <S as Store<Subscription>>::Filter> = &self.store;
            store
                .find_by_id(subscription_id)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn list_subscriptions(&self, channel: Option<&str>) -> EventResult<Vec<Subscription>> {
            let store: &dyn Store<Subscription, Filter = <S as Store<Subscription>>::Filter> = &self.store;

            let filter = channel.map(|c| FilterCondition::eq("channel", c));

            store
                .find_many(filter, QueryOptions::new())
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn delete_subscription(&self, subscription_id: &str) -> EventResult<bool> {
            let store: &dyn Store<Subscription, Filter = <S as Store<Subscription>>::Filter> = &self.store;
            store
                .delete(subscription_id)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn acknowledge_event(&self, subscription_id: &str, event_id: &str) -> EventResult<()> {
            let store: &dyn Store<Subscription, Filter = <S as Store<Subscription>>::Filter> = &self.store;

            let mut subscription = store
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

            store
                .update(&subscription)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))
        }

        async fn get_pending_events(&self, subscription_id: &str, limit: Option<usize>) -> EventResult<Vec<Event>> {
            let sub_store: &dyn Store<Subscription, Filter = <S as Store<Subscription>>::Filter> = &self.store;
            let subscription = sub_store
                .find_by_id(subscription_id)
                .await
                .map_err(|e| EventError::StorageError(e.to_string()))?
                .ok_or_else(|| EventError::SubscriptionNotFound {
                    subscription_id: subscription_id.to_string(),
                })?;

            let event_store: &dyn Store<Event, Filter = <S as Store<Event>>::Filter> = &self.store;

            let mut options = QueryOptions::new().with_sort("created_at", false);

            if let Some(limit) = limit {
                options = options.with_limit(limit as u32);
            }

            let events = event_store
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
}

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite", feature = "mongodb"))]
pub use persistent::PersistentEventStore;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_store_basic_operations() {
        let store = InMemoryEventStore::new();

        // Create a channel
        let channel = Channel::new("test-channel");
        store.upsert_channel(&channel).await.unwrap();

        // Store an event
        let event = Event::new("test.event", "test-channel").with_payload(serde_json::json!({"key": "value"}));
        store.store_event(&event).await.unwrap();

        // Retrieve the event
        let retrieved = store.get_event(&event.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().event_type, "test.event");

        // Query events
        let events = store
            .query_events(EventQuery::for_channel("test-channel"))
            .await
            .unwrap();
        assert_eq!(events.len(), 1);

        // Delete the event
        let deleted = store.delete_event(&event.id).await.unwrap();
        assert!(deleted);

        let retrieved = store.get_event(&event.id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_in_memory_store_subscriptions() {
        let store = InMemoryEventStore::new();

        // Create a channel and subscription
        let channel = Channel::new("test-channel");
        store.upsert_channel(&channel).await.unwrap();

        let subscription = Subscription::new("test-channel", crate::SubscriptionHandler::callback("test"));
        store.upsert_subscription(&subscription).await.unwrap();

        // Store an event
        let event = Event::new("test.event", "test-channel");
        store.store_event(&event).await.unwrap();

        // Get pending events
        let pending = store.get_pending_events(&subscription.id, None).await.unwrap();
        assert_eq!(pending.len(), 1);

        // Acknowledge the event
        store.acknowledge_event(&subscription.id, &event.id).await.unwrap();

        // Pending should be empty now
        let pending = store.get_pending_events(&subscription.id, None).await.unwrap();
        assert_eq!(pending.len(), 0);
    }

    #[tokio::test]
    async fn test_in_memory_store_max_events() {
        let store = InMemoryEventStore::new().with_max_events(3);

        let channel = Channel::new("test-channel");
        store.upsert_channel(&channel).await.unwrap();

        // Store 5 events
        for i in 0..5 {
            let event = Event::new(format!("test.event.{}", i), "test-channel");
            store.store_event(&event).await.unwrap();
        }

        // Should only have 3 events (oldest ones removed)
        let events = store
            .query_events(EventQuery::for_channel("test-channel"))
            .await
            .unwrap();
        assert_eq!(events.len(), 3);
    }
}
