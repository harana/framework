use async_trait::async_trait;
use base64::Engine;
use mongodb::{
    bson::{doc, Bson, DateTime as BsonDateTime, Document},
    options::ClientOptions,
    Client, Collection, Database, IndexModel,
};
use mongodb::options::IndexOptions;
use serde::de::DeserializeOwned;

use crate::error::{CacheError, CacheResult};
use crate::model::{GetOptions, KeyEntry, ListOptions, ListResponse, PutOptions};
use crate::service::CacheService;

#[cfg(test)]
mod tests;

const COLLECTION_NAME: &str = "cache";

/// A MongoDB-backed implementation of `CacheService`.
///
/// Documents are stored with the following schema:
///
/// ```json
/// {
///   "_id": "<key>",
///   "value": "<string value>",
///   "metadata": <optional JSON>,
///   "expires_at": <optional BsonDateTime>
/// }
/// ```
///
/// A TTL index on `expires_at` allows MongoDB to automatically remove expired
/// entries. Reads also filter out expired documents for immediate consistency
/// in case the TTL monitor has not yet run.
pub struct MongoCacheService {
    collection: Collection<Document>,
}

impl MongoCacheService {
    /// Create a new `MongoCacheService` from an existing `Database` handle, using
    /// the default collection name (`cache`).
    pub async fn new(database: &Database) -> CacheResult<Self> {
        Self::with_collection_name(database, COLLECTION_NAME).await
    }

    /// Create a new `MongoCacheService` from an existing `Database` handle with a
    /// custom collection name.
    pub async fn with_collection_name(database: &Database, collection_name: &str) -> CacheResult<Self> {
        let collection = database.collection::<Document>(collection_name);

        // Ensure a TTL index on `expires_at`.  Documents whose `expires_at`
        // value has passed are automatically deleted by MongoDB's background
        // TTL monitor (runs roughly every 60 s).
        let ttl_index = IndexModel::builder()
            .keys(doc! { "expires_at": 1 })
            .options(
                IndexOptions::builder()
                    .expire_after(std::time::Duration::from_secs(0))
                    .build(),
            )
            .build();

        collection
            .create_index(ttl_index)
            .await
            .map_err(|e| CacheError::ConnectionError(format!("Failed to create TTL index: {e}")))?;

        Ok(Self { collection })
    }

    /// Create from a MongoDB connection string.
    pub async fn connect(url: &str, database_name: &str) -> CacheResult<Self> {
        let client_options = ClientOptions::parse(url)
            .await
            .map_err(|e| CacheError::ConnectionError(format!("Failed to parse MongoDB URL: {e}")))?;

        let client = Client::with_options(client_options)
            .map_err(|e| CacheError::ConnectionError(format!("Failed to create MongoDB client: {e}")))?;

        let database = client.database(database_name);
        Self::new(&database).await
    }

    /// Returns a reference to the inner MongoDB `Collection`.
    pub fn collection(&self) -> &Collection<Document> {
        &self.collection
    }

    /// Compute the `expires_at` `BsonDateTime` from `PutOptions`.
    fn resolve_expiry(options: &PutOptions) -> Option<BsonDateTime> {
        if let Some(ttl) = options.expiration_ttl {
            let millis = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as i64
                + (ttl as i64 * 1000);
            Some(BsonDateTime::from_millis(millis))
        } else if let Some(exp) = options.expiration {
            Some(BsonDateTime::from_millis(exp as i64 * 1000))
        } else {
            None
        }
    }

    /// Build an upsert document from a string value and options.
    fn build_upsert_doc(value: &str, options: &PutOptions) -> Document {
        let mut set = doc! {
            "value": value,
        };

        if let Some(ref metadata) = options.metadata {
            if let Ok(bson_meta) = mongodb::bson::to_bson(metadata) {
                set.insert("metadata", bson_meta);
            }
        } else {
            // Explicitly unset metadata if not provided.
            set.insert("metadata", Bson::Null);
        }

        if let Some(exp) = Self::resolve_expiry(options) {
            set.insert("expires_at", exp);
        } else {
            // Remove any previous expiry.
            set.insert("expires_at", Bson::Null);
        }

        set
    }

    /// A filter that matches by `_id` and excludes expired documents.
    fn not_expired_filter(key: &str) -> Document {
        let now = BsonDateTime::now();
        doc! {
            "_id": key,
            "$or": [
                { "expires_at": Bson::Null },
                { "expires_at": { "$exists": false } },
                { "expires_at": { "$gt": now } },
            ]
        }
    }
}

#[async_trait]
impl CacheService for MongoCacheService {
    async fn put(&self, key: &str, value: &str, options: PutOptions) -> CacheResult<()> {
        let set_doc = Self::build_upsert_doc(value, &options);

        self.collection
            .update_one(
                doc! { "_id": key },
                doc! { "$set": set_doc },
            )
            .upsert(true)
            .await
            .map_err(|e| CacheError::BackendError(format!("put failed: {e}")))?;

        Ok(())
    }

    async fn put_bytes(&self, key: &str, value: &[u8], options: PutOptions) -> CacheResult<()> {
        let encoded = base64::engine::general_purpose::STANDARD.encode(value);
        self.put(key, &encoded, options).await
    }

    async fn get_text_with_options(&self, key: &str, _options: GetOptions) -> CacheResult<Option<String>> {
        let filter = Self::not_expired_filter(key);

        let result = self
            .collection
            .find_one(filter)
            .await
            .map_err(|e| CacheError::BackendError(format!("get_text failed: {e}")))?;

        match result {
            Some(doc) => {
                let value = doc
                    .get_str("value")
                    .map_err(|e| CacheError::SerializationError(format!("missing 'value' field: {e}")))?;
                Ok(Some(value.to_string()))
            }
            None => Ok(None),
        }
    }

    async fn get_bytes_with_options(&self, key: &str, options: GetOptions) -> CacheResult<Option<Vec<u8>>> {
        match self.get_text_with_options(key, options).await? {
            Some(text) => {
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(&text)
                    .map_err(|e| CacheError::SerializationError(format!("base64 decode: {e}")))?;
                Ok(Some(bytes))
            }
            None => Ok(None),
        }
    }

    async fn get_text_with_metadata<M: DeserializeOwned + Send>(
        &self,
        key: &str,
    ) -> CacheResult<(Option<String>, Option<M>)> {
        let filter = Self::not_expired_filter(key);

        let result = self
            .collection
            .find_one(filter)
            .await
            .map_err(|e| CacheError::BackendError(format!("get_text_with_metadata failed: {e}")))?;

        match result {
            Some(doc) => {
                let value = doc
                    .get_str("value")
                    .map_err(|e| CacheError::SerializationError(format!("missing 'value' field: {e}")))?
                    .to_string();

                let meta = match doc.get("metadata") {
                    Some(bson_val) if *bson_val != Bson::Null => {
                        let json_val: serde_json::Value = mongodb::bson::from_bson(bson_val.clone())
                            .map_err(|e| CacheError::SerializationError(format!("metadata bson->json: {e}")))?;
                        let m: M = serde_json::from_value(json_val)
                            .map_err(|e| CacheError::SerializationError(format!("metadata deserialize: {e}")))?;
                        Some(m)
                    }
                    _ => None,
                };

                Ok((Some(value), meta))
            }
            None => Ok((None, None)),
        }
    }

    async fn delete(&self, key: &str) -> CacheResult<()> {
        self.collection
            .delete_one(doc! { "_id": key })
            .await
            .map_err(|e| CacheError::BackendError(format!("delete failed: {e}")))?;
        Ok(())
    }

    async fn list(&self, options: ListOptions) -> CacheResult<ListResponse> {
        let now = BsonDateTime::now();
        let limit = options.limit.unwrap_or(1000) as i64;

        // Base filter: exclude expired documents.
        let mut filter = doc! {
            "$or": [
                { "expires_at": Bson::Null },
                { "expires_at": { "$exists": false } },
                { "expires_at": { "$gt": now } },
            ]
        };

        // Prefix filter via regex.
        if let Some(ref prefix) = options.prefix {
            let escaped = regex::escape(prefix);
            filter.insert("_id", doc! { "$regex": format!("^{escaped}") });
        }

        // Cursor-based pagination: skip keys <= cursor.
        if let Some(ref cursor) = options.cursor {
            filter.insert("_id", doc! { "$gt": cursor });
        }

        let find_options = mongodb::options::FindOptions::builder()
            .sort(doc! { "_id": 1 })
            .limit(limit + 1) // Fetch one extra to detect if there are more.
            .build();

        let mut cursor = self
            .collection
            .find(filter)
            .with_options(find_options)
            .await
            .map_err(|e| CacheError::BackendError(format!("list failed: {e}")))?;

        let mut keys = Vec::new();
        use futures::TryStreamExt;
        while let Some(doc) = cursor
            .try_next()
            .await
            .map_err(|e| CacheError::BackendError(format!("list cursor: {e}")))?
        {
            let name = doc
                .get_str("_id")
                .unwrap_or_default()
                .to_string();

            let expiration = doc.get("expires_at").and_then(|v| {
                if let Bson::DateTime(dt) = v {
                    Some((dt.timestamp_millis() / 1000) as u64)
                } else {
                    None
                }
            });

            let metadata = doc.get("metadata").and_then(|v| {
                if *v == Bson::Null {
                    None
                } else {
                    serde_json::to_value(v).ok()
                }
            });

            keys.push(KeyEntry {
                name,
                expiration,
                metadata,
            });
        }

        let list_complete = keys.len() as i64 <= limit;
        let next_cursor = if list_complete {
            None
        } else {
            keys.pop().map(|k| k.name)
        };

        Ok(ListResponse {
            keys,
            list_complete,
            cursor: next_cursor,
        })
    }
}
