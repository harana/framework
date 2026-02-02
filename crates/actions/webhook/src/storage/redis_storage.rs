// Harana Actions - Webhook Redis Storage Backend
// Uses Redis for persistent distributed storage.

use crate::{Webhook, WebhookDelivery};
use super::StorageBackend;
use async_trait::async_trait;
use redis::{aio::ConnectionManager, AsyncCommands, Client};

const WEBHOOK_PREFIX: &str = "webhook:";
const WEBHOOK_INDEX: &str = "webhooks:index";
const DELIVERY_PREFIX: &str = "delivery:";
const WEBHOOK_DELIVERIES_PREFIX: &str = "webhook:deliveries:";

/// Redis storage backend
pub struct RedisStorage {
    connection: ConnectionManager,
}

impl RedisStorage {
    /// Create a new Redis storage instance
    pub async fn new(url: &str) -> Result<Self, String> {
        let client = Client::open(url)
            .map_err(|e| format!("Failed to create Redis client: {}", e))?;
        
        let connection = ConnectionManager::new(client)
            .await
            .map_err(|e| format!("Failed to connect to Redis: {}", e))?;

        Ok(Self { connection })
    }

    fn webhook_key(webhook_id: &str) -> String {
        format!("{}{}", WEBHOOK_PREFIX, webhook_id)
    }

    fn delivery_key(delivery_id: &str) -> String {
        format!("{}{}", DELIVERY_PREFIX, delivery_id)
    }

    fn webhook_deliveries_key(webhook_id: &str) -> String {
        format!("{}{}", WEBHOOK_DELIVERIES_PREFIX, webhook_id)
    }
}

#[async_trait]
impl StorageBackend for RedisStorage {
    async fn save_webhook(&self, webhook: &Webhook) -> Result<(), String> {
        let mut conn = self.connection.clone();
        let key = Self::webhook_key(&webhook.webhook_id);
        
        let json = serde_json::to_string(webhook)
            .map_err(|e| format!("Failed to serialize webhook: {}", e))?;

        // Save the webhook
        conn.set::<_, _, ()>(&key, &json)
            .await
            .map_err(|e| format!("Failed to save webhook: {}", e))?;

        // Add to index
        conn.sadd::<_, _, ()>(WEBHOOK_INDEX, &webhook.webhook_id)
            .await
            .map_err(|e| format!("Failed to add webhook to index: {}", e))?;

        Ok(())
    }

    async fn get_webhook(&self, webhook_id: &str) -> Result<Option<Webhook>, String> {
        let mut conn = self.connection.clone();
        let key = Self::webhook_key(webhook_id);

        let result: Option<String> = conn.get(&key)
            .await
            .map_err(|e| format!("Failed to get webhook: {}", e))?;

        match result {
            Some(json) => {
                let webhook: Webhook = serde_json::from_str(&json)
                    .map_err(|e| format!("Failed to deserialize webhook: {}", e))?;
                Ok(Some(webhook))
            }
            None => Ok(None),
        }
    }

    async fn delete_webhook(&self, webhook_id: &str) -> Result<(), String> {
        let mut conn = self.connection.clone();
        let key = Self::webhook_key(webhook_id);

        // Delete the webhook
        conn.del::<_, ()>(&key)
            .await
            .map_err(|e| format!("Failed to delete webhook: {}", e))?;

        // Remove from index
        conn.srem::<_, _, ()>(WEBHOOK_INDEX, webhook_id)
            .await
            .map_err(|e| format!("Failed to remove webhook from index: {}", e))?;

        // Delete associated deliveries
        let deliveries_key = Self::webhook_deliveries_key(webhook_id);
        let delivery_ids: Vec<String> = conn.smembers(&deliveries_key)
            .await
            .unwrap_or_default();

        for delivery_id in delivery_ids {
            let delivery_key = Self::delivery_key(&delivery_id);
            let _ = conn.del::<_, ()>(&delivery_key).await;
        }
        let _ = conn.del::<_, ()>(&deliveries_key).await;

        Ok(())
    }

    async fn list_webhooks(
        &self,
        event: Option<&str>,
        active_only: Option<bool>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Webhook>, String> {
        let mut conn = self.connection.clone();
        let limit = limit.unwrap_or(100) as usize;
        let offset = offset.unwrap_or(0) as usize;
        let active_only = active_only.unwrap_or(false);

        // Get all webhook IDs from index
        let webhook_ids: Vec<String> = conn.smembers(WEBHOOK_INDEX)
            .await
            .map_err(|e| format!("Failed to get webhook index: {}", e))?;

        let mut webhooks = Vec::new();
        for webhook_id in webhook_ids {
            if let Some(webhook) = self.get_webhook(&webhook_id).await? {
                // Apply filters
                if active_only && !webhook.active {
                    continue;
                }
                if let Some(evt) = event {
                    if !webhook.events.contains(&evt.to_string()) {
                        continue;
                    }
                }
                webhooks.push(webhook);
            }
        }

        // Apply pagination
        let webhooks: Vec<Webhook> = webhooks
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();

        Ok(webhooks)
    }

    async fn save_delivery(&self, delivery: &WebhookDelivery) -> Result<(), String> {
        let mut conn = self.connection.clone();
        let key = Self::delivery_key(&delivery.delivery_id);

        let json = serde_json::to_string(delivery)
            .map_err(|e| format!("Failed to serialize delivery: {}", e))?;

        // Save the delivery
        conn.set::<_, _, ()>(&key, &json)
            .await
            .map_err(|e| format!("Failed to save delivery: {}", e))?;

        // Add to webhook's delivery index
        let webhook_deliveries_key = Self::webhook_deliveries_key(&delivery.webhook_id);
        conn.sadd::<_, _, ()>(&webhook_deliveries_key, &delivery.delivery_id)
            .await
            .map_err(|e| format!("Failed to add delivery to webhook index: {}", e))?;

        Ok(())
    }

    async fn get_delivery(&self, delivery_id: &str) -> Result<Option<WebhookDelivery>, String> {
        let mut conn = self.connection.clone();
        let key = Self::delivery_key(delivery_id);

        let result: Option<String> = conn.get(&key)
            .await
            .map_err(|e| format!("Failed to get delivery: {}", e))?;

        match result {
            Some(json) => {
                let delivery: WebhookDelivery = serde_json::from_str(&json)
                    .map_err(|e| format!("Failed to deserialize delivery: {}", e))?;
                Ok(Some(delivery))
            }
            None => Ok(None),
        }
    }

    async fn list_deliveries(
        &self,
        webhook_id: &str,
        status: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<WebhookDelivery>, String> {
        let mut conn = self.connection.clone();
        let limit = limit.unwrap_or(50) as usize;
        let offset = offset.unwrap_or(0) as usize;

        // Get delivery IDs for this webhook
        let webhook_deliveries_key = Self::webhook_deliveries_key(webhook_id);
        let delivery_ids: Vec<String> = conn.smembers(&webhook_deliveries_key)
            .await
            .map_err(|e| format!("Failed to get webhook deliveries: {}", e))?;

        let mut deliveries = Vec::new();
        for delivery_id in delivery_ids {
            if let Some(delivery) = self.get_delivery(&delivery_id).await? {
                // Apply status filter
                if let Some(status) = status {
                    if !delivery.status.eq_ignore_ascii_case(status) {
                        continue;
                    }
                }
                deliveries.push(delivery);
            }
        }

        // Sort by created_at descending
        deliveries.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        // Apply pagination
        let deliveries: Vec<WebhookDelivery> = deliveries
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();

        Ok(deliveries)
    }
}
