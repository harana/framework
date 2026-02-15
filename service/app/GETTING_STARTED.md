# Getting Started with Harana App

This guide will help you create your first application using the Harana framework.

## Quick Start

### 1. Create Your Project

Create a new Rust project:

```bash
cargo new my-harana-app
cd my-harana-app
```

### 2. Add Dependencies

Add Harana framework to your `Cargo.toml`:

```toml
[dependencies]
harana-components-app = { path = "../framework/service/app" }
harana-components-cache = { path = "../framework/service/cache" }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

### 3. Create Your Application

Edit `src/main.rs`:

```rust
use harana_components_app::{App, AppConfig};
use harana_components_cache::InMemoryCache; // or your cache implementation
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    App::init_tracing();

    // Configure your application
    let config = AppConfig::builder()
        .name("my-harana-app")
        .version(env!("CARGO_PKG_VERSION"))
        .description("My awesome Harana application")
        .environment("development")
        .build()?;

    // Create cache service (use your preferred implementation)
    let cache = Arc::new(InMemoryCache::new());

    // Create and run the application
    let app = App::with_cache(config, cache).await?;
    app.run().await?;

    Ok(())
}
```

### 4. Run Your Application

```bash
cargo run
```

## Extending the App Class

For more complex applications, you can create a custom struct that wraps the Harana App:

```rust
use harana_components_app::{App, AppConfig, AppError};
use std::sync::Arc;

pub struct MyApp {
    app: App,
    // Add your custom state here
    database: Arc<Database>,
    external_api: Arc<ExternalApiClient>,
}

impl MyApp {
    pub async fn new(config: AppConfig, cache: Arc<dyn CacheService>) -> Result<Self, AppError> {
        // Initialize your custom services
        let database = Arc::new(Database::connect(&config).await?);
        let external_api = Arc::new(ExternalApiClient::new());

        // Create the Harana app
        let app = App::with_cache(config, cache).await?;

        Ok(Self {
            app,
            database,
            external_api,
        })
    }

    pub async fn run(self) -> Result<(), AppError> {
        // Run any pre-startup tasks
        self.initialize_database().await?;

        // Run the Harana app
        self.app.run().await
    }

    async fn initialize_database(&self) -> Result<(), AppError> {
        // Your database initialization logic
        Ok(())
    }

    // Add your custom methods
    pub async fn process_data(&self, data: Data) -> Result<(), AppError> {
        // Your business logic
        Ok(())
    }
}
```

## Using Configuration Files

Create an `app.yml` file:

```yaml
name: my-harana-app
version: 1.0.0
description: My Harana Application
environment: production

server:
  host: 0.0.0.0
  port: 8080

# Custom configuration
database:
  url: postgresql://localhost/mydb
  max_connections: 100

api:
  timeout_seconds: 30
  retry_count: 3
```

Load it in your application:

```rust
let config = AppConfig::from_yaml_file("app.yml")?;

// Access custom configuration
let db_url: String = config.get_custom("database.url")
    .ok_or_else(|| AppError::ConfigError("Database URL not found".into()))?;
```

## Adding Lifecycle Hooks

Implement custom initialization and cleanup logic:

```rust
use harana_components_app::{LifecycleHook, AppConfig, AppError};
use async_trait::async_trait;

struct DatabaseHook {
    db: Arc<Database>,
}

#[async_trait]
impl LifecycleHook for DatabaseHook {
    async fn on_startup(&self, config: &AppConfig) -> Result<(), AppError> {
        println!("Connecting to database...");
        self.db.connect().await?;
        println!("Database connected");
        Ok(())
    }

    async fn on_shutdown(&self, config: &AppConfig) -> Result<(), AppError> {
        println!("Closing database connection...");
        self.db.disconnect().await?;
        println!("Database disconnected");
        Ok(())
    }
}

// Add the hook to your app
let mut app = App::with_cache(config, cache).await?;
app.add_hook(Box::new(DatabaseHook { db: database.clone() }));
app.run().await?;
```

## Feature Flags

Enable optional features in your `Cargo.toml`:

```toml
[dependencies]
harana-components-app = { 
    path = "../framework/service/app",
    features = ["server", "workflow", "events"]
}
```

Available features:
- `server` - HTTP server support (enabled by default)
- `workflow` - Workflow engine integration
- `events` - Event system support
- `scheduler` - Task scheduling support
- `full` - All features enabled

## Environment Variables

The App class supports environment-based configuration:

```bash
# Set logging level
export RUST_LOG=info,my_app=debug

# Set environment
export APP_ENV=production

cargo run
```

## Next Steps

- Explore the [examples](./examples/) directory
- Read the [API documentation](./README.md)
- Check out the [Harana framework documentation](../../README.md)

## Common Patterns

### Graceful Shutdown

The app automatically handles Ctrl+C for graceful shutdown. For custom shutdown logic:

```rust
use tokio::signal;

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    println!("Shutting down gracefully...");
}
```

### Health Checks

Implement health check endpoints using lifecycle hooks:

```rust
struct HealthCheckHook;

#[async_trait]
impl LifecycleHook for HealthCheckHook {
    async fn on_startup(&self, config: &AppConfig) -> Result<(), AppError> {
        // Register health check endpoint
        Ok(())
    }
}
```

### Metrics Collection

Use lifecycle hooks to initialize metrics:

```rust
struct MetricsHook;

#[async_trait]
impl LifecycleHook for MetricsHook {
    async fn on_startup(&self, config: &AppConfig) -> Result<(), AppError> {
        // Initialize Prometheus or other metrics
        Ok(())
    }
}
```
