pub mod auth;
pub mod config;
pub mod error;
pub mod extractors;
pub mod handlers;
pub mod middleware;
pub mod routes;
pub mod state;

pub use config::ServerConfig;
pub use error::ServerError;
pub use state::AppState;

use axum::Router;
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// HTTP Server for handling web requests with full auth support
pub struct HttpServer {
    config: ServerConfig,
    state: AppState,
}

impl HttpServer {
    pub fn new(config: ServerConfig) -> Self {
        let state = AppState::new(config.clone());
        Self { config, state }
    }

    pub fn with_state(config: ServerConfig, state: AppState) -> Self {
        Self { config, state }
    }

    /// Initialize tracing/logging
    pub fn init_tracing() {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "harana=debug,tower_http=debug,axum=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    /// Build the router with all routes configured
    pub fn build_router(&self) -> Router {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        Router::new()
            .merge(routes::general_routes())
            .merge(routes::auth_routes())
            .merge(routes::event_routes())
            .layer(axum::middleware::from_fn(middleware::auth_middleware))
            .layer(CompressionLayer::new())
            .layer(TraceLayer::new_for_http())
            .layer(cors)
            .with_state(self.state.clone())
    }

    /// Start the HTTP server
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let router = self.build_router();
        let addr: SocketAddr = self.config.address().parse()?;
        let listener = tokio::net::TcpListener::bind(addr).await?;

        tracing::info!("🚀 Server listening on {}", listener.local_addr()?);

        axum::serve(listener, router).await?;

        Ok(())
    }

    /// Get the server's address
    pub fn address(&self) -> String {
        self.config.address()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let config = ServerConfig::default();
        let server = HttpServer::new(config);
        assert_eq!(server.address(), "127.0.0.1:3000");
    }
}
