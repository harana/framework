use crate::auth::{JwtManager, OAuthManager, SessionManager};
use crate::config::ServerConfig;
#[cfg(feature = "standalone")]
use harana_components_blob::BlobService;
use harana_components_cache::CacheService;
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
    blob_service: Option<Arc<dyn BlobService>>,
}

impl AppState {
    pub fn new(config: ServerConfig, cache: Arc<dyn CacheService>) -> Self {
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
            blob_service: None,
        }
    }

    /// Set the blob service backend.
    #[cfg(feature = "standalone")]
    pub fn with_blob_service(mut self, service: Arc<dyn BlobService>) -> Self {
        self.blob_service = Some(service);
        self
    }

    /// Get a reference to the blob service, returning an error if not configured.
    #[cfg(feature = "standalone")]
    pub fn blob_service(&self) -> Result<&dyn BlobService, crate::error::ServerError> {
        self.blob_service
            .as_deref()
            .ok_or_else(|| crate::error::ServerError::Internal("Blob service not configured".to_string()))
    }
}
