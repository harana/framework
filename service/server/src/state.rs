use crate::auth::{JwtManager, OAuthManager, SessionManager};
use crate::config::ServerConfig;
#[cfg(feature = "standalone")]
use harana_components_blob::BlobStore;
use harana_components_cache::CacheStore;
#[cfg(feature = "standalone")]
use harana_components_events::{EventBus, EventBusConfig};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ServerConfig>,
    pub jwt: Arc<JwtManager>,
    pub oauth: Arc<OAuthManager>,
    pub sessions: Arc<SessionManager>,
    #[cfg(feature = "standalone")]
    pub event_bus: Arc<EventBus>,
    #[cfg(feature = "standalone")]
    blob_store: Option<Arc<dyn BlobStore>>,
}

impl AppState {
    pub fn new(config: ServerConfig, cache: Arc<dyn CacheStore>) -> Self {
        let jwt = JwtManager::new(config.jwt.clone());
        let oauth = OAuthManager::new(config.oauth.clone(), cache.clone());
        let sessions = SessionManager::new(config.session.clone(), cache);
        #[cfg(feature = "standalone")]
        let event_bus = EventBus::with_config(EventBusConfig::server());

        Self {
            config: Arc::new(config),
            jwt: Arc::new(jwt),
            oauth: Arc::new(oauth),
            sessions: Arc::new(sessions),
            #[cfg(feature = "standalone")]
            event_bus: Arc::new(event_bus),
            #[cfg(feature = "standalone")]
            blob_store: None,
        }
    }

    /// Set the blob store backend.
    #[cfg(feature = "standalone")]
    pub fn with_blob_store(mut self, store: Arc<dyn BlobStore>) -> Self {
        self.blob_store = Some(store);
        self
    }

    /// Get a reference to the blob store, returning an error if not configured.
    #[cfg(feature = "standalone")]
    pub fn blob_store(&self) -> Result<&dyn BlobStore, crate::error::ServerError> {
        self.blob_store
            .as_deref()
            .ok_or_else(|| crate::error::ServerError::Internal("Blob store not configured".to_string()))
    }
}
