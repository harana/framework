// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Event {
    pub attributes: std::collections::HashMap<String, String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub source: String,
    #[serde(default = "chrono::Utc::now")]
    pub occured_at: chrono::DateTime<chrono::Utc>,
    pub type: String,
}

impl Event {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

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
    pub subscription_id: Option<String>,
}

impl EventLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

