// Harana Actions - Cache Module
// This module provides in-memory cache actions using moka.

pub mod output;

use output::*;
use chrono::{DateTime, Duration, Utc};
use moka::sync::Cache;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Internal cache entry storing value and metadata.
#[derive(Clone, Debug)]
struct CacheEntry {
    value: Value,
    expires_at: Option<DateTime<Utc>>,
}

/// Thread-safe namespace to cache mapping.
type NamespaceCache = Arc<RwLock<HashMap<String, Cache<String, CacheEntry>>>>;

/// Global cache storage organized by namespace.
static CACHE_STORE: Lazy<NamespaceCache> = Lazy::new(|| {
    Arc::new(RwLock::new(HashMap::new()))
});

/// Default cache TTL if none specified (1 hour).
const DEFAULT_MAX_CAPACITY: u64 = 10_000;

/// Gets or creates a cache for the given namespace.
fn get_or_create_cache(namespace: &str) -> Cache<String, CacheEntry> {
    let mut store = CACHE_STORE.write();
    if let Some(cache) = store.get(namespace) {
        cache.clone()
    } else {
        let cache = Cache::builder()
            .max_capacity(DEFAULT_MAX_CAPACITY)
            .build();
        store.insert(namespace.to_string(), cache.clone());
        cache
    }
}

/// Build a namespaced key.
fn namespaced_key(namespace: &str, key: &str) -> String {
    format!("{}:{}", namespace, key)
}

/// Get Value From Cache.
pub async fn get(
    key: &str,
    namespace: Option<&str>,
) -> Result<GetOutput, String> {
    let namespace = namespace.unwrap_or("default");
    let cache = get_or_create_cache(namespace);
    let full_key = namespaced_key(namespace, key);
    
    match cache.get(&full_key) {
        Some(entry) => {
            // Check if expired
            if let Some(expires_at) = entry.expires_at {
                if Utc::now() > expires_at {
                    cache.invalidate(&full_key);
                    return Ok(GetOutput {
                        value: String::new(),
                        found: false,
                    });
                }
            }
            Ok(GetOutput {
                value: entry.value.to_string(),
                found: true,
            })
        }
        None => Ok(GetOutput {
            value: String::new(),
            found: false,
        }),
    }
}

/// Set Value In Cache.
pub async fn set(
    key: &str,
    value: &str,
    namespace: Option<&str>,
    ttl: Option<i32>,
) -> Result<SetOutput, String> {
    let namespace = namespace.unwrap_or("default");
    let cache = get_or_create_cache(namespace);
    let full_key = namespaced_key(namespace, key);
    
    let parsed_value: Value = serde_json::from_str(value)
        .unwrap_or_else(|_| Value::String(value.to_string()));
    
    let expires_at = ttl.map(|seconds| Utc::now() + Duration::seconds(seconds as i64));
    
    let entry = CacheEntry {
        value: parsed_value,
        expires_at,
    };
    
    cache.insert(full_key, entry);
    
    Ok(SetOutput { success: true })
}

/// Delete Value From Cache.
pub async fn delete(
    key: &str,
    namespace: Option<&str>,
) -> Result<DeleteOutput, String> {
    let namespace = namespace.unwrap_or("default");
    let cache = get_or_create_cache(namespace);
    let full_key = namespaced_key(namespace, key);
    
    cache.invalidate(&full_key);
    
    Ok(DeleteOutput { success: true })
}

/// Check If Key Exists.
pub async fn exists(
    key: &str,
    namespace: Option<&str>,
) -> Result<ExistsOutput, String> {
    let namespace = namespace.unwrap_or("default");
    let cache = get_or_create_cache(namespace);
    let full_key = namespaced_key(namespace, key);
    
    let exists = match cache.get(&full_key) {
        Some(entry) => {
            // Check if expired
            if let Some(expires_at) = entry.expires_at {
                if Utc::now() > expires_at {
                    cache.invalidate(&full_key);
                    false
                } else {
                    true
                }
            } else {
                true
            }
        }
        None => false,
    };
    
    Ok(ExistsOutput { exists })
}

/// Clear Cache Namespace.
pub async fn clear(
    namespace: Option<&str>,
    pattern: Option<&str>,
) -> Result<ClearOutput, String> {
    let namespace = namespace.unwrap_or("default");
    let cache = get_or_create_cache(namespace);
    let prefix = namespaced_key(namespace, "");
    
    let mut keys_deleted = 0;
    
    // Collect keys to delete
    let keys_to_delete: Vec<String> = cache
        .iter()
        .filter_map(|(k, _)| {
            let key_str: &str = k.as_ref();
            if key_str.starts_with(&prefix) {
                let key_part = key_str.strip_prefix(&prefix).unwrap_or(key_str);
                if let Some(pat) = pattern {
                    if key_part.contains(pat) || matches_glob_pattern(key_part, pat) {
                        Some(key_str.to_string())
                    } else {
                        None
                    }
                } else {
                    Some(key_str.to_string())
                }
            } else {
                None
            }
        })
        .collect();
    
    for key in keys_to_delete {
        cache.invalidate(&key);
        keys_deleted += 1;
    }
    
    Ok(ClearOutput { keys_deleted })
}

/// Simple glob pattern matching (supports * wildcard).
fn matches_glob_pattern(text: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    
    let parts: Vec<&str> = pattern.split('*').collect();
    if parts.len() == 1 {
        // No wildcards
        return text == pattern;
    }
    
    let mut remaining = text;
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        
        if i == 0 {
            // First part must be at the start
            if !remaining.starts_with(part) {
                return false;
            }
            remaining = &remaining[part.len()..];
        } else if i == parts.len() - 1 {
            // Last part must be at the end
            if !remaining.ends_with(part) {
                return false;
            }
        } else {
            // Middle parts can be anywhere
            if let Some(pos) = remaining.find(part) {
                remaining = &remaining[pos + part.len()..];
            } else {
                return false;
            }
        }
    }
    
    true
}

/// Get Multiple Values.
pub async fn get_many(
    keys: Vec<String>,
    namespace: Option<&str>,
) -> Result<GetManyOutput, String> {
    let namespace = namespace.unwrap_or("default");
    let cache = get_or_create_cache(namespace);
    
    let mut values = HashMap::new();
    let now = Utc::now();
    
    for key in keys {
        let full_key = namespaced_key(namespace, &key);
        if let Some(entry) = cache.get(&full_key) {
            // Check if expired
            let is_valid = match entry.expires_at {
                Some(expires_at) => {
                    if now > expires_at {
                        cache.invalidate(&full_key);
                        false
                    } else {
                        true
                    }
                }
                None => true,
            };
            
            if is_valid {
                values.insert(key, entry.value.clone());
            }
        }
    }
    
    Ok(GetManyOutput { values })
}

/// Set Multiple Values.
pub async fn set_many(
    entries: HashMap<String, Value>,
    ttl: Option<i32>,
    namespace: Option<&str>,
) -> Result<SetManyOutput, String> {
    let namespace = namespace.unwrap_or("default");
    let cache = get_or_create_cache(namespace);
    
    let expires_at = ttl.map(|seconds| Utc::now() + Duration::seconds(seconds as i64));
    
    for (key, value) in entries {
        let full_key = namespaced_key(namespace, &key);
        let entry = CacheEntry {
            value,
            expires_at,
        };
        cache.insert(full_key, entry);
    }
    
    Ok(SetManyOutput { success: true })
}

/// Increment Numeric Value.
pub async fn increment(
    key: &str,
    namespace: Option<&str>,
    amount: Option<i32>,
) -> Result<IncrementOutput, String> {
    let namespace = namespace.unwrap_or("default");
    let cache = get_or_create_cache(namespace);
    let full_key = namespaced_key(namespace, key);
    let amount = amount.unwrap_or(1);
    
    let current_value = match cache.get(&full_key) {
        Some(entry) => {
            // Check if expired
            if let Some(expires_at) = entry.expires_at {
                if Utc::now() > expires_at {
                    cache.invalidate(&full_key);
                    0
                } else {
                    entry.value.as_i64().unwrap_or(0) as i32
                }
            } else {
                entry.value.as_i64().unwrap_or(0) as i32
            }
        }
        None => 0,
    };
    
    let new_value = current_value + amount;
    
    let entry = CacheEntry {
        value: Value::Number(new_value.into()),
        expires_at: None,
    };
    cache.insert(full_key, entry);
    
    Ok(IncrementOutput { value: new_value })
}

/// Decrement Numeric Value.
pub async fn decrement(
    key: &str,
    amount: Option<i32>,
    namespace: Option<&str>,
) -> Result<DecrementOutput, String> {
    let namespace = namespace.unwrap_or("default");
    let cache = get_or_create_cache(namespace);
    let full_key = namespaced_key(namespace, key);
    let amount = amount.unwrap_or(1);
    
    let current_value = match cache.get(&full_key) {
        Some(entry) => {
            // Check if expired
            if let Some(expires_at) = entry.expires_at {
                if Utc::now() > expires_at {
                    cache.invalidate(&full_key);
                    0
                } else {
                    entry.value.as_i64().unwrap_or(0) as i32
                }
            } else {
                entry.value.as_i64().unwrap_or(0) as i32
            }
        }
        None => 0,
    };
    
    let new_value = current_value - amount;
    
    let entry = CacheEntry {
        value: Value::Number(new_value.into()),
        expires_at: None,
    };
    cache.insert(full_key, entry);
    
    Ok(DecrementOutput { value: new_value })
}

/// Get Time To Live.
pub async fn ttl(
    key: &str,
    namespace: Option<&str>,
) -> Result<TtlOutput, String> {
    let namespace = namespace.unwrap_or("default");
    let cache = get_or_create_cache(namespace);
    let full_key = namespaced_key(namespace, key);
    
    match cache.get(&full_key) {
        Some(entry) => {
            match entry.expires_at {
                Some(expires_at) => {
                    let now = Utc::now();
                    if now > expires_at {
                        cache.invalidate(&full_key);
                        Ok(TtlOutput {
                            ttl: -1,
                            expires_at: String::new(),
                        })
                    } else {
                        let ttl_duration = expires_at - now;
                        Ok(TtlOutput {
                            ttl: ttl_duration.num_seconds() as i32,
                            expires_at: expires_at.to_rfc3339(),
                        })
                    }
                }
                None => {
                    // No expiry set - key exists but has no TTL
                    Ok(TtlOutput {
                        ttl: -2, // -2 indicates no expiry
                        expires_at: String::new(),
                    })
                }
            }
        }
        None => {
            // Key doesn't exist
            Ok(TtlOutput {
                ttl: -1,
                expires_at: String::new(),
            })
        }
    }
}

#[cfg(test)]
mod tests;
