use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::{Channel, Event, EventQuery, EventResult, EventStatus, Subscription};

#[async_trait]
pub trait EventStore: Send + Sync {
    async fn store_event(&self, event: &Event) -> EventResult<()>;
    async fn store_events(&self, events: &[Event]) -> EventResult<usize>;
    async fn get_event(&self, event_id: &str) -> EventResult<Option<Event>>;
    async fn query_events(&self, query: EventQuery) -> EventResult<Vec<Event>>;
    async fn count_events(&self, query: EventQuery) -> EventResult<u64>;
    async fn update_event_status(&self, event_id: &str, status: EventStatus) -> EventResult<()>;
    async fn delete_event(&self, event_id: &str) -> EventResult<bool>;
    async fn delete_events_before(&self, before: DateTime<Utc>) -> EventResult<u64>;
    async fn delete_expired_events(&self) -> EventResult<u64>;

    async fn upsert_channel(&self, channel: &Channel) -> EventResult<()>;
    async fn get_channel(&self, name: &str) -> EventResult<Option<Channel>>;
    async fn list_channels(&self) -> EventResult<Vec<Channel>>;
    async fn delete_channel(&self, name: &str) -> EventResult<bool>;

    async fn upsert_subscription(&self, subscription: &Subscription) -> EventResult<()>;
    async fn get_subscription(&self, subscription_id: &str) -> EventResult<Option<Subscription>>;
    async fn list_subscriptions(&self, channel: Option<&str>) -> EventResult<Vec<Subscription>>;
    async fn delete_subscription(&self, subscription_id: &str) -> EventResult<bool>;

    async fn acknowledge_event(&self, subscription_id: &str, event_id: &str) -> EventResult<()>;
    async fn get_pending_events(&self, subscription_id: &str, limit: Option<usize>) -> EventResult<Vec<Event>>;
}
