// Harana Components - SQLite Implementation

use async_trait::async_trait;
use serde_json::Value as JsonValue;
use sqlx::{sqlite::SqliteRow, SqlitePool, Sqlite};
use std::time::Duration;

use crate::{Entity, StorageError, StorageResult, Store};
use super::{
    text_search::{SqlTextSearchFilter, SqlTextSearchBackend},
    ChangeOperation, SqlStore, TableChangeEvent, WatchableBackend,
};

pub type SqliteStore<T> = SqlStore<T, Sqlite>;

impl<T: Entity> SqliteStore<T>
where
    T: for<'r> sqlx::FromRow<'r, SqliteRow> + Send + Unpin,
{
    pub async fn connect(database_url: &str, table_name: &str) -> StorageResult<Self> {
        let pool = SqlitePool::connect(database_url)
            .await
            .map_err(|e| StorageError::ConnectionError(format!("Failed to connect: {}", e)))?;
        
        Ok(Self::new(pool, table_name))
    }
}

// ============================================================================
// SQLite Text Search Implementation (using FTS5)
// ============================================================================

/// SQLite FTS5 text search wrapper.
pub struct SqliteFtsStore<T: Entity> {
    inner: SqliteStore<T>,
    fts_table_name: String,
}

impl<T: Entity> SqliteFtsStore<T>
where
    T: for<'r> sqlx::FromRow<'r, SqliteRow> + Send + Unpin,
{
    /// Creates a new FTS backend wrapper.
    pub fn new(pool: SqlitePool, table_name: impl Into<String>) -> Self {
        let table_name = table_name.into();
        let fts_table_name = format!("{}_fts", table_name);
        Self {
            inner: SqlStore::new(pool, table_name),
            fts_table_name,
        }
    }

    /// Creates a new FTS backend with a custom FTS table name.
    pub fn with_fts_table(pool: SqlitePool, table_name: impl Into<String>, fts_table_name: impl Into<String>) -> Self {
        Self {
            inner: SqlStore::new(pool, table_name.into()),
            fts_table_name: fts_table_name.into(),
        }
    }

    /// Connects to the database and creates an FTS backend.
    pub async fn connect(database_url: &str, table_name: &str) -> StorageResult<Self> {
        let pool = SqlitePool::connect(database_url)
            .await
            .map_err(|e| StorageError::ConnectionError(format!("Failed to connect: {}", e)))?;
        
        Ok(Self::new(pool, table_name))
    }

    /// Returns the inner backend.
    pub fn inner(&self) -> &SqliteStore<T> {
        &self.inner
    }

    /// Returns the FTS table name.
    pub fn fts_table_name(&self) -> &str {
        &self.fts_table_name
    }

    /// Returns the pool.
    pub fn pool(&self) -> &SqlitePool {
        self.inner.pool()
    }

    /// Returns the main table name.
    pub fn table_name(&self) -> &str {
        self.inner.table_name()
    }

    /// Creates an FTS5 virtual table for full-text search.
    /// The FTS5 table will mirror the specified columns from the main table.
    pub async fn create_fts_table(&self, columns: &[&str]) -> StorageResult<()> {
        let columns_list = columns.join(", ");

        // Create FTS5 virtual table with content sync
        let query = format!(
            "CREATE VIRTUAL TABLE IF NOT EXISTS \"{}\" USING fts5({}, content=\"{}\", content_rowid=\"rowid\")",
            self.fts_table_name,
            columns_list,
            self.inner.table_name()
        );

        sqlx::query(&query)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create FTS5 table: {}", e)))?;

        // Create triggers to keep FTS table in sync
        self.create_fts_triggers(columns).await?;

        Ok(())
    }

    /// Creates triggers to keep the FTS table synchronized with the main table.
    async fn create_fts_triggers(&self, columns: &[&str]) -> StorageResult<()> {
        let table_name = self.inner.table_name();
        let fts_table = &self.fts_table_name;
        let columns_list = columns.join(", ");
        let new_columns = columns
            .iter()
            .map(|col| format!("NEW.{}", col))
            .collect::<Vec<_>>()
            .join(", ");
        let old_columns = columns
            .iter()
            .map(|col| format!("OLD.{}", col))
            .collect::<Vec<_>>()
            .join(", ");

        // Insert trigger
        let insert_trigger = format!(
            "CREATE TRIGGER IF NOT EXISTS \"{}_ai\" AFTER INSERT ON \"{}\" BEGIN \
                INSERT INTO \"{}\"(rowid, {}) VALUES (NEW.rowid, {}); \
            END",
            fts_table, table_name, fts_table, columns_list, new_columns
        );

        // Delete trigger
        let delete_trigger = format!(
            "CREATE TRIGGER IF NOT EXISTS \"{}_ad\" AFTER DELETE ON \"{}\" BEGIN \
                INSERT INTO \"{}\"(\"{}\", rowid, {}) VALUES ('delete', OLD.rowid, {}); \
            END",
            fts_table, table_name, fts_table, fts_table, columns_list, old_columns
        );

        // Update trigger
        let update_trigger = format!(
            "CREATE TRIGGER IF NOT EXISTS \"{}_au\" AFTER UPDATE ON \"{}\" BEGIN \
                INSERT INTO \"{}\"(\"{}\", rowid, {}) VALUES ('delete', OLD.rowid, {}); \
                INSERT INTO \"{}\"(rowid, {}) VALUES (NEW.rowid, {}); \
            END",
            fts_table, table_name, fts_table, fts_table, columns_list, old_columns,
            fts_table, columns_list, new_columns
        );

        sqlx::query(&insert_trigger)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create insert trigger: {}", e)))?;

        sqlx::query(&delete_trigger)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create delete trigger: {}", e)))?;

        sqlx::query(&update_trigger)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create update trigger: {}", e)))?;

        Ok(())
    }

    /// Drops the FTS5 table and its triggers.
    pub async fn drop_fts_table(&self) -> StorageResult<()> {
        let fts_table = &self.fts_table_name;

        // Drop triggers
        sqlx::query(&format!("DROP TRIGGER IF EXISTS \"{}_ai\"", fts_table))
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to drop insert trigger: {}", e)))?;

        sqlx::query(&format!("DROP TRIGGER IF EXISTS \"{}_ad\"", fts_table))
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to drop delete trigger: {}", e)))?;

        sqlx::query(&format!("DROP TRIGGER IF EXISTS \"{}_au\"", fts_table))
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to drop update trigger: {}", e)))?;

        // Drop FTS table
        sqlx::query(&format!("DROP TABLE IF EXISTS \"{}\"", fts_table))
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to drop FTS table: {}", e)))?;

        Ok(())
    }

    /// Rebuilds the FTS index from the main table.
    pub async fn rebuild_fts_index(&self) -> StorageResult<()> {
        let query = format!("INSERT INTO \"{}\"(\"{}\") VALUES ('rebuild')", self.fts_table_name, self.fts_table_name);

        sqlx::query(&query)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to rebuild FTS index: {}", e)))?;

        Ok(())
    }
}

#[async_trait]
impl<T: Entity> Store<T> for SqliteFtsStore<T>
where
    T: for<'r> sqlx::FromRow<'r, SqliteRow> + Send + Unpin,
{
    type Filter = SqlTextSearchFilter;

    async fn text_search(
        &self,
        text: &str,
        limit: Option<i64>,
        offset: Option<u64>,
    ) -> StorageResult<Vec<T>> {
        self.text_search_with_filter(text, SqlTextSearchFilter::default(), limit, offset).await
    }

    async fn text_search_with_filter(
        &self,
        text: &str,
        filter: SqlTextSearchFilter,
        limit: Option<i64>,
        offset: Option<u64>,
    ) -> StorageResult<Vec<T>> {
        // Build the query joining the main table with the FTS table
        // Note: For SQLite FTS5, search_columns are defined by the FTS table, so we ignore them here
        let mut query = format!(
            "SELECT t.*, bm25(\"{}\") as rank \
             FROM \"{}\" t \
             INNER JOIN \"{}\" fts ON t.rowid = fts.rowid \
             WHERE \"{}\" MATCH ?",
            self.fts_table_name,
            self.inner.table_name(),
            self.fts_table_name,
            self.fts_table_name
        );

        // Add filter conditions
        if let Some(JsonValue::Object(map)) = &filter.filters {
            for (key, _) in map {
                query.push_str(&format!(" AND t.\"{}\" = ?", key));
            }
        }

        // Add ordering by relevance (bm25 returns negative values, so we order ASC)
        query.push_str(" ORDER BY rank");

        // Add limit and offset
        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        // Execute the query
        let mut query_builder = sqlx::query_as::<_, T>(&query).bind(text);

        // Bind filter values
        if let Some(JsonValue::Object(map)) = &filter.filters {
            for (_, value) in map {
                match value {
                    JsonValue::String(s) => {
                        query_builder = query_builder.bind(s.clone());
                    }
                    JsonValue::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            query_builder = query_builder.bind(i);
                        } else if let Some(f) = n.as_f64() {
                            query_builder = query_builder.bind(f);
                        }
                    }
                    JsonValue::Bool(b) => {
                        query_builder = query_builder.bind(*b);
                    }
                    _ => {}
                }
            }
        }

        query_builder
            .fetch_all(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Text search failed: {}", e)))
    }

    // CRUD operations - not supported (use SqlStore directly or implement as needed)
    async fn create(&self, _entity: &T) -> StorageResult<()> {
        Err(StorageError::NotSupported("CRUD operations require a full SqlStore implementation".to_string()))
    }

    async fn create_many(&self, _entities: &[T]) -> StorageResult<usize> {
        Err(StorageError::NotSupported("CRUD operations require a full SqlStore implementation".to_string()))
    }

    async fn find_by_id(&self, _id: &str) -> StorageResult<Option<T>> {
        Err(StorageError::NotSupported("CRUD operations require a full SqlStore implementation".to_string()))
    }

    async fn find_many(&self, _filter: Option<crate::FilterCondition>, _options: crate::QueryOptions) -> StorageResult<Vec<T>> {
        Err(StorageError::NotSupported("CRUD operations require a full SqlStore implementation".to_string()))
    }

    async fn count(&self, _filter: Option<crate::FilterCondition>) -> StorageResult<u64> {
        Err(StorageError::NotSupported("CRUD operations require a full SqlStore implementation".to_string()))
    }

    async fn update(&self, _entity: &T) -> StorageResult<()> {
        Err(StorageError::NotSupported("CRUD operations require a full SqlStore implementation".to_string()))
    }

    async fn upsert(&self, _entity: &T) -> StorageResult<()> {
        Err(StorageError::NotSupported("CRUD operations require a full SqlStore implementation".to_string()))
    }

    async fn update_many(&self, _filter: crate::FilterCondition, _updates: serde_json::Map<String, serde_json::Value>) -> StorageResult<u64> {
        Err(StorageError::NotSupported("CRUD operations require a full SqlStore implementation".to_string()))
    }

    async fn delete(&self, _id: &str) -> StorageResult<bool> {
        Err(StorageError::NotSupported("CRUD operations require a full SqlStore implementation".to_string()))
    }

    async fn delete_many(&self, _filter: crate::FilterCondition) -> StorageResult<u64> {
        Err(StorageError::NotSupported("CRUD operations require a full SqlStore implementation".to_string()))
    }

    async fn delete_all(&self) -> StorageResult<u64> {
        Err(StorageError::NotSupported("CRUD operations require a full SqlStore implementation".to_string()))
    }

    // Queue operations - not supported for SqliteFtsStore
    async fn create_queue(&self, _queue_name: &str) -> StorageResult<()> {
        Err(StorageError::NotSupported("Queue operations are not supported for SqliteFtsStore".to_string()))
    }

    async fn queue_add<Q: serde::Serialize + Send + Sync>(
        &self,
        _queue_name: &str,
        _items: &[Q],
        _delay_secs: Option<i64>,
    ) -> StorageResult<()> {
        Err(StorageError::NotSupported("Queue operations are not supported for SqliteFtsStore".to_string()))
    }

    async fn queue_get<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _visibility_secs: i64,
    ) -> StorageResult<Option<crate::QueueMessage<Q>>> {
        Err(StorageError::NotSupported("Queue operations are not supported for SqliteFtsStore".to_string()))
    }

    async fn queue_ack<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _ack_id: &str,
    ) -> StorageResult<Option<Q>> {
        Err(StorageError::NotSupported("Queue operations are not supported for SqliteFtsStore".to_string()))
    }

    async fn queue_ping<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _ack_id: &str,
        _visibility_secs: i64,
    ) -> StorageResult<Option<Q>> {
        Err(StorageError::NotSupported("Queue operations are not supported for SqliteFtsStore".to_string()))
    }

    async fn queue_waiting_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for SqliteFtsStore".to_string()))
    }

    async fn queue_in_flight_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for SqliteFtsStore".to_string()))
    }

    async fn queue_completed_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for SqliteFtsStore".to_string()))
    }

    async fn queue_total_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for SqliteFtsStore".to_string()))
    }

    async fn queue_purge(&self, _queue_name: &str) -> StorageResult<u64> {
        Err(StorageError::NotSupported("Queue operations are not supported for SqliteFtsStore".to_string()))
    }
}

// Blanket implementation for SqlTextSearchBackend convenience methods
impl<T: Entity> SqlTextSearchBackend<T> for SqliteFtsStore<T>
where
    T: for<'r> sqlx::FromRow<'r, SqliteRow> + Send + Unpin,
{
}

impl<T: Entity> SqliteFtsStore<T>
where
    T: for<'r> sqlx::FromRow<'r, SqliteRow> + Send + Unpin,
{
    /// Performs a prefix search (useful for autocomplete).
    /// Use * at the end of words for prefix matching.
    pub async fn prefix_search(
        &self,
        prefix: &str,
        limit: Option<i64>,
    ) -> StorageResult<Vec<T>> {
        let search_query = format!("{}*", prefix);

        let mut query = format!(
            "SELECT t.*, bm25(\"{}\") as rank \
             FROM \"{}\" t \
             INNER JOIN \"{}\" fts ON t.rowid = fts.rowid \
             WHERE \"{}\" MATCH ? \
             ORDER BY rank",
            self.fts_table_name,
            self.inner.table_name(),
            self.fts_table_name,
            self.fts_table_name
        );

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        sqlx::query_as::<_, T>(&query)
            .bind(&search_query)
            .fetch_all(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Prefix search failed: {}", e)))
    }

    /// Performs a phrase search (words must appear in order).
    pub async fn phrase_search(
        &self,
        phrase: &str,
        limit: Option<i64>,
    ) -> StorageResult<Vec<T>> {
        let search_query = format!("\"{}\"", phrase);

        let mut query = format!(
            "SELECT t.*, bm25(\"{}\") as rank \
             FROM \"{}\" t \
             INNER JOIN \"{}\" fts ON t.rowid = fts.rowid \
             WHERE \"{}\" MATCH ? \
             ORDER BY rank",
            self.fts_table_name,
            self.inner.table_name(),
            self.fts_table_name,
            self.fts_table_name
        );

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        sqlx::query_as::<_, T>(&query)
            .bind(&search_query)
            .fetch_all(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Phrase search failed: {}", e)))
    }

    /// Performs a NEAR search (words must appear close to each other).
    pub async fn near_search(
        &self,
        words: &[&str],
        max_distance: Option<u32>,
        limit: Option<i64>,
    ) -> StorageResult<Vec<T>> {
        let distance = max_distance.unwrap_or(10);
        let search_query = format!("NEAR({}, {})", words.join(" "), distance);

        let mut query = format!(
            "SELECT t.*, bm25(\"{}\") as rank \
             FROM \"{}\" t \
             INNER JOIN \"{}\" fts ON t.rowid = fts.rowid \
             WHERE \"{}\" MATCH ? \
             ORDER BY rank",
            self.fts_table_name,
            self.inner.table_name(),
            self.fts_table_name,
            self.fts_table_name
        );

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        sqlx::query_as::<_, T>(&query)
            .bind(&search_query)
            .fetch_all(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("NEAR search failed: {}", e)))
    }
}

// ============================================================================
// SQLite Watch Implementation (Polling-based with change tracking table)
// ============================================================================

/// A watchable SQLite backend that uses a polling-based mechanism with a change tracking table.
/// 
/// Since SQLite doesn't have native LISTEN/NOTIFY like PostgreSQL, this implementation uses
/// a change tracking table and triggers to record changes, then polls for new changes.
pub struct WatchableSqliteStore<T: Entity> {
    inner: SqliteStore<T>,
    change_table_name: String,
    poll_interval: Duration,
}

impl<T: Entity> WatchableSqliteStore<T>
where
    T: for<'r> sqlx::FromRow<'r, SqliteRow> + Send + Unpin,
{
    /// Creates a new watchable backend.
    pub fn new(pool: SqlitePool, table_name: impl Into<String>) -> Self {
        let table_name = table_name.into();
        let change_table_name = format!("{}_changes", table_name.replace('"', ""));
        Self {
            inner: SqlStore::new(pool, table_name),
            change_table_name,
            poll_interval: Duration::from_millis(100),
        }
    }

    /// Creates a new watchable backend with a custom poll interval.
    pub fn with_poll_interval(pool: SqlitePool, table_name: impl Into<String>, poll_interval: Duration) -> Self {
        let mut repo = Self::new(pool, table_name);
        repo.poll_interval = poll_interval;
        repo
    }

    /// Connects to the database and creates a watchable backend.
    pub async fn connect(database_url: &str, table_name: &str) -> StorageResult<Self> {
        let pool = SqlitePool::connect(database_url)
            .await
            .map_err(|e| StorageError::ConnectionError(format!("Failed to connect: {}", e)))?;
        
        Ok(Self::new(pool, table_name))
    }

    /// Returns a reference to the underlying backend.
    pub fn inner(&self) -> &SqliteStore<T> {
        &self.inner
    }

    /// Returns the change tracking table name.
    pub fn change_table_name(&self) -> &str {
        &self.change_table_name
    }

    /// Returns a reference to the connection pool.
    pub fn pool(&self) -> &SqlitePool {
        self.inner.pool()
    }

    /// Returns the table name.
    pub fn table_name(&self) -> &str {
        self.inner.table_name()
    }

    /// Returns the poll interval.
    pub fn poll_interval(&self) -> Duration {
        self.poll_interval
    }

    fn trigger_name(&self, operation: &str) -> String {
        format!("{}_{}_trigger", self.inner.table_name().replace('"', ""), operation)
    }
}

#[async_trait]
impl<T: Entity> WatchableBackend for WatchableSqliteStore<T>
where
    T: for<'r> sqlx::FromRow<'r, SqliteRow> + Send + Unpin,
{
    async fn setup_change_notifications(&self) -> StorageResult<()> {
        let table_name = self.inner.table_name();
        let change_table = &self.change_table_name;

        // Create the change tracking table
        let create_table = format!(
            "CREATE TABLE IF NOT EXISTS \"{}\" (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                table_name TEXT NOT NULL,
                operation TEXT NOT NULL,
                row_id TEXT,
                old_data TEXT,
                new_data TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                processed INTEGER DEFAULT 0
            )",
            change_table
        );

        sqlx::query(&create_table)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create change table: {}", e)))?;

        // Create index for efficient polling
        let create_index = format!(
            "CREATE INDEX IF NOT EXISTS \"idx_{}_processed\" ON \"{}\" (processed, created_at)",
            change_table, change_table
        );

        sqlx::query(&create_index)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create index: {}", e)))?;

        // Create INSERT trigger
        let insert_trigger = format!(
            "CREATE TRIGGER IF NOT EXISTS \"{}\" AFTER INSERT ON \"{}\"
             BEGIN
                 INSERT INTO \"{}\" (table_name, operation, row_id, new_data)
                 VALUES ('{}', 'INSERT', NEW.id, json_object('id', NEW.id));
             END",
            self.trigger_name("insert"), table_name, change_table, table_name
        );

        sqlx::query(&insert_trigger)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create insert trigger: {}", e)))?;

        // Create UPDATE trigger
        let update_trigger = format!(
            "CREATE TRIGGER IF NOT EXISTS \"{}\" AFTER UPDATE ON \"{}\"
             BEGIN
                 INSERT INTO \"{}\" (table_name, operation, row_id, old_data, new_data)
                 VALUES ('{}', 'UPDATE', NEW.id, json_object('id', OLD.id), json_object('id', NEW.id));
             END",
            self.trigger_name("update"), table_name, change_table, table_name
        );

        sqlx::query(&update_trigger)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create update trigger: {}", e)))?;

        // Create DELETE trigger
        let delete_trigger = format!(
            "CREATE TRIGGER IF NOT EXISTS \"{}\" AFTER DELETE ON \"{}\"
             BEGIN
                 INSERT INTO \"{}\" (table_name, operation, row_id, old_data)
                 VALUES ('{}', 'DELETE', OLD.id, json_object('id', OLD.id));
             END",
            self.trigger_name("delete"), table_name, change_table, table_name
        );

        sqlx::query(&delete_trigger)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create delete trigger: {}", e)))?;

        Ok(())
    }

    async fn teardown_change_notifications(&self) -> StorageResult<()> {
        // Drop triggers
        for op in &["insert", "update", "delete"] {
            let drop_trigger = format!("DROP TRIGGER IF EXISTS \"{}\"", self.trigger_name(op));
            sqlx::query(&drop_trigger)
                .execute(self.inner.pool())
                .await
                .map_err(|e| StorageError::QueryError(format!("Failed to drop {} trigger: {}", op, e)))?;
        }

        // Drop change table
        let drop_table = format!("DROP TABLE IF EXISTS \"{}\"", self.change_table_name);
        sqlx::query(&drop_table)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to drop change table: {}", e)))?;

        Ok(())
    }

    fn watch_changes(
        &self,
    ) -> std::pin::Pin<Box<dyn futures::Stream<Item = StorageResult<TableChangeEvent>> + Send + '_>> {
        let pool = self.inner.pool().clone();
        let change_table = self.change_table_name.clone();
        let table_name = self.inner.table_name().to_string();
        let poll_interval = self.poll_interval;

        Box::pin(futures::stream::unfold(
            SqliteWatchState::new(pool, change_table, table_name, poll_interval),
            |mut state| async move {
                let result = state.next_event().await;
                Some((result, state))
            },
        ))
    }
}

/// Internal state for the SQLite watch stream.
struct SqliteWatchState {
    pool: SqlitePool,
    change_table: String,
    #[allow(dead_code)]
    table_name: String,
    poll_interval: Duration,
    last_id: i64,
}

impl SqliteWatchState {
    fn new(pool: SqlitePool, change_table: String, table_name: String, poll_interval: Duration) -> Self {
        Self {
            pool,
            change_table,
            table_name,
            poll_interval,
            last_id: 0,
        }
    }

    async fn next_event(&mut self) -> StorageResult<TableChangeEvent> {
        loop {
            // Query for new changes (without created_at to avoid type issues)
            let query = format!(
                "SELECT id, table_name, operation, row_id, old_data, new_data 
                 FROM \"{}\" 
                 WHERE id > ? AND processed = 0 
                 ORDER BY id ASC 
                 LIMIT 1",
                self.change_table
            );

            let row: Option<(i64, String, String, Option<String>, Option<String>, Option<String>)> = 
                sqlx::query_as(&query)
                    .bind(self.last_id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| StorageError::QueryError(format!("Failed to poll for changes: {}", e)))?;

            if let Some((id, tbl_name, operation, row_id, old_data, new_data)) = row {
                // Mark as processed
                let update_query = format!("UPDATE \"{}\" SET processed = 1 WHERE id = ?", self.change_table);
                let _ = sqlx::query(&update_query)
                    .bind(id)
                    .execute(&self.pool)
                    .await;

                self.last_id = id;

                let op: ChangeOperation = operation.parse()?;
                
                // Parse JSON data
                let payload = match op {
                    ChangeOperation::Delete => old_data.and_then(|s| serde_json::from_str(&s).ok()),
                    _ => new_data.and_then(|s| serde_json::from_str(&s).ok()),
                };

                return Ok(TableChangeEvent::new(tbl_name, op, row_id, payload));
            }

            // No new events, wait before polling again
            futures_timer::Delay::new(self.poll_interval).await;
        }
    }
}

impl<T: Entity> WatchableSqliteStore<T>
where
    T: for<'r> sqlx::FromRow<'r, SqliteRow> + Send + Unpin,
{
    /// Purges processed change records older than the specified duration.
    pub async fn purge_old_changes(&self, older_than: Duration) -> StorageResult<u64> {
        let threshold = chrono::Utc::now() - chrono::Duration::from_std(older_than)
            .map_err(|e| StorageError::InternalError(format!("Invalid duration: {}", e)))?;

        let query = format!(
            "DELETE FROM \"{}\" WHERE processed = 1 AND created_at < ?",
            self.change_table_name
        );

        let result = sqlx::query(&query)
            .bind(threshold.to_rfc3339())
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to purge old changes: {}", e)))?;

        Ok(result.rows_affected())
    }

    /// Gets the count of unprocessed changes.
    pub async fn unprocessed_count(&self) -> StorageResult<i64> {
        let query = format!(
            "SELECT COUNT(*) FROM \"{}\" WHERE processed = 0",
            self.change_table_name
        );

        let count: (i64,) = sqlx::query_as(&query)
            .fetch_one(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to count unprocessed changes: {}", e)))?;

        Ok(count.0)
    }
}
