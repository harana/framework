// Harana Actions - Cloudflare mTLS Module
// This module provides Cloudflare mTLS actions for making HTTP requests
// with mutual TLS client certificates.

pub mod output;

use output::*;
use std::collections::HashMap;
use worker::Env;

fn to_err(e: worker::Error) -> String {
    format!("mTLS error: {e}")
}

/// Fetch With mTLS Client Certificate
pub async fn fetch(
    env: &Env,
    certificate_binding: &str,
    url: &str,
    method: Option<&str>,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
) -> Result<FetchOutput, String> {
    let fetcher: worker::Fetcher = env.get_binding(certificate_binding).map_err(to_err)?;

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
        let body_str = serde_json::to_string(b).map_err(|e| format!("mTLS body serialization error: {e}"))?;
        req_init.body = Some(wasm_bindgen::JsValue::from_str(&body_str));
    }

    let request = worker::Request::new_with_init(url, &req_init).map_err(to_err)?;
    let http_response = fetcher.fetch_request(request).await.map_err(to_err)?;
    let mut response: worker::Response = http_response.try_into().map_err(to_err)?;

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
