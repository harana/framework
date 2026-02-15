// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkerFetchInvoked {
    pub service_binding: String,
    pub url: String,
    pub method: String,
    pub status_code: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub invoked_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkerFetchInvoked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkerRouteMatched {
    pub route_pattern: String,
    pub url: String,
    pub method: String,
    pub service_binding: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub matched_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkerRouteMatched {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkerCronTriggered {
    pub service_binding: String,
    pub cron: String,
    pub scheduled_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub triggered_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkerCronTriggered {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkerErrorOccurred {
    pub service_binding: Option<String>,
    pub url: Option<String>,
    pub method: String,
    pub error_message: String,
    pub status_code: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkerErrorOccurred {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkerVersionDeployed {
    pub binding: String,
    pub version_id: String,
    pub version_tag: Option<String>,
    pub version_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub deployed_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorkerVersionDeployed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareServiceBindingFetchInvoked {
    pub service: String,
    pub url: String,
    pub method: String,
    pub status_code: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub invoked_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareServiceBindingFetchInvoked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

