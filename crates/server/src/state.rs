//! Application state management

use crate::auth::{JwtManager, OAuthManager, SessionManager};
use crate::config::ServerConfig;
use harana_components_events::{EventBus, EventBusConfig};
use std::sync::Arc;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ServerConfig>,
    pub jwt: Arc<JwtManager>,
    pub oauth: Arc<OAuthManager>,
    pub sessions: Arc<SessionManager>,
    pub event_bus: Arc<EventBus>,
}

impl AppState {
    pub fn new(config: ServerConfig) -> Self {
        let jwt = JwtManager::new(config.jwt.clone());
        let oauth = OAuthManager::new(config.oauth.clone());
        let sessions = SessionManager::new(config.session.clone());
        let event_bus = EventBus::with_config(EventBusConfig::server());

        Self {
            config: Arc::new(config),
            jwt: Arc::new(jwt),
            oauth: Arc::new(oauth),
            sessions: Arc::new(sessions),
            event_bus: Arc::new(event_bus),
        }
    }
}
