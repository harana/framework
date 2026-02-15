//! Lifecycle hooks for application events.

use crate::{AppConfig, AppError};
use async_trait::async_trait;

/// Lifecycle phases in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecyclePhase {
    /// Before application initialization
    PreInit,
    
    /// After application initialization
    PostInit,
    
    /// Application startup
    Startup,
    
    /// Application running
    Running,
    
    /// Before application shutdown
    PreShutdown,
    
    /// Application shutdown
    Shutdown,
}

/// Trait for implementing lifecycle hooks.
///
/// Lifecycle hooks allow you to execute custom code at specific points
/// in the application lifecycle.
///
/// # Example
///
/// ```rust
/// use harana_components_app::{LifecycleHook, AppConfig, AppError};
/// use async_trait::async_trait;
///
/// struct MyHook;
///
/// #[async_trait]
/// impl LifecycleHook for MyHook {
///     async fn on_startup(&self, config: &AppConfig) -> Result<(), AppError> {
///         println!("Application {} is starting!", config.name);
///         Ok(())
///     }
///
///     async fn on_shutdown(&self, config: &AppConfig) -> Result<(), AppError> {
///         println!("Application {} is shutting down!", config.name);
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait LifecycleHook: Send + Sync {
    /// Called before the application initializes
    async fn on_pre_init(&self, _config: &AppConfig) -> Result<(), AppError> {
        Ok(())
    }

    /// Called after the application initializes
    async fn on_post_init(&self, _config: &AppConfig) -> Result<(), AppError> {
        Ok(())
    }

    /// Called when the application starts
    async fn on_startup(&self, _config: &AppConfig) -> Result<(), AppError> {
        Ok(())
    }

    /// Called before the application shuts down
    async fn on_pre_shutdown(&self, _config: &AppConfig) -> Result<(), AppError> {
        Ok(())
    }

    /// Called when the application shuts down
    async fn on_shutdown(&self, _config: &AppConfig) -> Result<(), AppError> {
        Ok(())
    }
}

/// A simple logging lifecycle hook for demonstration
pub struct LoggingHook;

#[async_trait]
impl LifecycleHook for LoggingHook {
    async fn on_startup(&self, config: &AppConfig) -> Result<(), AppError> {
        tracing::info!(
            "🚀 Starting {} v{} in {} mode",
            config.name,
            config.version,
            config.environment
        );
        Ok(())
    }

    async fn on_shutdown(&self, config: &AppConfig) -> Result<(), AppError> {
        tracing::info!("👋 Shutting down {}", config.name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHook {
        called: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
    }

    #[async_trait]
    impl LifecycleHook for TestHook {
        async fn on_startup(&self, _config: &AppConfig) -> Result<(), AppError> {
            self.called.lock().unwrap().push("startup".to_string());
            Ok(())
        }

        async fn on_shutdown(&self, _config: &AppConfig) -> Result<(), AppError> {
            self.called.lock().unwrap().push("shutdown".to_string());
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_lifecycle_hook() {
        let called = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let hook = TestHook {
            called: called.clone(),
        };

        let config = AppConfig::builder()
            .name("test")
            .version("1.0.0")
            .build()
            .unwrap();

        hook.on_startup(&config).await.unwrap();
        hook.on_shutdown(&config).await.unwrap();

        let calls = called.lock().unwrap();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0], "startup");
        assert_eq!(calls[1], "shutdown");
    }
}
