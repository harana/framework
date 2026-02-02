// Harana Actions - Webhook In-Memory Storage Backend
// Uses DashMap for concurrent in-memory storage.

use crate::{Webhook, WebhookDelivery};
use super::StorageBackend;
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;

/// In-memory storage backend using DashMap
pub struct InMemoryStorage {
    webhooks: Arc<DashMap<String, Webhook>>,
    deliveries: Arc<DashMap<String, WebhookDelivery>>,
}

impl InMemoryStorage {
    /// Create a new in-memory storage instance
    pub fn new() -> Self {
        Self {
            webhooks: Arc::new(DashMap::new()),
            deliveries: Arc::new(DashMap::new()),
        }
    }
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StorageBackend for InMemoryStorage {
    async fn save_webhook(&self, webhook: &Webhook) -> Result<(), String> {
        self.webhooks.insert(webhook.webhook_id.clone(), webhook.clone());
        Ok(())
    }

    async fn get_webhook(&self, webhook_id: &str) -> Result<Option<Webhook>, String> {
        Ok(self.webhooks.get(webhook_id).map(|w| w.clone()))
    }

    async fn delete_webhook(&self, webhook_id: &str) -> Result<(), String> {
        self.webhooks.remove(webhook_id);
        // Also remove associated deliveries
        self.deliveries.retain(|_, d| d.webhook_id != webhook_id);
        Ok(())
    }

    async fn list_webhooks(
        &self,
        event: Option<&str>,
        active_only: Option<bool>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Webhook>, String> {
        let limit = limit.unwrap_or(100) as usize;
        let offset = offset.unwrap_or(0) as usize;
        let active_only = active_only.unwrap_or(false);

        let webhooks: Vec<Webhook> = self
            .webhooks
            .iter()
            .filter(|w| {
                if active_only && !w.active {
                    return false;
                }
                if let Some(evt) = event {
                    return w.events.contains(&evt.to_string());
                }
                true
            })
            .map(|w| w.clone())
            .skip(offset)
            .take(limit)
            .collect();

        Ok(webhooks)
    }

    async fn save_delivery(&self, delivery: &WebhookDelivery) -> Result<(), String> {
        self.deliveries.insert(delivery.delivery_id.clone(), delivery.clone());
        Ok(())
    }

    async fn get_delivery(&self, delivery_id: &str) -> Result<Option<WebhookDelivery>, String> {
        Ok(self.deliveries.get(delivery_id).map(|d| d.clone()))
    }

    async fn list_deliveries(
        &self,
        webhook_id: &str,
        status: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<WebhookDelivery>, String> {
        let limit = limit.unwrap_or(50) as usize;
        let offset = offset.unwrap_or(0) as usize;

        let deliveries: Vec<WebhookDelivery> = self
            .deliveries
            .iter()
            .filter(|d| {
                if d.webhook_id != webhook_id {
                    return false;
                }
                if let Some(status) = status {
                    return d.status.eq_ignore_ascii_case(status);
                }
                true
            })
            .map(|d| d.clone())
            .skip(offset)
            .take(limit)
            .collect();

        Ok(deliveries)
    }
}
