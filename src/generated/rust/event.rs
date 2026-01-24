// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Event
/// Class: event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Event {
    pub attributes: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub source: String,
    #[serde(default = "chrono::Utc::now")]
    pub occured_at: chrono::DateTime<chrono::Utc>,
    pub type: String,
}

impl Event {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Event Subscription
/// Class: event_subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventSubscription {
    pub callback_url: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub event_type: String,
    pub filter: Option<String>,
    pub id: String,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl EventSubscription {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Event Log
/// Class: event_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventLog {
    pub error_message: Option<String>,
    pub event_id: String,
    pub id: String,
    #[serde(default = "chrono::Utc::now")]
    pub processed_at: chrono::DateTime<chrono::Utc>,
    pub retry_count: i64,
    pub status: String,
    /// Reference: EventSubscription.id
    pub subscription_id: Option<String>,
}

impl EventLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

