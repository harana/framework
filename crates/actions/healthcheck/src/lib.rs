// Harana Actions - Healthcheck Module
// This module provides healthcheck actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde_json::json;
use sysinfo::{Disks, System};
use output::*;

/// Run All Health Checks
pub async fn check_all(
    fail_fast: Option<bool>,
    timeout: Option<i32>,
) -> Result<CheckAllOutput, String> {
    let fail_fast = fail_fast.unwrap_or(false);
    let _timeout_ms = timeout.unwrap_or(5000);
    
    let mut checks = Vec::new();
    let mut passed_count = 0;
    let mut failed_count = 0;
    
    // Check memory
    let memory_result = check_memory(Some(90)).await;
    let memory_check = match memory_result {
        Ok(output) => {
            if output.healthy {
                passed_count += 1;
            } else {
                failed_count += 1;
            }
            let mut check = HashMap::new();
            check.insert("name".to_string(), json!("memory"));
            check.insert("healthy".to_string(), json!(output.healthy));
            check.insert("percent_used".to_string(), json!(output.percent_used));
            check
        }
        Err(e) => {
            failed_count += 1;
            let mut check = HashMap::new();
            check.insert("name".to_string(), json!("memory"));
            check.insert("healthy".to_string(), json!(false));
            check.insert("error".to_string(), json!(e));
            check
        }
    };
    checks.push(memory_check);
    
    if fail_fast && failed_count > 0 {
        return Ok(CheckAllOutput {
            healthy: false,
            passed_count,
            failed_count,
            checks,
        });
    }
    
    // Check disk space
    let disk_result = check_disk_space(Some(90), None).await;
    let disk_check = match disk_result {
        Ok(output) => {
            if output.healthy {
                passed_count += 1;
            } else {
                failed_count += 1;
            }
            let mut check = HashMap::new();
            check.insert("name".to_string(), json!("disk_space"));
            check.insert("healthy".to_string(), json!(output.healthy));
            check.insert("percent_used".to_string(), json!(output.percent_used));
            check
        }
        Err(e) => {
            failed_count += 1;
            let mut check = HashMap::new();
            check.insert("name".to_string(), json!("disk_space"));
            check.insert("healthy".to_string(), json!(false));
            check.insert("error".to_string(), json!(e));
            check
        }
    };
    checks.push(disk_check);
    
    if fail_fast && failed_count > 0 {
        return Ok(CheckAllOutput {
            healthy: false,
            passed_count,
            failed_count,
            checks,
        });
    }
    
    Ok(CheckAllOutput {
        healthy: failed_count == 0,
        passed_count,
        failed_count,
        checks,
    })
}

/// Check Cache Connection
/// Note: This is a placeholder that simulates a cache check.
/// In a real implementation, this would connect to Redis/Memcached.
pub async fn check_cache(
    cache_name: Option<&str>,
    timeout: Option<i32>,
) -> Result<CheckCacheOutput, String> {
    let name = cache_name.unwrap_or("default");
    let timeout_ms = timeout.unwrap_or(1000);
    
    let start = Instant::now();
    
    // Simulate cache ping - in real implementation, this would use redis crate
    tokio::time::sleep(Duration::from_millis(1)).await;
    
    let latency = start.elapsed().as_millis() as i32;
    
    let mut details = HashMap::new();
    details.insert("cache_name".to_string(), json!(name));
    details.insert("timeout_ms".to_string(), json!(timeout_ms));
    details.insert("type".to_string(), json!("simulated"));
    
    Ok(CheckCacheOutput {
        healthy: latency < timeout_ms,
        latency_ms: latency,
        details,
    })
}

/// Check Database Connection
/// Note: This is a placeholder that simulates a database check.
/// In a real implementation, this would use sqlx or similar.
pub async fn check_database(
    connection_name: Option<&str>,
    timeout: Option<i32>,
) -> Result<CheckDatabaseOutput, String> {
    let name = connection_name.unwrap_or("default");
    let timeout_ms = timeout.unwrap_or(5000);
    
    let start = Instant::now();
    
    // Simulate database ping - in real implementation, this would use sqlx
    tokio::time::sleep(Duration::from_millis(1)).await;
    
    let latency = start.elapsed().as_millis() as i32;
    
    let mut details = HashMap::new();
    details.insert("connection_name".to_string(), json!(name));
    details.insert("timeout_ms".to_string(), json!(timeout_ms));
    details.insert("type".to_string(), json!("simulated"));
    
    Ok(CheckDatabaseOutput {
        healthy: latency < timeout_ms,
        latency_ms: latency,
        details,
    })
}

/// Check Disk Space
pub async fn check_disk_space(
    threshold_percent: Option<i32>,
    path: Option<&str>,
) -> Result<CheckDiskSpaceOutput, String> {
    let threshold = threshold_percent.unwrap_or(90) as f64;
    let _path = path.unwrap_or("/");
    
    let disks = Disks::new_with_refreshed_list();
    
    // Get the first disk or the disk matching the path
    let disk = disks.list().first();
    
    match disk {
        Some(d) => {
            let total = d.total_space() as i64;
            let available = d.available_space() as i64;
            let used = total - available;
            let percent_used = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            
            Ok(CheckDiskSpaceOutput {
                healthy: percent_used < threshold,
                total_bytes: total as i32,
                used_bytes: used as i32,
                free_bytes: available as i32,
                percent_used,
            })
        }
        None => {
            // Return a simulated response if no disks found
            Ok(CheckDiskSpaceOutput {
                healthy: true,
                total_bytes: 0,
                used_bytes: 0,
                free_bytes: 0,
                percent_used: 0.0,
            })
        }
    }
}

/// Check External API
pub async fn check_external_api(
    url: &str,
    expected_status: Option<i32>,
    method: Option<&str>,
    timeout: Option<i32>,
) -> Result<CheckExternalApiOutput, String> {
    let expected = expected_status.unwrap_or(200);
    let http_method = method.unwrap_or("GET");
    let timeout_secs = timeout.unwrap_or(30);
    
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs as u64))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    let start = Instant::now();
    
    let request = match http_method.to_uppercase().as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "HEAD" => client.head(url),
        _ => client.get(url),
    };
    
    let response = request.send().await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    let latency = start.elapsed().as_millis() as i32;
    let status_code = response.status().as_u16() as i32;
    
    Ok(CheckExternalApiOutput {
        healthy: status_code == expected,
        latency_ms: latency,
        status_code,
    })
}

/// Check Memory Usage
pub async fn check_memory(
    threshold_percent: Option<i32>,
) -> Result<CheckMemoryOutput, String> {
    let threshold = threshold_percent.unwrap_or(90) as f64;
    
    let mut sys = System::new_all();
    sys.refresh_memory();
    
    let total = sys.total_memory() as i64;
    let used = sys.used_memory() as i64;
    let free = total - used;
    let percent_used = if total > 0 {
        (used as f64 / total as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(CheckMemoryOutput {
        healthy: percent_used < threshold,
        total_bytes: total as i32,
        used_bytes: used as i32,
        free_bytes: free as i32,
        percent_used,
    })
}

/// Check Service Health
/// Note: This is a placeholder for service-specific health checks.
pub async fn check_service(
    service_name: &str,
    timeout: Option<i32>,
) -> Result<CheckServiceOutput, String> {
    let timeout_ms = timeout.unwrap_or(5000);
    
    let start = Instant::now();
    
    // Simulate service check
    tokio::time::sleep(Duration::from_millis(1)).await;
    
    let latency = start.elapsed().as_millis() as i32;
    
    let mut details = HashMap::new();
    details.insert("service_name".to_string(), json!(service_name));
    details.insert("timeout_ms".to_string(), json!(timeout_ms));
    details.insert("type".to_string(), json!("simulated"));
    
    Ok(CheckServiceOutput {
        healthy: true,
        status: "healthy".to_string(),
        latency_ms: latency,
        details,
    })
}

/// Get System Status
pub async fn get_system_status(
    include_metrics: Option<bool>,
) -> Result<GetSystemStatusOutput, String> {
    let include = include_metrics.unwrap_or(true);
    
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let uptime = System::uptime() as i32;
    
    let mut metrics = HashMap::new();
    
    if include {
        // Memory metrics
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        metrics.insert("memory_total_bytes".to_string(), json!(total_memory));
        metrics.insert("memory_used_bytes".to_string(), json!(used_memory));
        metrics.insert("memory_percent_used".to_string(), 
            json!((used_memory as f64 / total_memory as f64) * 100.0));
        
        // CPU metrics
        metrics.insert("cpu_count".to_string(), json!(sys.cpus().len()));
        
        // Process count
        metrics.insert("process_count".to_string(), json!(sys.processes().len()));
        
        // Disk metrics from first disk
        let disks = Disks::new_with_refreshed_list();
        if let Some(disk) = disks.list().first() {
            metrics.insert("disk_total_bytes".to_string(), json!(disk.total_space()));
            metrics.insert("disk_available_bytes".to_string(), json!(disk.available_space()));
        }
    }
    
    Ok(GetSystemStatusOutput {
        status: "healthy".to_string(),
        uptime_seconds: uptime,
        version: env!("CARGO_PKG_VERSION").to_string(),
        metrics,
    })
}

#[cfg(test)]
mod tests;
