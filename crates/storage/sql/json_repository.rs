// Harana Components - JSON SQL Store
// Stores entities as JSON documents in a single JSONB column.

use async_trait::async_trait;
use std::marker::PhantomData;

use crate::{Store, Entity, FilterCondition, QueryOptions, StorageError, StorageResult};
use super::SqlQueryBuilder;

/// Stores entities as JSON documents in a single column.
pub struct JsonSqlStore<T: Entity> {
    #[cfg(feature = "postgres")]
    pool: sqlx::PgPool,
    #[cfg(not(feature = "postgres"))]
    pool: (),
    table_name: String,
    _phantom: PhantomData<T>,
}

#[cfg(feature = "postgres")]
impl<T: Entity> JsonSqlStore<T> {
    pub fn new(pool: sqlx::PgPool, table_name: impl Into<String>) -> Self {
        Self {
            pool,
            table_name: table_name.into(),
            _phantom: PhantomData,
        }
    }

    pub async fn connect(database_url: &str, table_name: &str) -> StorageResult<Self> {
        let pool = sqlx::PgPool::connect(database_url)
            .await
            .map_err(|e| StorageError::ConnectionError(format!("Failed to connect: {}", e)))?;
        
        Ok(Self::new(pool, table_name))
    }

    pub async fn init_table(&self) -> StorageResult<()> {
        let query = format!(
            r#"
            CREATE TABLE IF NOT EXISTS "{}" (
                id VARCHAR(255) PRIMARY KEY,
                data JSONB NOT NULL,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            )
            "#,
            self.table_name
        );

        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to create table: {}", e)))?;

        Ok(())
    }
}

#[cfg(feature = "postgres")]
#[async_trait]
impl<T: Entity> Store<T> for JsonSqlStore<T> {
    type Filter = ();

    async fn create(&self, entity: &T) -> StorageResult<()> {
        let id = entity.id();
        let data = serde_json::to_value(entity)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;

        let query = format!(
            r#"INSERT INTO "{}" (id, data) VALUES ($1, $2)"#,
            self.table_name
        );

        sqlx::query(&query)
            .bind(id)
            .bind(&data)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
                    StorageError::DuplicateKey {
                        entity_type: T::entity_type().to_string(),
                        id: id.to_string(),
                    }
                } else {
                    StorageError::QueryError(format!("Failed to create entity: {}", e))
                }
            })?;

        Ok(())
    }

    async fn create_many(&self, entities: &[T]) -> StorageResult<usize> {
        if entities.is_empty() {
            return Ok(0);
        }

        let mut count = 0;
        for entity in entities {
            self.create(entity).await?;
            count += 1;
        }
        Ok(count)
    }

    async fn find_by_id(&self, id: &str) -> StorageResult<Option<T>> {
        let query = format!(
            r#"SELECT data FROM "{}" WHERE id = $1"#,
            self.table_name
        );

        let row: Option<(serde_json::Value,)> = sqlx::query_as(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to find entity: {}", e)))?;

        match row {
            Some((data,)) => {
                let entity = serde_json::from_value(data)
                    .map_err(|e| StorageError::SerializationError(e.to_string()))?;
                Ok(Some(entity))
            }
            None => Ok(None),
        }
    }

    async fn find_many(
        &self,
        filter: Option<FilterCondition>,
        options: QueryOptions,
    ) -> StorageResult<Vec<T>> {
        let mut query = format!(r#"SELECT data FROM "{}""#, self.table_name);
        let mut builder = SqlQueryBuilder::new();

        if let Some(ref f) = filter {
            // For JSON storage, we need to query into the JSONB data column
            let where_clause = build_jsonb_where(&mut builder, f)?;
            query.push_str(&format!(" WHERE {}", where_clause));
        }

        if let Some(order_by) = SqlQueryBuilder::build_order_by(&options) {
            query.push_str(&format!(" {}", order_by));
        }

        if let Some(limit) = SqlQueryBuilder::build_limit(&options) {
            query.push_str(&format!(" {}", limit));
        }

        if let Some(offset) = SqlQueryBuilder::build_offset(&options) {
            query.push_str(&format!(" {}", offset));
        }

        let rows: Vec<(serde_json::Value,)> = sqlx::query_as(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to find entities: {}", e)))?;

        let entities: Result<Vec<T>, _> = rows
            .into_iter()
            .map(|(data,)| serde_json::from_value(data))
            .collect();

        entities.map_err(|e| StorageError::SerializationError(e.to_string()))
    }

    async fn count(&self, filter: Option<FilterCondition>) -> StorageResult<u64> {
        let mut query = format!(r#"SELECT COUNT(*) FROM "{}""#, self.table_name);
        let mut builder = SqlQueryBuilder::new();

        if let Some(ref f) = filter {
            let where_clause = build_jsonb_where(&mut builder, f)?;
            query.push_str(&format!(" WHERE {}", where_clause));
        }

        let (count,): (i64,) = sqlx::query_as(&query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to count entities: {}", e)))?;

        Ok(count as u64)
    }

    async fn update(&self, entity: &T) -> StorageResult<()> {
        let id = entity.id();
        let data = serde_json::to_value(entity)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;

        let query = format!(
            r#"UPDATE "{}" SET data = $1, updated_at = NOW() WHERE id = $2"#,
            self.table_name
        );

        let result = sqlx::query(&query)
            .bind(&data)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to update entity: {}", e)))?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound {
                entity_type: T::entity_type().to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }

    async fn upsert(&self, entity: &T) -> StorageResult<()> {
        let id = entity.id();
        let data = serde_json::to_value(entity)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;

        let query = format!(
            r#"
            INSERT INTO "{}" (id, data) VALUES ($1, $2)
            ON CONFLICT (id) DO UPDATE SET data = $2, updated_at = NOW()
            "#,
            self.table_name
        );

        sqlx::query(&query)
            .bind(id)
            .bind(&data)
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to upsert entity: {}", e)))?;

        Ok(())
    }

    async fn update_many(
        &self,
        filter: FilterCondition,
        updates: serde_json::Map<String, serde_json::Value>,
    ) -> StorageResult<u64> {
        // For JSON storage, we update the JSON fields within the data column
        let mut builder = SqlQueryBuilder::new();
        let where_clause = build_jsonb_where(&mut builder, &filter)?;

        let updates_json = serde_json::Value::Object(updates);
        let query = format!(
            r#"UPDATE "{}" SET data = data || $1, updated_at = NOW() WHERE {}"#,
            self.table_name, where_clause
        );

        let result = sqlx::query(&query)
            .bind(&updates_json)
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to update entities: {}", e)))?;

        Ok(result.rows_affected())
    }

    async fn delete(&self, id: &str) -> StorageResult<bool> {
        let query = format!(r#"DELETE FROM "{}" WHERE id = $1"#, self.table_name);

        let result = sqlx::query(&query)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to delete entity: {}", e)))?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete_many(&self, filter: FilterCondition) -> StorageResult<u64> {
        let mut builder = SqlQueryBuilder::new();
        let where_clause = build_jsonb_where(&mut builder, &filter)?;

        let query = format!(r#"DELETE FROM "{}" WHERE {}"#, self.table_name, where_clause);

        let result = sqlx::query(&query)
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to delete entities: {}", e)))?;

        Ok(result.rows_affected())
    }

    async fn delete_all(&self) -> StorageResult<u64> {
        let query = format!(r#"DELETE FROM "{}""#, self.table_name);

        let result = sqlx::query(&query)
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::QueryError(format!("Failed to delete all entities: {}", e)))?;

        Ok(result.rows_affected())
    }

    // Text search operations - not supported for JSON storage
    async fn text_search(
        &self,
        _text: &str,
        _limit: Option<i64>,
        _offset: Option<u64>,
    ) -> StorageResult<Vec<T>> {
        Err(StorageError::NotSupported("Text search is not supported for JSON storage".to_string()))
    }

    async fn text_search_with_filter(
        &self,
        _text: &str,
        _filter: Self::Filter,
        _limit: Option<i64>,
        _offset: Option<u64>,
    ) -> StorageResult<Vec<T>> {
        Err(StorageError::NotSupported("Text search is not supported for JSON storage".to_string()))
    }

    // Queue operations - not supported for JSON storage
    async fn create_queue(&self, _queue_name: &str) -> StorageResult<()> {
        Err(StorageError::NotSupported("Queue operations are not supported for JSON storage".to_string()))
    }

    async fn queue_add<Q: serde::Serialize + Send + Sync>(
        &self,
        _queue_name: &str,
        _items: &[Q],
        _delay_secs: Option<i64>,
    ) -> StorageResult<()> {
        Err(StorageError::NotSupported("Queue operations are not supported for JSON storage".to_string()))
    }

    async fn queue_get<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _visibility_secs: i64,
    ) -> StorageResult<Option<crate::QueueMessage<Q>>> {
        Err(StorageError::NotSupported("Queue operations are not supported for JSON storage".to_string()))
    }

    async fn queue_ack<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _ack_id: &str,
    ) -> StorageResult<Option<Q>> {
        Err(StorageError::NotSupported("Queue operations are not supported for JSON storage".to_string()))
    }

    async fn queue_ping<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _ack_id: &str,
        _visibility_secs: i64,
    ) -> StorageResult<Option<Q>> {
        Err(StorageError::NotSupported("Queue operations are not supported for JSON storage".to_string()))
    }

    async fn queue_waiting_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for JSON storage".to_string()))
    }

    async fn queue_in_flight_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for JSON storage".to_string()))
    }

    async fn queue_completed_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for JSON storage".to_string()))
    }

    async fn queue_total_count(&self, _queue_name: &str) -> StorageResult<i64> {
        Err(StorageError::NotSupported("Queue operations are not supported for JSON storage".to_string()))
    }

    async fn queue_purge(&self, _queue_name: &str) -> StorageResult<u64> {
        Err(StorageError::NotSupported("Queue operations are not supported for JSON storage".to_string()))
    }
}

#[cfg(feature = "postgres")]
fn build_jsonb_where(builder: &mut SqlQueryBuilder, filter: &FilterCondition) -> StorageResult<String> {
    match filter {
        FilterCondition::Eq(field, value) => {
            Ok(format!("data->>'{}' = '{}'", field, json_value_to_sql_string(value)))
        }
        FilterCondition::Ne(field, value) => {
            Ok(format!("data->>'{}' != '{}'", field, json_value_to_sql_string(value)))
        }
        FilterCondition::Gt(field, value) => {
            Ok(format!("(data->'{}')::numeric > {}", field, json_value_to_sql_string(value)))
        }
        FilterCondition::Gte(field, value) => {
            Ok(format!("(data->'{}')::numeric >= {}", field, json_value_to_sql_string(value)))
        }
        FilterCondition::Lt(field, value) => {
            Ok(format!("(data->'{}')::numeric < {}", field, json_value_to_sql_string(value)))
        }
        FilterCondition::Lte(field, value) => {
            Ok(format!("(data->'{}')::numeric <= {}", field, json_value_to_sql_string(value)))
        }
        FilterCondition::In(field, values) => {
            let vals: Vec<String> = values.iter().map(|v| format!("'{}'", json_value_to_sql_string(v))).collect();
            Ok(format!("data->>'{}' IN ({})", field, vals.join(", ")))
        }
        FilterCondition::NotIn(field, values) => {
            let vals: Vec<String> = values.iter().map(|v| format!("'{}'", json_value_to_sql_string(v))).collect();
            Ok(format!("data->>'{}' NOT IN ({})", field, vals.join(", ")))
        }
        FilterCondition::Contains(field, value) => {
            Ok(format!("data->>'{}' ILIKE '%{}%'", field, value))
        }
        FilterCondition::StartsWith(field, value) => {
            Ok(format!("data->>'{}' LIKE '{}%'", field, value))
        }
        FilterCondition::EndsWith(field, value) => {
            Ok(format!("data->>'{}' LIKE '%{}'", field, value))
        }
        FilterCondition::IsNull(field) => {
            Ok(format!("data->>'{}' IS NULL", field))
        }
        FilterCondition::IsNotNull(field) => {
            Ok(format!("data->>'{}' IS NOT NULL", field))
        }
        FilterCondition::And(conditions) => {
            let clauses: Result<Vec<String>, _> = conditions
                .iter()
                .map(|c| build_jsonb_where(builder, c))
                .collect();
            Ok(format!("({})", clauses?.join(" AND ")))
        }
        FilterCondition::Or(conditions) => {
            let clauses: Result<Vec<String>, _> = conditions
                .iter()
                .map(|c| build_jsonb_where(builder, c))
                .collect();
            Ok(format!("({})", clauses?.join(" OR ")))
        }
    }
}

fn json_value_to_sql_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "NULL".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(arr) => serde_json::to_string(arr).unwrap_or_default(),
        serde_json::Value::Object(obj) => serde_json::to_string(obj).unwrap_or_default(),
    }
}
