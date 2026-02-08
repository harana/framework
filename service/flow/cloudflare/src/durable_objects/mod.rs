// Harana Actions - Cloudflare Durable Objects Module
// This module provides Cloudflare Durable Objects actions for managing stateful
// objects, storage, alarms, and WebSocket connections.

pub mod output;

use output::*;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use worker::Env;

fn to_err(e: worker::Error) -> String {
    format!("Durable Objects error: {e}")
}

/// Get Durable Object By Name
pub async fn id_from_name(env: &Env, namespace: &str, name: &str) -> Result<IdFromNameOutput, String> {
    let ns = env.durable_object(namespace).map_err(to_err)?;
    let id = ns.id_from_name(name).map_err(to_err)?;
    Ok(IdFromNameOutput { id: id.to_string() })
}

/// Get Durable Object By ID
pub async fn id_from_string(env: &Env, namespace: &str, id: &str) -> Result<IdFromStringOutput, String> {
    let ns = env.durable_object(namespace).map_err(to_err)?;
    let obj_id = ns.id_from_string(id).map_err(to_err)?;
    Ok(IdFromStringOutput { id: obj_id.to_string() })
}

/// Generate Unique Durable Object ID
pub async fn new_unique_id(
    env: &Env,
    namespace: &str,
    _jurisdiction: Option<&str>,
) -> Result<NewUniqueIdOutput, String> {
    let ns = env.durable_object(namespace).map_err(to_err)?;
    let id = ns.unique_id().map_err(to_err)?;
    Ok(NewUniqueIdOutput { id: id.to_string() })
}

/// Get Durable Object Stub
pub async fn get_stub(env: &Env, namespace: &str, id: &str) -> Result<GetStubOutput, String> {
    let ns = env.durable_object(namespace).map_err(to_err)?;
    let obj_id = ns.id_from_string(id).map_err(to_err)?;
    let _stub = obj_id.get_stub().map_err(to_err)?;

    Ok(GetStubOutput {
        stub: DurableObjectStub {
            id: id.to_string(),
            name: None,
        },
    })
}

/// Send Fetch To Durable Object
#[allow(clippy::too_many_arguments)]
pub async fn fetch(
    env: &Env,
    namespace: &str,
    id: &str,
    url: &str,
    method: Option<&str>,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
) -> Result<FetchOutput, String> {
    let ns = env.durable_object(namespace).map_err(to_err)?;
    let obj_id = ns.id_from_string(id).map_err(to_err)?;
    let stub = obj_id.get_stub().map_err(to_err)?;

    let mut req_init = worker::RequestInit::new();
    req_init.method = match method.unwrap_or("GET").to_uppercase().as_str() {
        "GET" => worker::Method::Get,
        "POST" => worker::Method::Post,
        "PUT" => worker::Method::Put,
        "DELETE" => worker::Method::Delete,
        "PATCH" => worker::Method::Patch,
        "HEAD" => worker::Method::Head,
        "OPTIONS" => worker::Method::Options,
        _ => worker::Method::Get,
    };

    if let Some(hdrs) = &headers {
        let worker_headers = worker::Headers::new();
        for (k, v) in hdrs {
            worker_headers.set(k, v).map_err(to_err)?;
        }
        req_init.headers = worker_headers;
    }

    if let Some(b) = &body {
        let body_str = serde_json::to_string(b).map_err(|e| format!("DO body serialization error: {e}"))?;
        req_init.body = Some(JsValue::from_str(&body_str));
    }

    let request = worker::Request::new_with_init(url, &req_init).map_err(to_err)?;
    let mut response = stub.fetch_with_request(request).await.map_err(to_err)?;

    let status_code = response.status_code() as i32;
    let resp_headers: HashMap<String, String> = response.headers().entries().map(|(k, v)| (k, v)).collect();

    let body_text = response.text().await.map_err(to_err)?;
    let body_json = serde_json::from_str(&body_text).unwrap_or(serde_json::Value::String(body_text));

    Ok(FetchOutput {
        body: body_json,
        headers: resp_headers,
        status_code,
    })
}

// ---- Storage Operations ----
// The following operations require a `State` object which is only available
// inside a Durable Object handler. They use raw JS interop via the
// state's storage API.

/// Get Durable Object Storage Value
///
/// Note: Storage operations require a Durable Object `State` context.
/// These must be called from within a Durable Object implementation.
pub async fn storage_get(_key: &str) -> Result<StorageGetOutput, String> {
    Err("storage_get requires a Durable Object State context. \
         Call state.storage().get(key) from within a Durable Object."
        .to_string())
}

/// Get Durable Object Storage Values
pub async fn storage_get_multiple(_keys: Vec<String>) -> Result<StorageGetMultipleOutput, String> {
    Err("storage_get_multiple requires a Durable Object State context. \
         Call state.storage().get_multiple(keys) from within a Durable Object."
        .to_string())
}

/// Put Durable Object Storage Value
pub async fn storage_put(_key: &str, _value: serde_json::Value) -> Result<StoragePutOutput, String> {
    Err("storage_put requires a Durable Object State context. \
         Call state.storage().put(key, value) from within a Durable Object."
        .to_string())
}

/// Put Durable Object Storage Values
pub async fn storage_put_multiple(_entries: serde_json::Value) -> Result<StoragePutMultipleOutput, String> {
    Err("storage_put_multiple requires a Durable Object State context. \
         Call state.storage().put_multiple(entries) from within a Durable Object."
        .to_string())
}

/// Delete Durable Object Storage Value
pub async fn storage_delete(_key: &str) -> Result<StorageDeleteOutput, String> {
    Err("storage_delete requires a Durable Object State context. \
         Call state.storage().delete(key) from within a Durable Object."
        .to_string())
}

/// Delete Durable Object Storage Values
pub async fn storage_delete_multiple(_keys: Vec<String>) -> Result<StorageDeleteMultipleOutput, String> {
    Err("storage_delete_multiple requires a Durable Object State context. \
         Call state.storage().delete_multiple(keys) from within a Durable Object."
        .to_string())
}

/// Delete All Durable Object Storage
pub async fn storage_delete_all() -> Result<StorageDeleteAllOutput, String> {
    Err("storage_delete_all requires a Durable Object State context. \
         Call state.storage().delete_all() from within a Durable Object."
        .to_string())
}

/// List Durable Object Storage Keys
pub async fn storage_list(
    _prefix: Option<&str>,
    _start: Option<&str>,
    _end: Option<&str>,
    _limit: Option<i32>,
    _reverse: Option<bool>,
) -> Result<StorageListOutput, String> {
    Err("storage_list requires a Durable Object State context. \
         Call state.storage().list() from within a Durable Object."
        .to_string())
}

/// Execute Durable Object Storage Transaction
pub async fn storage_transaction(_closure: serde_json::Value) -> Result<StorageTransactionOutput, String> {
    Err("storage_transaction requires a Durable Object State context. \
         Call state.storage().transaction() from within a Durable Object."
        .to_string())
}

/// Execute Durable Object SQL
pub async fn sql_exec(_sql: &str, _params: Option<Vec<serde_json::Value>>) -> Result<SqlExecOutput, String> {
    Err("sql_exec requires a Durable Object State context. \
         Call state.storage().sql().exec() from within a Durable Object."
        .to_string())
}

/// Set Durable Object Alarm
pub async fn set_alarm(_scheduled_time: &str) -> Result<SetAlarmOutput, String> {
    Err("set_alarm requires a Durable Object State context. \
         Call state.storage().set_alarm() from within a Durable Object."
        .to_string())
}

/// Get Durable Object Alarm
pub async fn get_alarm() -> Result<GetAlarmOutput, String> {
    Err("get_alarm requires a Durable Object State context. \
         Call state.storage().get_alarm() from within a Durable Object."
        .to_string())
}

/// Delete Durable Object Alarm
pub async fn delete_alarm() -> Result<DeleteAlarmOutput, String> {
    Err("delete_alarm requires a Durable Object State context. \
         Call state.storage().delete_alarm() from within a Durable Object."
        .to_string())
}

/// Accept Durable Object WebSocket
pub async fn accept_websocket(_tags: Option<Vec<String>>) -> Result<AcceptWebsocketOutput, String> {
    Err("accept_websocket requires a Durable Object State context. \
         Call state.accept_web_socket() from within a Durable Object."
        .to_string())
}

/// Get Durable Object WebSockets
pub async fn get_websockets(_tag: Option<&str>) -> Result<GetWebsocketsOutput, String> {
    Err("get_websockets requires a Durable Object State context. \
         Call state.get_websockets() from within a Durable Object."
        .to_string())
}

/// Set Durable Object WebSocket Auto Response
pub async fn set_websocket_auto_response(
    _response: &str,
    _request: Option<&str>,
) -> Result<SetWebsocketAutoResponseOutput, String> {
    Err("set_websocket_auto_response requires a Durable Object State context. \
         Call state.set_websocket_auto_response() from within a Durable Object."
        .to_string())
}

/// Hibernate Durable Object WebSocket
pub async fn hibernate_websocket(_websocket: WebSocket) -> Result<HibernateWebsocketOutput, String> {
    Err("hibernate_websocket requires a Durable Object State context.".to_string())
}
