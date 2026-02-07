// Harana Actions - Cloudflare Durable Objects Module Output Types

use serde::{Deserialize, Serialize};

// id_from_name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdFromNameOutput {
    pub id: String,
}

// id_from_string
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdFromStringOutput {
    pub id: String,
}

// new_unique_id
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUniqueIdOutput {
    pub id: String,
}

// get_stub
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStubOutput {
    pub stub: DurableObjectStub,
}

// fetch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchOutput {
    pub body: serde_json::Value,
    pub headers: std::collections::HashMap<String, String>,
    pub status_code: i32,
}

// storage_get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageGetOutput {
    pub found: bool,
    pub value: serde_json::Value,
}

// storage_get_multiple
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageGetMultipleOutput {
    pub values: serde_json::Value,
}

// storage_put
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePutOutput {
    pub success: bool,
}

// storage_put_multiple
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePutMultipleOutput {
    pub success: bool,
}

// storage_delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDeleteOutput {
    pub deleted: bool,
}

// storage_delete_multiple
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDeleteMultipleOutput {
    pub deleted_count: i32,
}

// storage_delete_all
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDeleteAllOutput {
    pub success: bool,
}

// storage_list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageListOutput {
    pub entries: serde_json::Value,
}

// storage_transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageTransactionOutput {
    pub result: serde_json::Value,
}

// sql_exec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlExecOutput {
    pub changes: i32,
    pub results: Vec<serde_json::Value>,
    pub rows_read: i32,
    pub rows_written: i32,
}

// set_alarm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetAlarmOutput {
    pub success: bool,
}

// get_alarm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAlarmOutput {
    pub scheduled_time: String,
}

// delete_alarm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAlarmOutput {
    pub success: bool,
}

// accept_websocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptWebsocketOutput {
    pub success: bool,
}

// get_websockets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWebsocketsOutput {
    pub websockets: Vec<WebSocket>,
}

// set_websocket_auto_response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetWebsocketAutoResponseOutput {
    pub success: bool,
}

// hibernate_websocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HibernateWebsocketOutput {
    pub success: bool,
}

// Helper structs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurableObjectStub {
    pub id: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocket {
    pub id: String,
    pub tags: Vec<String>,
}
