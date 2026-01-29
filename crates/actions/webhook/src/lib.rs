// Harana Actions - Webhook Module
// This module provides webhook actions and functionality.

pub mod output;

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use hmac::{Hmac, Mac};
use output::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Sha256, Sha512};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;
type HmacSha512 = Hmac<Sha512>;

// Internal data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Webhook {
    webhook_id: String,
    url: String,
    events: Vec<String>,
    secret: String,
    active: bool,
    description: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WebhookDelivery {
    delivery_id: String,
    webhook_id: String,
    event: String,
    status: String,
    status_code: i32,
    response_time_ms: i32,
    error: String,
    created_at: DateTime<Utc>,
}

// Global in-memory storage
lazy_static::lazy_static! {
    static ref WEBHOOKS: Arc<DashMap<String, Webhook>> = Arc::new(DashMap::new());
    static ref DELIVERIES: Arc<DashMap<String, WebhookDelivery>> = Arc::new(DashMap::new());
}

/// Register Webhook Endpoint
pub async fn register(
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

    WEBHOOKS.insert(webhook_id.clone(), webhook);

    Ok(RegisterOutput {
        success: true,
        secret,
        webhook_id,
    })
}

/// Update Webhook Configuration
pub async fn update(
    webhook_id: &str,
    events: Option<Vec<String>>,
    active: Option<bool>,
    description: Option<&str>,
    secret: Option<&str>,
    url: Option<&str>,
) -> Result<UpdateOutput, String> {
    let mut webhook = WEBHOOKS
        .get_mut(webhook_id)
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

    Ok(UpdateOutput { success: true })
}

/// Unregister Webhook Endpoint
pub async fn unregister(webhook_id: &str) -> Result<UnregisterOutput, String> {
    WEBHOOKS
        .remove(webhook_id)
        .ok_or_else(|| format!("Webhook not found: {}", webhook_id))?;

    Ok(UnregisterOutput { success: true })
}

/// Trigger Webhook Event
pub async fn trigger(
    event: &str,
    payload: &str,
    webhook_ids: Option<Vec<String>>,
) -> Result<TriggerOutput, String> {
    let mut triggered_count = 0;

    let webhooks_to_trigger: Vec<Webhook> = if let Some(ids) = webhook_ids {
        ids.iter()
            .filter_map(|id| WEBHOOKS.get(id).map(|w| w.clone()))
            .collect()
    } else {
        WEBHOOKS
            .iter()
            .filter(|w| w.active && w.events.contains(&event.to_string()))
            .map(|w| w.clone())
            .collect()
    };

    let client = reqwest::Client::new();

    for webhook in webhooks_to_trigger {
        let delivery_id = Uuid::new_v4().to_string();
        let start = Instant::now();

        let result = client
            .post(&webhook.url)
            .header("Content-Type", "application/json")
            .header("X-Webhook-Event", event)
            .header(
                "X-Webhook-Signature",
                compute_signature(payload, &webhook.secret, "Sha256")?,
            )
            .body(payload.to_string())
            .send()
            .await;

        let response_time_ms = start.elapsed().as_millis() as i32;

        let delivery = match result {
            Ok(response) => {
                let status_code = response.status().as_u16() as i32;
                triggered_count += 1;

                WebhookDelivery {
                    delivery_id: delivery_id.clone(),
                    webhook_id: webhook.webhook_id.clone(),
                    event: event.to_string(),
                    status: if response.status().is_success() {
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
                status: "Failed".to_string(),
                status_code: 0,
                response_time_ms,
                error: e.to_string(),
                created_at: Utc::now(),
            },
        };

        DELIVERIES.insert(delivery_id, delivery);
    }

    Ok(TriggerOutput {
        success: triggered_count > 0,
        triggered_count,
    })
}

/// Get Webhook Details
pub async fn get(webhook_id: &str) -> Result<GetOutput, String> {
    let webhook = WEBHOOKS
        .get(webhook_id)
        .ok_or_else(|| format!("Webhook not found: {}", webhook_id))?;

    Ok(GetOutput {
        events: webhook.events.clone(),
        url: webhook.url.clone(),
        active: webhook.active,
        created_at: webhook.created_at.to_rfc3339(),
        description: webhook.description.clone(),
    })
}

/// List Registered Webhooks
pub async fn lists(
    event: Option<&str>,
    active_only: Option<bool>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<ListsOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    let offset = offset.unwrap_or(0) as usize;
    let active_only = active_only.unwrap_or(false);

    let mut webhooks: Vec<Webhook> = WEBHOOKS
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
        .collect();

    let total = webhooks.len() as i32;
    webhooks = webhooks.into_iter().skip(offset).take(limit).collect();

    let webhooks_json: Vec<HashMap<String, Value>> = webhooks
        .iter()
        .map(|w| {
            let mut map = HashMap::new();
            map.insert("webhook_id".to_string(), Value::String(w.webhook_id.clone()));
            map.insert("url".to_string(), Value::String(w.url.clone()));
            map.insert("events".to_string(), serde_json::to_value(&w.events).unwrap());
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

/// Test Webhook Endpoint
pub async fn test(webhook_id: &str, payload: Option<&str>) -> Result<TestOutput, String> {
    let webhook = WEBHOOKS
        .get(webhook_id)
        .ok_or_else(|| format!("Webhook not found: {}", webhook_id))?
        .clone();

    let payload = payload.unwrap_or(r#"{"test": true}"#);
    let client = reqwest::Client::new();
    let start = Instant::now();

    match client
        .post(&webhook.url)
        .header("Content-Type", "application/json")
        .header("X-Webhook-Event", "test")
        .header(
            "X-Webhook-Signature",
            compute_signature(payload, &webhook.secret, "Sha256")?,
        )
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

/// Get Webhook Delivery Log
pub async fn get_deliveries(
    webhook_id: &str,
    status: Option<&str>,
    offset: Option<i32>,
    limit: Option<i32>,
) -> Result<GetDeliveriesOutput, String> {
    let limit = limit.unwrap_or(50) as usize;
    let offset = offset.unwrap_or(0) as usize;

    let mut deliveries: Vec<WebhookDelivery> = DELIVERIES
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
        .collect();

    let total = deliveries.len() as i32;
    deliveries = deliveries.into_iter().skip(offset).take(limit).collect();

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

/// Retry Failed Delivery
pub async fn retry_delivery(delivery_id: &str) -> Result<RetryDeliveryOutput, String> {
    let delivery = DELIVERIES
        .get(delivery_id)
        .ok_or_else(|| format!("Delivery not found: {}", delivery_id))?
        .clone();

    let webhook = WEBHOOKS
        .get(&delivery.webhook_id)
        .ok_or_else(|| format!("Webhook not found: {}", delivery.webhook_id))?
        .clone();

    let client = reqwest::Client::new();
    let payload = r#"{"retry": true}"#;

    match client
        .post(&webhook.url)
        .header("Content-Type", "application/json")
        .header("X-Webhook-Event", &delivery.event)
        .header(
            "X-Webhook-Signature",
            compute_signature(payload, &webhook.secret, "Sha256")?,
        )
        .body(payload)
        .send()
        .await
    {
        Ok(response) => {
            let status_code = response.status().as_u16() as i32;
            Ok(RetryDeliveryOutput {
                success: response.status().is_success(),
                status_code,
            })
        }
        Err(_) => Ok(RetryDeliveryOutput {
            success: false,
            status_code: 0,
        }),
    }
}

/// Verify Webhook Signature
pub async fn verify_signature(
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

/// Rotate Webhook Secret
pub async fn rotate_secret(webhook_id: &str) -> Result<RotateSecretOutput, String> {
    let mut webhook = WEBHOOKS
        .get_mut(webhook_id)
        .ok_or_else(|| format!("Webhook not found: {}", webhook_id))?;

    let new_secret = Uuid::new_v4().to_string();
    webhook.secret = new_secret.clone();

    Ok(RotateSecretOutput {
        new_secret,
        success: true,
    })
}

// Helper function to compute HMAC signature
fn compute_signature(payload: &str, secret: &str, algorithm: &str) -> Result<String, String> {
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
