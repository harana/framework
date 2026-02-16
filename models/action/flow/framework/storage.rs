// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageQueryOptions {
    pub limit: i64,
    pub offset: i64,
    pub sort_by: String,
    #[serde(default)]
    pub sort_desc: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageFilterCondition {
    pub field: String,
    pub operator: String,
    pub value: String,
    pub values: Vec<String>,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageQueueStats {
    pub completed: i64,
    pub in_flight: i64,
    pub total: i64,
    pub waiting: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageQueueMessage {
    pub ack_id: String,
    pub payload: String,
    pub tries: i64,
}

