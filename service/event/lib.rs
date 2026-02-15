mod error;
pub mod memory;
mod service;
#[cfg(any(feature = "durable_object", feature = "mongodb"))]
pub mod storage;

#[cfg(test)]
mod tests;

// Re-export core types
pub use error::{EventError, EventResult};
pub use memory::InMemoryEventService;
pub use service::{EventBus, EventBusConfig, EventService};

// Re-export locking types from the lock component
pub use harana_components_lock::{
    DistributedLock, DistributedLockManager, LockConfig, LockHandle, LockManager, MultiLockGuard,
    channel_lock_resource, event_lock_resource, subscription_lock_resource,
};

// Re-export persistent stores when storage features are enabled
#[cfg(feature = "durable_object")]
pub use storage::DurableObjectEventService;

#[cfg(feature = "mongodb")]
pub use storage::MongoEventService;
