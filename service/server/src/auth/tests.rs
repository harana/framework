#[cfg(test)]
mod tests {

    use super::*;
    use async_trait::async_trait;
    use harana_components_cache::{
        CacheError, CacheResult, CacheStore, GetOptions, KeyEntry, ListOptions, ListResponse,
        PutOptions,
    };
    use parking_lot::RwLock;

    pub(crate) struct MemoryCache {
        data: RwLock<HashMap<String, String>>,
    }

    impl MemoryCache {
        fn new() -> Arc<Self> {
            Arc::new(Self {
                data: RwLock::new(HashMap::new()),
            })
        }
    }

    /// Convenience factory for use in other test modules.
    pub(crate) fn memory_cache() -> Arc<dyn CacheStore> {
        MemoryCache::new()
    }

    #[async_trait]
    impl CacheStore for MemoryCache {
        async fn put(&self, key: &str, value: &str, _options: PutOptions) -> CacheResult<()> {
            self.data.write().insert(key.to_string(), value.to_string());
            Ok(())
        }

        async fn put_bytes(&self, key: &str, value: &[u8], _options: PutOptions) -> CacheResult<()> {
            self.data
                .write()
                .insert(key.to_string(), String::from_utf8_lossy(value).to_string());
            Ok(())
        }

        async fn get_text_with_options(
            &self,
            key: &str,
            _options: GetOptions,
        ) -> CacheResult<Option<String>> {
            Ok(self.data.read().get(key).cloned())
        }

        async fn get_bytes_with_options(
            &self,
            key: &str,
            _options: GetOptions,
        ) -> CacheResult<Option<Vec<u8>>> {
            Ok(self.data.read().get(key).map(|s| s.as_bytes().to_vec()))
        }

        async fn get_text_with_metadata<M: serde::de::DeserializeOwned + Send>(
            &self,
            key: &str,
        ) -> CacheResult<(Option<String>, Option<M>)> {
            Ok((self.data.read().get(key).cloned(), None))
        }

        async fn delete(&self, key: &str) -> CacheResult<()> {
            self.data.write().remove(key);
            Ok(())
        }

        async fn list(&self, _options: ListOptions) -> CacheResult<ListResponse> {
            Ok(ListResponse {
                keys: vec![],
                list_complete: true,
                cursor: None,
            })
        }
    }

    #[tokio::test]
    async fn test_create_and_get_session() {
        let cache = MemoryCache::new();
        let config = SessionConfig::default();
        let manager = SessionManager::new(config, cache);

        let session = manager
            .create_session(
                "user123",
                Some("test@example.com"),
                vec!["user".to_string()],
                AuthMethod::Password,
            )
            .await
            .unwrap();

        let retrieved = manager.get_session(&session.id).await.unwrap();
        assert_eq!(retrieved.user_id, "user123");
        assert_eq!(retrieved.email, Some("test@example.com".to_string()));
    }

    #[tokio::test]
    async fn test_destroy_session() {
        let cache = MemoryCache::new();
        let config = SessionConfig::default();
        let manager = SessionManager::new(config, cache);

        let session = manager
            .create_session("user123", None, vec![], AuthMethod::Passkey)
            .await
            .unwrap();
        manager.destroy_session(&session.id).await;

        assert!(manager.get_session(&session.id).await.is_err());
    }

}
