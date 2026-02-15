//! Complete example showing how to extend the App class for your own application.
//!
//! This demonstrates creating a custom application struct that wraps the Harana App
//! and adds application-specific functionality.

use async_trait::async_trait;
use harana_components_app::{App, AppBuilder, AppConfig, AppError, LifecycleHook};
use std::sync::Arc;

/// Your custom application struct
pub struct MyApplication {
    /// The underlying Harana app
    app: App,

    /// Your custom application state
    custom_state: Arc<MyAppState>,
}

/// Your custom application state
pub struct MyAppState {
    // Add your application-specific state here
    pub app_name: String,
    pub startup_time: std::time::Instant,
}

impl MyApplication {
    /// Create a new instance of your application
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        // Create your custom state
        let custom_state = Arc::new(MyAppState {
            app_name: config.name.clone(),
            startup_time: std::time::Instant::now(),
        });

        // You would create a real cache implementation here
        // For this example, we'll show the pattern:
        // let cache = Arc::new(YourCacheImplementation::new());

        // For demonstration purposes, we'll use a placeholder
        // In a real app, uncomment the line above
        return Err(AppError::ConfigError(
            "This is a demonstration example. Provide a real cache implementation.".to_string(),
        ));

        // Build the Harana app with hooks
        // let app = AppBuilder::new()
        //     .name(config.name.clone())
        //     .version(config.version.clone())
        //     .description(config.description.clone())
        //     .cache(cache)
        //     .hook(Box::new(MyStartupHook::new(custom_state.clone())))
        //     .build()
        //     .await?;
        //
        // Ok(Self { app, custom_state })
    }

    /// Run your application
    pub async fn run(self) -> Result<(), AppError> {
        tracing::info!(
            "Running {} (uptime: {:?})",
            self.custom_state.app_name,
            self.custom_state.startup_time.elapsed()
        );

        // Run the underlying Harana app
        self.app.run().await
    }

    /// Get the application configuration
    pub fn config(&self) -> &AppConfig {
        self.app.config()
    }

    /// Access your custom state
    pub fn state(&self) -> Arc<MyAppState> {
        self.custom_state.clone()
    }

    /// Add your custom methods here
    pub async fn do_something_custom(&self) -> Result<(), AppError> {
        tracing::info!("Executing custom application logic...");
        Ok(())
    }
}

/// Custom lifecycle hook for your application
struct MyStartupHook {
    state: Arc<MyAppState>,
}

impl MyStartupHook {
    fn new(state: Arc<MyAppState>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl LifecycleHook for MyStartupHook {
    async fn on_startup(&self, _config: &AppConfig) -> Result<(), AppError> {
        tracing::info!("🎯 Custom startup hook for {}", self.state.app_name);

        // Initialize your application-specific resources
        // For example:
        // - Connect to databases
        // - Load configuration
        // - Initialize caches
        // - Register services

        Ok(())
    }

    async fn on_shutdown(&self, _config: &AppConfig) -> Result<(), AppError> {
        tracing::info!("👋 Custom shutdown hook for {}", self.state.app_name);

        // Clean up your application-specific resources

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    App::init_tracing();

    // Create configuration
    let config = AppConfig::builder()
        .name("my-custom-app")
        .version("1.0.0")
        .description("My custom application extending Harana framework")
        .environment("development")
        .custom("database_url", "postgresql://localhost/mydb")
        .custom("max_connections", 100)
        .build()?;

    tracing::info!("Creating custom application...");

    // This will error because it's a demonstration
    // In a real app, you would provide the cache implementation
    match MyApplication::new(config).await {
        Ok(app) => {
            // Do some custom initialization
            app.do_something_custom().await?;

            // Run the application
            app.run().await?;
        }
        Err(e) => {
            tracing::info!("ℹ️  {}", e);
            tracing::info!("This is a demonstration showing how to extend the App class.");
            tracing::info!("In a real application, provide a cache implementation.");
        }
    }

    Ok(())
}
