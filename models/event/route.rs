// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpResponseSent {
    pub event_route: bool,
    pub http_method: String,
}

impl HttpResponseSent {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteMatched {
    pub handler: String,
    #[serde(default = "chrono::Utc::now")]
    pub matched_at: chrono::DateTime<chrono::Utc>,
    pub path_params: Option<String>,
    pub request_id: String,
    pub route_name: Option<String>,
    pub route_pattern: String,
}

impl RouteMatched {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteNotFound {
    pub method: String,
    #[serde(default = "chrono::Utc::now")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub path: String,
    pub request_id: String,
}

impl RouteNotFound {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestAuthenticated {
    pub auth_method: String,
    #[serde(default = "chrono::Utc::now")]
    pub authenticated_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_authenticated: bool,
    pub request_id: String,
    pub user_id: Option<String>,
}

impl RequestAuthenticated {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestRateLimited {
    pub current_count: i64,
    pub ip_address: Option<String>,
    pub limit: i64,
    #[serde(default = "chrono::Utc::now")]
    pub limited_at: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
    pub user_id: Option<String>,
    pub window_seconds: i64,
}

impl RequestRateLimited {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

