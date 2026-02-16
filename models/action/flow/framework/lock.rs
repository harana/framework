// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DistributedLock {
    #[serde(default = "chrono::Utc::now")]
    pub acquired_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub fencing_token: i64,
    pub id: String,
    pub metadata: String,
    pub owner_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LockConfig {
    pub default_ttl_seconds: i64,
    #[serde(default)]
    pub enable_lock_ordering: bool,
    pub max_locks_per_owner: i64,
    pub retry_interval_ms: i64,
    pub stale_threshold_seconds: i64,
    pub wait_timeout_ms: i64,
}

