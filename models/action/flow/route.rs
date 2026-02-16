// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ForwardOutput {
    pub body: String,
    pub headers: std::collections::HashMap<String, String>,
    pub status_code: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RateLimitOutput {
    pub allowed: bool,
    pub remaining: i64,
    pub reset_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MatchRouteOutput {
    pub matched: bool,
    pub params: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Route {
    pub path: String,
    pub method: String,
    pub handler: String,
    pub middleware: Vec<String>,
    pub params: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteGroup {
    pub middleware: String,
    pub prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteGroupAssignment {
    pub group_id: String,
    pub route_id: String,
}

#[async_trait]
pub trait RouteAction {
    async fn redirect(&self, preserve_query: bool, status_code: i64, url: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn forward(&self, headers: std::collections::HashMap<String, String>, preserve_host: bool, target_url: String, timeout: i64) -> Result<ForwardOutput, Box<dyn std::error::Error>>;
    async fn rewrite_path(&self, pattern: String, replacement: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn add_headers(&self, headers: std::collections::HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_headers(&self, headers: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn set_status(&self, message: String, status_code: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn json_response(&self, body: String, headers: std::collections::HashMap<String, String>, status_code: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn html_response(&self, body: String, headers: std::collections::HashMap<String, String>, status_code: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn rate_limit(&self, key: String, limit: i64, window_seconds: i64) -> Result<RateLimitOutput, Box<dyn std::error::Error>>;
    async fn match_route(&self, method: String, path: String, pattern: String) -> Result<MatchRouteOutput, Box<dyn std::error::Error>>;
    async fn proxy_websocket(&self, headers: std::collections::HashMap<String, String>, target_url: String) -> Result<(), Box<dyn std::error::Error>>;
}
