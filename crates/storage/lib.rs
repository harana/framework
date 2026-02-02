// Harana Components - Storage
// Generic CRUD interface with type support for SQL and MongoDB implementations.

mod entity;
mod error;
mod filter;
mod repository;

#[cfg(feature = "mongodb")]
pub mod mongodb;

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
pub mod sql;

#[cfg(test)]
mod tests;

// ============================================================================
// Core Types (always available)
// ============================================================================

pub use entity::Entity;
pub use error::{StorageError, StorageResult};
pub use filter::{FilterCondition, QueryOptions};
pub use repository::{QueueMessage, QueueStats, Store, StoreExt};

// ============================================================================
// SQL Types (available with any SQL feature)
// ============================================================================

#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
pub use sql::{
    ChangeOperation, SqlTextSearchBackend, SqlTextSearchFilter, TableChangeEvent, TextSearchConfig, WatchableBackend,
};

#[cfg(feature = "postgres")]
pub use sql::postgres::{MultiTableWatcher, WatchablePgBackend};

#[cfg(feature = "mysql")]
pub use sql::mysql::WatchableMySqlBackend;

#[cfg(feature = "sqlite")]
pub use sql::sqlite::{SqliteFtsBackend, WatchableSqliteBackend};

// ============================================================================
// MongoDB Types (available with mongodb feature)
// ============================================================================

#[cfg(feature = "mongodb")]
pub use mongodb::{
    ChangeOperation as MongoChangeOperation, CollectionChangeEvent, UpdateDescription as MongoUpdateDescription,
    WatchOptions as MongoWatchOptions,
};
