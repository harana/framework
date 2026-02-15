// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// http response sent
/// Class: event_route_response_sent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HttpResponseSent {
}

impl HttpResponseSent {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route matched
/// Class: event_route_matched
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteMatched {
}

impl RouteMatched {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// route not found
/// Class: event_route_not_found
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RouteNotFound {
}

impl RouteNotFound {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// request authenticated
/// Class: event_route_request_authenticated
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestAuthenticated {
}

impl RequestAuthenticated {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// request rate limited
/// Class: event_route_request_rate_limited
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestRateLimited {
}

impl RequestRateLimited {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

