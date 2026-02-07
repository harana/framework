// Harana Components - PostgreSQL Implementation

use async_trait::async_trait;
use serde_json::Value as JsonValue;
use sqlx::{postgres::{PgListener, PgRow}, PgPool, Postgres};

use crate::{Entity, StorageError, StorageResult, Store};
use super::{
    text_search::{SqlTextSearchFilter, SqlTextSearchBackend},
    ChangeOperation, SqlStore, TableChangeEvent, WatchableBackend,
};

pub type PgStore<T> = SqlStore<T, Postgres>;

impl<T: Entity> PgStore<T>
where
    T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
{
    pub async fn connect(database_url: &str, table_name: &str) -> StorageResult<Self> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(|e| StorageError::ConnectionError(format!("Failed to connect: {}", e)))?;
        
        Ok(Self::new(pool, table_name))
    }
}

/// A watchable PostgreSQL backend that supports LISTEN/NOTIFY for change detection.
pub struct WatchablePgStore<T: Entity> {
    inner: PgStore<T>,
    channel_name: String,
}

impl<T: Entity> WatchablePgStore<T>
where
    T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
{
    /// Creates a new watchable backend with a custom notification channel name.
    pub fn new(pool: PgPool, table_name: impl Into<String>) -> Self {
        let table_name = table_name.into();
        let channel_name = format!("{}_changes", table_name.replace('"', ""));
        Self {
            inner: SqlStore::new(pool, table_name),
            channel_name,
        }
    }

    /// Creates a new watchable backend with a custom channel name.
    pub fn with_channel(pool: PgPool, table_name: impl Into<String>, channel_name: impl Into<String>) -> Self {
        Self {
            inner: SqlStore::new(pool, table_name.into()),
            channel_name: channel_name.into(),
        }
    }

    /// Connects to the database and creates a watchable backend.
    pub async fn connect(database_url: &str, table_name: &str) -> StorageResult<Self> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(|e| StorageError::ConnectionError(format!("Failed to connect: {}", e)))?;
        
        Ok(Self::new(pool, table_name))
    }

    /// Returns a reference to the underlying backend.
    pub fn inner(&self) -> &PgStore<T> {
        &self.inner
    }

    /// Returns the notification channel name.
    pub fn channel_name(&self) -> &str {
        &self.channel_name
    }

    /// Returns a reference to the connection pool.
    pub fn pool(&self) -> &PgPool {
        self.inner.pool()
    }

    /// Returns the table name.
    pub fn table_name(&self) -> &str {
        self.inner.table_name()
    }

    fn trigger_function_name(&self) -> String {
        format!("{}_notify_trigger", self.inner.table_name().replace('"', ""))
    }

    fn trigger_name(&self) -> String {
        format!("{}_notify", self.inner.table_name().replace('"', ""))
    }
}

#[async_trait]
impl<T: Entity> WatchableBackend for WatchablePgStore<T>
where
    T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
{
    async fn setup_change_notifications(&self) -> StorageResult<()> {
        let function_name = self.trigger_function_name();
        let trigger_name = self.trigger_name();
        let table_name = self.inner.table_name();
        let channel_name = &self.channel_name;

        // Create the notification function
        let create_function = format!(
            r#"
            CREATE OR REPLACE FUNCTION "{}"()
            RETURNS TRIGGER AS $$
            DECLARE
                payload JSONB;
                row_id TEXT;
            BEGIN
                IF TG_OP = 'DELETE' THEN
                    row_id := OLD.id::TEXT;
                    payload := jsonb_build_object(
                        'table', TG_TABLE_NAME,
                        'operation', TG_OP,
                        'row_id', row_id,
                        'old_data', to_jsonb(OLD)
                    );
                ELSIF TG_OP = 'UPDATE' THEN
                    row_id := NEW.id::TEXT;
                    payload := jsonb_build_object(
                        'table', TG_TABLE_NAME,
                        'operation', TG_OP,
                        'row_id', row_id,
                        'old_data', to_jsonb(OLD),
                        'new_data', to_jsonb(NEW)
                    );
                ELSE
                    row_id := NEW.id::TEXT;
                    payload := jsonb_build_object(
                        'table', TG_TABLE_NAME,
                        'operation', TG_OP,
                        'row_id', row_id,
                        'new_data', to_jsonb(NEW)
                    );
                END IF;
                
                PERFORM pg_notify('{}', payload::TEXT);
                
                IF TG_OP = 'DELETE' THEN
                    RETURN OLD;
                ELSE
                    RETURN NEW;
                END IF;
            END;
            $$ LANGUAGE plpgsql;
            "#,
            function_name, channel_name
        );

        sqlx::query(&create_function)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create trigger function: {}", e)))?;

        // Create the trigger
        let create_trigger = format!(
            r#"
            DROP TRIGGER IF EXISTS "{}" ON "{}";
            CREATE TRIGGER "{}"
            AFTER INSERT OR UPDATE OR DELETE ON "{}"
            FOR EACH ROW
            EXECUTE FUNCTION "{}"();
            "#,
            trigger_name, table_name, trigger_name, table_name, function_name
        );

        sqlx::query(&create_trigger)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create trigger: {}", e)))?;

        Ok(())
    }

    async fn teardown_change_notifications(&self) -> StorageResult<()> {
        let function_name = self.trigger_function_name();
        let trigger_name = self.trigger_name();
        let table_name = self.inner.table_name();

        // Drop the trigger
        let drop_trigger = format!(
            r#"DROP TRIGGER IF EXISTS "{}" ON "{}""#,
            trigger_name, table_name
        );

        sqlx::query(&drop_trigger)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to drop trigger: {}", e)))?;

        // Drop the function
        let drop_function = format!(
            r#"DROP FUNCTION IF EXISTS "{}"()"#,
            function_name
        );

        sqlx::query(&drop_function)
            .execute(self.inner.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to drop trigger function: {}", e)))?;

        Ok(())
    }

    fn watch_changes(
        &self,
    ) -> std::pin::Pin<Box<dyn futures::Stream<Item = StorageResult<TableChangeEvent>> + Send + '_>> {
        let channel_name = self.channel_name.clone();
        let pool = self.inner.pool().clone();
        let table_name = self.inner.table_name().to_string();

        Box::pin(futures::stream::unfold(
            WatchState::new(pool, channel_name, table_name),
            |mut state| async move {
                let result = state.next_event().await;
                Some((result, state))
            },
        ))
    }
}

/// Internal state for the watch stream.
struct WatchState {
    pool: PgPool,
    channel_name: String,
    table_name: String,
    listener: Option<PgListener>,
}

impl WatchState {
    fn new(pool: PgPool, channel_name: String, table_name: String) -> Self {
        Self {
            pool,
            channel_name,
            table_name,
            listener: None,
        }
    }

    async fn next_event(&mut self) -> StorageResult<TableChangeEvent> {
        // Initialize listener if needed
        if self.listener.is_none() {
            let mut listener = PgListener::connect_with(&self.pool)
                .await
                .map_err(|e| StorageError::ConnectionError(format!("Failed to create listener: {}", e)))?;
            
            listener.listen(&self.channel_name)
                .await
                .map_err(|e| StorageError::ConnectionError(format!("Failed to listen to channel: {}", e)))?;
            
            self.listener = Some(listener);
        }

        // Wait for next notification
        let listener = self.listener.as_mut().unwrap();
        let notification = listener.recv()
            .await
            .map_err(|e| {
                // Reset listener on error to allow reconnection
                self.listener = None;
                StorageError::ConnectionError(format!("Failed to receive notification: {}", e))
            })?;

        parse_notification_payload(notification.payload(), &self.table_name)
    }
}

/// Parses a notification payload from PostgreSQL NOTIFY.
fn parse_notification_payload(payload: &str, default_table: &str) -> StorageResult<TableChangeEvent> {
    let json: serde_json::Value = serde_json::from_str(payload)
        .map_err(|e| StorageError::SerializationError(format!("Failed to parse notification payload: {}", e)))?;

    let table_name = json.get("table")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| default_table.to_string());

    let operation_str = json.get("operation")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StorageError::SerializationError("Missing operation field".to_string()))?;

    let operation: ChangeOperation = operation_str.parse()?;

    let row_id = json.get("row_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    // Get the relevant data based on operation
    let payload_data = match operation {
        ChangeOperation::Delete => json.get("old_data").cloned(),
        _ => json.get("new_data").cloned(),
    };

    Ok(TableChangeEvent::new(table_name, operation, row_id, payload_data))
}

/// A convenience struct for watching multiple tables with a single listener.
pub struct MultiTableWatcher {
    pool: PgPool,
    channels: Vec<String>,
}

impl MultiTableWatcher {
    /// Creates a new multi-table watcher.
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            channels: Vec::new(),
        }
    }

    /// Adds a channel to watch.
    pub fn add_channel(mut self, channel: impl Into<String>) -> Self {
        self.channels.push(channel.into());
        self
    }

    /// Starts watching all registered channels and returns a stream of events.
    pub fn watch(
        self,
    ) -> std::pin::Pin<Box<dyn futures::Stream<Item = StorageResult<TableChangeEvent>> + Send>> {
        Box::pin(futures::stream::unfold(
            MultiWatchState::new(self.pool, self.channels),
            |mut state| async move {
                let result = state.next_event().await;
                Some((result, state))
            },
        ))
    }
}

/// Internal state for the multi-table watch stream.
struct MultiWatchState {
    pool: PgPool,
    channels: Vec<String>,
    listener: Option<PgListener>,
}

impl MultiWatchState {
    fn new(pool: PgPool, channels: Vec<String>) -> Self {
        Self {
            pool,
            channels,
            listener: None,
        }
    }

    async fn next_event(&mut self) -> StorageResult<TableChangeEvent> {
        // Initialize listener if needed
        if self.listener.is_none() {
            let mut listener = PgListener::connect_with(&self.pool)
                .await
                .map_err(|e| StorageError::ConnectionError(format!("Failed to create listener: {}", e)))?;
            
            for channel in &self.channels {
                listener.listen(channel)
                    .await
                    .map_err(|e| StorageError::ConnectionError(format!("Failed to listen to channel {}: {}", channel, e)))?;
            }
            
            self.listener = Some(listener);
        }

        // Wait for next notification
        let listener = self.listener.as_mut().unwrap();
        let notification = listener.recv()
            .await
            .map_err(|e| {
                // Reset listener on error to allow reconnection
                self.listener = None;
                StorageError::ConnectionError(format!("Failed to receive notification: {}", e))
            })?;

        parse_notification_payload(notification.payload(), "unknown")
    }
}

// ============================================================================
// PostgreSQL Text Search Implementation
// ============================================================================

#[async_trait]
impl<T: Entity> Store<T> for PgStore<T>
where
    T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
{
    type Filter = SqlTextSearchFilter;

    async fn text_search(
        &self,
        _text: &str,
        _limit: Option<i64>,
        _offset: Option<u64>,
    ) -> StorageResult<Vec<T>> {
        // Default to searching all text columns - requires at least one column to be specified
        // Users should use text_search_with_filter with SqlTextSearchFilter for explicit column selection
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

        // Build the tsvector expression from columns
        let tsvector_expr = filter.search_columns
            .iter()
            .map(|col| format!("coalesce(\"{}\", '')", col))
            .collect::<Vec<_>>()
            .join(" || ' ' || ");

        // Build the base query using PostgreSQL full-text search
        let mut query = format!(
            "SELECT *, ts_rank(to_tsvector('english', {}), plainto_tsquery('english', $1)) as rank \
             FROM \"{}\" \
             WHERE to_tsvector('english', {}) @@ plainto_tsquery('english', $1)",
            tsvector_expr,
            self.table_name(),
            tsvector_expr
        );

        // Add filter conditions
        if let Some(JsonValue::Object(map)) = &filter.filters {
            for (key, _) in map {
                query.push_str(&format!(" AND \"{}\" = ${}", key, 2));
            }
        }

        // Add ordering by relevance
        query.push_str(" ORDER BY rank DESC");

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

    // Queue operations - not supported for PgStore
    async fn create_queue(&self, _queue_name: &str) -> StorageResult<()> {
        Err(StorageError::NotSupported("Queue operations are not supported for PgStore".to_string()))
    }

    async fn queue_add<Q: serde::Serialize + Send + Sync>(
        &self,
        _queue_name: &str,
        _items: &[Q],
        _delay_secs: Option<i64>,
    ) -> StorageResult<()> {
        Err(StorageError::NotSupported("Queue operations are not supported for PgStore".to_string()))
    }

    async fn queue_get<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _visibility_secs: i64,
    ) -> StorageResult<Option<crate::QueueMessage<Q>>> {
        Err(StorageError::NotSupported("Queue operations are not supported for PgStore".to_string()))
    }

    async fn queue_ack<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _ack_id: &str,
    ) -> StorageResult<Option<Q>> {
        Err(StorageError::NotSupported("Queue operations are not supported for PgStore".to_string()))
    }

    async fn queue_ping<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _ack_id: &str,
        _visibility_secs: i64,
    ) -> StorageResult<Option<Q>> {
        Err(StorageError::NotSupported("Queue operations are not supported for PgStore".to_string()))
    }

    async fn queue_waiting_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for PgStore".to_string()))
    }

    async fn queue_in_flight_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for PgStore".to_string()))
    }

    async fn queue_completed_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for PgStore".to_string()))
    }

    async fn queue_total_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for PgStore".to_string()))
    }

    async fn queue_purge(&self, _queue_name: &str) -> StorageResult<u64> {
        Err(StorageError::NotSupported("Queue operations are not supported for PgStore".to_string()))
    }
}

// Blanket implementation for SqlTextSearchBackend convenience methods
impl<T: Entity> SqlTextSearchBackend<T> for PgStore<T>
where
    T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
{
}

/// PostgreSQL-specific text search utilities
impl<T: Entity> PgStore<T>
where
    T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
{
    /// Creates a GIN index for full-text search on the specified columns.
    /// This should be called once during table setup.
    pub async fn create_text_search_index(
        &self,
        index_name: &str,
        columns: &[&str],
        language: &str,
    ) -> StorageResult<()> {
        let tsvector_expr = columns
            .iter()
            .map(|col| format!("coalesce(\"{}\", '')", col))
            .collect::<Vec<_>>()
            .join(" || ' ' || ");

        let query = format!(
            "CREATE INDEX IF NOT EXISTS \"{}\" ON \"{}\" USING GIN (to_tsvector('{}', {}))",
            index_name,
            self.table_name(),
            language,
            tsvector_expr
        );

        sqlx::query(&query)
            .execute(self.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create text search index: {}", e)))?;

        Ok(())
    }

    /// Drops a text search index.
    pub async fn drop_text_search_index(&self, index_name: &str) -> StorageResult<()> {
        let query = format!("DROP INDEX IF EXISTS \"{}\"", index_name);

        sqlx::query(&query)
            .execute(self.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to drop text search index: {}", e)))?;

        Ok(())
    }

    /// Performs a phrase search (words must appear in order).
    pub async fn phrase_search(
        &self,
        phrase: &str,
        search_columns: &[&str],
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> StorageResult<Vec<T>> {
        if search_columns.is_empty() {
            return Err(StorageError::QueryError("At least one search column must be specified".to_string()));
        }

        let tsvector_expr = search_columns
            .iter()
            .map(|col| format!("coalesce(\"{}\", '')", col))
            .collect::<Vec<_>>()
            .join(" || ' ' || ");

        let mut query = format!(
            "SELECT *, ts_rank(to_tsvector('english', {}), phraseto_tsquery('english', $1)) as rank \
             FROM \"{}\" \
             WHERE to_tsvector('english', {}) @@ phraseto_tsquery('english', $1) \
             ORDER BY rank DESC",
            tsvector_expr,
            self.table_name(),
            tsvector_expr
        );

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        sqlx::query_as::<_, T>(&query)
            .bind(phrase)
            .fetch_all(self.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Phrase search failed: {}", e)))
    }

    /// Performs a prefix search (useful for autocomplete).
    pub async fn prefix_search(
        &self,
        prefix: &str,
        search_columns: &[&str],
        limit: Option<i64>,
    ) -> StorageResult<Vec<T>> {
        if search_columns.is_empty() {
            return Err(StorageError::QueryError("At least one search column must be specified".to_string()));
        }

        let tsvector_expr = search_columns
            .iter()
            .map(|col| format!("coalesce(\"{}\", '')", col))
            .collect::<Vec<_>>()
            .join(" || ' ' || ");

        // Use prefix matching with :*
        let tsquery = format!("{}:*", prefix.replace(' ', " & "));

        let mut query = format!(
            "SELECT *, ts_rank(to_tsvector('english', {}), to_tsquery('english', $1)) as rank \
             FROM \"{}\" \
             WHERE to_tsvector('english', {}) @@ to_tsquery('english', $1) \
             ORDER BY rank DESC",
            tsvector_expr,
            self.table_name(),
            tsvector_expr
        );

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        sqlx::query_as::<_, T>(&query)
            .bind(&tsquery)
            .fetch_all(self.pool())
            .await
            .map_err(|e| StorageError::QueryError(format!("Prefix search failed: {}", e)))
    }
}
