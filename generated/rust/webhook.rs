// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Webhook
/// Class: webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Webhook {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub secret: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
}

impl Webhook {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Webhook Delivery
/// Class: webhook_delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebhookDelivery {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub event_type: String,
    pub payload: String,
    pub request_headers: Option<String>,
    pub response_body: Option<String>,
    pub response_code: Option<i64>,
    pub response_headers: Option<String>,
    pub retry_count: i64,
    pub status: String,
    pub webhook_id: String,
}

impl WebhookDelivery {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Webhook Event
/// Class: webhook_event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WebhookEvent {
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub payload_schema: Option<String>,
}

impl WebhookEvent {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

