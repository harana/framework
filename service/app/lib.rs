//! # Harana Application Framework
//!
//! This module provides the base `App` class for building applications with the Harana framework.
//! It offers a structured way to configure and run applications with support for HTTP servers,
//! workflows, events, and scheduled tasks.
//!
//! ## Example
//!
//! ```rust
//! use harana_components_app::{App, AppConfig};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = AppConfig::builder()
//!         .name("my-app")
//!         .version("1.0.0")
//!         .build()?;
//!
//!     let app = App::new(config).await?;
//!     app.run().await?;
//!
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod error;
pub mod lifecycle;

pub use config::{AppConfig, AppConfigBuilder};
pub use error::AppError;
pub use lifecycle::{LifecycleHook, LifecyclePhase};

use async_trait::async_trait;
use harana_components_cache::CacheService;
use std::sync::Arc;
use tracing::{info, warn};

#[cfg(feature = "http")]
use harana_components_http_server::{HttpServer, ServerConfig};

#[cfg(feature = "workflow")]
use harana_actions_workflow as workflow;

#[cfg(feature = "events")]
use harana_components_events as events;

#[cfg(feature = "scheduler")]
use harana_components_schedule as schedule;

/// The main application class that serves as the entry point for Harana framework applications.
///
/// The `App` struct provides a unified interface for managing application lifecycle,
/// including initialization, configuration, and execution of various services.
pub struct App {
    /// Application configuration
    config: AppConfig,

    /// Cache service instance
    cache: Arc<dyn CacheService>,

    /// Optional HTTP service instance
    #[cfg(feature = "http")]
    http_service: Option<HttpServer>,

    /// Lifecycle hooks
    hooks: Vec<Box<dyn LifecycleHook>>,
}

impl App {
    /// Create a new application instance with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Application configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use harana_components_app::{App, AppConfig};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = AppConfig::builder()
    ///     .name("my-app")
    ///     .version("1.0.0")
    ///     .build()?;
    ///
    /// let app = App::new(config).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        Self::with_cache(config, Self::create_default_cache()?).await
    }

    /// Create a new application instance with a custom cache service.
    ///
    /// # Arguments
    ///
    /// * `config` - Application configuration
    /// * `cache` - Custom cache service implementation
    pub async fn with_cache(config: AppConfig, cache: Arc<dyn CacheService>) -> Result<Self, AppError> {
        info!(
            "Initializing {} v{} ({})",
            config.name, config.version, config.description
        );

        #[cfg(feature = "http")]
        let http_service = if let Some(http_config) = &config.http {
            Some(HttpServer::new(http_config.clone(), cache.clone()))
        } else {
            None
        };

        Ok(Self {
            config,
            cache,
            #[cfg(feature = "http")]
            http_service,
            hooks: Vec::new(),
        })
    }

    /// Add a lifecycle hook to the application.
    ///
    /// Lifecycle hooks allow you to execute custom code at specific points
    /// in the application lifecycle (startup, shutdown, etc.).
    pub fn add_hook(&mut self, hook: Box<dyn LifecycleHook>) {
        self.hooks.push(hook);
    }

    /// Initialize the application.
    ///
    /// This method runs all startup hooks and prepares the application for execution.
    pub async fn init(&mut self) -> Result<(), AppError> {
        info!("Initializing application...");

        // Run startup hooks
        for hook in &self.hooks {
            hook.on_startup(&self.config).await?;
        }

        info!("Application initialized successfully");
        Ok(())
    }

    /// Run the application.
    ///
    /// This is the main entry point that starts all configured services
    /// (HTTP server, workflow engine, etc.) and runs until shutdown.
    pub async fn run(mut self) -> Result<(), AppError> {
        self.init().await?;

        info!("Starting application...");

        // Run the HTTP service if configured
        #[cfg(feature = "http")]
        if let Some(http_service) = self.http_service {
            info!("Starting HTTP service on {}", http_service.address());
            http_service.run().await.map_err(AppError::HttpError)?;
        } else {
            warn!("No HTTP service configured. Application will run without HTTP.");
            // Keep the application running
            tokio::signal::ctrl_c()
                .await
                .map_err(|e| AppError::RuntimeError(format!("Failed to listen for shutdown signal: {}", e)))?;
        }

        #[cfg(not(feature = "http"))]
        {
            warn!("HTTP feature not enabled. Application will run until shutdown signal.");
            tokio::signal::ctrl_c()
                .await
                .map_err(|e| AppError::RuntimeError(format!("Failed to listen for shutdown signal: {}", e)))?;
        }

        self.shutdown().await?;

        Ok(())
    }

    /// Gracefully shutdown the application.
    ///
    /// This method runs all shutdown hooks and cleans up resources.
    pub async fn shutdown(&self) -> Result<(), AppError> {
        info!("Shutting down application...");

        // Run shutdown hooks
        for hook in &self.hooks {
            hook.on_shutdown(&self.config).await?;
        }

        info!("Application shut down successfully");
        Ok(())
    }

    /// Get a reference to the application configuration.
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// Get a reference to the cache service.
    pub fn cache(&self) -> Arc<dyn CacheService> {
        self.cache.clone()
    }

    /// Initialize tracing/logging for the application.
    ///
    /// This should be called before creating the App instance if you want
    /// to see logs during initialization.
    pub fn init_tracing() {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,harana=debug".into()),
            )
            .init();
    }

    /// Create a default in-memory cache service.
    ///
    /// This is used when no custom cache is provided.
    fn create_default_cache() -> Result<Arc<dyn CacheService>, AppError> {
        // This will need to be implemented based on the actual cache service
        // For now, we'll return an error suggesting to use with_cache
        Err(AppError::ConfigError(
            "No cache service provided. Use App::with_cache() to provide a cache implementation".to_string(),
        ))
    }
}

/// Builder pattern support for App construction
pub struct AppBuilder {
    config_builder: AppConfigBuilder,
    cache: Option<Arc<dyn CacheService>>,
    hooks: Vec<Box<dyn LifecycleHook>>,
}

impl AppBuilder {
    /// Create a new AppBuilder
    pub fn new() -> Self {
        Self {
            config_builder: AppConfig::builder(),
            cache: None,
            hooks: Vec::new(),
        }
    }

    /// Set the application name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.config_builder = self.config_builder.name(name);
        self
    }

    /// Set the application version
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.config_builder = self.config_builder.version(version);
        self
    }

    /// Set the application description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.config_builder = self.config_builder.description(description);
        self
    }

    /// Set a custom cache service
    pub fn cache(mut self, cache: Arc<dyn CacheService>) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Add a lifecycle hook
    pub fn hook(mut self, hook: Box<dyn LifecycleHook>) -> Self {
        self.hooks.push(hook);
        self
    }

    /// Build the App instance
    pub async fn build(self) -> Result<App, AppError> {
        let config = self.config_builder.build()?;
        let cache = self
            .cache
            .ok_or_else(|| AppError::ConfigError("Cache service is required".to_string()))?;

        let mut app = App::with_cache(config, cache).await?;

        for hook in self.hooks {
            app.add_hook(hook);
        }

        Ok(app)
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_builder() {
        let builder = AppBuilder::new()
            .name("test-app")
            .version("1.0.0")
            .description("A test application");

        // We can't fully build without a cache, but we can test the builder
        assert!(builder.cache.is_none());
    }
}
