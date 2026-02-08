//! Application state management

use crate::auth::{JwtManager, OAuthManager, SessionManager};
use crate::config::ServerConfig;
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
        }
    }
}
