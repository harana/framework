pub mod output;

use output::*;
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Duration;

/// Creates a reqwest client with the specified timeout.
fn create_client(timeout_secs: i32) -> Result<Client, String> {
    Client::builder()
        .timeout(Duration::from_secs(timeout_secs as u64))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))
}

/// Converts HashMap headers to reqwest HeaderMap.
fn build_headers(headers: Option<HashMap<String, String>>) -> reqwest::header::HeaderMap {
    let mut header_map = reqwest::header::HeaderMap::new();
    if let Some(h) = headers {
        for (key, value) in h {
            if let (Ok(name), Ok(val)) = (
                reqwest::header::HeaderName::from_bytes(key.as_bytes()),
                reqwest::header::HeaderValue::from_str(&value),
            ) {
                header_map.insert(name, val);
            }
        }
    }
    header_map
}

/// Extracts response headers as a HashMap.
fn extract_headers(headers: &reqwest::header::HeaderMap) -> HashMap<String, String> {
    headers
        .iter()
        .filter_map(|(name, value)| {
            value.to_str().ok().map(|v| (name.to_string(), v.to_string()))
        })
        .collect()
}

/// Send HTTP GET Request.
pub async fn get(
    url: &str,
    headers: Option<HashMap<String, String>>,
    timeout: Option<i32>,
    query_params: Option<HashMap<String, String>>,
) -> Result<GetOutput, String> {
    let timeout = timeout.unwrap_or(30);
    let client = create_client(timeout)?;
    
    let mut request = client.get(url).headers(build_headers(headers));
    
    if let Some(params) = query_params {
        request = request.query(&params);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("GET request failed: {}", e))?;
    
    let status_code = response.status().as_u16() as i32;
    let response_headers = extract_headers(response.headers());
    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;
    
    Ok(GetOutput {
        body,
        headers: response_headers,
        status_code,
    })
}

/// Send HTTP POST Request.
pub async fn post(
    url: &str,
    body: Option<&str>,
    content_type: Option<&str>,
    headers: Option<HashMap<String, String>>,
    query_params: Option<HashMap<String, String>>,
    timeout: Option<i32>,
) -> Result<PostOutput, String> {
    let timeout = timeout.unwrap_or(30);
    let content_type = content_type.unwrap_or("application/json");
    let client = create_client(timeout)?;
    
    let mut header_map = build_headers(headers);
    if let Ok(ct) = reqwest::header::HeaderValue::from_str(content_type) {
        header_map.insert(reqwest::header::CONTENT_TYPE, ct);
    }
    
    let mut request = client.post(url).headers(header_map);
    
    if let Some(params) = query_params {
        request = request.query(&params);
    }
    
    if let Some(b) = body {
        request = request.body(b.to_string());
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("POST request failed: {}", e))?;
    
    let status_code = response.status().as_u16() as i32;
    let response_headers = extract_headers(response.headers());
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;
    
    Ok(PostOutput {
        body: response_body,
        headers: response_headers,
        status_code,
    })
}

/// Send HTTP PUT Request.
pub async fn put(
    url: &str,
    body: Option<&str>,
    content_type: Option<&str>,
    headers: Option<HashMap<String, String>>,
    query_params: Option<HashMap<String, String>>,
    timeout: Option<i32>,
) -> Result<PutOutput, String> {
    let timeout = timeout.unwrap_or(30);
    let content_type = content_type.unwrap_or("application/json");
    let client = create_client(timeout)?;
    
    let mut header_map = build_headers(headers);
    if let Ok(ct) = reqwest::header::HeaderValue::from_str(content_type) {
        header_map.insert(reqwest::header::CONTENT_TYPE, ct);
    }
    
    let mut request = client.put(url).headers(header_map);
    
    if let Some(params) = query_params {
        request = request.query(&params);
    }
    
    if let Some(b) = body {
        request = request.body(b.to_string());
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("PUT request failed: {}", e))?;
    
    let status_code = response.status().as_u16() as i32;
    let response_headers = extract_headers(response.headers());
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;
    
    Ok(PutOutput {
        body: response_body,
        headers: response_headers,
        status_code,
    })
}

/// Send HTTP PATCH Request.
pub async fn patch(
    url: &str,
    body: Option<&str>,
    content_type: Option<&str>,
    headers: Option<HashMap<String, String>>,
    query_params: Option<HashMap<String, String>>,
    timeout: Option<i32>,
) -> Result<PatchOutput, String> {
    let timeout = timeout.unwrap_or(30);
    let content_type = content_type.unwrap_or("application/json");
    let client = create_client(timeout)?;
    
    let mut header_map = build_headers(headers);
    if let Ok(ct) = reqwest::header::HeaderValue::from_str(content_type) {
        header_map.insert(reqwest::header::CONTENT_TYPE, ct);
    }
    
    let mut request = client.patch(url).headers(header_map);
    
    if let Some(params) = query_params {
        request = request.query(&params);
    }
    
    if let Some(b) = body {
        request = request.body(b.to_string());
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("PATCH request failed: {}", e))?;
    
    let status_code = response.status().as_u16() as i32;
    let response_headers = extract_headers(response.headers());
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;
    
    Ok(PatchOutput {
        body: response_body,
        headers: response_headers,
        status_code,
    })
}

/// Send HTTP DELETE Request.
pub async fn delete(
    url: &str,
    headers: Option<HashMap<String, String>>,
    query_params: Option<HashMap<String, String>>,
    timeout: Option<i32>,
) -> Result<DeleteOutput, String> {
    let timeout = timeout.unwrap_or(30);
    let client = create_client(timeout)?;
    
    let mut request = client.delete(url).headers(build_headers(headers));
    
    if let Some(params) = query_params {
        request = request.query(&params);
    }
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("DELETE request failed: {}", e))?;
    
    let status_code = response.status().as_u16() as i32;
    let response_headers = extract_headers(response.headers());
    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;
    
    Ok(DeleteOutput {
        body,
        headers: response_headers,
        status_code,
    })
}

/// Download File From URL.
pub async fn download(
    url: &str,
    headers: Option<HashMap<String, String>>,
    timeout: Option<i32>,
) -> Result<DownloadOutput, String> {
    let timeout = timeout.unwrap_or(300);
    let client = create_client(timeout)?;
    
    let response = client
        .get(url)
        .headers(build_headers(headers))
        .send()
        .await
        .map_err(|e| format!("Download request failed: {}", e))?;
    
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();
    
    let content = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read download content: {}", e))?
        .to_vec();
    
    let size = content.len() as i32;
    
    Ok(DownloadOutput {
        content,
        content_type,
        size,
    })
}

/// Upload File To URL.
pub async fn upload(
    file: &[u8],
    filename: &str,
    url: &str,
    headers: Option<HashMap<String, String>>,
    timeout: Option<i32>,
) -> Result<UploadOutput, String> {
    let timeout = timeout.unwrap_or(300);
    let client = create_client(timeout)?;
    
    let part = reqwest::multipart::Part::bytes(file.to_vec())
        .file_name(filename.to_string())
        .mime_str("application/octet-stream")
        .map_err(|e| format!("Failed to create multipart: {}", e))?;
    
    let form = reqwest::multipart::Form::new().part("file", part);
    
    let response = client
        .post(url)
        .headers(build_headers(headers))
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Upload request failed: {}", e))?;
    
    let status_code = response.status().as_u16() as i32;
    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read upload response: {}", e))?;
    
    Ok(UploadOutput { body, status_code })
}

/// Send GraphQL Query.
pub async fn graphql_query(
    url: &str,
    query: &str,
    headers: Option<HashMap<String, String>>,
    variables: Option<HashMap<String, Value>>,
    timeout: Option<i32>,
) -> Result<GraphqlQueryOutput, String> {
    let timeout = timeout.unwrap_or(30);
    let client = create_client(timeout)?;
    
    let mut header_map = build_headers(headers);
    header_map.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    
    let body = json!({
        "query": query,
        "variables": variables.unwrap_or_default()
    });
    
    let response = client
        .post(url)
        .headers(header_map)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("GraphQL request failed: {}", e))?;
    
    let response_body: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse GraphQL response: {}", e))?;
    
    let data = response_body.get("data").cloned();
    let errors = response_body
        .get("errors")
        .and_then(|e| e.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|e| {
                    let message = e.get("message")?.as_str()?.to_string();
                    let path = e
                        .get("path")
                        .and_then(|p| p.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default();
                    let locations = e
                        .get("locations")
                        .and_then(|l| l.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|loc| {
                                    Some(GraphqlErrorLocation {
                                        line: loc.get("line")?.as_i64()? as i32,
                                        column: loc.get("column")?.as_i64()? as i32,
                                    })
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                    Some(GraphqlError {
                        message,
                        path,
                        locations,
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    
    Ok(GraphqlQueryOutput { data, errors })
}

#[cfg(test)]
mod tests;
