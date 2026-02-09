mod bus;
mod channel;
mod error;
mod event;
mod event_query;
mod event_store_trait;
mod store;
mod subscription;

#[cfg(test)]
mod tests;

// Re-export core types
pub use bus::{EventBus, EventBusConfig};
pub use channel::{Channel, ChannelConfig};
pub use error::{EventError, EventResult};
pub use event::{Event, EventMetadata, EventPriority, EventStatus};
pub use event_query::EventQuery;
pub use event_store_trait::EventStore;
pub use store::InMemoryEventStore;
pub use subscription::{Subscription, SubscriptionFilter, SubscriptionHandler};

// Re-export locking types from the lock component
pub use harana_components_lock::{
    DistributedLock, DistributedLockManager, LockConfig, LockHandle, LockManager, MultiLockGuard,
    channel_lock_resource, event_lock_resource, subscription_lock_resource,
};

// Re-export persistent store when storage features are enabled
#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite", feature = "mongodb"))]
pub use store::PersistentEventStore;
