// Harana Actions - Cloudflare Worker Module
// This module provides Cloudflare Worker actions for invoking workers, managing
// service bindings, environment variables, secrets, and cron triggers.

pub mod output;

use output::*;
use std::collections::HashMap;
use worker::Env;

fn to_err(e: worker::Error) -> String {
    format!("Worker error: {e}")
}

/// Helper to perform a fetch via a Fetcher (service binding)
async fn do_fetch(
    fetcher: &worker::Fetcher,
    url: &str,
    method: Option<&str>,
    headers: Option<&HashMap<String, String>>,
    body: Option<&serde_json::Value>,
) -> Result<(serde_json::Value, HashMap<String, String>, i32), String> {
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

    if let Some(hdrs) = headers {
        let worker_headers = worker::Headers::new();
        for (k, v) in hdrs {
            worker_headers.set(k, v).map_err(to_err)?;
        }
        req_init.headers = worker_headers;
    }

    if let Some(b) = body {
        let body_str = serde_json::to_string(b).map_err(|e| format!("Worker body serialization error: {e}"))?;
        req_init.body = Some(wasm_bindgen::JsValue::from_str(&body_str));
    }

    let request = worker::Request::new_with_init(url, &req_init).map_err(to_err)?;
    let http_response = fetcher.fetch_request(request).await.map_err(to_err)?;
    let mut response: worker::Response = http_response.try_into().map_err(to_err)?;

    let status_code = response.status_code() as i32;
    let resp_headers: HashMap<String, String> = response.headers().entries().map(|(k, v)| (k, v)).collect();

    let body_text = response.text().await.map_err(to_err)?;
    let body_json = serde_json::from_str(&body_text).unwrap_or(serde_json::Value::String(body_text));

    Ok((body_json, resp_headers, status_code))
}

/// Invoke Worker Fetch
pub async fn fetch(
    env: &Env,
    service_binding: &str,
    url: &str,
    method: Option<&str>,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
) -> Result<FetchOutput, String> {
    let fetcher = env.service(service_binding).map_err(to_err)?;
    let (body_json, resp_headers, status_code) =
        do_fetch(&fetcher, url, method, headers.as_ref(), body.as_ref()).await?;

    Ok(FetchOutput {
        body: body_json,
        headers: resp_headers,
        status_code,
    })
}

/// Get Worker Environment Variable
pub async fn get_var(env: &Env, name: &str) -> Result<GetVarOutput, String> {
    let var = env.var(name).map_err(to_err)?;
    Ok(GetVarOutput { value: var.to_string() })
}

/// Get Worker Secret
pub async fn get_secret(env: &Env, name: &str) -> Result<GetSecretOutput, String> {
    let secret = env.secret(name).map_err(to_err)?;
    Ok(GetSecretOutput {
        value: secret.to_string(),
    })
}

/// Schedule Worker Cron Trigger
///
/// Note: Cron triggers are configured in wrangler.toml and handled via `#[event(scheduled)]`.
/// This function returns metadata about the requested schedule but cannot programmatically
/// create cron triggers at runtime.
pub async fn scheduled(_env: &Env, _service_binding: &str, cron: &str) -> Result<ScheduledOutput, String> {
    Ok(ScheduledOutput {
        scheduled_time: cron.to_string(),
        success: true,
    })
}

/// Worker Wait Until
///
/// Note: `waitUntil` extends the lifetime of the request handler. It requires
/// a Context object, which is only available in request handlers.
pub async fn wait_until(_promise: serde_json::Value) -> Result<WaitUntilOutput, String> {
    Err("waitUntil requires a Context object from a request handler. \
         Use ctx.wait_until() in your fetch handler."
        .to_string())
}

/// Worker Pass Through
///
/// Note: Pass-through functionality forwards requests to the origin.
/// This requires an active request context.
pub async fn pass_through(_request: serde_json::Value) -> Result<PassThroughOutput, String> {
    Err("pass_through requires an active request context to forward to origin.".to_string())
}

/// Fetch External Service Binding
pub async fn service_binding_fetch(
    env: &Env,
    service: &str,
    url: &str,
    method: Option<&str>,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
) -> Result<ServiceBindingFetchOutput, String> {
    let fetcher = env.service(service).map_err(to_err)?;
    let (body_json, resp_headers, status_code) =
        do_fetch(&fetcher, url, method, headers.as_ref(), body.as_ref()).await?;

    Ok(ServiceBindingFetchOutput {
        body: body_json,
        headers: resp_headers,
        status_code,
    })
}

/// Get Worker Version Metadata
pub async fn get_version(env: &Env, binding: &str) -> Result<GetVersionOutput, String> {
    let version: worker::WorkerVersionMetadata = env.get_binding(binding).map_err(to_err)?;

    Ok(GetVersionOutput {
        id: version.id(),
        message: String::new(),
        tag: version.tag(),
        timestamp: version.timestamp(),
    })
}
