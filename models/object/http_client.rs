// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpClient {
    pub base_url: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub default_headers: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub max_retries: i64,
    pub timeout_seconds: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl HttpClient {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpRequestLog {
    pub client_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    pub method: String,
    pub request_body: Option<String>,
    pub request_headers: Option<String>,
    pub response_body: Option<String>,
    pub response_headers: Option<String>,
    pub status_code: Option<i64>,
    pub url: String,
}

impl HttpRequestLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub headers: String,
    pub query_params: String,
    pub body: String,
    pub timeout: i64,
}

impl HttpRequest {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Headers {
    pub content_type: String,
    pub authorization: String,
    pub accept: String,
    pub user_agent: String,
}

impl Headers {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QueryParams {
    pub key: String,
    pub value: String,
}

impl QueryParams {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Variables {
    pub key: String,
    pub value: String,
}

impl Variables {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphQlError {
    pub message: String,
    pub path: Vec<String>,
    pub locations: Vec<String>,
}

impl GraphQlError {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GraphQlErrorLocation {
    pub line: i64,
    pub column: i64,
}

impl GraphQlErrorLocation {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

