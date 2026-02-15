// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageQueryOptions {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort_by: Option<String>,
    #[serde(default)]
    pub sort_desc: bool,
}

impl StorageQueryOptions {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageFilterCondition {
    pub field: String,
    pub operator: String,
    pub value: Option<String>,
    pub values: Option<Vec<String>>,
    pub conditions: Option<Vec<String>>,
}

impl StorageFilterCondition {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageQueueStats {
    pub completed: i64,
    pub in_flight: i64,
    pub total: i64,
    pub waiting: i64,
}

impl StorageQueueStats {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageQueueMessage {
    pub ack_id: String,
    pub payload: String,
    pub tries: i64,
}

impl StorageQueueMessage {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

