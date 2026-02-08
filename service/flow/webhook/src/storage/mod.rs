// Harana Actions - Webhook Storage Backends
// This module provides storage backend implementations for webhooks.

mod inmemory;
mod mongodb_storage;
mod redis_storage;

pub use inmemory::InMemoryStorage;
pub use mongodb_storage::MongoDbStorage;
pub use redis_storage::RedisStorage;

use crate::{Webhook, WebhookDelivery};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum StorageType {
    InMemory,
    Redis { url: String },
    MongoDB { url: String, database: String },
}

impl Default for StorageType {
    fn default() -> Self {
        StorageType::InMemory
    }
}

/// Trait defining the storage backend interface
#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// Save or update a webhook
    async fn save_webhook(&self, webhook: &Webhook) -> Result<(), String>;

    /// Get a webhook by ID
    async fn get_webhook(&self, webhook_id: &str) -> Result<Option<Webhook>, String>;

    /// Delete a webhook by ID
    async fn delete_webhook(&self, webhook_id: &str) -> Result<(), String>;

    /// List webhooks with optional filtering
    async fn list_webhooks(
        &self,
        event: Option<&str>,
        active_only: Option<bool>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Webhook>, String>;

    /// Save a delivery record
    async fn save_delivery(&self, delivery: &WebhookDelivery) -> Result<(), String>;

    /// Get a delivery by ID
    async fn get_delivery(&self, delivery_id: &str) -> Result<Option<WebhookDelivery>, String>;

    /// List deliveries for a webhook with optional status filter
    async fn list_deliveries(
        &self,
        webhook_id: &str,
        status: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<WebhookDelivery>, String>;
}

/// Create a storage backend based on the storage type
pub async fn create_storage(storage_type: StorageType) -> Result<Arc<dyn StorageBackend>, String> {
    match storage_type {
        StorageType::InMemory => {
            Ok(Arc::new(InMemoryStorage::new()))
        }
        StorageType::Redis { url } => {
            let storage = RedisStorage::new(&url).await?;
            Ok(Arc::new(storage))
        }
        StorageType::MongoDB { url, database } => {
            let storage = MongoDbStorage::new(&url, &database).await?;
            Ok(Arc::new(storage))
        }
    }
}
