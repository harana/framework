// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectNamespace {
    pub account_id: String,
    pub class_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub namespace_id: Option<String>,
    pub namespace_name: String,
    pub script_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectNamespace {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectInstance {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub has_storage: bool,
    pub jurisdiction: Option<String>,
    pub namespace_id: String,
    pub object_id: String,
    pub object_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectInstance {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectStorageEntry {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub instance_id: String,
    pub key: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub value: Option<String>,
}

impl CloudflareDurableObjectStorageEntry {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectAlarm {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub instance_id: String,
    pub scheduled_time: chrono::DateTime<chrono::Utc>,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareDurableObjectAlarm {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareDurableObjectWebsocket {
    #[serde(default = "chrono::Utc::now")]
    pub accepted_at: chrono::DateTime<chrono::Utc>,
    pub auto_response_request: Option<String>,
    pub auto_response_value: Option<String>,
    pub instance_id: String,
    #[serde(default)]
    pub is_hibernated: bool,
    pub status: String,
    pub tags: Option<String>,
    pub websocket_id: String,
}

impl CloudflareDurableObjectWebsocket {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

