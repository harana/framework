# Harana App Component

The base application class for building applications with the Harana framework.

## Overview

The `harana-components-app` crate provides a structured foundation for building Harana framework applications. It offers a unified `App` class that handles application lifecycle, configuration, and integration with various Harana services.

## Features

- **Lifecycle Management**: Structured startup and shutdown with hooks
- **Configuration**: Flexible configuration via builders, YAML, or JSON
- **HTTP Server**: Optional HTTP server integration (via `server` feature)
- **Workflow Engine**: Optional workflow support (via `workflow` feature)
- **Event System**: Optional event handling (via `events` feature)
- **Task Scheduling**: Optional scheduled tasks (via `scheduler` feature)
- **Extensibility**: Custom lifecycle hooks for application-specific logic

## Usage

### Basic Example

```rust
use harana_components_app::{App, AppConfig};
use harana_components_cache::InMemoryCache;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    App::init_tracing();

    // Configure the application
    let config = AppConfig::builder()
        .name("my-app")
        .version("1.0.0")
        .description("My Harana Application")
        .environment("production")
        .build()?;

    // Create cache service
    let cache = Arc::new(InMemoryCache::new());

    // Create and run the application
    let app = App::with_cache(config, cache).await?;
    app.run().await?;

    Ok(())
}
```

### Using the Builder Pattern

```rust
use harana_components_app::AppBuilder;
use harana_components_cache::InMemoryCache;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    App::init_tracing();

    let cache = Arc::new(InMemoryCache::new());

    let app = AppBuilder::new()
        .name("my-app")
        .version("1.0.0")
        .description("Built with AppBuilder")
        .cache(cache)
        .build()
        .await?;

    app.run().await?;
    Ok(())
}
```

### Loading Configuration from YAML

```rust
use harana_components_app::{App, AppConfig};
use harana_components_cache::InMemoryCache;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    App::init_tracing();

    // Load configuration from FML file
    let config = AppConfig::from_yaml_file("app.fml")?;
    let cache = Arc::new(InMemoryCache::new());

    let app = App::with_cache(config, cache).await?;
    app.run().await?;

    Ok(())
}
```

### Adding Lifecycle Hooks

```rust
use harana_components_app::{AppBuilder, LifecycleHook, AppConfig, AppError};
use async_trait::async_trait;
use std::sync::Arc;

struct DatabaseHook;

#[async_trait]
impl LifecycleHook for DatabaseHook {
    async fn on_startup(&self, config: &AppConfig) -> Result<(), AppError> {
        println!("Connecting to database for {}...", config.name);
        // Initialize database connection
        Ok(())
    }

    async fn on_shutdown(&self, config: &AppConfig) -> Result<(), AppError> {
        println!("Closing database connection for {}...", config.name);
        // Close database connection
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cache = Arc::new(InMemoryCache::new());

    let app = AppBuilder::new()
        .name("my-app")
        .version("1.0.0")
        .cache(cache)
        .hook(Box::new(DatabaseHook))
        .build()
        .await?;

    app.run().await?;
    Ok(())
}
```

## Configuration

### AppConfig Structure

```rust
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub description: String,
    pub environment: String,
    pub server: Option<ServerConfig>, // If server feature is enabled
    pub custom: serde_json::Value,    // Custom configuration
}
```

### YAML Configuration Example

```yaml
name: my-application
version: 1.0.0
description: My awesome Harana application
environment: production

server:
  host: 0.0.0.0
  port: 8080

# Custom configuration
database:
  host: localhost
  port: 5432
  name: mydb

logging:
  level: info
```

## Features

Enable features in your `Cargo.toml`:

```toml
[dependencies]
harana-components-app = { path = "../framework/service/app", features = ["full"] }

# Or select individual features:
# harana-components-app = { path = "../framework/service/app", features = ["server", "workflow"] }
```

Available features:
- `server` - HTTP server support (default)
- `workflow` - Workflow engine integration
- `events` - Event system support
- `scheduler` - Task scheduling support
- `full` - All features enabled

## Lifecycle Hooks

Implement the `LifecycleHook` trait to execute custom code during application lifecycle:

```rust
#[async_trait]
pub trait LifecycleHook: Send + Sync {
    async fn on_pre_init(&self, config: &AppConfig) -> Result<(), AppError>;
    async fn on_post_init(&self, config: &AppConfig) -> Result<(), AppError>;
    async fn on_startup(&self, config: &AppConfig) -> Result<(), AppError>;
    async fn on_pre_shutdown(&self, config: &AppConfig) -> Result<(), AppError>;
    async fn on_shutdown(&self, config: &AppConfig) -> Result<(), AppError>;
}
```

All methods have default implementations that do nothing, so you only need to implement the ones you need.

## Error Handling

The `AppError` enum provides error types for various scenarios:

```rust
pub enum AppError {
    ConfigError(String),
    ServerError(Box<dyn Error + Send + Sync>),
    WorkflowError(String),
    EventError(String),
    ScheduleError(String),
    RuntimeError(String),
    LifecycleError(String),
    Other(Box<dyn Error + Send + Sync>),
}
```

## License

MIT OR Apache-2.0
