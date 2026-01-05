// Harana Actions - Healthcheck Module Output Types
// Auto-generated output structs for Healthcheck action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// check_service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckServiceOutput {
    pub details: HashMap<String, Value>,
    pub healthy: bool,
    pub latency_ms: i32,
    pub status: String,
}

// check_database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckDatabaseOutput {
    pub details: HashMap<String, Value>,
    pub healthy: bool,
    pub latency_ms: i32,
}

// check_cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckCacheOutput {
    pub details: HashMap<String, Value>,
    pub healthy: bool,
    pub latency_ms: i32,
}

// check_external_api
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckExternalApiOutput {
    pub healthy: bool,
    pub latency_ms: i32,
    pub status_code: i32,
}

// check_disk_space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckDiskSpaceOutput {
    pub free_bytes: i64,
    pub healthy: bool,
    pub percent_used: f32,
    pub total_bytes: i64,
    pub used_bytes: i64,
}

// check_memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckMemoryOutput {
    pub free_bytes: i64,
    pub healthy: bool,
    pub percent_used: f32,
    pub total_bytes: i64,
    pub used_bytes: i64,
}

// check_all
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckAllOutput {
    pub checks: Vec<HashMap<String, Value>>,
    pub failed_count: i32,
    pub healthy: bool,
    pub passed_count: i32,
}

// get_system_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSystemStatusOutput {
    pub metrics: HashMap<String, Value>,
    pub status: String, // healthy | degraded | unhealthy
    pub uptime_seconds: i64,
    pub version: String,
}
