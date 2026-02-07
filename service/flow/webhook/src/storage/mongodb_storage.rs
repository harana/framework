// Harana Actions - Webhook MongoDB Storage Backend
// Uses MongoDB for document-based persistent storage.

use crate::{Webhook, WebhookDelivery};
use super::StorageBackend;
use async_trait::async_trait;
use mongodb::{
    bson::doc,
    options::{ClientOptions, FindOptions, ReplaceOptions},
    Client, Collection, Database,
};
use futures::TryStreamExt;

const WEBHOOKS_COLLECTION: &str = "webhooks";
const DELIVERIES_COLLECTION: &str = "webhook_deliveries";

/// MongoDB storage backend
pub struct MongoDbStorage {
    database: Database,
}

impl MongoDbStorage {
    /// Create a new MongoDB storage instance
    pub async fn new(url: &str, database_name: &str) -> Result<Self, String> {
        let client_options = ClientOptions::parse(url)
            .await
            .map_err(|e| format!("Failed to parse MongoDB URL: {}", e))?;

        let client = Client::with_options(client_options)
            .map_err(|e| format!("Failed to create MongoDB client: {}", e))?;

        let database = client.database(database_name);

        // Create indexes for better query performance
        let storage = Self { database };
        storage.ensure_indexes().await?;

        Ok(storage)
    }

    async fn ensure_indexes(&self) -> Result<(), String> {
        // Create indexes on webhooks collection
        let webhooks: Collection<Webhook> = self.database.collection(WEBHOOKS_COLLECTION);
        webhooks
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "webhook_id": 1 })
                    .options(mongodb::options::IndexOptions::builder().unique(true).build())
                    .build(),
            )
            .await
            .map_err(|e| format!("Failed to create webhook index: {}", e))?;

        // Create indexes on deliveries collection
        let deliveries: Collection<WebhookDelivery> = self.database.collection(DELIVERIES_COLLECTION);
        deliveries
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "delivery_id": 1 })
                    .options(mongodb::options::IndexOptions::builder().unique(true).build())
                    .build(),
            )
            .await
            .map_err(|e| format!("Failed to create delivery index: {}", e))?;

        deliveries
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "webhook_id": 1, "created_at": -1 })
                    .build(),
            )
            .await
            .map_err(|e| format!("Failed to create delivery webhook index: {}", e))?;

        Ok(())
    }

    fn webhooks_collection(&self) -> Collection<Webhook> {
        self.database.collection(WEBHOOKS_COLLECTION)
    }

    fn deliveries_collection(&self) -> Collection<WebhookDelivery> {
        self.database.collection(DELIVERIES_COLLECTION)
    }
}

#[async_trait]
impl StorageBackend for MongoDbStorage {
    async fn save_webhook(&self, webhook: &Webhook) -> Result<(), String> {
        let collection = self.webhooks_collection();

        // Upsert the webhook
        let filter = doc! { "webhook_id": &webhook.webhook_id };
        let options = ReplaceOptions::builder().upsert(true).build();

        collection
            .replace_one(filter, webhook)
            .with_options(options)
            .await
            .map_err(|e| format!("Failed to save webhook: {}", e))?;

        Ok(())
    }

    async fn get_webhook(&self, webhook_id: &str) -> Result<Option<Webhook>, String> {
        let collection = self.webhooks_collection();
        let filter = doc! { "webhook_id": webhook_id };

        collection
            .find_one(filter)
            .await
            .map_err(|e| format!("Failed to get webhook: {}", e))
    }

    async fn delete_webhook(&self, webhook_id: &str) -> Result<(), String> {
        let webhooks = self.webhooks_collection();
        let deliveries = self.deliveries_collection();

        // Delete the webhook
        let filter = doc! { "webhook_id": webhook_id };
        webhooks
            .delete_one(filter.clone())
            .await
            .map_err(|e| format!("Failed to delete webhook: {}", e))?;

        // Delete associated deliveries
        deliveries
            .delete_many(filter)
            .await
            .map_err(|e| format!("Failed to delete webhook deliveries: {}", e))?;

        Ok(())
    }

    async fn list_webhooks(
        &self,
        event: Option<&str>,
        active_only: Option<bool>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Webhook>, String> {
        let collection = self.webhooks_collection();
        let limit = limit.unwrap_or(100) as i64;
        let offset = offset.unwrap_or(0) as u64;

        // Build filter
        let mut filter = doc! {};
        if let Some(true) = active_only {
            filter.insert("active", true);
        }
        if let Some(evt) = event {
            filter.insert("events", doc! { "$in": [evt] });
        }

        let options = FindOptions::builder()
            .skip(offset)
            .limit(limit)
            .sort(doc! { "created_at": -1 })
            .build();

        let cursor = collection
            .find(filter)
            .with_options(options)
            .await
            .map_err(|e| format!("Failed to list webhooks: {}", e))?;

        let webhooks: Vec<Webhook> = cursor
            .try_collect()
            .await
            .map_err(|e| format!("Failed to collect webhooks: {}", e))?;

        Ok(webhooks)
    }

    async fn save_delivery(&self, delivery: &WebhookDelivery) -> Result<(), String> {
        let collection = self.deliveries_collection();

        // Upsert the delivery
        let filter = doc! { "delivery_id": &delivery.delivery_id };
        let options = ReplaceOptions::builder().upsert(true).build();

        collection
            .replace_one(filter, delivery)
            .with_options(options)
            .await
            .map_err(|e| format!("Failed to save delivery: {}", e))?;

        Ok(())
    }

    async fn get_delivery(&self, delivery_id: &str) -> Result<Option<WebhookDelivery>, String> {
        let collection = self.deliveries_collection();
        let filter = doc! { "delivery_id": delivery_id };

        collection
            .find_one(filter)
            .await
            .map_err(|e| format!("Failed to get delivery: {}", e))
    }

    async fn list_deliveries(
        &self,
        webhook_id: &str,
        status: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<WebhookDelivery>, String> {
        let collection = self.deliveries_collection();
        let limit = limit.unwrap_or(50) as i64;
        let offset = offset.unwrap_or(0) as u64;

        // Build filter
        let mut filter = doc! { "webhook_id": webhook_id };
        if let Some(status) = status {
            filter.insert("status", doc! { "$regex": status, "$options": "i" });
        }

        let options = FindOptions::builder()
            .skip(offset)
            .limit(limit)
            .sort(doc! { "created_at": -1 })
            .build();

        let cursor = collection
            .find(filter)
            .with_options(options)
            .await
            .map_err(|e| format!("Failed to list deliveries: {}", e))?;

        let deliveries: Vec<WebhookDelivery> = cursor
            .try_collect()
            .await
            .map_err(|e| format!("Failed to collect deliveries: {}", e))?;

        Ok(deliveries)
    }
}
