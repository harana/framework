// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// cloudflare_worker
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorker {
    pub account_id: String,
    pub compatibility_date: Option<String>,
    pub compatibility_flags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub etag: Option<String>,
    pub id: String,
    pub size: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CloudflareWorker {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cloudflare_worker_service_binding
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkerServiceBinding {
    pub binding_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub environment: Option<String>,
    pub target_service: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Reference: cf_worker.id
    pub worker_id: String,
}

impl CloudflareWorkerServiceBinding {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cloudflare_worker_cron_trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkerCronTrigger {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub cron: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Reference: cf_worker.id
    pub worker_id: String,
}

impl CloudflareWorkerCronTrigger {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// cloudflare_worker_version
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CloudflareWorkerVersion {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub message: Option<String>,
    pub tag: Option<String>,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub version_id: String,
    /// Reference: cf_worker.id
    pub worker_id: String,
}

impl CloudflareWorkerVersion {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

