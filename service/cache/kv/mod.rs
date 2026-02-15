use async_trait::async_trait;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use worker::kv::KvStore;

use crate::error::{CacheError, CacheResult};
use crate::model::{GetOptions, KeyEntry, ListOptions, ListResponse, PutOptions};
use crate::service::CacheService;

pub struct KvCacheService {
    kv: KvStore,
}

// `KvStore` is already Send + Sync via `unsafe impl` in workers-rs.
// We mirror that here so `CacheService` trait bounds are satisfied.
unsafe impl Send for KvCacheService {}
unsafe impl Sync for KvCacheService {}

impl KvCacheService {
    /// Wrap an existing `worker::KvStore`.
    pub fn new(kv: KvStore) -> Self {
        Self { kv }
    }

    /// Create from a `worker::Env` by KV binding name.
    pub fn from_env(env: &worker::Env, binding: &str) -> CacheResult<Self> {
        let kv = env
            .kv(binding)
            .map_err(|e| CacheError::ConnectionError(format!("Failed to get KV binding '{}': {}", binding, e)))?;
        Ok(Self::new(kv))
    }

    /// Returns a reference to the inner `KvStore`.
    pub fn inner(&self) -> &KvStore {
        &self.kv
    }

    /// Apply `PutOptions` to a workers-rs `PutOptionsBuilder` and execute.
    async fn execute_put(&self, key: &str, value: &str, options: PutOptions) -> CacheResult<()> {
        let mut builder = self
            .kv
            .put(key, value)
            .map_err(|e| CacheError::BackendError(e.to_string()))?;

        if let Some(exp) = options.expiration {
            builder = builder.expiration(exp);
        }
        if let Some(ttl) = options.expiration_ttl {
            builder = builder.expiration_ttl(ttl);
        }
        if let Some(meta) = options.metadata {
            builder = builder
                .metadata(meta)
                .map_err(|e| CacheError::SerializationError(e.to_string()))?;
        }

        builder
            .execute()
            .await
            .map_err(|e| CacheError::BackendError(e.to_string()))
    }

    /// Apply `PutOptions` to a workers-rs byte put and execute.
    async fn execute_put_bytes(&self, key: &str, value: &[u8], options: PutOptions) -> CacheResult<()> {
        let mut builder = self
            .kv
            .put_bytes(key, value)
            .map_err(|e| CacheError::BackendError(e.to_string()))?;

        if let Some(exp) = options.expiration {
            builder = builder.expiration(exp);
        }
        if let Some(ttl) = options.expiration_ttl {
            builder = builder.expiration_ttl(ttl);
        }
        if let Some(meta) = options.metadata {
            builder = builder
                .metadata(meta)
                .map_err(|e| CacheError::SerializationError(e.to_string()))?;
        }

        builder
            .execute()
            .await
            .map_err(|e| CacheError::BackendError(e.to_string()))
    }
}

#[async_trait]
impl CacheService for KvCacheService {
    async fn put(&self, key: &str, value: &str, options: PutOptions) -> CacheResult<()> {
        self.execute_put(key, value, options).await
    }

    async fn put_bytes(&self, key: &str, value: &[u8], options: PutOptions) -> CacheResult<()> {
        self.execute_put_bytes(key, value, options).await
    }

    async fn get_text_with_options(&self, key: &str, options: GetOptions) -> CacheResult<Option<String>> {
        let mut builder = self.kv.get(key);
        if let Some(ttl) = options.cache_ttl {
            builder = builder.cache_ttl(ttl);
        }
        builder
            .text()
            .await
            .map_err(|e| CacheError::BackendError(e.to_string()))
    }

    async fn get_bytes_with_options(&self, key: &str, options: GetOptions) -> CacheResult<Option<Vec<u8>>> {
        let mut builder = self.kv.get(key);
        if let Some(ttl) = options.cache_ttl {
            builder = builder.cache_ttl(ttl);
        }
        builder
            .bytes()
            .await
            .map_err(|e| CacheError::BackendError(e.to_string()))
    }

    async fn get_json_with_options<T: DeserializeOwned + Send>(
        &self,
        key: &str,
        options: GetOptions,
    ) -> CacheResult<Option<T>> {
        let mut builder = self.kv.get(key);
        if let Some(ttl) = options.cache_ttl {
            builder = builder.cache_ttl(ttl);
        }
        builder
            .json::<T>()
            .await
            .map_err(|e| CacheError::BackendError(e.to_string()))
    }

    async fn get_text_with_metadata<M: DeserializeOwned + Send>(
        &self,
        key: &str,
    ) -> CacheResult<(Option<String>, Option<M>)> {
        self.kv
            .get(key)
            .text_with_metadata::<M>()
            .await
            .map_err(|e| CacheError::BackendError(e.to_string()))
    }

    async fn delete(&self, key: &str) -> CacheResult<()> {
        self.kv
            .delete(key)
            .await
            .map_err(|e| CacheError::BackendError(e.to_string()))
    }

    async fn list(&self, options: ListOptions) -> CacheResult<ListResponse> {
        let mut builder = self.kv.list();

        if let Some(limit) = options.limit {
            builder = builder.limit(limit);
        }
        if let Some(cursor) = options.cursor {
            builder = builder.cursor(cursor);
        }
        if let Some(prefix) = options.prefix {
            builder = builder.prefix(prefix);
        }

        let resp = builder
            .execute()
            .await
            .map_err(|e| CacheError::BackendError(e.to_string()))?;

        let keys = resp
            .keys
            .into_iter()
            .map(|k| KeyEntry {
                name: k.name,
                expiration: k.expiration,
                metadata: k.metadata,
            })
            .collect();

        Ok(ListResponse {
            keys,
            list_complete: resp.list_complete,
            cursor: resp.cursor,
        })
    }
}
