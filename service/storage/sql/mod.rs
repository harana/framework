// Harana Components - Storage SQL Implementation
// This module provides SQL-based storage implementations.

mod query_builder;
mod repository;
mod text_search;
mod watch;

#[cfg(feature = "postgres")]
mod json_repository;

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "sqlite")]
pub mod sqlite;

#[cfg(feature = "mysql")]
pub mod mysql;

// Re-export core types
pub use query_builder::SqlQueryBuilder;
pub use repository::SqlStore;
pub use text_search::{SqlTextSearchBackend, SqlTextSearchFilter, TextSearchConfig};
pub use watch::{ChangeOperation, TableChangeEvent, WatchableBackend};

#[cfg(feature = "postgres")]
pub use json_repository::JsonSqlStore;

#[cfg(feature = "postgres")]
pub use postgres::{MultiTableWatcher, PgStore, WatchablePgStore};

#[cfg(feature = "sqlite")]
pub use sqlite::{SqliteFtsStore, SqliteStore, WatchableSqliteStore};

#[cfg(feature = "mysql")]
pub use mysql::{MySqlStore, WatchableMySqlStore};
