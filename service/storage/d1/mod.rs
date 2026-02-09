mod query_builder;

use async_trait::async_trait;
use serde_json::{Map, Value as JsonValue};
use sqlx_d1::D1Connection;
use sqlx_d1::sqlx_core::database::Database;
use std::marker::PhantomData;

use crate::{Entity, FilterCondition, QueryOptions, QueueMessage, StorageError, StorageResult, Store};
use query_builder::D1QueryBuilder;

/// Re-export `sqlx_d1::FromRow` so consumers can derive it alongside `Entity`.
pub use sqlx_d1::FromRow;

/// Trait that entities must implement to work with D1Store in ORM mode.
///
/// This extends the base `Entity` trait with D1-specific column metadata so that
/// INSERT / UPDATE statements can be generated at runtime without JSON roundtrips.
pub trait D1Entity: Entity {
    /// Returns the ordered list of column names that map to struct fields.
    /// The first column should be `"id"`.
    fn columns() -> &'static [&'static str];

    /// Returns the value for a given column, ready for binding to a query.
    fn bind_column(&self, col: &str) -> D1BindValue;
}

#[derive(Debug, Clone)]
pub enum D1BindValue {
    Text(String),
    Integer(i64),
    Real(f64),
    Bool(bool),
    Null,
    OptText(Option<String>),
    OptInteger(Option<i64>),
}

impl D1BindValue {
    /// Helper to create a `D1BindValue` from a `serde_json::Value`.
    pub fn from_json(v: &JsonValue) -> Self {
        match v {
            JsonValue::String(s) => D1BindValue::Text(s.clone()),
            JsonValue::Number(n) => {
                if let Some(i) = n.as_i64() {
                    D1BindValue::Integer(i)
                } else if let Some(f) = n.as_f64() {
                    D1BindValue::Real(f)
                } else {
                    D1BindValue::Text(n.to_string())
                }
            }
            JsonValue::Bool(b) => D1BindValue::Bool(*b),
            JsonValue::Null => D1BindValue::Null,
            other => D1BindValue::Text(other.to_string()),
        }
    }
}

// ============================================================================
// D1Store — the main storage backend
// ============================================================================

pub struct D1Store<T: Entity> {
    conn: D1Connection,
    table_name: String,
    _phantom: PhantomData<T>,
}

impl<T: Entity> D1Store<T> {
    /// Creates a new D1 store with the given connection and table name.
    pub fn new(conn: D1Connection, table_name: impl Into<String>) -> Self {
        Self {
            conn,
            table_name: table_name.into(),
            _phantom: PhantomData,
        }
    }

    /// Creates a new D1 store using `Entity::entity_type()` as the table name.
    pub fn with_entity_type(conn: D1Connection) -> Self {
        Self::new(conn, T::entity_type())
    }

    /// Creates a new D1 store from a `worker::Env` D1 binding name.
    pub fn from_env(env: &worker::Env, binding: &str, table_name: impl Into<String>) -> StorageResult<Self> {
        let d1 = env
            .d1(binding)
            .map_err(|e| StorageError::ConnectionError(format!("Failed to get D1 binding '{}': {}", binding, e)))?;
        let conn = D1Connection::new(d1);
        Ok(Self::new(conn, table_name))
    }

    /// Returns a reference to the underlying D1 connection.
    pub fn connection(&self) -> &D1Connection {
        &self.conn
    }

    /// Returns the table name.
    pub fn table_name(&self) -> &str {
        &self.table_name
    }
}

// ============================================================================
// Macro to bind a D1Value to a query without fighting sqlx generics.
//
// Because `Query::bind` changes the generic `A` parameter on every call, we
// cannot write a helper *function* that takes `Query<'q, D1, A>` and returns
// `Query<'q, D1, A'>` — the types are different.  The simplest solution is
// to expand the match inline at each call-site via this macro.
// ============================================================================

/// Bind a single `D1BindValue` to an sqlx-d1 query (or query_as).
macro_rules! bind_d1 {
    ($query:expr, $val:expr) => {
        match $val {
            D1BindValue::Text(s) => $query.bind(s),
            D1BindValue::Integer(i) => $query.bind(i),
            D1BindValue::Real(f) => $query.bind(f),
            D1BindValue::Bool(b) => $query.bind(b),
            D1BindValue::Null => $query.bind(Option::<String>::None),
            D1BindValue::OptText(o) => $query.bind(o),
            D1BindValue::OptInteger(o) => $query.bind(o),
        }
    };
}

/// Bind a `serde_json::Value` to an sqlx-d1 query.
macro_rules! bind_json {
    ($query:expr, $val:expr) => {
        match $val {
            JsonValue::String(s) => $query.bind(s.clone()),
            JsonValue::Number(n) => {
                if let Some(i) = n.as_i64() {
                    $query.bind(i)
                } else if let Some(f) = n.as_f64() {
                    $query.bind(f)
                } else {
                    $query.bind(n.to_string())
                }
            }
            JsonValue::Bool(b) => $query.bind(*b),
            JsonValue::Null => $query.bind(Option::<String>::None),
            _ => $query.bind($val.to_string()),
        }
    };
}

// ============================================================================
// Store implementation
// ============================================================================

#[async_trait]
impl<T> Store<T> for D1Store<T>
where
    T: D1Entity + for<'r> sqlx_d1::FromRow<'r, <sqlx_d1::D1 as Database>::Row> + Send + Unpin,
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

        let mut query = sqlx_d1::query(&sql);
        for &col in cols {
            query = bind_d1!(query, entity.bind_column(col));
        }

        query.execute(&self.conn).await.map_err(|e| {
            let msg = e.to_string();
            if msg.contains("UNIQUE constraint failed") || msg.contains("duplicate") {
                StorageError::DuplicateKey {
                    entity_type: T::entity_type().to_string(),
                    id: entity.id().to_string(),
                }
            } else {
                StorageError::QueryError(format!("Failed to create entity: {}", e))
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

        sqlx_d1::query_as::<T>(&sql)
            .bind(id)
            .fetch_optional(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to find entity by id: {}", e)))
    }

    async fn find_many(&self, filter: Option<FilterCondition>, options: QueryOptions) -> StorageResult<Vec<T>> {
        let mut builder = D1QueryBuilder::new();
        let mut sql = format!("SELECT * FROM \"{}\"", self.table_name);

        if let Some(ref f) = filter {
            let clause = builder.build_where(f)?;
            sql.push_str(&format!(" WHERE {}", clause));
        }
        if let Some(order) = D1QueryBuilder::build_order_by(&options) {
            sql.push_str(&format!(" {}", order));
        }
        if let Some(limit) = D1QueryBuilder::build_limit(&options) {
            sql.push_str(&format!(" {}", limit));
        }
        if let Some(offset) = D1QueryBuilder::build_offset(&options) {
            sql.push_str(&format!(" {}", offset));
        }

        let mut query = sqlx_d1::query_as::<T>(&sql);
        for param in builder.params() {
            query = bind_json!(query, param);
        }

        query
            .fetch_all(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to find entities: {}", e)))
    }

    async fn count(&self, filter: Option<FilterCondition>) -> StorageResult<u64> {
        let mut builder = D1QueryBuilder::new();
        let mut sql = format!("SELECT COUNT(*) as count FROM \"{}\"", self.table_name);

        if let Some(ref f) = filter {
            let clause = builder.build_where(f)?;
            sql.push_str(&format!(" WHERE {}", clause));
        }

        let mut query = sqlx_d1::query_scalar::<i64>(&sql);
        for param in builder.params() {
            query = bind_json!(query, param);
        }

        let count = query
            .fetch_one(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to count entities: {}", e)))?;

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

        let mut query = sqlx_d1::query(&sql);
        // Bind SET values
        for &col in &update_cols {
            query = bind_d1!(query, entity.bind_column(col));
        }
        // Bind WHERE id
        query = query.bind(entity.id().to_string());

        let result = query
            .execute(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to update entity: {}", e)))?;

        if result.rows_affected == 0 {
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

        let mut query = sqlx_d1::query(&sql);
        for &col in cols {
            query = bind_d1!(query, entity.bind_column(col));
        }

        query
            .execute(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to upsert entity: {}", e)))?;

        Ok(())
    }

    async fn update_many(&self, filter: FilterCondition, updates: Map<String, JsonValue>) -> StorageResult<u64> {
        if updates.is_empty() {
            return Ok(0);
        }

        let mut builder = D1QueryBuilder::new();

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
        let mut query = sqlx_d1::query(&sql);
        for value in updates.values() {
            query = bind_json!(query, value);
        }
        for param in builder.params() {
            query = bind_json!(query, param);
        }

        let result = query
            .execute(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to update entities: {}", e)))?;

        Ok(result.rows_affected as u64)
    }

    async fn delete(&self, id: &str) -> StorageResult<bool> {
        let sql = format!("DELETE FROM \"{}\" WHERE \"id\" = ?", self.table_name);

        let result = sqlx_d1::query(&sql)
            .bind(id)
            .execute(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to delete entity: {}", e)))?;

        Ok(result.rows_affected > 0)
    }

    async fn delete_many(&self, filter: FilterCondition) -> StorageResult<u64> {
        let mut builder = D1QueryBuilder::new();
        let where_clause = builder.build_where(&filter)?;

        let sql = format!("DELETE FROM \"{}\" WHERE {}", self.table_name, where_clause);

        let mut query = sqlx_d1::query(&sql);
        for param in builder.params() {
            query = bind_json!(query, param);
        }

        let result = query
            .execute(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to delete entities: {}", e)))?;

        Ok(result.rows_affected as u64)
    }

    async fn delete_all(&self) -> StorageResult<u64> {
        let sql = format!("DELETE FROM \"{}\"", self.table_name);

        let result = sqlx_d1::query(&sql)
            .execute(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to delete all: {}", e)))?;

        Ok(result.rows_affected as u64)
    }

    async fn text_search(&self, _text: &str, _limit: Option<i64>, _offset: Option<u64>) -> StorageResult<Vec<T>> {
        Err(StorageError::NotSupported(
            "Full-text search is not natively supported on D1. \
             Use find_many with FilterCondition::Contains, or \
             D1Store::like_search for multi-column LIKE queries."
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
            "Full-text search is not natively supported on D1. \
             Use find_many with FilterCondition::Contains, or \
             D1Store::like_search for multi-column LIKE queries."
                .to_string(),
        ))
    }

    // ========================================================================
    // Queue operations — lightweight queue on top of a D1 table
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

        sqlx_d1::query(&sql)
            .execute(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create queue '{}': {}", queue_name, e)))?;

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

            sqlx_d1::query(&sql)
                .bind(&payload)
                .execute(&self.conn)
                .await
                .map_err(|e| StorageError::QueryError(format!("Failed to add to queue '{}': {}", queue_name, e)))?;
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

        let result = sqlx_d1::query(&update_sql)
            .bind(&ack_id)
            .execute(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to get from queue '{}': {}", queue_name, e)))?;

        if result.rows_affected == 0 {
            return Ok(None);
        }

        // Fetch the claimed message
        let (payload_str, tries): (String, i32) = sqlx_d1::query_as(&format!(
            "SELECT payload, tries FROM \"_queue_{}\" WHERE ack_id = ?",
            queue_name
        ))
        .bind(&ack_id)
        .fetch_one(&self.conn)
        .await
        .map_err(|e| StorageError::QueryError(format!("Failed to fetch queue message: {}", e)))?;

        let payload: Q = serde_json::from_str(&payload_str)
            .map_err(|e| StorageError::SerializationError(format!("Failed to deserialize queue payload: {}", e)))?;

        Ok(Some(QueueMessage { ack_id, payload, tries }))
    }

    async fn queue_ack<Q: serde::de::DeserializeOwned + Send>(
        &self,
        queue_name: &str,
        ack_id: &str,
    ) -> StorageResult<Option<Q>> {
        // Fetch + complete in two steps (D1 doesn't support RETURNING)
        let row: Option<(String,)> = sqlx_d1::query_as(&format!(
            "SELECT payload FROM \"_queue_{}\" WHERE ack_id = ? AND status = 'in_flight'",
            queue_name
        ))
        .bind(ack_id)
        .fetch_optional(&self.conn)
        .await
        .map_err(|e| StorageError::QueryError(format!("Failed to ack queue message: {}", e)))?;

        match row {
            Some((payload_str,)) => {
                sqlx_d1::query(&format!(
                    "UPDATE \"_queue_{}\" SET status = 'completed', \
                     completed_at = datetime('now') WHERE ack_id = ?",
                    queue_name
                ))
                .bind(ack_id)
                .execute(&self.conn)
                .await
                .map_err(|e| StorageError::QueryError(format!("Failed to complete queue message: {}", e)))?;

                let payload: Q = serde_json::from_str(&payload_str).map_err(|e| {
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
        let result = sqlx_d1::query(&format!(
            "UPDATE \"_queue_{}\" SET visible_at = datetime('now', '+{} seconds') \
             WHERE ack_id = ? AND status = 'in_flight'",
            queue_name, visibility_secs
        ))
        .bind(ack_id)
        .execute(&self.conn)
        .await
        .map_err(|e| StorageError::QueryError(format!("Failed to ping queue message: {}", e)))?;

        if result.rows_affected == 0 {
            return Ok(None);
        }

        let row: Option<(String,)> = sqlx_d1::query_as(&format!(
            "SELECT payload FROM \"_queue_{}\" WHERE ack_id = ?",
            queue_name
        ))
        .bind(ack_id)
        .fetch_optional(&self.conn)
        .await
        .map_err(|e| StorageError::QueryError(format!("Failed to fetch pinged message: {}", e)))?;

        match row {
            Some((payload_str,)) => {
                let payload: Q = serde_json::from_str(&payload_str).map_err(|e| {
                    StorageError::SerializationError(format!("Failed to deserialize queue payload: {}", e))
                })?;
                Ok(Some(payload))
            }
            None => Ok(None),
        }
    }

    async fn queue_waiting_count(&self, queue_name: &str) -> StorageResult<i64> {
        sqlx_d1::query_scalar::<i64>(&format!(
            "SELECT COUNT(*) FROM \"_queue_{}\" WHERE status = 'waiting'",
            queue_name
        ))
        .fetch_one(&self.conn)
        .await
        .map_err(|e| StorageError::QueryError(format!("Failed to count waiting: {}", e)))
    }

    async fn queue_in_flight_count(&self, queue_name: &str) -> StorageResult<i64> {
        sqlx_d1::query_scalar::<i64>(&format!(
            "SELECT COUNT(*) FROM \"_queue_{}\" WHERE status = 'in_flight'",
            queue_name
        ))
        .fetch_one(&self.conn)
        .await
        .map_err(|e| StorageError::QueryError(format!("Failed to count in-flight: {}", e)))
    }

    async fn queue_completed_count(&self, queue_name: &str) -> StorageResult<i64> {
        sqlx_d1::query_scalar::<i64>(&format!(
            "SELECT COUNT(*) FROM \"_queue_{}\" WHERE status = 'completed'",
            queue_name
        ))
        .fetch_one(&self.conn)
        .await
        .map_err(|e| StorageError::QueryError(format!("Failed to count completed: {}", e)))
    }

    async fn queue_total_count(&self, queue_name: &str) -> StorageResult<i64> {
        sqlx_d1::query_scalar::<i64>(&format!("SELECT COUNT(*) FROM \"_queue_{}\"", queue_name))
            .fetch_one(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to count total: {}", e)))
    }

    async fn queue_purge(&self, queue_name: &str) -> StorageResult<u64> {
        let result = sqlx_d1::query(&format!(
            "DELETE FROM \"_queue_{}\" WHERE status = 'completed'",
            queue_name
        ))
        .execute(&self.conn)
        .await
        .map_err(|e| StorageError::QueryError(format!("Failed to purge queue: {}", e)))?;

        Ok(result.rows_affected as u64)
    }
}

// ============================================================================
// Extension methods
// ============================================================================

impl<T> D1Store<T>
where
    T: D1Entity + for<'r> sqlx_d1::FromRow<'r, <sqlx_d1::D1 as Database>::Row> + Send + Unpin,
{
    /// Execute a raw SQL query and materialise rows into entities.
    pub async fn raw_query(&self, sql: &str) -> StorageResult<Vec<T>> {
        sqlx_d1::query_as::<T>(sql)
            .fetch_all(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Raw query failed: {}", e)))
    }

    /// Execute a raw SQL statement (INSERT / UPDATE / DELETE / DDL).
    pub async fn raw_execute(&self, sql: &str) -> StorageResult<u64> {
        let result = sqlx_d1::query(sql)
            .execute(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Raw execute failed: {}", e)))?;

        Ok(result.rows_affected as u64)
    }

    /// LIKE-based text search across one or more columns.
    ///
    /// D1 (SQLite) doesn't support FTS5 virtual tables via the HTTP API, so
    /// this provides a simple multi-column `LIKE '%text%'` alternative.
    pub async fn like_search(
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
        let mut query = sqlx_d1::query_as::<T>(&sql);
        for _ in columns {
            query = query.bind(pattern.clone());
        }

        query
            .fetch_all(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Like search failed: {}", e)))
    }

    /// Create the table for this entity if it doesn't exist.
    ///
    /// Uses the column names from `D1Entity::columns()`. All columns default
    /// to `TEXT` except `id` which becomes `TEXT PRIMARY KEY`. Override via
    /// `column_defs` if you need specific types.
    pub async fn create_table(&self) -> StorageResult<()> {
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

        sqlx_d1::query(&sql)
            .execute(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create table '{}': {}", self.table_name, e)))?;

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
    /// ]).await?;
    /// ```
    pub async fn create_table_with_defs(&self, column_defs: &[(&str, &str)]) -> StorageResult<()> {
        let defs: Vec<String> = column_defs
            .iter()
            .map(|(name, typ)| format!("\"{}\" {}", name, typ))
            .collect();

        let sql = format!(
            "CREATE TABLE IF NOT EXISTS \"{}\" ({})",
            self.table_name,
            defs.join(", ")
        );

        sqlx_d1::query(&sql)
            .execute(&self.conn)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create table '{}': {}", self.table_name, e)))?;

        Ok(())
    }
}
