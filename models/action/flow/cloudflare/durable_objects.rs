// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FetchOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageGetOutput {
    pub found: bool,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlExecOutput {
    pub changes: i64,
    pub results: Vec<String>,
    pub rows_read: i64,
    pub rows_written: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfDurableObjectNamespace {
    pub account_id: String,
    pub class_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub namespace_id: String,
    pub namespace_name: String,
    pub script_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfDurableObjectInstance {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub has_storage: bool,
    pub jurisdiction: String,
    pub namespace_id: String,
    pub object_id: String,
    pub object_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfDurableObjectStorageEntry {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub instance_id: String,
    pub key: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfDurableObjectAlarm {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub instance_id: String,
    pub scheduled_time: chrono::DateTime<chrono::Utc>,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CfDurableObjectWebsocket {
    #[serde(default = "chrono::Utc::now")]
    pub accepted_at: chrono::DateTime<chrono::Utc>,
    pub auto_response_request: String,
    pub auto_response_value: String,
    pub instance_id: String,
    #[serde(default)]
    pub is_hibernated: bool,
    pub status: String,
    pub tags: String,
    pub websocket_id: String,
}

#[async_trait]
pub trait DurableObjectsAction {
    async fn id_from_name(&self, name: String, namespace: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn id_from_string(&self, id: String, namespace: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn new_unique_id(&self, jurisdiction: String, namespace: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_stub(&self, id: String, namespace: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn fetch(&self, body: String, headers: String, id: String, method: String, namespace: String, url: String) -> Result<FetchOutput, Box<dyn std::error::Error>>;
    async fn storage_get(&self, key: String) -> Result<StorageGetOutput, Box<dyn std::error::Error>>;
    async fn storage_get_multiple(&self, keys: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn storage_put(&self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn storage_put_multiple(&self, entries: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn storage_delete(&self, key: String) -> Result<bool, Box<dyn std::error::Error>>;
    async fn storage_delete_multiple(&self, keys: Vec<String>) -> Result<i64, Box<dyn std::error::Error>>;
    async fn storage_delete_all(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn storage_list(&self, end: String, limit: i64, prefix: String, reverse: bool, start: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn storage_transaction(&self, closure: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn sql_exec(&self, params: Vec<String>, sql: String) -> Result<SqlExecOutput, Box<dyn std::error::Error>>;
    async fn set_alarm(&self, scheduled_time: chrono::DateTime<chrono::Utc>) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_alarm(&self) -> Result<chrono::DateTime<chrono::Utc>, Box<dyn std::error::Error>>;
    async fn delete_alarm(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn accept_websocket(&self, tags: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_websockets(&self, tag: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn set_websocket_auto_response(&self, request: String, response: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn hibernate_websocket(&self, websocket: String) -> Result<(), Box<dyn std::error::Error>>;
}
