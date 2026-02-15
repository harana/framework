// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectCreated {
    pub namespace: String,
    pub object_id: String,
    pub name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectCreated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectFetchReceived {
    pub namespace: String,
    pub object_id: String,
    pub url: String,
    pub method: String,
    pub status_code: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub received_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectFetchReceived {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectStorageUpdated {
    pub operation: String,
    pub key: Option<String>,
    pub key_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectStorageUpdated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectAlarmTriggered {
    pub scheduled_time: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub triggered_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectAlarmTriggered {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectAlarmSet {
    pub scheduled_time: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub set_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectAlarmSet {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectAlarmDeleted {
    #[serde(default = "chrono::Utc::now")]
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectAlarmDeleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectWebsocketAccepted {
    pub tag_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub accepted_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectWebsocketAccepted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectWebsocketMessageReceived {
    pub tag: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub received_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectWebsocketMessageReceived {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectWebsocketClosed {
    pub code: Option<i64>,
    pub reason: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub closed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectWebsocketClosed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectWebsocketError {
    pub error_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectWebsocketError {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectSqlExecuted {
    pub rows_read: Option<i64>,
    pub rows_written: Option<i64>,
    pub changes: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub executed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectSqlExecuted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

