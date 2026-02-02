// Harana Components - Storage Backend Traits

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};

use crate::{Entity, FilterCondition, QueryOptions, StorageError, StorageResult};

#[derive(Debug, Clone, Default)]
pub struct QueueStats {
    pub waiting: i64,
    pub in_flight: i64,
    pub completed: i64,
    pub total: i64,
}

#[derive(Debug, Clone)]
pub struct QueueMessage<T> {
    pub ack_id: String,
    pub payload: T,
    pub tries: i32,
}

#[async_trait]
pub trait Store<T: Entity>: Send + Sync {
    type Filter: Send + Sync;

    async fn create(&self, entity: &T) -> StorageResult<()>;
    async fn create_many(&self, entities: &[T]) -> StorageResult<usize>;
    async fn find_by_id(&self, id: &str) -> StorageResult<Option<T>>;
    async fn find_many(&self, filter: Option<FilterCondition>, options: QueryOptions) -> StorageResult<Vec<T>>;
    async fn count(&self, filter: Option<FilterCondition>) -> StorageResult<u64>;
    async fn update(&self, entity: &T) -> StorageResult<()>;
    async fn upsert(&self, entity: &T) -> StorageResult<()>;
    async fn update_many(&self, filter: FilterCondition, updates: Map<String, Value>) -> StorageResult<u64>;
    async fn delete(&self, id: &str) -> StorageResult<bool>;
    async fn delete_many(&self, filter: FilterCondition) -> StorageResult<u64>;
    async fn delete_all(&self) -> StorageResult<u64>;

    async fn find_all(&self, options: QueryOptions) -> StorageResult<Vec<T>> {
        self.find_many(None, options).await
    }

    async fn find_one(&self, filter: FilterCondition) -> StorageResult<Option<T>> {
        self.find_many(Some(filter), QueryOptions::new().with_limit(1))
            .await
            .map(|mut v| v.pop())
    }

    async fn exists(&self, id: &str) -> StorageResult<bool> {
        self.find_by_id(id).await.map(|opt| opt.is_some())
    }

    async fn text_search(
        &self,
        text: &str,
        limit: Option<i64>,
        offset: Option<u64>,
    ) -> StorageResult<Vec<T>>;

    async fn text_search_with_filter(
        &self,
        text: &str,
        filter: Self::Filter,
        limit: Option<i64>,
        offset: Option<u64>,
    ) -> StorageResult<Vec<T>>;

    async fn create_queue(&self, queue_name: &str) -> StorageResult<()>;

    async fn queue_add<Q: Serialize + Send + Sync>(
        &self,
        queue_name: &str,
        items: &[Q],
        delay_secs: Option<i64>,
    ) -> StorageResult<()>;

    async fn queue_get<Q: DeserializeOwned + Send>(
        &self,
        queue_name: &str,
        visibility_secs: i64,
    ) -> StorageResult<Option<QueueMessage<Q>>>;

    async fn queue_ack<Q: DeserializeOwned + Send>(
        &self,
        queue_name: &str,
        ack_id: &str,
    ) -> StorageResult<Option<Q>>;

    async fn queue_ping<Q: DeserializeOwned + Send>(
        &self,
        queue_name: &str,
        ack_id: &str,
        visibility_secs: i64,
    ) -> StorageResult<Option<Q>>;

    async fn queue_waiting_count(&self, queue_name: &str) -> StorageResult<i64>;
    async fn queue_in_flight_count(&self, queue_name: &str) -> StorageResult<i64>;
    async fn queue_completed_count(&self, queue_name: &str) -> StorageResult<i64>;
    async fn queue_total_count(&self, queue_name: &str) -> StorageResult<i64>;

    async fn queue_stats(&self, queue_name: &str) -> StorageResult<QueueStats> {
        let waiting = self.queue_waiting_count(queue_name).await?;
        let in_flight = self.queue_in_flight_count(queue_name).await?;
        let completed = self.queue_completed_count(queue_name).await?;
        let total = self.queue_total_count(queue_name).await?;

        Ok(QueueStats {
            waiting,
            in_flight,
            completed,
            total,
        })
    }

    async fn queue_purge(&self, queue_name: &str) -> StorageResult<u64>;
}

#[async_trait]
pub trait StoreExt<T: Entity>: Store<T> {
    async fn find_by_id_or_error(&self, id: &str) -> StorageResult<T> {
        self.find_by_id(id).await?.ok_or_else(|| StorageError::NotFound {
            entity_type: T::entity_type().to_string(),
            id: id.to_string(),
        })
    }

    async fn find_paginated(
        &self,
        filter: Option<FilterCondition>,
        page: u32,
        page_size: u32,
    ) -> StorageResult<(Vec<T>, u64)> {
        let options = QueryOptions::new()
            .with_limit(page_size)
            .with_offset(page * page_size);

        let entities = self.find_many(filter.clone(), options).await?;
        let total = self.count(filter).await?;

        Ok((entities, total))
    }
}

impl<T: Entity, S: Store<T>> StoreExt<T> for S {}
