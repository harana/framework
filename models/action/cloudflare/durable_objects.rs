// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IdFromNameInput {
    pub name: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IdFromNameOutput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IdFromStringInput {
    pub id: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IdFromStringOutput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewUniqueIdInput {
    pub jurisdiction: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewUniqueIdOutput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetStubInput {
    pub id: String,
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetStubOutput {
    pub stub: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FetchInput {
    pub body: String,
    pub headers: String,
    pub id: String,
    pub method: String,
    pub namespace: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FetchOutput {
    pub body: String,
    pub headers: String,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageGetInput {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageGetOutput {
    pub found: bool,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageGetMultipleInput {
    pub keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageGetMultipleOutput {
    pub values: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StoragePutInput {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StoragePutOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StoragePutMultipleInput {
    pub entries: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StoragePutMultipleOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageDeleteInput {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageDeleteOutput {
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageDeleteMultipleInput {
    pub keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageDeleteMultipleOutput {
    pub deleted_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageDeleteAllOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageListInput {
    pub end: String,
    pub limit: i64,
    pub prefix: String,
    #[serde(default)]
    pub reverse: bool,
    pub start: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageListOutput {
    pub entries: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageTransactionInput {
    pub closure: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StorageTransactionOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SqlExecInput {
    pub params: Vec<String>,
    pub sql: String,
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
pub struct SetAlarmInput {
    pub scheduled_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetAlarmOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAlarmOutput {
    pub scheduled_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteAlarmOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AcceptWebsocketInput {
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AcceptWebsocketOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetWebsocketsInput {
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetWebsocketsOutput {
    pub websockets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetWebsocketAutoResponseInput {
    pub request: String,
    pub response: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetWebsocketAutoResponseOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HibernateWebsocketInput {
    pub websocket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HibernateWebsocketOutput {
    pub success: bool,
}

#[async_trait]
pub trait DurableObjectsAction {
    async fn id_from_name(&self, input: IdFromNameInput) -> Result<IdFromNameOutput, Box<dyn std::error::Error>>;
    async fn id_from_string(&self, input: IdFromStringInput) -> Result<IdFromStringOutput, Box<dyn std::error::Error>>;
    async fn new_unique_id(&self, input: NewUniqueIdInput) -> Result<NewUniqueIdOutput, Box<dyn std::error::Error>>;
    async fn get_stub(&self, input: GetStubInput) -> Result<GetStubOutput, Box<dyn std::error::Error>>;
    async fn fetch(&self, input: FetchInput) -> Result<FetchOutput, Box<dyn std::error::Error>>;
    async fn storage_get(&self, input: StorageGetInput) -> Result<StorageGetOutput, Box<dyn std::error::Error>>;
    async fn storage_get_multiple(&self, input: StorageGetMultipleInput) -> Result<StorageGetMultipleOutput, Box<dyn std::error::Error>>;
    async fn storage_put(&self, input: StoragePutInput) -> Result<StoragePutOutput, Box<dyn std::error::Error>>;
    async fn storage_put_multiple(&self, input: StoragePutMultipleInput) -> Result<StoragePutMultipleOutput, Box<dyn std::error::Error>>;
    async fn storage_delete(&self, input: StorageDeleteInput) -> Result<StorageDeleteOutput, Box<dyn std::error::Error>>;
    async fn storage_delete_multiple(&self, input: StorageDeleteMultipleInput) -> Result<StorageDeleteMultipleOutput, Box<dyn std::error::Error>>;
    async fn storage_delete_all(&self) -> Result<StorageDeleteAllOutput, Box<dyn std::error::Error>>;
    async fn storage_list(&self, input: StorageListInput) -> Result<StorageListOutput, Box<dyn std::error::Error>>;
    async fn storage_transaction(&self, input: StorageTransactionInput) -> Result<StorageTransactionOutput, Box<dyn std::error::Error>>;
    async fn sql_exec(&self, input: SqlExecInput) -> Result<SqlExecOutput, Box<dyn std::error::Error>>;
    async fn set_alarm(&self, input: SetAlarmInput) -> Result<SetAlarmOutput, Box<dyn std::error::Error>>;
    async fn get_alarm(&self) -> Result<GetAlarmOutput, Box<dyn std::error::Error>>;
    async fn delete_alarm(&self) -> Result<DeleteAlarmOutput, Box<dyn std::error::Error>>;
    async fn accept_websocket(&self, input: AcceptWebsocketInput) -> Result<AcceptWebsocketOutput, Box<dyn std::error::Error>>;
    async fn get_websockets(&self, input: GetWebsocketsInput) -> Result<GetWebsocketsOutput, Box<dyn std::error::Error>>;
    async fn set_websocket_auto_response(&self, input: SetWebsocketAutoResponseInput) -> Result<SetWebsocketAutoResponseOutput, Box<dyn std::error::Error>>;
    async fn hibernate_websocket(&self, input: HibernateWebsocketInput) -> Result<HibernateWebsocketOutput, Box<dyn std::error::Error>>;
}
