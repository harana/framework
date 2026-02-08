//! General route handlers

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde::Serialize;
use serde_json::json;
#[cfg(feature = "standalone")]
use sysinfo::{Disks, System};

use crate::extractors::{OptionalAuth, Auth};

/// Index page
pub async fn index() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Harana</title>
    <script type="module" src="https://cdn.jsdelivr.net/npm/@starfederation/datastar@1.0.0-beta.11/bundles/datastar.min.js"></script>
    <style>
        body { font-family: system-ui, -apple-system, sans-serif; max-width: 800px; margin: 0 auto; padding: 2rem; }
        h1 { color: #333; }
        .card { background: #f5f5f5; padding: 1rem; border-radius: 8px; margin: 1rem 0; }
        button { background: #007bff; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer; }
        button:hover { background: #0056b3; }
    </style>
</head>
<body>
    <h1>🌺 Harana</h1>
    <div class="card">
        <p>Welcome to the Harana Framework HTTP Server.</p>
        <p>This server supports:</p>
        <ul>
            <li>✅ Passkey/WebAuthn authentication</li>
            <li>✅ OAuth 2.0 / OpenID Connect</li>
            <li>✅ JWT token management</li>
            <li>✅ Datastar real-time updates</li>
        </ul>
    </div>
    
    <div class="card" data-signals="{message: '', count: 0}">
        <h3>Datastar Demo</h3>
        <p>Message: <span data-text="$message">Loading...</span></p>
        <p>Count: <span data-text="$count">0</span></p>
        <button data-on:click="@get('/api/datastar/increment')">Increment</button>
        <button data-on:click="@get('/api/datastar/hello')">Say Hello</button>
    </div>
    
    <div class="card">
        <h3>API Endpoints</h3>
        <ul>
            <li><a href="/health">/health</a> - Health check</li>
            <li><a href="/about">/about</a> - About info</li>
            <li><a href="/.well-known/openid-configuration">/.well-known/openid-configuration</a> - OpenID Discovery</li>
        </ul>
    </div>
</body>
</html>"#)
}

/// Health check endpoint
/// Performs comprehensive system health checks including:
/// - CPU utilization (should be < 80%)
/// - Memory usage (should be < 80%)
/// - Disk space (should have > 20% free)
#[cfg(feature = "standalone")]
pub async fn health() -> impl IntoResponse {
    let health_result = tokio::task::spawn_blocking(perform_health_check)
        .await
        .unwrap_or_else(|_| HealthCheckResult {
            status: HealthStatus::Unhealthy,
            timestamp: chrono::Utc::now().to_rfc3339(),
            checks: vec![HealthCheck {
                name: "system".to_string(),
                status: HealthStatus::Unhealthy,
                message: "Failed to perform health check".to_string(),
                value: None,
                threshold: None,
            }],
        });

    let status_code = match health_result.status {
        HealthStatus::Healthy => StatusCode::OK,
        HealthStatus::Degraded => StatusCode::OK,
        HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };

    (status_code, Json(health_result))
}

/// Lightweight health check for Cloudflare Workers (no sysinfo)
#[cfg(not(feature = "standalone"))]
pub async fn health() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "runtime": "cloudflare-workers"
    }))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Serialize)]
pub struct HealthCheck {
        pub name: String,
        pub status: HealthStatus,
        pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct HealthCheckResult {
        pub status: HealthStatus,
        pub timestamp: String,
        pub checks: Vec<HealthCheck>,
}

#[cfg(feature = "standalone")]
fn perform_health_check() -> HealthCheckResult {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Give CPU time to collect accurate readings
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu_all();

    let mut checks = Vec::new();
    let mut overall_status = HealthStatus::Healthy;

    // CPU Check - should be < 80%
    let cpu_usage = sys.global_cpu_usage() as f64;
    let cpu_threshold = 80.0;
    let cpu_status = if cpu_usage < cpu_threshold {
        HealthStatus::Healthy
    } else if cpu_usage < 90.0 {
        HealthStatus::Degraded
    } else {
        HealthStatus::Unhealthy
    };
    
    checks.push(HealthCheck {
        name: "cpu".to_string(),
        status: cpu_status,
        message: format!("CPU utilization: {:.1}%", cpu_usage),
        value: Some(cpu_usage),
        threshold: Some(cpu_threshold),
    });

    // Memory Check - should be < 80%
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_usage = if total_memory > 0 {
        (used_memory as f64 / total_memory as f64) * 100.0
    } else {
        0.0
    };
    let memory_threshold = 80.0;
    let memory_status = if memory_usage < memory_threshold {
        HealthStatus::Healthy
    } else if memory_usage < 90.0 {
        HealthStatus::Degraded
    } else {
        HealthStatus::Unhealthy
    };

    checks.push(HealthCheck {
        name: "memory".to_string(),
        status: memory_status,
        message: format!(
            "Memory usage: {:.1}% ({} / {} bytes)",
            memory_usage,
            format_bytes(used_memory),
            format_bytes(total_memory)
        ),
        value: Some(memory_usage),
        threshold: Some(memory_threshold),
    });

    // Disk Check - should have > 20% free space
    let disks = Disks::new_with_refreshed_list();
    let mut disk_checks = Vec::new();
    
    for disk in disks.list() {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        
        if total_space == 0 {
            continue;
        }

        let free_percentage = (available_space as f64 / total_space as f64) * 100.0;
        let used_percentage = 100.0 - free_percentage;
        let disk_threshold = 80.0; // Alert when usage exceeds 80%
        
        let disk_status = if used_percentage < disk_threshold {
            HealthStatus::Healthy
        } else if used_percentage < 90.0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Unhealthy
        };

        let mount_point = disk.mount_point().to_string_lossy().to_string();
        
        disk_checks.push(HealthCheck {
            name: format!("disk:{}", mount_point),
            status: disk_status,
            message: format!(
                "Disk '{}': {:.1}% used ({} / {} available)",
                mount_point,
                used_percentage,
                format_bytes(available_space),
                format_bytes(total_space)
            ),
            value: Some(used_percentage),
            threshold: Some(disk_threshold),
        });
    }

    // If no disks found, add an error check
    if disk_checks.is_empty() {
        checks.push(HealthCheck {
            name: "disk".to_string(),
            status: HealthStatus::Degraded,
            message: "No disk information available".to_string(),
            value: None,
            threshold: None,
        });
    } else {
        checks.extend(disk_checks);
    }

    // Calculate overall status (worst of all checks)
    for check in &checks {
        match (overall_status, check.status) {
            (_, HealthStatus::Unhealthy) => overall_status = HealthStatus::Unhealthy,
            (HealthStatus::Healthy, HealthStatus::Degraded) => overall_status = HealthStatus::Degraded,
            _ => {}
        }
    }

    HealthCheckResult {
        status: overall_status,
        timestamp: chrono::Utc::now().to_rfc3339(),
        checks,
    }
}

#[cfg(feature = "standalone")]
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// About page
pub async fn about() -> impl IntoResponse {
    Json(json!({
        "name": "Harana",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "A modern Rust web framework with real-time capabilities",
        "features": [
            "Axum web framework",
            "Datastar SSE integration",
            "Passkey/WebAuthn authentication",
            "OAuth 2.0 / OpenID Connect",
            "JWT token management",
            "Session management"
        ]
    }))
}

/// Protected endpoint example (requires authentication)
pub async fn protected(auth: Auth) -> impl IntoResponse {
    Json(json!({
        "message": "You are authenticated!",
        "user_id": auth.user_id,
        "email": auth.email,
        "roles": auth.roles
    }))
}

/// Optionally protected endpoint
pub async fn maybe_protected(OptionalAuth(auth): OptionalAuth) -> impl IntoResponse {
    match auth {
        Some(user) => Json(json!({
            "authenticated": true,
            "user_id": user.user_id,
            "email": user.email
        })),
        None => Json(json!({
            "authenticated": false,
            "message": "You are not logged in"
        })),
    }
}
