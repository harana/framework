// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait HttpResponseAction {
    async fn render_template(&self, content_type: String, data: String, template: String, template_engine: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn render_template_file(&self, content_type: String, data: String, template_engine: String, template_path: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn json(&self, data: String, pretty: bool, status_code: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn text(&self, content_type: String, status_code: i64, text: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn html(&self, html: String, status_code: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn xml(&self, status_code: i64, xml: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn binary(&self, content_type: String, data: String, filename: String, status_code: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn sse_event(&self, data: String, event: String, id: String, retry: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn sse_stream(&self, events: String, keep_alive_interval: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn not_found(&self, custom_data: String, include_body: bool, message: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn bad_request(&self, custom_data: String, errors: String, message: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn unauthorized(&self, custom_data: String, message: String, www_authenticate: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn forbidden(&self, custom_data: String, message: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn internal_server_error(&self, custom_data: String, error: String, include_details: bool, message: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn bad_gateway(&self, custom_data: String, message: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn service_unavailable(&self, custom_data: String, message: String, retry_after: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn error(&self, details: String, error_code: String, message: String, status_code: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn set_cookie(&self, domain: String, expires: chrono::DateTime<chrono::Utc>, http_only: bool, max_age: i64, name: String, path: String, same_site: String, secure: bool, value: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_cookie(&self, domain: String, name: String, path: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn file_download(&self, content_type: String, file_path: String, filename: String, inline: bool) -> Result<(), Box<dyn std::error::Error>>;
    async fn streaming(&self, chunk_size: i64, content_type: String, stream_source: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn empty(&self, status_code: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn redirect(&self, preserve_method: bool, status_code: i64, url: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn cors_preflight(&self, allowed_headers: String, allowed_methods: String, allowed_origins: String, max_age: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn graphql(&self, data: String, errors: String, extensions: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn paginated(&self, data: String, include_metadata: bool, page: i64, page_size: i64, total: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn csv(&self, data: String, delimiter: String, filename: String, include_headers: bool) -> Result<(), Box<dyn std::error::Error>>;
}
