//! Example demonstrating the basic usage of the Harana App component.
//!
//! This example shows how to:
//! - Create a basic application with configuration
//! - Add lifecycle hooks
//! - Run the application
//!
//! Run this example with:
//! ```bash
//! cargo run --example basic
//! ```

use async_trait::async_trait;
use harana_components_app::{App, AppBuilder, AppConfig, AppError, LifecycleHook};
use std::sync::Arc;

// Example lifecycle hook for database initialization
struct DatabaseHook;

#[async_trait]
impl LifecycleHook for DatabaseHook {
    async fn on_startup(&self, config: &AppConfig) -> Result<(), AppError> {
        tracing::info!("🗄️  Initializing database for {}...", config.name);
        // Here you would initialize your actual database connection
        tracing::info!("✅ Database initialized");
        Ok(())
    }

    async fn on_shutdown(&self, _config: &AppConfig) -> Result<(), AppError> {
        tracing::info!("🗄️  Closing database connections...");
        // Here you would close your database connections
        tracing::info!("✅ Database connections closed");
        Ok(())
    }
}

// Example lifecycle hook for cache initialization
struct CacheHook;

#[async_trait]
impl LifecycleHook for CacheHook {
    async fn on_startup(&self, config: &AppConfig) -> Result<(), AppError> {
        tracing::info!("💾 Warming up cache for {}...", config.name);
        // Here you would warm up your cache
        tracing::info!("✅ Cache ready");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing/logging
    App::init_tracing();

    tracing::info!("Starting Harana application example...");

    // Option 1: Using AppConfig builder directly
    // let config = AppConfig::builder()
    //     .name("example-app")
    //     .version("1.0.0")
    //     .description("An example Harana application")
    //     .environment("development")
    //     .custom("max_connections", 100)
    //     .custom("timeout_seconds", 30)
    //     .build()?;
    //
    // // You would need to provide a real cache implementation
    // // let cache = Arc::new(YourCacheImpl::new());
    // // let app = App::with_cache(config, cache).await?;

    // Option 2: Using AppBuilder (recommended)
    // Note: This example won't actually run without a real cache implementation
    // You would need to uncomment and use a real cache service:

    // use harana_components_cache::YourCacheImpl;
    // let cache = Arc::new(YourCacheImpl::new());
    //
    // let app = AppBuilder::new()
    //     .name("example-app")
    //     .version("1.0.0")
    //     .description("An example Harana application")
    //     .cache(cache)
    //     .hook(Box::new(DatabaseHook))
    //     .hook(Box::new(CacheHook))
    //     .build()
    //     .await?;
    //
    // app.run().await?;

    tracing::info!("📝 This is a demonstration example.");
    tracing::info!("To run a real application, you need to provide a cache implementation.");
    tracing::info!("See the README.md for complete usage examples.");

    Ok(())
}
