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
use harana_components_cache::CacheStore;
use std::sync::Arc;

#[cfg(feature = "standalone")]
use std::net::SocketAddr;
#[cfg(feature = "standalone")]
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
#[cfg(feature = "standalone")]
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Build the shared axum Router with all routes configured.
/// This router is used by both standalone and Cloudflare Workers deployments.
pub fn build_router(state: AppState) -> Router {
    let router = Router::new()
        .merge(routes::general_routes())
        .merge(routes::auth_routes())
        .merge(routes::event_routes())
        .merge(routes::blob_routes())
        .layer(axum::middleware::from_fn(middleware::auth_middleware))
        .with_state(state);

    #[cfg(feature = "standalone")]
    let router = router
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));

    router
}

// =============================================================================
// Standalone server (native binary with tokio)
// =============================================================================

#[cfg(feature = "standalone")]
pub struct HttpServer {
    config: ServerConfig,
    state: AppState,
}

#[cfg(feature = "standalone")]
impl HttpServer {
    pub fn new(config: ServerConfig, cache: Arc<dyn CacheStore>) -> Self {
        let state = AppState::new(config.clone(), cache);
        Self { config, state }
    }

    pub fn with_state(config: ServerConfig, state: AppState) -> Self {
        Self { config, state }
    }
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
        build_router(self.state.clone())
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

// =============================================================================
// Cloudflare Workers entry point
// =============================================================================

#[cfg(feature = "cloudflare")]
mod cf {
    use super::*;
    use tower_service::Service;
    use worker::*;

    #[event(fetch)]
    async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<axum::http::Response<axum::body::Body>> {
        let config = ServerConfig::default();

        // Use Cloudflare KV binding "SESSIONS" as the cache backend.
        let cache: Arc<dyn CacheStore> = Arc::new(
            harana_components_cache::KvCacheStore::from_env(&env, "SESSIONS")
                .map_err(|e| worker::Error::RustError(format!("KV init: {e}")))?,
        );

        let state = AppState::new(config, cache);
        let mut router = build_router(state);
        Ok(router.call(req).await?)
    }
}

#[cfg(test)]
mod tests;
