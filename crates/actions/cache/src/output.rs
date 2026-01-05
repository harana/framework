// Harana Actions - Cache Module Output Types
// Auto-generated output structs for Cache action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// cache_get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheGetOutput {
    pub found: bool,
    pub value: Value,
}

// cache_set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSetOutput {
    pub success: bool,
}

// cache_delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheDeleteOutput {
    pub success: bool,
}

// cache_exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheExistsOutput {
    pub exists: bool,
}

// cache_clear
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheClearOutput {
    pub keys_deleted: i32,
}

// cache_get_many
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheGetManyOutput {
    pub values: HashMap<String, Value>,
}

// cache_set_many
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSetManyOutput {
    pub success: bool,
}

// cache_increment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheIncrementOutput {
    pub value: i32,
}

// cache_decrement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheDecrementOutput {
    pub value: i32,
}

// cache_ttl
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheTtlOutput {
    pub expires_at: String, // datetime
    pub ttl: i32,
}
