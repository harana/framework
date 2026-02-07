// Harana Actions - HTTP Response Module
// This module provides HTTP response generation actions and functionality.

pub mod output;

use std::collections::HashMap;
use handlebars::Handlebars;
use serde_json::{json, Value};
use output::*;

/// Helper function to create an HttpResponse
fn create_response(
    status_code: i32,
    content_type: &str,
    body: Option<Value>,
    extra_headers: Option<HashMap<String, String>>,
) -> HttpResponse {
    let mut headers = extra_headers.unwrap_or_default();
    headers.insert("Content-Type".to_string(), content_type.to_string());
    
    HttpResponse {
        status_code,
        headers,
        body,
        content_type: content_type.to_string(),
    }
}

/// Render Template using Handlebars templating engine.
pub async fn render_template(
    template: &str,
    content_type: Option<&str>,
    data: Option<HashMap<String, Value>>,
    _template_engine: Option<&str>,
) -> Result<RenderTemplateOutput, String> {
    let handlebars = Handlebars::new();
    let data = data.unwrap_or_default();
    
    let rendered = handlebars
        .render_template(template, &data)
        .map_err(|e| format!("Template rendering failed: {}", e))?;
    
    let _content_type = content_type.unwrap_or("text/html");
    
    Ok(RenderTemplateOutput {
        rendered,
        success: true,
    })
}

/// Render Template From File using Handlebars templating engine.
pub async fn render_template_file(
    template_path: &str,
    content_type: Option<&str>,
    data: Option<HashMap<String, Value>>,
    _template_engine: Option<&str>,
) -> Result<RenderTemplateFileOutput, String> {
    let template_content = std::fs::read_to_string(template_path)
        .map_err(|e| format!("Failed to read template file: {}", e))?;
    
    let handlebars = Handlebars::new();
    let data = data.unwrap_or_default();
    
    let rendered = handlebars
        .render_template(&template_content, &data)
        .map_err(|e| format!("Template rendering failed: {}", e))?;
    
    let _content_type = content_type.unwrap_or("text/html");
    
    Ok(RenderTemplateFileOutput {
        rendered,
        success: true,
    })
}

/// Return JSON Response
pub async fn json(
    data: Value,
    pretty: Option<bool>,
    status_code: Option<i32>,
) -> Result<JsonOutput, String> {
    let status_code = status_code.unwrap_or(200);
    let pretty = pretty.unwrap_or(false);
    
    let body = if pretty {
        Value::String(serde_json::to_string_pretty(&data)
            .map_err(|e| format!("JSON serialization failed: {}", e))?)
    } else {
        data
    };
    
    let response = create_response(status_code, "application/json", Some(body), None);
    
    Ok(JsonOutput {
        response,
        success: true,
    })
}

/// Return Plain Text Response
pub async fn text(
    text: &str,
    content_type: Option<&str>,
    status_code: Option<i32>,
) -> Result<TextOutput, String> {
    let status_code = status_code.unwrap_or(200);
    let content_type = content_type.unwrap_or("text/plain");
    
    let response = create_response(
        status_code,
        content_type,
        Some(Value::String(text.to_string())),
        None,
    );
    
    Ok(TextOutput {
        response,
        success: true,
    })
}

/// Return HTML Response
pub async fn html(
    html: &str,
    status_code: Option<i32>,
) -> Result<HtmlOutput, String> {
    let status_code = status_code.unwrap_or(200);
    
    let response = create_response(
        status_code,
        "text/html",
        Some(Value::String(html.to_string())),
        None,
    );
    
    Ok(HtmlOutput {
        response,
        success: true,
    })
}

/// Return XML Response
pub async fn xml(
    xml: &str,
    status_code: Option<i32>,
) -> Result<XmlOutput, String> {
    let status_code = status_code.unwrap_or(200);
    
    let response = create_response(
        status_code,
        "application/xml",
        Some(Value::String(xml.to_string())),
        None,
    );
    
    Ok(XmlOutput {
        response,
        success: true,
    })
}

/// Return Binary Response
pub async fn binary(
    content_type: &str,
    data: &[u8],
    filename: Option<&str>,
    status_code: Option<i32>,
) -> Result<BinaryOutput, String> {
    let status_code = status_code.unwrap_or(200);
    
    let mut headers = HashMap::new();
    if let Some(filename) = filename {
        headers.insert(
            "Content-Disposition".to_string(),
            format!("attachment; filename=\"{}\"", filename),
        );
    }
    headers.insert("Content-Length".to_string(), data.len().to_string());
    
    // Encode binary data as base64 for JSON representation
    let encoded = base64_encode(data);
    let response = create_response(
        status_code,
        content_type,
        Some(Value::String(encoded)),
        Some(headers),
    );
    
    Ok(BinaryOutput {
        response,
        success: true,
    })
}

/// Simple base64 encoding for binary data
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
        let b2 = chunk.get(2).copied().unwrap_or(0) as usize;
        
        result.push(ALPHABET[b0 >> 2] as char);
        result.push(ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);
        
        if chunk.len() > 1 {
            result.push(ALPHABET[((b1 & 0x0F) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(ALPHABET[b2 & 0x3F] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}

/// Stream SSE Event - Format a single Server-Sent Event
pub async fn sse_event(
    data: Value,
    event: Option<&str>,
    id: Option<&str>,
    retry: Option<i32>,
) -> Result<SseEventOutput, String> {
    let mut formatted = String::new();
    
    if let Some(id) = id {
        formatted.push_str(&format!("id: {}\n", id));
    }
    
    if let Some(event) = event {
        formatted.push_str(&format!("event: {}\n", event));
    }
    
    if let Some(retry) = retry {
        formatted.push_str(&format!("retry: {}\n", retry));
    }
    
    let data_str = match &data {
        Value::String(s) => s.clone(),
        _ => serde_json::to_string(&data)
            .map_err(|e| format!("Failed to serialize SSE data: {}", e))?,
    };
    
    // Handle multi-line data
    for line in data_str.lines() {
        formatted.push_str(&format!("data: {}\n", line));
    }
    
    formatted.push('\n');
    
    Ok(SseEventOutput {
        formatted,
        success: true,
    })
}

/// Stream SSE Events - Format multiple Server-Sent Events
pub async fn sse_stream(
    events: Vec<SseEvent>,
    _keep_alive_interval: Option<i32>,
) -> Result<SseStreamOutput, String> {
    let mut formatted_events = Vec::with_capacity(events.len());
    
    for event in events {
        let result = sse_event(
            event.data,
            event.event.as_deref(),
            event.id.as_deref(),
            event.retry,
        ).await?;
        formatted_events.push(result.formatted);
    }
    
    Ok(SseStreamOutput {
        events: formatted_events,
        success: true,
    })
}

/// Return 404 Not Found
pub async fn not_found(
    custom_data: Option<HashMap<String, Value>>,
    include_body: Option<bool>,
    message: Option<&str>,
) -> Result<NotFoundOutput, String> {
    let message = message.unwrap_or("Not Found");
    let include_body = include_body.unwrap_or(true);
    
    let body = if include_body {
        let mut body_data = json!({
            "error": "Not Found",
            "message": message,
            "status_code": 404
        });
        
        if let Some(custom) = custom_data {
            if let Value::Object(ref mut map) = body_data {
                for (k, v) in custom {
                    map.insert(k, v);
                }
            }
        }
        
        Some(body_data)
    } else {
        None
    };
    
    let response = create_response(404, "application/json", body, None);
    
    Ok(NotFoundOutput {
        response,
        success: true,
    })
}

/// Return 400 Bad Request
pub async fn bad_request(
    custom_data: Option<HashMap<String, Value>>,
    errors: Option<Vec<String>>,
    message: Option<&str>,
) -> Result<BadRequestOutput, String> {
    let message = message.unwrap_or("Bad Request");
    
    let mut body_data = json!({
        "error": "Bad Request",
        "message": message,
        "status_code": 400
    });
    
    if let Some(errors) = errors {
        if let Value::Object(ref mut map) = body_data {
            map.insert("errors".to_string(), json!(errors));
        }
    }
    
    if let Some(custom) = custom_data {
        if let Value::Object(ref mut map) = body_data {
            for (k, v) in custom {
                map.insert(k, v);
            }
        }
    }
    
    let response = create_response(400, "application/json", Some(body_data), None);
    
    Ok(BadRequestOutput {
        response,
        success: true,
    })
}

/// Return 401 Unauthorized
pub async fn unauthorized(
    custom_data: Option<HashMap<String, Value>>,
    message: Option<&str>,
    www_authenticate: Option<&str>,
) -> Result<UnauthorizedOutput, String> {
    let message = message.unwrap_or("Unauthorized");
    
    let mut body_data = json!({
        "error": "Unauthorized",
        "message": message,
        "status_code": 401
    });
    
    if let Some(custom) = custom_data {
        if let Value::Object(ref mut map) = body_data {
            for (k, v) in custom {
                map.insert(k, v);
            }
        }
    }
    
    let mut headers = HashMap::new();
    if let Some(www_auth) = www_authenticate {
        headers.insert("WWW-Authenticate".to_string(), www_auth.to_string());
    }
    
    let response = create_response(401, "application/json", Some(body_data), Some(headers));
    
    Ok(UnauthorizedOutput {
        response,
        success: true,
    })
}

/// Return 403 Forbidden
pub async fn forbidden(
    custom_data: Option<HashMap<String, Value>>,
    message: Option<&str>,
) -> Result<ForbiddenOutput, String> {
    let message = message.unwrap_or("Forbidden");
    
    let mut body_data = json!({
        "error": "Forbidden",
        "message": message,
        "status_code": 403
    });
    
    if let Some(custom) = custom_data {
        if let Value::Object(ref mut map) = body_data {
            for (k, v) in custom {
                map.insert(k, v);
            }
        }
    }
    
    let response = create_response(403, "application/json", Some(body_data), None);
    
    Ok(ForbiddenOutput {
        response,
        success: true,
    })
}

/// Return 500 Internal Server Error
pub async fn internal_error(
    custom_data: Option<HashMap<String, Value>>,
    error: Option<&str>,
    include_details: Option<bool>,
    message: Option<&str>,
) -> Result<InternalErrorOutput, String> {
    let message = message.unwrap_or("Internal Server Error");
    let include_details = include_details.unwrap_or(false);
    
    let mut body_data = json!({
        "error": "Internal Server Error",
        "message": message,
        "status_code": 500
    });
    
    if include_details {
        if let Some(error) = error {
            if let Value::Object(ref mut map) = body_data {
                map.insert("details".to_string(), json!(error));
            }
        }
    }
    
    if let Some(custom) = custom_data {
        if let Value::Object(ref mut map) = body_data {
            for (k, v) in custom {
                map.insert(k, v);
            }
        }
    }
    
    let response = create_response(500, "application/json", Some(body_data), None);
    
    Ok(InternalErrorOutput {
        response,
        success: true,
    })
}

/// Return 502 Bad Gateway
pub async fn bad_gateway(
    custom_data: Option<HashMap<String, Value>>,
    message: Option<&str>,
) -> Result<BadGatewayOutput, String> {
    let message = message.unwrap_or("Bad Gateway");
    
    let mut body_data = json!({
        "error": "Bad Gateway",
        "message": message,
        "status_code": 502
    });
    
    if let Some(custom) = custom_data {
        if let Value::Object(ref mut map) = body_data {
            for (k, v) in custom {
                map.insert(k, v);
            }
        }
    }
    
    let response = create_response(502, "application/json", Some(body_data), None);
    
    Ok(BadGatewayOutput {
        response,
        success: true,
    })
}

/// Return 503 Service Unavailable
pub async fn service_unavailable(
    custom_data: Option<HashMap<String, Value>>,
    message: Option<&str>,
    retry_after: Option<i32>,
) -> Result<ServiceUnavailableOutput, String> {
    let message = message.unwrap_or("Service Unavailable");
    
    let mut body_data = json!({
        "error": "Service Unavailable",
        "message": message,
        "status_code": 503
    });
    
    if let Some(custom) = custom_data {
        if let Value::Object(ref mut map) = body_data {
            for (k, v) in custom {
                map.insert(k, v);
            }
        }
    }
    
    let mut headers = HashMap::new();
    if let Some(retry) = retry_after {
        headers.insert("Retry-After".to_string(), retry.to_string());
    }
    
    let response = create_response(503, "application/json", Some(body_data), Some(headers));
    
    Ok(ServiceUnavailableOutput {
        response,
        success: true,
    })
}

/// Return Custom Error Response
pub async fn error(
    details: Option<Value>,
    error_code: Option<&str>,
    message: &str,
    status_code: i32,
) -> Result<ErrorOutput, String> {
    let mut body_data = json!({
        "error": true,
        "message": message,
        "status_code": status_code
    });
    
    if let Value::Object(ref mut map) = body_data {
        if let Some(code) = error_code {
            map.insert("error_code".to_string(), json!(code));
        }
        if let Some(details) = details {
            map.insert("details".to_string(), details);
        }
    }
    
    let response = create_response(status_code, "application/json", Some(body_data), None);
    
    Ok(ErrorOutput {
        response,
        success: true,
    })
}

/// Set Response Cookie
pub async fn set_cookie(
    name: &str,
    value: &str,
    options: Option<CookieOptions>,
) -> Result<SetCookieOutput, String> {
    let options = options.unwrap_or_default();
    
    let mut cookie_parts = vec![format!("{}={}", name, value)];
    
    if let Some(path) = &options.path {
        cookie_parts.push(format!("Path={}", path));
    }
    
    if let Some(domain) = &options.domain {
        cookie_parts.push(format!("Domain={}", domain));
    }
    
    if let Some(max_age) = options.max_age {
        cookie_parts.push(format!("Max-Age={}", max_age));
    }
    
    if let Some(expires) = options.expires {
        // Format as HTTP date
        let datetime = chrono::DateTime::from_timestamp(expires, 0)
            .unwrap_or_else(|| chrono::Utc::now());
        cookie_parts.push(format!("Expires={}", datetime.format("%a, %d %b %Y %H:%M:%S GMT")));
    }
    
    if options.secure {
        cookie_parts.push("Secure".to_string());
    }
    
    if options.http_only {
        cookie_parts.push("HttpOnly".to_string());
    }
    
    if let Some(same_site) = &options.same_site {
        cookie_parts.push(format!("SameSite={}", same_site));
    }
    
    let header_value = cookie_parts.join("; ");
    
    Ok(SetCookieOutput {
        header_value,
        success: true,
    })
}

/// Delete Response Cookie
pub async fn delete_cookie(
    name: &str,
    domain: Option<&str>,
    path: Option<&str>,
) -> Result<DeleteCookieOutput, String> {
    let path = path.unwrap_or("/");
    
    let mut cookie_parts = vec![
        format!("{}=", name),
        format!("Path={}", path),
        "Max-Age=0".to_string(),
        "Expires=Thu, 01 Jan 1970 00:00:00 GMT".to_string(),
    ];
    
    if let Some(domain) = domain {
        cookie_parts.push(format!("Domain={}", domain));
    }
    
    let header_value = cookie_parts.join("; ");
    
    Ok(DeleteCookieOutput {
        header_value,
        success: true,
    })
}

/// Return File Download Response
pub async fn file_download(
    file_path: &str,
    content_type: Option<&str>,
    filename: Option<&str>,
    inline: Option<bool>,
) -> Result<FileDownloadOutput, String> {
    let data = std::fs::read(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let filename = filename.unwrap_or_else(|| {
        std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("download")
    });
    
    let content_type = content_type.unwrap_or("application/octet-stream");
    let inline = inline.unwrap_or(false);
    
    let disposition = if inline {
        format!("inline; filename=\"{}\"", filename)
    } else {
        format!("attachment; filename=\"{}\"", filename)
    };
    
    let mut headers = HashMap::new();
    headers.insert("Content-Disposition".to_string(), disposition);
    headers.insert("Content-Length".to_string(), data.len().to_string());
    
    let encoded = base64_encode(&data);
    let response = create_response(
        200,
        content_type,
        Some(Value::String(encoded)),
        Some(headers),
    );
    
    Ok(FileDownloadOutput {
        response,
        success: true,
    })
}

/// Return Empty Response
pub async fn empty(
    status_code: Option<i32>,
) -> Result<EmptyOutput, String> {
    let status_code = status_code.unwrap_or(204);
    
    let response = create_response(status_code, "text/plain", None, None);
    
    Ok(EmptyOutput {
        response,
        success: true,
    })
}

/// Redirect To URL
pub async fn redirect(
    url: &str,
    preserve_method: Option<bool>,
    status_code: Option<i32>,
) -> Result<RedirectOutput, String> {
    let preserve_method = preserve_method.unwrap_or(false);
    
    // Default status codes based on preserve_method
    let status_code = status_code.unwrap_or(if preserve_method { 307 } else { 302 });
    
    let mut headers = HashMap::new();
    headers.insert("Location".to_string(), url.to_string());
    
    let response = create_response(status_code, "text/plain", None, Some(headers));
    
    Ok(RedirectOutput {
        response,
        success: true,
    })
}

/// Return CORS Preflight Response
pub async fn cors_preflight(
    allowed_headers: Option<Vec<String>>,
    allowed_methods: Option<Vec<String>>,
    allowed_origins: Option<Vec<String>>,
    max_age: Option<i32>,
) -> Result<CorsPreflightOutput, String> {
    let allowed_headers = allowed_headers.unwrap_or_else(|| vec!["*".to_string()]);
    let allowed_methods = allowed_methods.unwrap_or_else(|| {
        vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]
            .into_iter()
            .map(String::from)
            .collect()
    });
    let allowed_origins = allowed_origins.unwrap_or_else(|| vec!["*".to_string()]);
    let max_age = max_age.unwrap_or(86400);
    
    let mut headers = HashMap::new();
    headers.insert(
        "Access-Control-Allow-Origin".to_string(),
        allowed_origins.join(", "),
    );
    headers.insert(
        "Access-Control-Allow-Methods".to_string(),
        allowed_methods.join(", "),
    );
    headers.insert(
        "Access-Control-Allow-Headers".to_string(),
        allowed_headers.join(", "),
    );
    headers.insert("Access-Control-Max-Age".to_string(), max_age.to_string());
    
    let response = create_response(204, "text/plain", None, Some(headers));
    
    Ok(CorsPreflightOutput {
        response,
        success: true,
    })
}

/// Return GraphQL Response
pub async fn graphql(
    data: Option<Value>,
    errors: Option<Vec<GraphQLError>>,
    extensions: Option<HashMap<String, Value>>,
) -> Result<GraphqlOutput, String> {
    let mut body = serde_json::Map::new();
    
    if let Some(data) = data {
        body.insert("data".to_string(), data);
    } else {
        body.insert("data".to_string(), Value::Null);
    }
    
    if let Some(errors) = errors {
        let errors_json: Vec<Value> = errors
            .into_iter()
            .map(|e| {
                let mut error_obj = serde_json::Map::new();
                error_obj.insert("message".to_string(), json!(e.message));
                if let Some(locations) = e.locations {
                    error_obj.insert("locations".to_string(), json!(locations));
                }
                if let Some(path) = e.path {
                    error_obj.insert("path".to_string(), json!(path));
                }
                if let Some(extensions) = e.extensions {
                    error_obj.insert("extensions".to_string(), json!(extensions));
                }
                Value::Object(error_obj)
            })
            .collect();
        body.insert("errors".to_string(), json!(errors_json));
    }
    
    if let Some(extensions) = extensions {
        body.insert("extensions".to_string(), json!(extensions));
    }
    
    let response = create_response(200, "application/json", Some(Value::Object(body)), None);
    
    Ok(GraphqlOutput {
        response,
        success: true,
    })
}

/// Return Paginated Response
pub async fn paginated(
    data: Vec<Value>,
    include_metadata: Option<bool>,
    page: Option<i32>,
    page_size: Option<i32>,
    total: i32,
) -> Result<PaginatedOutput, String> {
    let include_metadata = include_metadata.unwrap_or(true);
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(20);
    
    let total_pages = (total as f64 / page_size as f64).ceil() as i32;
    let has_next = page < total_pages;
    let has_previous = page > 1;
    
    let body = if include_metadata {
        json!({
            "data": data,
            "metadata": {
                "page": page,
                "page_size": page_size,
                "total": total,
                "total_pages": total_pages,
                "has_next": has_next,
                "has_previous": has_previous
            }
        })
    } else {
        json!({ "data": data })
    };
    
    let response = create_response(200, "application/json", Some(body), None);
    
    Ok(PaginatedOutput {
        response,
        success: true,
    })
}

/// Return CSV Response
pub async fn csv(
    data: Vec<HashMap<String, Value>>,
    delimiter: Option<&str>,
    filename: Option<&str>,
    include_headers: Option<bool>,
) -> Result<CsvOutput, String> {
    let delimiter = delimiter.unwrap_or(",");
    let filename = filename.unwrap_or("data.csv");
    let include_headers = include_headers.unwrap_or(true);
    
    if data.is_empty() {
        let mut headers = HashMap::new();
        headers.insert(
            "Content-Disposition".to_string(),
            format!("attachment; filename=\"{}\"", filename),
        );
        
        let response = create_response(
            200,
            "text/csv",
            Some(Value::String(String::new())),
            Some(headers),
        );
        
        return Ok(CsvOutput {
            response,
            success: true,
        });
    }
    
    // Collect all unique column names
    let mut columns: Vec<String> = data
        .iter()
        .flat_map(|row| row.keys().cloned())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    columns.sort();
    
    let mut csv_content = String::new();
    
    // Write headers
    if include_headers {
        csv_content.push_str(&columns.join(delimiter));
        csv_content.push('\n');
    }
    
    // Write rows
    for row in &data {
        let values: Vec<String> = columns
            .iter()
            .map(|col| {
                row.get(col)
                    .map(|v| match v {
                        Value::String(s) => {
                            if s.contains(delimiter) || s.contains('"') || s.contains('\n') {
                                format!("\"{}\"", s.replace('"', "\"\""))
                            } else {
                                s.clone()
                            }
                        }
                        Value::Null => String::new(),
                        other => other.to_string(),
                    })
                    .unwrap_or_default()
            })
            .collect();
        csv_content.push_str(&values.join(delimiter));
        csv_content.push('\n');
    }
    
    let mut headers = HashMap::new();
    headers.insert(
        "Content-Disposition".to_string(),
        format!("attachment; filename=\"{}\"", filename),
    );
    
    let response = create_response(
        200,
        "text/csv",
        Some(Value::String(csv_content)),
        Some(headers),
    );
    
    Ok(CsvOutput {
        response,
        success: true,
    })
}

/// Set Response Header (helper function)
pub async fn set_header(
    name: &str,
    value: &str,
) -> Result<SetHeaderOutput, String> {
    Ok(SetHeaderOutput {
        name: name.to_string(),
        value: value.to_string(),
        success: true,
    })
}

#[cfg(test)]
mod tests;
