// Harana Actions - Webhook Module
// This module provides webhook actions and functionality with multiple storage backends.

pub mod output;
pub mod storage;

use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use output::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Sha256, Sha512};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use storage::{StorageBackend, StorageType};
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;
type HmacSha512 = Hmac<Sha512>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    pub webhook_id: String,
    pub url: String,
    pub events: Vec<String>,
    pub secret: String,
    pub active: bool,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDelivery {
    pub delivery_id: String,
    pub webhook_id: String,
    pub event: String,
    pub payload: String,
    pub status: String,
    pub status_code: i32,
    pub response_time_ms: i32,
    pub error: String,
    pub created_at: DateTime<Utc>,
}

pub struct WebhookService {
    storage: Arc<dyn StorageBackend>,
    http_client: reqwest::Client,
}

impl WebhookService {
    /// Create a new WebhookService with the specified storage type
    pub async fn new(storage_type: StorageType) -> Result<Self, String> {
        let storage = storage::create_storage(storage_type).await?;
        Ok(Self {
            storage,
            http_client: reqwest::Client::new(),
        })
    }

    /// Create a new WebhookService with a custom storage backend
    pub fn with_storage(storage: Arc<dyn StorageBackend>) -> Self {
        Self {
            storage,
            http_client: reqwest::Client::new(),
        }
    }

    /// Register a new webhook endpoint
    pub async fn register(
        &self,
        url: &str,
        events: Vec<String>,
        active: Option<bool>,
        secret: Option<&str>,
        description: Option<&str>,
    ) -> Result<RegisterOutput, String> {
        let webhook_id = Uuid::new_v4().to_string();
        let secret = secret
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        let webhook = Webhook {
            webhook_id: webhook_id.clone(),
            url: url.to_string(),
            events,
            secret: secret.clone(),
            active: active.unwrap_or(true),
            description: description.unwrap_or("").to_string(),
            created_at: Utc::now(),
        };

        self.storage.save_webhook(&webhook).await?;

        Ok(RegisterOutput {
            success: true,
            secret,
            webhook_id,
        })
    }

    /// Update an existing webhook configuration
    pub async fn update(
        &self,
        webhook_id: &str,
        events: Option<Vec<String>>,
        active: Option<bool>,
        description: Option<&str>,
        secret: Option<&str>,
        url: Option<&str>,
    ) -> Result<UpdateOutput, String> {
        let mut webhook = self
            .storage
            .get_webhook(webhook_id)
            .await?
            .ok_or_else(|| format!("Webhook not found: {}", webhook_id))?;

        if let Some(events) = events {
            webhook.events = events;
        }
        if let Some(active) = active {
            webhook.active = active;
        }
        if let Some(description) = description {
            webhook.description = description.to_string();
        }
        if let Some(secret) = secret {
            webhook.secret = secret.to_string();
        }
        if let Some(url) = url {
            webhook.url = url.to_string();
        }

        self.storage.save_webhook(&webhook).await?;

        Ok(UpdateOutput { success: true })
    }

    /// Unregister a webhook endpoint
    pub async fn unregister(&self, webhook_id: &str) -> Result<UnregisterOutput, String> {
        let exists = self.storage.get_webhook(webhook_id).await?.is_some();
        if !exists {
            return Err(format!("Webhook not found: {}", webhook_id));
        }

        self.storage.delete_webhook(webhook_id).await?;

        Ok(UnregisterOutput { success: true })
    }

    /// Trigger webhook events
    pub async fn trigger(
        &self,
        event: &str,
        payload: &str,
        webhook_ids: Option<Vec<String>>,
    ) -> Result<TriggerOutput, String> {
        let mut triggered_count = 0;

        let webhooks_to_trigger: Vec<Webhook> = if let Some(ids) = webhook_ids {
            let mut webhooks = Vec::new();
            for id in ids {
                if let Some(webhook) = self.storage.get_webhook(&id).await? {
                    webhooks.push(webhook);
                }
            }
            webhooks
        } else {
            self.storage
                .list_webhooks(Some(event), Some(true), None, None)
                .await?
        };

        for webhook in webhooks_to_trigger {
            if !webhook.active {
                continue;
            }

            let delivery_id = Uuid::new_v4().to_string();
            let start = Instant::now();

            let signature = compute_signature(payload, &webhook.secret, "Sha256")?;

            let result = self
                .http_client
                .post(&webhook.url)
                .header("Content-Type", "application/json")
                .header("X-Webhook-Event", event)
                .header("X-Webhook-Signature", &signature)
                .header("X-Webhook-Delivery", &delivery_id)
                .body(payload.to_string())
                .send()
                .await;

            let response_time_ms = start.elapsed().as_millis() as i32;

            let delivery = match result {
                Ok(response) => {
                    let status_code = response.status().as_u16() as i32;
                    let success = response.status().is_success();
                    if success {
                        triggered_count += 1;
                    }

                    WebhookDelivery {
                        delivery_id: delivery_id.clone(),
                        webhook_id: webhook.webhook_id.clone(),
                        event: event.to_string(),
                        payload: payload.to_string(),
                        status: if success {
                            "Success".to_string()
                        } else {
                            "Failed".to_string()
                        },
                        status_code,
                        response_time_ms,
                        error: String::new(),
                        created_at: Utc::now(),
                    }
                }
                Err(e) => WebhookDelivery {
                    delivery_id: delivery_id.clone(),
                    webhook_id: webhook.webhook_id.clone(),
                    event: event.to_string(),
                    payload: payload.to_string(),
                    status: "Failed".to_string(),
                    status_code: 0,
                    response_time_ms,
                    error: e.to_string(),
                    created_at: Utc::now(),
                },
            };

            self.storage.save_delivery(&delivery).await?;
        }

        Ok(TriggerOutput {
            success: triggered_count > 0,
            triggered_count,
        })
    }

    /// Get webhook details
    pub async fn get(&self, webhook_id: &str) -> Result<GetOutput, String> {
        let webhook = self
            .storage
            .get_webhook(webhook_id)
            .await?
            .ok_or_else(|| format!("Webhook not found: {}", webhook_id))?;

        Ok(GetOutput {
            events: webhook.events,
            url: webhook.url,
            active: webhook.active,
            created_at: webhook.created_at.to_rfc3339(),
            description: webhook.description,
        })
    }

    /// List registered webhooks
    pub async fn lists(
        &self,
        event: Option<&str>,
        active_only: Option<bool>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<ListsOutput, String> {
        let webhooks = self
            .storage
            .list_webhooks(event, active_only, limit, offset)
            .await?;

        let total = webhooks.len() as i32;

        let webhooks_json: Vec<HashMap<String, Value>> = webhooks
            .iter()
            .map(|w| {
                let mut map = HashMap::new();
                map.insert("webhook_id".to_string(), Value::String(w.webhook_id.clone()));
                map.insert("url".to_string(), Value::String(w.url.clone()));
                map.insert(
                    "events".to_string(),
                    serde_json::to_value(&w.events).unwrap_or(Value::Array(vec![])),
                );
                map.insert("active".to_string(), Value::Bool(w.active));
                map.insert(
                    "description".to_string(),
                    Value::String(w.description.clone()),
                );
                map.insert(
                    "created_at".to_string(),
                    Value::String(w.created_at.to_rfc3339()),
                );
                map
            })
            .collect();

        Ok(ListsOutput {
            webhooks: webhooks_json,
            total,
        })
    }

    /// Test a webhook endpoint
    pub async fn test(&self, webhook_id: &str, payload: Option<&str>) -> Result<TestOutput, String> {
        let webhook = self
            .storage
            .get_webhook(webhook_id)
            .await?
            .ok_or_else(|| format!("Webhook not found: {}", webhook_id))?;

        let payload = payload.unwrap_or(r#"{"test": true}"#);
        let start = Instant::now();

        let signature = compute_signature(payload, &webhook.secret, "Sha256")?;

        match self
            .http_client
            .post(&webhook.url)
            .header("Content-Type", "application/json")
            .header("X-Webhook-Event", "test")
            .header("X-Webhook-Signature", &signature)
            .body(payload.to_string())
            .send()
            .await
        {
            Ok(response) => {
                let status_code = response.status().as_u16() as i32;
                let response_time_ms = start.elapsed().as_millis() as i32;

                Ok(TestOutput {
                    success: response.status().is_success(),
                    status_code,
                    error: String::new(),
                    response_time_ms,
                })
            }
            Err(e) => {
                let response_time_ms = start.elapsed().as_millis() as i32;
                Ok(TestOutput {
                    success: false,
                    status_code: 0,
                    error: e.to_string(),
                    response_time_ms,
                })
            }
        }
    }

    /// Get webhook delivery log
    pub async fn get_deliveries(
        &self,
        webhook_id: &str,
        status: Option<&str>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<GetDeliveriesOutput, String> {
        let deliveries = self
            .storage
            .list_deliveries(webhook_id, status, limit, offset)
            .await?;

        let total = deliveries.len() as i32;

        let deliveries_json: Vec<HashMap<String, Value>> = deliveries
            .iter()
            .map(|d| {
                let mut map = HashMap::new();
                map.insert(
                    "delivery_id".to_string(),
                    Value::String(d.delivery_id.clone()),
                );
                map.insert(
                    "webhook_id".to_string(),
                    Value::String(d.webhook_id.clone()),
                );
                map.insert("event".to_string(), Value::String(d.event.clone()));
                map.insert("status".to_string(), Value::String(d.status.clone()));
                map.insert(
                    "status_code".to_string(),
                    Value::Number(d.status_code.into()),
                );
                map.insert(
                    "response_time_ms".to_string(),
                    Value::Number(d.response_time_ms.into()),
                );
                map.insert("error".to_string(), Value::String(d.error.clone()));
                map.insert(
                    "created_at".to_string(),
                    Value::String(d.created_at.to_rfc3339()),
                );
                map
            })
            .collect();

        Ok(GetDeliveriesOutput {
            deliveries: deliveries_json,
            total,
        })
    }

    /// Retry a failed delivery
    pub async fn retry_delivery(&self, delivery_id: &str) -> Result<RetryDeliveryOutput, String> {
        let delivery = self
            .storage
            .get_delivery(delivery_id)
            .await?
            .ok_or_else(|| format!("Delivery not found: {}", delivery_id))?;

        let webhook = self
            .storage
            .get_webhook(&delivery.webhook_id)
            .await?
            .ok_or_else(|| format!("Webhook not found: {}", delivery.webhook_id))?;

        let payload = if delivery.payload.is_empty() {
            r#"{"retry": true}"#.to_string()
        } else {
            delivery.payload.clone()
        };

        let signature = compute_signature(&payload, &webhook.secret, "Sha256")?;
        let new_delivery_id = Uuid::new_v4().to_string();
        let start = Instant::now();

        let result = self
            .http_client
            .post(&webhook.url)
            .header("Content-Type", "application/json")
            .header("X-Webhook-Event", &delivery.event)
            .header("X-Webhook-Signature", &signature)
            .header("X-Webhook-Delivery", &new_delivery_id)
            .header("X-Webhook-Retry-Of", delivery_id)
            .body(payload.clone())
            .send()
            .await;

        let response_time_ms = start.elapsed().as_millis() as i32;

        let (success, status_code, error) = match result {
            Ok(response) => {
                let code = response.status().as_u16() as i32;
                (response.status().is_success(), code, String::new())
            }
            Err(e) => (false, 0, e.to_string()),
        };

        // Save the retry delivery record
        let new_delivery = WebhookDelivery {
            delivery_id: new_delivery_id,
            webhook_id: webhook.webhook_id.clone(),
            event: delivery.event.clone(),
            payload,
            status: if success {
                "Success".to_string()
            } else {
                "Failed".to_string()
            },
            status_code,
            response_time_ms,
            error,
            created_at: Utc::now(),
        };

        self.storage.save_delivery(&new_delivery).await?;

        Ok(RetryDeliveryOutput {
            success,
            status_code,
        })
    }

    /// Verify a webhook signature
    pub async fn verify_signature(
        &self,
        signature: &str,
        payload: &str,
        secret: &str,
        algorithm: Option<&str>,
    ) -> Result<VerifySignatureOutput, String> {
        let algorithm = algorithm.unwrap_or("Sha256");
        let expected_signature = compute_signature(payload, secret, algorithm)?;

        Ok(VerifySignatureOutput {
            valid: signature == expected_signature,
        })
    }

    /// Rotate webhook secret
    pub async fn rotate_secret(&self, webhook_id: &str) -> Result<RotateSecretOutput, String> {
        let mut webhook = self
            .storage
            .get_webhook(webhook_id)
            .await?
            .ok_or_else(|| format!("Webhook not found: {}", webhook_id))?;

        let new_secret = Uuid::new_v4().to_string();
        webhook.secret = new_secret.clone();

        self.storage.save_webhook(&webhook).await?;

        Ok(RotateSecretOutput {
            new_secret,
            success: true,
        })
    }
}

/// Compute HMAC signature for webhook payload
pub fn compute_signature(payload: &str, secret: &str, algorithm: &str) -> Result<String, String> {
    match algorithm {
        "Sha256" => {
            let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
                .map_err(|e| format!("Invalid key length: {}", e))?;
            mac.update(payload.as_bytes());
            Ok(hex::encode(mac.finalize().into_bytes()))
        }
        "Sha512" => {
            let mut mac = HmacSha512::new_from_slice(secret.as_bytes())
                .map_err(|e| format!("Invalid key length: {}", e))?;
            mac.update(payload.as_bytes());
            Ok(hex::encode(mac.finalize().into_bytes()))
        }
        _ => Err(format!("Unsupported algorithm: {}", algorithm)),
    }
}

#[cfg(test)]
mod tests;
