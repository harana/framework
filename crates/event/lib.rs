// Harana Components - Events System
// A unified event system suitable for both client and server-side applications.
// Supports in-memory storage for clients and persistent storage via the storage component.

mod bus;
mod channel;
mod error;
mod event;
mod store;
mod subscription;

#[cfg(test)]
mod tests;

// Re-export core types
pub use bus::{EventBus, EventBusConfig};
pub use channel::{Channel, ChannelConfig};
pub use error::{EventError, EventResult};
pub use event::{Event, EventMetadata, EventPriority, EventStatus};
pub use store::{EventStore, InMemoryEventStore};
pub use subscription::{Subscription, SubscriptionFilter, SubscriptionHandler};

// Re-export locking types from the lock component
pub use harana_components_lock::{
    DistributedLock, DistributedLockManager, LockConfig, LockHandle, LockManager, MultiLockGuard,
    channel_lock_resource, event_lock_resource, subscription_lock_resource,
};

// Re-export persistent store when storage features are enabled
#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite", feature = "mongodb"))]
pub use store::PersistentEventStore;
