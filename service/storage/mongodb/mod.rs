use crate::{Store, Entity, FilterCondition, QueryOptions, StorageError, StorageResult, QueueMessage};
use async_trait::async_trait;
use futures::{Stream, StreamExt, TryStreamExt};
use mongodb::{
    bson::{doc, Document},
    options::{ChangeStreamOptions, ClientOptions, FindOptions, FullDocumentType, ReplaceOptions, FindOneAndUpdateOptions, IndexOptions, ReturnDocument},
    Client, Collection, Database, IndexModel,
};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;
use ulid::Ulid;

// Re-export watch types
pub use watch::{ChangeOperation, CollectionChangeEvent, UpdateDescription, WatchOptions};

mod watch;
mod utils;

#[cfg(test)]
mod tests;

pub(crate) use utils::*;

pub struct MongoStore<T: Entity> {
    collection: Collection<T>,
    client: Client,
    database_name: String,
    _phantom: PhantomData<T>,
}

impl<T: Entity> MongoStore<T> {
    pub fn new(client: Client, database: &Database, collection_name: &str) -> Self {
        Self {
            collection: database.collection(collection_name),
            client,
            database_name: database.name().to_string(),
            _phantom: PhantomData,
        }
    }

    pub fn with_entity_type(client: Client, database: &Database) -> Self {
        Self::new(client, database, T::entity_type())
    }

    pub async fn connect(
        url: &str,
        database_name: &str,
        collection_name: &str,
    ) -> StorageResult<Self> {
        let client_options = ClientOptions::parse(url)
            .await
            .map_err(map_connection_error("Failed to parse MongoDB URL"))?;

        let client = Client::with_options(client_options)
            .map_err(map_connection_error("Failed to create MongoDB client"))?;

        let database = client.database(database_name);
        Ok(Self::new(client, &database, collection_name))
    }

    pub fn collection(&self) -> &Collection<T> {
        &self.collection
    }    

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    /// Watches for changes on the collection using MongoDB Change Streams.
    pub async fn watch_changes(
        &self,
        options: WatchOptions,
    ) -> StorageResult<impl Stream<Item = StorageResult<CollectionChangeEvent>> + '_> {
        let mut change_stream_options = ChangeStreamOptions::default();
        change_stream_options.full_document = options.full_document;
        change_stream_options.full_document_before_change = options.full_document_before_change;
        change_stream_options.max_await_time = options.max_await_time_ms.map(std::time::Duration::from_millis);
        change_stream_options.batch_size = options.batch_size;

        let change_stream = self.collection.clone_with_type::<Document>()
            .watch()
            .pipeline(options.pipeline.unwrap_or_default())
            .with_options(change_stream_options)
            .await
            .map_err(map_connection_error("Failed to create change stream"))?;

        let database_name = self.database_name.clone();
        
        Ok(change_stream.map(move |result| {
            result
                .map_err(map_mongo_error("Change stream error"))
                .and_then(|event| watch::convert_change_event(event, &database_name))
        }))
    }

    /// Watches for changes on the collection, filtered by specific operations.
    pub async fn watch_operations(
        &self,
        operations: &[ChangeOperation],
    ) -> StorageResult<impl Stream<Item = StorageResult<CollectionChangeEvent>> + '_> {
        self.watch_changes(
            WatchOptions::new()
                .with_full_document(FullDocumentType::UpdateLookup)
                .with_pipeline(WatchOptions::filter_operations(operations))
        ).await
    }

    /// Watches for insert operations only.
    pub async fn watch_inserts(
        &self,
    ) -> StorageResult<impl Stream<Item = StorageResult<CollectionChangeEvent>> + '_> {
        self.watch_operations(&[ChangeOperation::Insert]).await
    }

    /// Watches for update operations only.
    pub async fn watch_updates(
        &self,
    ) -> StorageResult<impl Stream<Item = StorageResult<CollectionChangeEvent>> + '_> {
        self.watch_operations(&[ChangeOperation::Update, ChangeOperation::Replace]).await
    }

    /// Watches for delete operations only.
    pub async fn watch_deletes(
        &self,
    ) -> StorageResult<impl Stream<Item = StorageResult<CollectionChangeEvent>> + '_> {
        self.watch_operations(&[ChangeOperation::Delete]).await
    }

    fn build_find_options(options: &QueryOptions) -> FindOptions {
        let sort = options.sort_by.as_ref().map(|sort_by| {
            let sort_order = if options.sort_desc { -1 } else { 1 };
            doc! { sort_by: sort_order }
        });
        build_find_options(
            options.limit.map(|l| l as i64),
            options.offset.map(|o| o as u64),
            sort,
            None,
        )
    }

    fn id_field() -> &'static str {
        "id"
    }

    /// Deserialize payload from a document, returning an error on failure.
    fn deserialize_payload<P: DeserializeOwned>(doc: &Document) -> StorageResult<P> {
        let payload_doc = doc.get_document("payload")
            .map_err(|_| StorageError::SerializationError("Missing payload field".to_string()))?;
        mongodb::bson::from_document(payload_doc.clone())
            .map_err(|e| StorageError::SerializationError(format!("Failed to deserialize payload: {}", e)))
    }
}

#[async_trait]
impl<T: Entity> Store<T> for MongoStore<T> {
    type Filter = Document;

    async fn create(&self, entity: &T) -> StorageResult<()> {
        let id = entity.id();
        if self.exists(id).await? {
            return Err(StorageError::DuplicateKey {
                entity_type: T::entity_type().to_string(),
                id: id.to_string(),
            });
        }

        self.collection
            .insert_one(entity)
            .await
            .map_err(map_mongo_error("Failed to create entity"))?;

        Ok(())
    }

    async fn create_many(&self, entities: &[T]) -> StorageResult<usize> {
        if entities.is_empty() {
            return Ok(0);
        }

        let result = self.collection
            .insert_many(entities)
            .await
            .map_err(map_mongo_error("Failed to create entities"))?;

        Ok(result.inserted_ids.len())
    }

    async fn find_by_id(&self, id: &str) -> StorageResult<Option<T>> {
        self.collection
            .find_one(doc! { Self::id_field(): id })
            .await
            .map_err(map_mongo_error("Failed to find entity"))
    }

    async fn find_many(
        &self,
        filter: Option<FilterCondition>,
        options: QueryOptions,
    ) -> StorageResult<Vec<T>> {
        let filter_doc = filter.map(|f| filter_to_document(&f)).transpose()?.unwrap_or_default();
        let find_options = Self::build_find_options(&options);

        self.collection
            .find(filter_doc)
            .with_options(find_options)
            .await
            .map_err(map_mongo_error("Failed to find entities"))?
            .try_collect()
            .await
            .map_err(map_mongo_error("Failed to collect results"))
    }

    async fn count(&self, filter: Option<FilterCondition>) -> StorageResult<u64> {
        let filter_doc = filter.map(|f| filter_to_document(&f)).transpose()?.unwrap_or_default();

        self.collection
            .count_documents(filter_doc)
            .await
            .map_err(map_mongo_error("Failed to count entities"))
    }

    async fn update(&self, entity: &T) -> StorageResult<()> {
        let id = entity.id();
        let result = self.collection
            .replace_one(doc! { Self::id_field(): id }, entity)
            .await
            .map_err(map_mongo_error("Failed to update entity"))?;

        if result.matched_count == 0 {
            return Err(StorageError::NotFound {
                entity_type: T::entity_type().to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }

    async fn upsert(&self, entity: &T) -> StorageResult<()> {
        self.collection
            .replace_one(doc! { Self::id_field(): entity.id() }, entity)
            .with_options(ReplaceOptions::builder().upsert(true).build())
            .await
            .map_err(map_mongo_error("Failed to upsert entity"))?;

        Ok(())
    }

    async fn update_many(
        &self,
        filter: FilterCondition,
        updates: serde_json::Map<String, serde_json::Value>,
    ) -> StorageResult<u64> {
        let filter_doc = filter_to_document(&filter)?;

        let mut update_doc = Document::new();
        for (key, value) in updates {
            update_doc.insert(key, json_value_to_bson(&value)?);
        }

        let result = self.collection
            .update_many(filter_doc, doc! { "$set": update_doc })
            .await
            .map_err(map_mongo_error("Failed to update entities"))?;

        Ok(result.modified_count)
    }

    async fn delete(&self, id: &str) -> StorageResult<bool> {
        let result = self.collection
            .delete_one(doc! { Self::id_field(): id })
            .await
            .map_err(map_mongo_error("Failed to delete entity"))?;

        Ok(result.deleted_count > 0)
    }

    async fn delete_many(&self, filter: FilterCondition) -> StorageResult<u64> {
        let result = self.collection
            .delete_many(filter_to_document(&filter)?)
            .await
            .map_err(map_mongo_error("Failed to delete entities"))?;

        Ok(result.deleted_count)
    }

    async fn delete_all(&self) -> StorageResult<u64> {
        let result = self.collection
            .delete_many(doc! {})
            .await
            .map_err(map_mongo_error("Failed to delete all entities"))?;

        Ok(result.deleted_count)
    }

    // === Text Search Operations ===

    async fn text_search(
        &self,
        text: &str,
        limit: Option<i64>,
        offset: Option<u64>,
    ) -> StorageResult<Vec<T>> {
        self.text_search_with_filter(text, doc! {}, limit, offset).await
    }

    async fn text_search_with_filter(
        &self,
        text: &str,
        additional_filter: Document,
        limit: Option<i64>,
        offset: Option<u64>,
    ) -> StorageResult<Vec<T>> {
        let filter = if additional_filter.is_empty() {
            doc! { "$text": { "$search": text } }
        } else {
            doc! { "$and": [{ "$text": { "$search": text } }, additional_filter] }
        };

        let opts = build_find_options(
            limit,
            offset,
            Some(doc! { "score": { "$meta": "textScore" } }),
            Some(doc! { "score": { "$meta": "textScore" } }),
        );

        self.collection
            .find(filter)
            .with_options(opts)
            .await
            .map_err(map_mongo_error("Text search failed"))?
            .try_collect()
            .await
            .map_err(map_mongo_error("Failed to collect text search results"))
    }

    // === Queue Operations ===

    async fn create_queue(&self, queue_name: &str) -> StorageResult<()> {
        let database = self.client.database(&self.database_name);
        let _ = database.create_collection(queue_name).await;

        // Create indexes for queue operations
        let collection: Collection<Document> = database.collection(queue_name);
        
        let mut opts1 = IndexOptions::default();
        opts1.unique = Some(false);
        opts1.sparse = Some(false);
        collection
            .create_index(IndexModel::builder().keys(doc! { "deleted": 1, "visible": 1 }).options(opts1).build())
            .await
            .map_err(map_mongo_error("Failed to create index"))?;

        let mut opts2 = IndexOptions::default();
        opts2.unique = Some(true);
        opts2.sparse = Some(true);
        collection
            .create_index(IndexModel::builder().keys(doc! { "ack": 1 }).options(opts2).build())
            .await
            .map_err(map_mongo_error("Failed to create index"))?;

        Ok(())
    }

    async fn queue_add<Q: Serialize + Send + Sync>(
        &self,
        queue_name: &str,
        items: &[Q],
        delay_secs: Option<i64>,
    ) -> StorageResult<()> {
        if items.is_empty() {
            return Ok(());
        }

        let now = chrono::Utc::now();
        let visible_at = (now + chrono::Duration::seconds(delay_secs.unwrap_or(0))).timestamp_millis();
        
        let docs: Vec<Document> = items
            .iter()
            .filter_map(|item| {
                Some(doc! {
                    "_id": Ulid::new().to_string(),
                    "tries": 0i32,
                    "deleted": mongodb::bson::Bson::Int32(-1),
                    "visible": visible_at,
                    "payload": mongodb::bson::to_document(item).ok()?,
                })
            })
            .collect();

        if !docs.is_empty() {
            let database = self.client.database(&self.database_name);
            let collection: Collection<Document> = database.collection(queue_name);
            collection
                .insert_many(docs)
                .await
                .map_err(map_mongo_error("Failed to add items to queue"))?;
        }

        Ok(())
    }

    async fn queue_get<Q: DeserializeOwned + Send>(
        &self,
        queue_name: &str,
        visibility_secs: i64,
    ) -> StorageResult<Option<QueueMessage<Q>>> {
        let now_ms = chrono::Utc::now().timestamp_millis();
        let ack_id = Ulid::new().to_string();
        
        let database = self.client.database(&self.database_name);
        let collection: Collection<Document> = database.collection(queue_name);
        
        let result = collection
            .find_one_and_update(
                doc! { "visible": { "$lte": now_ms }, "deleted": -1 },
                doc! { "$inc": { "tries": 1 }, "$set": { "ack": &ack_id, "visible": now_ms + visibility_secs * 1000 } },
            )
            .with_options(FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .sort(doc! { "_id": 1 })
                .build())
            .await
            .map_err(map_mongo_error("Failed to get item from queue"))?;

        result.map(|doc| {
            Ok(QueueMessage {
                ack_id,
                payload: Self::deserialize_payload(&doc)?,
                tries: doc.get_i32("tries").unwrap_or(1),
            })
        }).transpose()
    }

    async fn queue_ack<Q: DeserializeOwned + Send>(
        &self,
        queue_name: &str,
        ack_id: &str,
    ) -> StorageResult<Option<Q>> {
        let now_ms = chrono::Utc::now().timestamp_millis();
        
        let database = self.client.database(&self.database_name);
        let collection: Collection<Document> = database.collection(queue_name);
        
        let result = collection
            .find_one_and_update(
                doc! { "ack": ack_id, "deleted": -1, "visible": { "$gt": now_ms } },
                doc! { "$set": { "deleted": now_ms } },
            )
            .with_options(FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build())
            .await
            .map_err(map_mongo_error("Failed to acknowledge message"))?;

        result.map(|doc| Self::deserialize_payload(&doc)).transpose()
    }

    async fn queue_ping<Q: DeserializeOwned + Send>(
        &self,
        queue_name: &str,
        ack_id: &str,
        visibility_secs: i64,
    ) -> StorageResult<Option<Q>> {
        let now_ms = chrono::Utc::now().timestamp_millis();
        
        let database = self.client.database(&self.database_name);
        let collection: Collection<Document> = database.collection(queue_name);
        
        let result = collection
            .find_one_and_update(
                doc! { "ack": ack_id, "deleted": -1, "visible": { "$gt": now_ms } },
                doc! { "$set": { "visible": now_ms + visibility_secs * 1000 } },
            )
            .with_options(FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build())
            .await
            .map_err(map_mongo_error("Failed to ping message"))?;

        result.map(|doc| Self::deserialize_payload(&doc)).transpose()
    }

    async fn queue_waiting_count(&self, queue_name: &str) -> StorageResult<i64> {
        let now_ms = chrono::Utc::now().timestamp_millis();
        let database = self.client.database(&self.database_name);
        let collection: Collection<Document> = database.collection(queue_name);
        collection
            .count_documents(doc! { "deleted": -1, "visible": { "$lte": now_ms } })
            .await
            .map(|c| c as i64)
            .map_err(map_mongo_error("Failed to count documents"))
    }

    async fn queue_in_flight_count(&self, queue_name: &str) -> StorageResult<i64> {
        let now_ms = chrono::Utc::now().timestamp_millis();
        let database = self.client.database(&self.database_name);
        let collection: Collection<Document> = database.collection(queue_name);
        collection
            .count_documents(doc! { "ack": { "$exists": true }, "deleted": -1, "visible": { "$gt": now_ms } })
            .await
            .map(|c| c as i64)
            .map_err(map_mongo_error("Failed to count documents"))
    }

    async fn queue_completed_count(&self, queue_name: &str) -> StorageResult<i64> {
        let database = self.client.database(&self.database_name);
        let collection: Collection<Document> = database.collection(queue_name);
        collection
            .count_documents(doc! { "deleted": { "$ne": -1 } })
            .await
            .map(|c| c as i64)
            .map_err(map_mongo_error("Failed to count documents"))
    }

    async fn queue_total_count(&self, queue_name: &str) -> StorageResult<i64> {
        let database = self.client.database(&self.database_name);
        let collection: Collection<Document> = database.collection(queue_name);
        collection
            .count_documents(doc! {})
            .await
            .map(|c| c as i64)
            .map_err(map_mongo_error("Failed to count documents"))
    }

    async fn queue_purge(&self, queue_name: &str) -> StorageResult<u64> {
        let database = self.client.database(&self.database_name);
        let collection: Collection<Document> = database.collection(queue_name);
        collection
            .delete_many(doc! { "deleted": { "$ne": -1 } })
            .await
            .map(|r| r.deleted_count)
            .map_err(map_mongo_error("Failed to purge queue"))
    }
}
