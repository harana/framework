// Harana Components - MySQL Implementation

use async_trait::async_trait;
use serde_json::Value as JsonValue;
use sqlx::{mysql::MySqlRow, MySqlPool, MySql};
use std::time::Duration;

use crate::{Entity, StorageError, StorageResult, Store};
use super::{
    text_search::{SqlTextSearchFilter, SqlTextSearchBackend},
    ChangeOperation, SqlStore, TableChangeEvent, WatchableBackend,
};

pub type MySqlStore<T> = SqlStore<T, MySql>;

impl<T: Entity> MySqlStore<T>
where
    T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin,
{
    pub async fn connect(database_url: &str, table_name: &str) -> StorageResult<Self> {
        let pool = MySqlPool::connect(database_url)
            .await
            .map_err(|e| StorageError::ConnectionError(format!("Failed to connect: {}", e)))?;
        
        Ok(Self::new(pool, table_name))
    }
}

// ============================================================================
// MySQL Text Search Implementation
// ============================================================================

#[async_trait]
impl<T: Entity> Store<T> for MySqlStore<T>
where
    T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin,
{
    type Filter = SqlTextSearchFilter;

    async fn text_search(
        &self,
        _text: &str,
        _limit: Option<i64>,
        _offset: Option<u64>,
    ) -> StorageResult<Vec<T>> {
        // Default implementation requires explicit column specification
        Err(StorageError::QueryError(
            "SQL text search requires explicit column specification. Use text_search_with_filter with SqlTextSearchFilter".to_string()
        ))
    }

    async fn text_search_with_filter(
        &self,
        text: &str,
        filter: SqlTextSearchFilter,
        limit: Option<i64>,
        offset: Option<u64>,
    ) -> StorageResult<Vec<T>> {
        if filter.search_columns.is_empty() {
            return Err(StorageError::QueryError("At least one search column must be specified".to_string()));
        }

        // Build column list for MATCH clause
        let columns_list = filter.search_columns
            .iter()
            .map(|col| format!("`{}`", col))
            .collect::<Vec<_>>()
            .join(", ");

        // Build the base query using MySQL FULLTEXT search with natural language mode
        let mut query = format!(
            "SELECT *, MATCH({}) AGAINST(? IN NATURAL LANGUAGE MODE) as relevance \
             FROM `{}` \
             WHERE MATCH({}) AGAINST(? IN NATURAL LANGUAGE MODE)",
            columns_list,
            self.table_name(),
            columns_list
        );

        // Add filter conditions
        if let Some(JsonValue::Object(map)) = &filter.filters {
            for (key, _) in map {
                query.push_str(&format!(" AND `{}` = ?", key));
            }
        }

        // Add ordering by relevance
        query.push_str(" ORDER BY relevance DESC");

        // Add limit and offset
        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        // Execute the query - bind text twice (for SELECT and WHERE)
        let mut query_builder = sqlx::query_as::<_, T>(&query)
            .bind(text)
            .bind(text);

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
            .fetch_all(self.pool())
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

    // Queue operations - not supported for MySqlStore
    async fn create_queue(&self, _queue_name: &str) -> StorageResult<()> {
        Err(StorageError::NotSupported("Queue operations are not supported for MySqlStore".to_string()))
    }

    async fn queue_add<Q: serde::Serialize + Send + Sync>(
        &self,
        _queue_name: &str,
        _items: &[Q],
        _delay_secs: Option<i64>,
    ) -> StorageResult<()> {
        Err(StorageError::NotSupported("Queue operations are not supported for MySqlStore".to_string()))
    }

    async fn queue_get<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _visibility_secs: i64,
    ) -> StorageResult<Option<crate::QueueMessage<Q>>> {
        Err(StorageError::NotSupported("Queue operations are not supported for MySqlStore".to_string()))
    }

    async fn queue_ack<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _ack_id: &str,
    ) -> StorageResult<Option<Q>> {
        Err(StorageError::NotSupported("Queue operations are not supported for MySqlStore".to_string()))
    }

    async fn queue_ping<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _ack_id: &str,
        _visibility_secs: i64,
    ) -> StorageResult<Option<Q>> {
        Err(StorageError::NotSupported("Queue operations are not supported for MySqlStore".to_string()))
    }

    async fn queue_waiting_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for MySqlStore".to_string()))
    }

    async fn queue_in_flight_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for MySqlStore".to_string()))
    }

    async fn queue_completed_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for MySqlStore".to_string()))
    }

    async fn queue_total_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for MySqlStore".to_string()))
    }

    async fn queue_purge(&self, _queue_name: &str) -> StorageResult<u64> {
        Err(StorageError::NotSupported("Queue operations are not supported for MySqlStore".to_string()))
    }
}

// Blanket implementation for SqlTextSearchBackend convenience methods
impl<T: Entity> SqlTextSearchBackend<T> for MySqlStore<T>
where
    T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin,
{
}

/// MySQL-specific text search utilities
impl<T: Entity> MySqlStore<T>
where
    T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin,
{
    /// Creates a FULLTEXT index for text search on the specified columns.
    /// This should be called once during table setup.
    pub async fn create_fulltext_index(
        &self,
        index_name: &str,
        columns: &[&str],
    ) -> StorageResult<()> {
        let columns_list = columns
            .iter()
            .map(|col| format!("`{}`", col))
            .collect::<Vec<_>>()
            .join(", ");

        let query = format!(
            "CREATE FULLTEXT INDEX `{}` ON `{}`({})",
            index_name,
            self.table_name(),
            columns_list
        );

        sqlx::query(&query)
            .execute(self.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create fulltext index: {}", e)))?;

        Ok(())
    }

    /// Drops a FULLTEXT index.
    pub async fn drop_fulltext_index(&self, index_name: &str) -> StorageResult<()> {
        let query = format!("DROP INDEX `{}` ON `{}`", index_name, self.table_name());

        sqlx::query(&query)
            .execute(self.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to drop fulltext index: {}", e)))?;

        Ok(())
    }

    /// Performs a boolean mode search with more control over matching.
    /// Supports operators like + (must include), - (must exclude), * (wildcard).
    pub async fn boolean_search(
        &self,
        text: &str,
        search_columns: &[&str],
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> StorageResult<Vec<T>> {
        if search_columns.is_empty() {
            return Err(StorageError::QueryError("At least one search column must be specified".to_string()));
        }

        let columns_list = search_columns
            .iter()
            .map(|col| format!("`{}`", col))
            .collect::<Vec<_>>()
            .join(", ");

        let mut query = format!(
            "SELECT *, MATCH({}) AGAINST(? IN BOOLEAN MODE) as relevance \
             FROM `{}` \
             WHERE MATCH({}) AGAINST(? IN BOOLEAN MODE) \
             ORDER BY relevance DESC",
            columns_list,
            self.table_name(),
            columns_list
        );

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        sqlx::query_as::<_, T>(&query)
            .bind(text)
            .bind(text)
            .fetch_all(self.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Boolean search failed: {}", e)))
    }

    /// Performs a query expansion search that finds related terms.
    pub async fn query_expansion_search(
        &self,
        text: &str,
        search_columns: &[&str],
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> StorageResult<Vec<T>> {
        if search_columns.is_empty() {
            return Err(StorageError::QueryError("At least one search column must be specified".to_string()));
        }

        let columns_list = search_columns
            .iter()
            .map(|col| format!("`{}`", col))
            .collect::<Vec<_>>()
            .join(", ");

        let mut query = format!(
            "SELECT *, MATCH({}) AGAINST(? WITH QUERY EXPANSION) as relevance \
             FROM `{}` \
             WHERE MATCH({}) AGAINST(? WITH QUERY EXPANSION) \
             ORDER BY relevance DESC",
            columns_list,
            self.table_name(),
            columns_list
        );

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        sqlx::query_as::<_, T>(&query)
            .bind(text)
            .bind(text)
            .fetch_all(self.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Query expansion search failed: {}", e)))
    }
}

// ============================================================================
// MySQL Watch Implementation (Polling-based with change tracking table)
// ============================================================================

/// A watchable MySQL backend that uses a polling-based mechanism with a change tracking table.
/// 
/// Since MySQL doesn't have native LISTEN/NOTIFY like PostgreSQL, this implementation uses
/// a change tracking table and triggers to record changes, then polls for new changes.
pub struct WatchableMySqlStore<T: Entity> {
    inner: MySqlStore<T>,
    change_table_name: String,
    poll_interval: Duration,
}

impl<T: Entity> WatchableMySqlStore<T>
where
    T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin,
{
    /// Creates a new watchable backend.
    pub fn new(pool: MySqlPool, table_name: impl Into<String>) -> Self {
        let table_name = table_name.into();
        let change_table_name = format!("{}_changes", table_name.replace('`', ""));
        Self {
            inner: SqlStore::new(pool, table_name),
            change_table_name,
            poll_interval: Duration::from_millis(100),
        }
    }

    /// Creates a new watchable backend with a custom poll interval.
    pub fn with_poll_interval(pool: MySqlPool, table_name: impl Into<String>, poll_interval: Duration) -> Self {
        let mut repo = Self::new(pool, table_name);
        repo.poll_interval = poll_interval;
        repo
    }

    /// Connects to the database and creates a watchable backend.
    pub async fn connect(database_url: &str, table_name: &str) -> StorageResult<Self> {
        let pool = MySqlPool::connect(database_url)
            .await
            .map_err(|e| StorageError::ConnectionError(format!("Failed to connect: {}", e)))?;
        
        Ok(Self::new(pool, table_name))
    }

    /// Returns a reference to the underlying backend.
    pub fn inner(&self) -> &MySqlStore<T> {
        &self.inner
    }

    /// Returns the change tracking table name.
    pub fn change_table_name(&self) -> &str {
        &self.change_table_name
    }

    /// Returns a reference to the connection pool.
    pub fn pool(&self) -> &MySqlPool {
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
        format!("{}_{}_trigger", self.inner.table_name().replace('`', ""), operation)
    }
}

#[async_trait]
impl<T: Entity> WatchableBackend for WatchableMySqlStore<T>
where
    T: for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin,
{
    async fn setup_change_notifications(&self) -> StorageResult<()> {
        let table_name = self.inner.table_name();
        let change_table = &self.change_table_name;

        // Create the change tracking table
        let create_table = format!(
            "CREATE TABLE IF NOT EXISTS `{}` (
                id BIGINT AUTO_INCREMENT PRIMARY KEY,
                table_name VARCHAR(255) NOT NULL,
                operation VARCHAR(10) NOT NULL,
                row_id VARCHAR(255),
                old_data JSON,
                new_data JSON,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                processed BOOLEAN DEFAULT FALSE,
                INDEX idx_processed_created (processed, created_at)
            )",
            change_table
        );

        sqlx::query(&create_table)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create change table: {}", e)))?;

        // Create INSERT trigger
        let insert_trigger = format!(
            "CREATE TRIGGER `{}` AFTER INSERT ON `{}` FOR EACH ROW
             INSERT INTO `{}` (table_name, operation, row_id, new_data)
             VALUES ('{}', 'INSERT', NEW.id, JSON_OBJECT('data', NEW.id))",
            self.trigger_name("insert"), table_name, change_table, table_name
        );

        // Try to create the trigger, ignore if it already exists
        let _ = sqlx::query(&insert_trigger)
            .execute(self.inner.pool())
            .await;

        // Create UPDATE trigger
        let update_trigger = format!(
            "CREATE TRIGGER `{}` AFTER UPDATE ON `{}` FOR EACH ROW
             INSERT INTO `{}` (table_name, operation, row_id, old_data, new_data)
             VALUES ('{}', 'UPDATE', NEW.id, JSON_OBJECT('data', OLD.id), JSON_OBJECT('data', NEW.id))",
            self.trigger_name("update"), table_name, change_table, table_name
        );

        let _ = sqlx::query(&update_trigger)
            .execute(self.inner.pool())
            .await;

        // Create DELETE trigger
        let delete_trigger = format!(
            "CREATE TRIGGER `{}` AFTER DELETE ON `{}` FOR EACH ROW
             INSERT INTO `{}` (table_name, operation, row_id, old_data)
             VALUES ('{}', 'DELETE', OLD.id, JSON_OBJECT('data', OLD.id))",
            self.trigger_name("delete"), table_name, change_table, table_name
        );

        let _ = sqlx::query(&delete_trigger)
            .execute(self.inner.pool())
            .await;

        Ok(())
    }

    async fn teardown_change_notifications(&self) -> StorageResult<()> {
        // Drop triggers
        for op in &["insert", "update", "delete"] {
            let drop_trigger = format!("DROP TRIGGER IF EXISTS `{}`", self.trigger_name(op));
            sqlx::query(&drop_trigger)
                .execute(self.inner.pool())
                .await
                .map_err(|e| StorageError::QueryError(format!("Failed to drop {} trigger: {}", op, e)))?;
        }

        // Drop change table
        let drop_table = format!("DROP TABLE IF EXISTS `{}`", self.change_table_name);
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
            MySqlWatchState::new(pool, change_table, table_name, poll_interval),
            |mut state| async move {
                let result = state.next_event().await;
                Some((result, state))
            },
        ))
    }
}

/// Internal state for the MySQL watch stream.
struct MySqlWatchState {
    pool: MySqlPool,
    change_table: String,
    #[allow(dead_code)]
    table_name: String,
    poll_interval: Duration,
    last_id: i64,
}

impl MySqlWatchState {
    fn new(pool: MySqlPool, change_table: String, table_name: String, poll_interval: Duration) -> Self {
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
            // Query for new changes - use String types for portability
            let query = format!(
                "SELECT id, table_name, operation, row_id, old_data, new_data 
                 FROM `{}` 
                 WHERE id > ? AND processed = FALSE 
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
                let update_query = format!("UPDATE `{}` SET processed = TRUE WHERE id = ?", self.change_table);
                let _ = sqlx::query(&update_query)
                    .bind(id)
                    .execute(&self.pool)
                    .await;

                self.last_id = id;

                let op: ChangeOperation = operation.parse()?;
                
                // Parse JSON data from string
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
