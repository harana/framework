mod query_builder;

#[cfg(test)]
mod tests;

use async_trait::async_trait;
use serde_json::{Map, Value as JsonValue};
use std::marker::PhantomData;
use worker::{SqlCursor, SqlStorage, SqlStorageValue};

use crate::{Entity, FilterCondition, QueryOptions, QueueMessage, StorageError, StorageResult, Store};
use query_builder::DOQueryBuilder;

/// Trait that entities must implement to work with DOStore.
///
/// This extends the base `Entity` trait with column metadata so that
/// INSERT / UPDATE statements can be generated at runtime.
pub trait DOEntity: Entity {
    /// Returns the ordered list of column names that map to struct fields.
    /// The first column should be `"id"`.
    fn columns() -> &'static [&'static str];

    /// Returns the value for a given column, ready for binding to a query.
    fn bind_column(&self, col: &str) -> DOBindValue;
}

#[derive(Debug, Clone)]
pub enum DOBindValue {
    Text(String),
    Integer(i64),
    Real(f64),
    Bool(bool),
    Null,
    OptText(Option<String>),
    OptInteger(Option<i64>),
}

impl DOBindValue {
    /// Helper to create a `DOBindValue` from a `serde_json::Value`.
    pub fn from_json(v: &JsonValue) -> Self {
        match v {
            JsonValue::String(s) => DOBindValue::Text(s.clone()),
            JsonValue::Number(n) => {
                if let Some(i) = n.as_i64() {
                    DOBindValue::Integer(i)
                } else if let Some(f) = n.as_f64() {
                    DOBindValue::Real(f)
                } else {
                    DOBindValue::Text(n.to_string())
                }
            }
            JsonValue::Bool(b) => DOBindValue::Bool(*b),
            JsonValue::Null => DOBindValue::Null,
            other => DOBindValue::Text(other.to_string()),
        }
    }

    /// Convert to `SqlStorageValue` for binding to Durable Object SQL queries.
    pub fn to_sql_value(&self) -> SqlStorageValue {
        match self {
            DOBindValue::Text(s) => SqlStorageValue::String(s.clone()),
            DOBindValue::Integer(i) => SqlStorageValue::Integer(*i),
            DOBindValue::Real(f) => SqlStorageValue::Float(*f),
            DOBindValue::Bool(b) => SqlStorageValue::Boolean(*b),
            DOBindValue::Null => SqlStorageValue::Null,
            DOBindValue::OptText(Some(s)) => SqlStorageValue::String(s.clone()),
            DOBindValue::OptText(None) => SqlStorageValue::Null,
            DOBindValue::OptInteger(Some(i)) => SqlStorageValue::Integer(*i),
            DOBindValue::OptInteger(None) => SqlStorageValue::Null,
        }
    }
}

/// Convert a `serde_json::Value` to `SqlStorageValue`.
fn json_to_sql_value(v: &JsonValue) -> SqlStorageValue {
    match v {
        JsonValue::String(s) => SqlStorageValue::String(s.clone()),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                SqlStorageValue::Integer(i)
            } else if let Some(f) = n.as_f64() {
                SqlStorageValue::Float(f)
            } else {
                SqlStorageValue::String(n.to_string())
            }
        }
        JsonValue::Bool(b) => SqlStorageValue::Boolean(*b),
        JsonValue::Null => SqlStorageValue::Null,
        other => SqlStorageValue::String(other.to_string()),
    }
}

// ============================================================================
// DOStore — Durable Object SQL storage backend
// ============================================================================

pub struct DOStore<T: Entity> {
    sql: SqlStorage,
    table_name: String,
    _phantom: PhantomData<T>,
}

impl<T: Entity> DOStore<T> {
    /// Creates a new DO store with the given SqlStorage and table name.
    pub fn new(sql: SqlStorage, table_name: impl Into<String>) -> Self {
        Self {
            sql,
            table_name: table_name.into(),
            _phantom: PhantomData,
        }
    }

    /// Creates a new DO store using `Entity::entity_type()` as the table name.
    pub fn with_entity_type(sql: SqlStorage) -> Self {
        Self::new(sql, T::entity_type())
    }

    /// Creates a new DO store from a `worker::durable::State`.
    pub fn from_state(state: &worker::durable::State, table_name: impl Into<String>) -> Self {
        Self::new(state.storage().sql(), table_name)
    }

    /// Returns a reference to the underlying SqlStorage.
    pub fn storage(&self) -> &SqlStorage {
        &self.sql
    }

    /// Returns the table name.
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    /// Helper: execute a query and deserialize all rows into `Vec<T>`.
    fn query_entities(&self, sql: &str, params: &[SqlStorageValue]) -> StorageResult<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let bindings: Vec<SqlStorageValue> = params.to_vec();
        let cursor = self
            .sql
            .exec(sql, bindings)
            .map_err(|e| StorageError::QueryError(format!("Query failed: {}", e)))?;

        cursor
            .to_array::<T>()
            .map_err(|e| StorageError::QueryError(format!("Failed to deserialize rows: {}", e)))
    }

    /// Helper: execute a query that returns a single i64 scalar (e.g. COUNT).
    fn query_scalar_i64(&self, sql: &str, params: &[SqlStorageValue]) -> StorageResult<i64> {
        let bindings: Vec<SqlStorageValue> = params.to_vec();
        let cursor = self
            .sql
            .exec(sql, bindings)
            .map_err(|e| StorageError::QueryError(format!("Query failed: {}", e)))?;

        #[derive(serde::Deserialize)]
        struct CountRow {
            count: i64,
        }

        let row: CountRow = cursor
            .one()
            .map_err(|e| StorageError::QueryError(format!("Failed to read scalar: {}", e)))?;

        Ok(row.count)
    }

    /// Helper: execute a write statement and return rows_written.
    fn execute(&self, sql: &str, params: &[SqlStorageValue]) -> StorageResult<usize> {
        let bindings: Vec<SqlStorageValue> = params.to_vec();
        let cursor = self
            .sql
            .exec(sql, bindings)
            .map_err(|e| StorageError::QueryError(format!("Execute failed: {}", e)))?;

        Ok(cursor.rows_written())
    }
}

// ============================================================================
// Store implementation
// ============================================================================

#[async_trait]
impl<T> Store<T> for DOStore<T>
where
    T: DOEntity + serde::de::DeserializeOwned + Send + Sync,
{
    type Filter = ();

    async fn create(&self, entity: &T) -> StorageResult<()> {
        let cols = T::columns();
        let col_list = cols.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>().join(", ");
        let placeholders = cols.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

        let sql = format!(
            "INSERT INTO \"{}\" ({}) VALUES ({})",
            self.table_name, col_list, placeholders
        );

        let params: Vec<SqlStorageValue> = cols.iter().map(|&col| entity.bind_column(col).to_sql_value()).collect();

        self.execute(&sql, &params).map_err(|e| {
            let msg = format!("{}", e);
            if msg.contains("UNIQUE constraint failed") || msg.contains("duplicate") {
                StorageError::DuplicateKey {
                    entity_type: T::entity_type().to_string(),
                    id: entity.id().to_string(),
                }
            } else {
                e
            }
        })?;

        Ok(())
    }

    async fn create_many(&self, entities: &[T]) -> StorageResult<usize> {
        let mut count = 0;
        for entity in entities {
            self.create(entity).await?;
            count += 1;
        }
        Ok(count)
    }

    async fn find_by_id(&self, id: &str) -> StorageResult<Option<T>> {
        let sql = format!("SELECT * FROM \"{}\" WHERE \"id\" = ? LIMIT 1", self.table_name);
        let params = vec![SqlStorageValue::String(id.to_string())];

        let results = self.query_entities(&sql, &params)?;
        Ok(results.into_iter().next())
    }

    async fn find_many(&self, filter: Option<FilterCondition>, options: QueryOptions) -> StorageResult<Vec<T>> {
        let mut builder = DOQueryBuilder::new();
        let mut sql = format!("SELECT * FROM \"{}\"", self.table_name);

        if let Some(ref f) = filter {
            let clause = builder.build_where(f)?;
            sql.push_str(&format!(" WHERE {}", clause));
        }
        if let Some(order) = DOQueryBuilder::build_order_by(&options) {
            sql.push_str(&format!(" {}", order));
        }
        if let Some(limit) = DOQueryBuilder::build_limit(&options) {
            sql.push_str(&format!(" {}", limit));
        }
        if let Some(offset) = DOQueryBuilder::build_offset(&options) {
            sql.push_str(&format!(" {}", offset));
        }

        let params: Vec<SqlStorageValue> = builder
            .into_params()
            .into_iter()
            .map(|v| json_to_sql_value(&v))
            .collect();

        self.query_entities(&sql, &params)
    }

    async fn count(&self, filter: Option<FilterCondition>) -> StorageResult<u64> {
        let mut builder = DOQueryBuilder::new();
        let mut sql = format!("SELECT COUNT(*) as count FROM \"{}\"", self.table_name);

        if let Some(ref f) = filter {
            let clause = builder.build_where(f)?;
            sql.push_str(&format!(" WHERE {}", clause));
        }

        let params: Vec<SqlStorageValue> = builder
            .into_params()
            .into_iter()
            .map(|v| json_to_sql_value(&v))
            .collect();

        let count = self.query_scalar_i64(&sql, &params)?;
        Ok(count as u64)
    }

    async fn update(&self, entity: &T) -> StorageResult<()> {
        let cols = T::columns();
        let update_cols: Vec<&&str> = cols.iter().filter(|c| **c != "id").collect();

        if update_cols.is_empty() {
            return Ok(());
        }

        let set_clause = update_cols
            .iter()
            .map(|c| format!("\"{}\" = ?", c))
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!("UPDATE \"{}\" SET {} WHERE \"id\" = ?", self.table_name, set_clause);

        let mut params: Vec<SqlStorageValue> = update_cols
            .iter()
            .map(|&&col| entity.bind_column(col).to_sql_value())
            .collect();
        params.push(SqlStorageValue::String(entity.id().to_string()));

        let rows = self.execute(&sql, &params)?;

        if rows == 0 {
            return Err(StorageError::NotFound {
                entity_type: T::entity_type().to_string(),
                id: entity.id().to_string(),
            });
        }

        Ok(())
    }

    async fn upsert(&self, entity: &T) -> StorageResult<()> {
        let cols = T::columns();
        let col_list = cols.iter().map(|c| format!("\"{}\"", c)).collect::<Vec<_>>().join(", ");
        let placeholders = cols.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let update_clause = cols
            .iter()
            .filter(|c| **c != "id")
            .map(|c| format!("\"{}\" = excluded.\"{}\"", c, c))
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "INSERT INTO \"{}\" ({}) VALUES ({}) ON CONFLICT(\"id\") DO UPDATE SET {}",
            self.table_name, col_list, placeholders, update_clause
        );

        let params: Vec<SqlStorageValue> = cols.iter().map(|&col| entity.bind_column(col).to_sql_value()).collect();

        self.execute(&sql, &params)?;
        Ok(())
    }

    async fn update_many(&self, filter: FilterCondition, updates: Map<String, JsonValue>) -> StorageResult<u64> {
        if updates.is_empty() {
            return Ok(0);
        }

        let mut builder = DOQueryBuilder::new();

        let set_clause = updates
            .keys()
            .map(|k| format!("\"{}\" = ?", k))
            .collect::<Vec<_>>()
            .join(", ");
        let where_clause = builder.build_where(&filter)?;

        let sql = format!(
            "UPDATE \"{}\" SET {} WHERE {}",
            self.table_name, set_clause, where_clause
        );

        // Bind SET values first, then WHERE params
        let mut params: Vec<SqlStorageValue> = updates.values().map(|v| json_to_sql_value(v)).collect();
        params.extend(builder.into_params().into_iter().map(|v| json_to_sql_value(&v)));

        let rows = self.execute(&sql, &params)?;
        Ok(rows as u64)
    }

    async fn delete(&self, id: &str) -> StorageResult<bool> {
        let sql = format!("DELETE FROM \"{}\" WHERE \"id\" = ?", self.table_name);
        let params = vec![SqlStorageValue::String(id.to_string())];

        let rows = self.execute(&sql, &params)?;
        Ok(rows > 0)
    }

    async fn delete_many(&self, filter: FilterCondition) -> StorageResult<u64> {
        let mut builder = DOQueryBuilder::new();
        let where_clause = builder.build_where(&filter)?;

        let sql = format!("DELETE FROM \"{}\" WHERE {}", self.table_name, where_clause);

        let params: Vec<SqlStorageValue> = builder
            .into_params()
            .into_iter()
            .map(|v| json_to_sql_value(&v))
            .collect();

        let rows = self.execute(&sql, &params)?;
        Ok(rows as u64)
    }

    async fn delete_all(&self) -> StorageResult<u64> {
        let sql = format!("DELETE FROM \"{}\"", self.table_name);
        let rows = self.execute(&sql, &[])?;
        Ok(rows as u64)
    }

    async fn text_search(&self, _text: &str, _limit: Option<i64>, _offset: Option<u64>) -> StorageResult<Vec<T>> {
        Err(StorageError::NotSupported(
            "Full-text search is not natively supported on Durable Object SQL storage. \
             Use find_many with FilterCondition::Contains, or \
             DOStore::like_search for multi-column LIKE queries."
                .to_string(),
        ))
    }

    async fn text_search_with_filter(
        &self,
        _text: &str,
        _filter: (),
        _limit: Option<i64>,
        _offset: Option<u64>,
    ) -> StorageResult<Vec<T>> {
        Err(StorageError::NotSupported(
            "Full-text search is not natively supported on Durable Object SQL storage. \
             Use find_many with FilterCondition::Contains, or \
             DOStore::like_search for multi-column LIKE queries."
                .to_string(),
        ))
    }

    // ========================================================================
    // Queue operations — lightweight queue on top of a DO SQL table
    // ========================================================================

    async fn create_queue(&self, queue_name: &str) -> StorageResult<()> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS \"_queue_{}\" (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                payload TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'waiting',
                ack_id TEXT,
                tries INTEGER NOT NULL DEFAULT 0,
                visible_at TEXT NOT NULL DEFAULT (datetime('now')),
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                completed_at TEXT
            )",
            queue_name
        );

        self.execute(&sql, &[])
            .map_err(|_| StorageError::QueryError(format!("Failed to create queue '{}'", queue_name)))?;

        Ok(())
    }

    async fn queue_add<Q: serde::Serialize + Send + Sync>(
        &self,
        queue_name: &str,
        items: &[Q],
        delay_secs: Option<i64>,
    ) -> StorageResult<()> {
        let visible_at = match delay_secs {
            Some(secs) => format!("datetime('now', '+{} seconds')", secs),
            None => "datetime('now')".to_string(),
        };

        for item in items {
            let payload = serde_json::to_string(item)
                .map_err(|e| StorageError::SerializationError(format!("Failed to serialize queue item: {}", e)))?;

            let sql = format!(
                "INSERT INTO \"_queue_{}\" (payload, visible_at) VALUES (?, {})",
                queue_name, visible_at
            );

            self.execute(&sql, &[SqlStorageValue::String(payload)])
                .map_err(|_| StorageError::QueryError(format!("Failed to add to queue '{}'", queue_name)))?;
        }

        Ok(())
    }

    async fn queue_get<Q: serde::de::DeserializeOwned + Send>(
        &self,
        queue_name: &str,
        visibility_secs: i64,
    ) -> StorageResult<Option<QueueMessage<Q>>> {
        let ack_id = ulid::Ulid::new().to_string();

        // Atomically claim the oldest visible message
        let update_sql = format!(
            "UPDATE \"_queue_{}\" SET status = 'in_flight', ack_id = ?, tries = tries + 1, \
             visible_at = datetime('now', '+{} seconds') \
             WHERE id = (\
               SELECT id FROM \"_queue_{}\" \
               WHERE status = 'waiting' AND visible_at <= datetime('now') \
               ORDER BY id ASC LIMIT 1\
             )",
            queue_name, visibility_secs, queue_name
        );

        let rows = self
            .execute(&update_sql, &[SqlStorageValue::String(ack_id.clone())])
            .map_err(|_| StorageError::QueryError(format!("Failed to get from queue '{}'", queue_name)))?;

        if rows == 0 {
            return Ok(None);
        }

        // Fetch the claimed message
        #[derive(serde::Deserialize)]
        struct QueueRow {
            payload: String,
            tries: i32,
        }

        let fetch_sql = format!("SELECT payload, tries FROM \"_queue_{}\" WHERE ack_id = ?", queue_name);

        let cursor = self
            .sql
            .exec(&fetch_sql, vec![SqlStorageValue::String(ack_id.clone())])
            .map_err(|e| StorageError::QueryError(format!("Failed to fetch queue message: {}", e)))?;

        let row: QueueRow = cursor
            .one()
            .map_err(|e| StorageError::QueryError(format!("Failed to read queue message: {}", e)))?;

        let payload: Q = serde_json::from_str(&row.payload)
            .map_err(|e| StorageError::SerializationError(format!("Failed to deserialize queue payload: {}", e)))?;

        Ok(Some(QueueMessage {
            ack_id,
            payload,
            tries: row.tries,
        }))
    }

    async fn queue_ack<Q: serde::de::DeserializeOwned + Send>(
        &self,
        queue_name: &str,
        ack_id: &str,
    ) -> StorageResult<Option<Q>> {
        // Fetch the payload first
        #[derive(serde::Deserialize)]
        struct PayloadRow {
            payload: String,
        }

        let fetch_sql = format!(
            "SELECT payload FROM \"_queue_{}\" WHERE ack_id = ? AND status = 'in_flight'",
            queue_name
        );

        let cursor = self
            .sql
            .exec(&fetch_sql, vec![SqlStorageValue::String(ack_id.to_string())])
            .map_err(|e| StorageError::QueryError(format!("Failed to ack queue message: {}", e)))?;

        let rows: Vec<PayloadRow> = cursor
            .to_array()
            .map_err(|e| StorageError::QueryError(format!("Failed to read ack result: {}", e)))?;

        match rows.into_iter().next() {
            Some(row) => {
                let complete_sql = format!(
                    "UPDATE \"_queue_{}\" SET status = 'completed', \
                     completed_at = datetime('now') WHERE ack_id = ?",
                    queue_name
                );

                self.execute(&complete_sql, &[SqlStorageValue::String(ack_id.to_string())])
                    .map_err(|_| StorageError::QueryError("Failed to complete queue message".to_string()))?;

                let payload: Q = serde_json::from_str(&row.payload).map_err(|e| {
                    StorageError::SerializationError(format!("Failed to deserialize queue payload: {}", e))
                })?;

                Ok(Some(payload))
            }
            None => Ok(None),
        }
    }

    async fn queue_ping<Q: serde::de::DeserializeOwned + Send>(
        &self,
        queue_name: &str,
        ack_id: &str,
        visibility_secs: i64,
    ) -> StorageResult<Option<Q>> {
        let update_sql = format!(
            "UPDATE \"_queue_{}\" SET visible_at = datetime('now', '+{} seconds') \
             WHERE ack_id = ? AND status = 'in_flight'",
            queue_name, visibility_secs
        );

        let rows = self
            .execute(&update_sql, &[SqlStorageValue::String(ack_id.to_string())])
            .map_err(|_| StorageError::QueryError("Failed to ping queue message".to_string()))?;

        if rows == 0 {
            return Ok(None);
        }

        #[derive(serde::Deserialize)]
        struct PayloadRow {
            payload: String,
        }

        let fetch_sql = format!("SELECT payload FROM \"_queue_{}\" WHERE ack_id = ?", queue_name);

        let cursor = self
            .sql
            .exec(&fetch_sql, vec![SqlStorageValue::String(ack_id.to_string())])
            .map_err(|e| StorageError::QueryError(format!("Failed to fetch pinged message: {}", e)))?;

        let rows: Vec<PayloadRow> = cursor
            .to_array()
            .map_err(|e| StorageError::QueryError(format!("Failed to read pinged message: {}", e)))?;

        match rows.into_iter().next() {
            Some(row) => {
                let payload: Q = serde_json::from_str(&row.payload).map_err(|e| {
                    StorageError::SerializationError(format!("Failed to deserialize queue payload: {}", e))
                })?;
                Ok(Some(payload))
            }
            None => Ok(None),
        }
    }

    async fn queue_waiting_count(&self, queue_name: &str) -> StorageResult<i64> {
        let sql = format!(
            "SELECT COUNT(*) as count FROM \"_queue_{}\" WHERE status = 'waiting'",
            queue_name
        );
        self.query_scalar_i64(&sql, &[])
    }

    async fn queue_in_flight_count(&self, queue_name: &str) -> StorageResult<i64> {
        let sql = format!(
            "SELECT COUNT(*) as count FROM \"_queue_{}\" WHERE status = 'in_flight'",
            queue_name
        );
        self.query_scalar_i64(&sql, &[])
    }

    async fn queue_completed_count(&self, queue_name: &str) -> StorageResult<i64> {
        let sql = format!(
            "SELECT COUNT(*) as count FROM \"_queue_{}\" WHERE status = 'completed'",
            queue_name
        );
        self.query_scalar_i64(&sql, &[])
    }

    async fn queue_total_count(&self, queue_name: &str) -> StorageResult<i64> {
        let sql = format!("SELECT COUNT(*) as count FROM \"_queue_{}\"", queue_name);
        self.query_scalar_i64(&sql, &[])
    }

    async fn queue_purge(&self, queue_name: &str) -> StorageResult<u64> {
        let sql = format!("DELETE FROM \"_queue_{}\" WHERE status = 'completed'", queue_name);
        let rows = self.execute(&sql, &[])?;
        Ok(rows as u64)
    }
}

// ============================================================================
// Extension methods
// ============================================================================

impl<T> DOStore<T>
where
    T: DOEntity + serde::de::DeserializeOwned + Send + Sync,
{
    /// Execute a raw SQL query and materialise rows into entities.
    pub fn raw_query(&self, sql: &str) -> StorageResult<Vec<T>> {
        let cursor = self
            .sql
            .exec(sql, None)
            .map_err(|e| StorageError::QueryError(format!("Raw query failed: {}", e)))?;

        cursor
            .to_array::<T>()
            .map_err(|e| StorageError::QueryError(format!("Failed to deserialize rows: {}", e)))
    }

    /// Execute a raw SQL statement (INSERT / UPDATE / DELETE / DDL).
    pub fn raw_execute(&self, sql: &str) -> StorageResult<u64> {
        let cursor = self
            .sql
            .exec(sql, None)
            .map_err(|e| StorageError::QueryError(format!("Raw execute failed: {}", e)))?;

        Ok(cursor.rows_written() as u64)
    }

    /// LIKE-based text search across one or more columns.
    ///
    /// Durable Object SQL storage (SQLite) supports LIKE queries, so this
    /// provides a simple multi-column `LIKE '%text%'` search.
    pub fn like_search(
        &self,
        text: &str,
        columns: &[&str],
        limit: Option<i64>,
        offset: Option<u64>,
    ) -> StorageResult<Vec<T>> {
        if columns.is_empty() {
            return Err(StorageError::QueryError(
                "At least one search column must be specified".to_string(),
            ));
        }

        let like_clauses: Vec<String> = columns.iter().map(|col| format!("\"{}\" LIKE ?", col)).collect();

        let mut sql = format!(
            "SELECT * FROM \"{}\" WHERE ({})",
            self.table_name,
            like_clauses.join(" OR ")
        );

        if let Some(limit) = limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let pattern = format!("%{}%", text);
        let params: Vec<SqlStorageValue> = columns
            .iter()
            .map(|_| SqlStorageValue::String(pattern.clone()))
            .collect();

        self.query_entities(&sql, &params)
    }

    /// Create the table for this entity if it doesn't exist.
    ///
    /// Uses the column names from `DOEntity::columns()`. All columns default
    /// to `TEXT` except `id` which becomes `TEXT PRIMARY KEY`.
    pub fn create_table(&self) -> StorageResult<()> {
        let cols = T::columns();
        let col_defs: Vec<String> = cols
            .iter()
            .map(|&c| {
                if c == "id" {
                    format!("\"{}\" TEXT PRIMARY KEY NOT NULL", c)
                } else {
                    format!("\"{}\" TEXT", c)
                }
            })
            .collect();

        let sql = format!(
            "CREATE TABLE IF NOT EXISTS \"{}\" ({})",
            self.table_name,
            col_defs.join(", ")
        );

        self.execute(&sql, &[])
            .map_err(|_| StorageError::QueryError(format!("Failed to create table '{}'", self.table_name)))?;

        Ok(())
    }

    /// Create the table with explicit column definitions.
    ///
    /// Pass tuples of `(column_name, sql_type)`:
    /// ```ignore
    /// store.create_table_with_defs(&[
    ///     ("id", "TEXT PRIMARY KEY NOT NULL"),
    ///     ("name", "TEXT NOT NULL"),
    ///     ("age", "INTEGER"),
    ///     ("active", "BOOLEAN NOT NULL DEFAULT 1"),
    /// ])?;
    /// ```
    pub fn create_table_with_defs(&self, column_defs: &[(&str, &str)]) -> StorageResult<()> {
        let defs: Vec<String> = column_defs
            .iter()
            .map(|(name, typ)| format!("\"{}\" {}", name, typ))
            .collect();

        let sql = format!(
            "CREATE TABLE IF NOT EXISTS \"{}\" ({})",
            self.table_name,
            defs.join(", ")
        );

        self.execute(&sql, &[])
            .map_err(|_| StorageError::QueryError(format!("Failed to create table '{}'", self.table_name)))?;

        Ok(())
    }

    /// Returns the database size in bytes.
    pub fn database_size(&self) -> usize {
        self.sql.database_size()
    }
}
