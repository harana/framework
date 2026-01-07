// Harana Actions - Healthcheck Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// check_all
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckAllOutput {
    pub failed_count: i32,
    pub checks: Vec<HashMap<String, Value>>,
    pub healthy: bool,
    pub passed_count: i32
}

// check_cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckCacheOutput {
    pub details: HashMap<String, Value>,
    pub healthy: bool,
    pub latency_ms: i32
}

// check_database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckDatabaseOutput {
    pub details: HashMap<String, Value>,
    pub latency_ms: i32,
    pub healthy: bool
}

// check_disk_space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckDiskSpaceOutput {
    pub total_bytes: i32,
    pub free_bytes: i32,
    pub percent_used: f64,
    pub healthy: bool,
    pub used_bytes: i32
}

// check_external_api
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckExternalApiOutput {
    pub latency_ms: i32,
    pub status_code: i32,
    pub healthy: bool
}

// check_memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckMemoryOutput {
    pub percent_used: f64,
    pub healthy: bool,
    pub free_bytes: i32,
    pub used_bytes: i32,
    pub total_bytes: i32
}

// check_service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckServiceOutput {
    pub latency_ms: i32,
    pub details: HashMap<String, Value>,
    pub status: String,
    pub healthy: bool
}

// get_system_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSystemStatusOutput {
    pub uptime_seconds: i32,
    pub status: String,
    pub version: String,
    pub metrics: HashMap<String, Value>
}
