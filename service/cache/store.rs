use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value as JsonValue;

use crate::CacheResult;

#[derive(Debug, Clone, Default)]
pub struct PutOptions {
    pub expiration: Option<u64>,
    pub expiration_ttl: Option<u64>,
    pub metadata: Option<JsonValue>,
}

impl PutOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_expiration(mut self, expiration: u64) -> Self {
        self.expiration = Some(expiration);
        self
    }

    pub fn with_ttl(mut self, ttl_secs: u64) -> Self {
        self.expiration_ttl = Some(ttl_secs);
        self
    }

    pub fn with_metadata(mut self, metadata: impl Serialize) -> CacheResult<Self> {
        self.metadata =
            Some(serde_json::to_value(metadata).map_err(|e| crate::CacheError::SerializationError(e.to_string()))?);
        Ok(self)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GetOptions {
    pub cache_ttl: Option<u64>,
}

impl GetOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cache_ttl(mut self, ttl: u64) -> Self {
        self.cache_ttl = Some(ttl);
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListOptions {
    pub limit: Option<u64>,
    pub cursor: Option<String>,
    pub prefix: Option<String>,
}

impl ListOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct KeyEntry {
    pub name: String,
    pub expiration: Option<u64>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct ListResponse {
    pub keys: Vec<KeyEntry>,
    pub list_complete: bool,
    pub cursor: Option<String>,
}

#[async_trait]
pub trait CacheStore: Send + Sync {
    async fn put(&self, key: &str, value: &str, options: PutOptions) -> CacheResult<()>;
    async fn put_bytes(&self, key: &str, value: &[u8], options: PutOptions) -> CacheResult<()>;
    async fn put_json<T: Serialize + Send + Sync>(&self, key: &str, value: &T, options: PutOptions) -> CacheResult<()>
    where
        Self: Sized,
    {
        let json_str =
            serde_json::to_string(value).map_err(|e| crate::CacheError::SerializationError(e.to_string()))?;
        self.put(key, &json_str, options).await
    }

    async fn get_text(&self, key: &str) -> CacheResult<Option<String>> {
        self.get_text_with_options(key, GetOptions::default()).await
    }

    async fn get_text_with_options(&self, key: &str, options: GetOptions) -> CacheResult<Option<String>>;

    async fn get_bytes(&self, key: &str) -> CacheResult<Option<Vec<u8>>> {
        self.get_bytes_with_options(key, GetOptions::default()).await
    }

    async fn get_bytes_with_options(&self, key: &str, options: GetOptions) -> CacheResult<Option<Vec<u8>>>;

    async fn get_json<T: DeserializeOwned + Send>(&self, key: &str) -> CacheResult<Option<T>>
    where
        Self: Sized,
    {
        self.get_json_with_options(key, GetOptions::default()).await
    }

    async fn get_json_with_options<T: DeserializeOwned + Send>(
        &self,
        key: &str,
        options: GetOptions,
    ) -> CacheResult<Option<T>>
    where
        Self: Sized,
    {
        match self.get_text_with_options(key, options).await? {
            Some(text) => {
                let val =
                    serde_json::from_str(&text).map_err(|e| crate::CacheError::SerializationError(e.to_string()))?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }

    async fn get_text_with_metadata<M: DeserializeOwned + Send>(
        &self,
        key: &str,
    ) -> CacheResult<(Option<String>, Option<M>)>
    where
        Self: Sized;

    async fn get_json_with_metadata<T: DeserializeOwned + Send, M: DeserializeOwned + Send>(
        &self,
        key: &str,
    ) -> CacheResult<(Option<T>, Option<M>)>
    where
        Self: Sized,
    {
        let (text, meta) = self.get_text_with_metadata::<M>(key).await?;
        let value = text
            .map(|t| serde_json::from_str(&t))
            .transpose()
            .map_err(|e| crate::CacheError::SerializationError(e.to_string()))?;
        Ok((value, meta))
    }

    async fn delete(&self, key: &str) -> CacheResult<()>;

    async fn list(&self, options: ListOptions) -> CacheResult<ListResponse>;

    async fn exists(&self, key: &str) -> CacheResult<bool> {
        Ok(self.get_text(key).await?.is_some())
    }

    async fn get_or_insert<T, F, Fut>(&self, key: &str, ttl_secs: Option<u64>, f: F) -> CacheResult<T>
    where
        Self: Sized,
        T: Serialize + DeserializeOwned + Send + Sync,
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = CacheResult<T>> + Send,
    {
        if let Some(cached) = self.get_json::<T>(key).await? {
            return Ok(cached);
        }
        let value = f().await?;
        let mut opts = PutOptions::new();
        if let Some(ttl) = ttl_secs {
            opts = opts.with_ttl(ttl);
        }
        self.put_json(key, &value, opts).await?;
        Ok(value)
    }
}
