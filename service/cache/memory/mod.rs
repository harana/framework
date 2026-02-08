use async_trait::async_trait;
use base64::Engine;
use dashmap::DashMap;
use serde::de::DeserializeOwned;
use std::time::{Duration, Instant};

use crate::error::{CacheError, CacheResult};
use crate::store::{CacheStore, GetOptions, KeyEntry, ListOptions, ListResponse, PutOptions};

struct Entry {
    value: String,
    metadata: Option<serde_json::Value>,
    expires_at: Option<Instant>,
}

impl Entry {
    fn is_expired(&self) -> bool {
        self.expires_at.map_or(false, |exp| Instant::now() >= exp)
    }
}

pub struct MemoryCacheStore {
    data: DashMap<String, Entry>,
}

impl Default for MemoryCacheStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryCacheStore {
    /// Create a new, empty in-memory cache.
    pub fn new() -> Self {
        Self { data: DashMap::new() }
    }

    /// Resolve the expiry `Instant` from `PutOptions`.
    fn resolve_expiry(options: &PutOptions) -> Option<Instant> {
        if let Some(ttl) = options.expiration_ttl {
            Some(Instant::now() + Duration::from_secs(ttl))
        } else if let Some(exp) = options.expiration {
            // `expiration` is an absolute unix timestamp (seconds).
            let now_unix = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            if exp > now_unix {
                Some(Instant::now() + Duration::from_secs(exp - now_unix))
            } else {
                // Already expired – store it but it will be evicted on read.
                Some(Instant::now())
            }
        } else {
            None
        }
    }
}

#[async_trait]
impl CacheStore for MemoryCacheStore {
    async fn put(&self, key: &str, value: &str, options: PutOptions) -> CacheResult<()> {
        let expires_at = Self::resolve_expiry(&options);
        self.data.insert(
            key.to_string(),
            Entry {
                value: value.to_string(),
                metadata: options.metadata.clone(),
                expires_at,
            },
        );
        Ok(())
    }

    async fn put_bytes(&self, key: &str, value: &[u8], options: PutOptions) -> CacheResult<()> {
        // Store bytes as a base64-encoded string for simplicity.
        let encoded = base64::engine::general_purpose::STANDARD.encode(value);
        self.put(key, &encoded, options).await
    }

    async fn get_text_with_options(&self, key: &str, _options: GetOptions) -> CacheResult<Option<String>> {
        match self.data.get(key) {
            Some(entry) if !entry.is_expired() => Ok(Some(entry.value.clone())),
            Some(_) => {
                // Lazy eviction of expired entries.
                drop(self.data.remove(key));
                Ok(None)
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
        match self.data.get(key) {
            Some(entry) if !entry.is_expired() => {
                let meta = entry
                    .metadata
                    .as_ref()
                    .map(|m| serde_json::from_value(m.clone()))
                    .transpose()
                    .map_err(|e| CacheError::SerializationError(e.to_string()))?;
                Ok((Some(entry.value.clone()), meta))
            }
            Some(_) => {
                drop(self.data.remove(key));
                Ok((None, None))
            }
            None => Ok((None, None)),
        }
    }

    async fn delete(&self, key: &str) -> CacheResult<()> {
        self.data.remove(key);
        Ok(())
    }

    async fn list(&self, options: ListOptions) -> CacheResult<ListResponse> {
        let prefix = options.prefix.as_deref().unwrap_or("");
        let limit = options.limit.unwrap_or(1000) as usize;

        let mut keys: Vec<KeyEntry> = self
            .data
            .iter()
            .filter(|entry| !entry.value().is_expired() && entry.key().starts_with(prefix))
            .map(|entry| KeyEntry {
                name: entry.key().clone(),
                expiration: entry.value().expires_at.map(|exp| {
                    let remaining = exp.saturating_duration_since(Instant::now());
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                        + remaining.as_secs()
                }),
                metadata: entry.value().metadata.clone(),
            })
            .collect();

        keys.sort_by(|a, b| a.name.cmp(&b.name));

        let list_complete = keys.len() <= limit;
        let cursor = if list_complete {
            None
        } else {
            keys.get(limit).map(|k| k.name.clone())
        };

        keys.truncate(limit);

        Ok(ListResponse {
            keys,
            list_complete,
            cursor,
        })
    }
}

#[cfg(test)]
mod tests;
