pub mod output;

use output::*;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Instant;
use tokio::io::AsyncReadExt;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration};

/// Process tracking data
struct TrackedProcess {
    child: Arc<Mutex<Child>>,
    command: String,
    started_at: Instant,
    stdout_buffer: Arc<RwLock<String>>,
    stderr_buffer: Arc<RwLock<String>>,
    exit_code: Arc<RwLock<Option<i32>>>,
}

/// Global process storage
static PROCESSES: Lazy<DashMap<u32, TrackedProcess>> = Lazy::new(DashMap::new);

/// Start Process.
pub async fn start(
    command: &str,
    args: Option<Vec<String>>,
    working_directory: Option<&str>,
    environment: Option<HashMap<String, String>>,
    detach: Option<bool>,
) -> Result<StartOutput, String> {
    let args = args.unwrap_or_default();
    let _detach = detach.unwrap_or(false);
    
    let mut cmd = Command::new(command);
    cmd.args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    
    if let Some(dir) = working_directory {
        cmd.current_dir(dir);
    }
    
    if let Some(env) = environment {
        for (key, value) in env {
            cmd.env(key, value);
        }
    }
    
    let child = cmd.spawn().map_err(|e| format!("Failed to start process: {}", e))?;
    
    let process_id = child.id().ok_or("Failed to get process ID")?;
    
    let full_command = if args.is_empty() {
        command.to_string()
    } else {
        format!("{} {}", command, args.join(" "))
    };
    
    let tracked = TrackedProcess {
        child: Arc::new(Mutex::new(child)),
        command: full_command,
        started_at: Instant::now(),
        stdout_buffer: Arc::new(RwLock::new(String::new())),
        stderr_buffer: Arc::new(RwLock::new(String::new())),
        exit_code: Arc::new(RwLock::new(None)),
    };
    
    PROCESSES.insert(process_id, tracked);
    
    Ok(StartOutput {
        process_id,
        started: true,
        error: None,
    })
}

/// Stop Process.
pub async fn stop(
    process_id: u32,
    force: Option<bool>,
    timeout_secs: Option<i32>,
) -> Result<StopOutput, String> {
    let force = force.unwrap_or(false);
    let timeout_secs = timeout_secs.unwrap_or(30);
    
    let tracked = PROCESSES
        .get(&process_id)
        .ok_or_else(|| format!("Process not found: {}", process_id))?;
    
    let mut child = tracked.child.lock().await;
    
    // Try graceful termination first (unless force is specified)
    if !force {
        #[cfg(unix)]
        {
            // Send SIGTERM
            unsafe {
                libc::kill(process_id as i32, libc::SIGTERM);
            }
        }
        
        #[cfg(not(unix))]
        {
            let _ = child.start_kill();
        }
        
        // Wait for process to exit with timeout
        match timeout(
            Duration::from_secs(timeout_secs as u64),
            child.wait(),
        ).await {
            Ok(Ok(status)) => {
                let exit_code = status.code();
                *tracked.exit_code.write() = exit_code;
                drop(child);
                drop(tracked);
                PROCESSES.remove(&process_id);
                return Ok(StopOutput {
                    stopped: true,
                    exit_code,
                    error: None,
                });
            }
            _ => {
                // Timeout or error, force kill
            }
        }
    }
    
    // Force kill
    child.kill().await.map_err(|e| format!("Failed to kill process: {}", e))?;
    let status = child.wait().await.map_err(|e| format!("Failed to wait for process: {}", e))?;
    let exit_code = status.code();
    
    *tracked.exit_code.write() = exit_code;
    drop(child);
    drop(tracked);
    PROCESSES.remove(&process_id);
    
    Ok(StopOutput {
        stopped: true,
        exit_code,
        error: None,
    })
}

/// Kill Process.
pub async fn kill(
    process_id: u32,
    signal: Option<&str>,
) -> Result<KillOutput, String> {
    let _signal = signal.unwrap_or("SIGTERM");
    
    let tracked = PROCESSES
        .get(&process_id)
        .ok_or_else(|| format!("Process not found: {}", process_id))?;
    
    let child = tracked.child.lock().await;
    
    #[cfg(unix)]
    {
        let sig = match _signal.to_uppercase().as_str() {
            "SIGKILL" | "9" => libc::SIGKILL,
            "SIGINT" | "2" => libc::SIGINT,
            "SIGHUP" | "1" => libc::SIGHUP,
            _ => libc::SIGTERM, // Default to SIGTERM
        };
        
        unsafe {
            if libc::kill(process_id as i32, sig) == 0 {
                drop(child);
                drop(tracked);
                PROCESSES.remove(&process_id);
                return Ok(KillOutput {
                    killed: true,
                    error: None,
                });
            } else {
                return Err("Failed to send signal".to_string());
            }
        }
    }
    
    #[cfg(not(unix))]
    {
        child.kill().await.map_err(|e| format!("Failed to kill process: {}", e))?;
        drop(child);
        drop(tracked);
        PROCESSES.remove(&process_id);
        
        Ok(KillOutput {
            killed: true,
            error: None,
        })
    }
}

/// Check Process Status.
pub async fn status(
    process_id: u32,
) -> Result<StatusOutput, String> {
    let tracked = PROCESSES
        .get(&process_id)
        .ok_or_else(|| format!("Process not found: {}", process_id))?;
    
    let mut child = tracked.child.lock().await;
    
    // Check if process is still running
    let running = match child.try_wait() {
        Ok(Some(status)) => {
            *tracked.exit_code.write() = status.code();
            false
        }
        Ok(None) => true,
        Err(_) => false,
    };
    
    let uptime_secs = tracked.started_at.elapsed().as_secs() as i64;
    let exit_code = *tracked.exit_code.read();
    
    Ok(StatusOutput {
        status: ProcessStatus {
            process_id,
            running,
            exit_code,
            cpu_usage: None, // Would require sysinfo crate for accurate data
            memory_usage: None,
            uptime_secs: Some(uptime_secs),
        },
        error: None,
    })
}

/// List Running Processes.
pub async fn list(
    filter: Option<&str>,
    _user: Option<&str>,
) -> Result<ListOutput, String> {
    let mut processes = Vec::new();
    
    for entry in PROCESSES.iter() {
        let pid = *entry.key();
        let tracked = entry.value();
        
        // Apply filter if provided
        if let Some(f) = filter {
            if !tracked.command.contains(f) {
                continue;
            }
        }
        
        let mut child = tracked.child.lock().await;
        let running = match child.try_wait() {
            Ok(Some(_)) => false,
            Ok(None) => true,
            Err(_) => false,
        };
        drop(child);
        
        let uptime_secs = tracked.started_at.elapsed().as_secs() as i64;
        
        processes.push(ProcessInfo {
            process_id: pid,
            name: tracked.command.split_whitespace().next().unwrap_or("unknown").to_string(),
            command: tracked.command.clone(),
            user: None,
            cpu_usage: None,
            memory_usage: None,
            status: if running { "running" } else { "stopped" }.to_string(),
            uptime_secs: Some(uptime_secs),
        });
    }
    
    Ok(ListOutput { processes })
}

/// Wait For Process.
pub async fn wait(
    process_id: u32,
    timeout_secs: Option<i32>,
) -> Result<WaitOutput, String> {
    let timeout_secs = timeout_secs.unwrap_or(60);
    
    let tracked = PROCESSES
        .get(&process_id)
        .ok_or_else(|| format!("Process not found: {}", process_id))?;
    
    let child_arc = tracked.child.clone();
    let exit_code_arc = tracked.exit_code.clone();
    drop(tracked);
    
    let mut child = child_arc.lock().await;
    
    match timeout(
        Duration::from_secs(timeout_secs as u64),
        child.wait(),
    ).await {
        Ok(Ok(status)) => {
            let code = status.code();
            *exit_code_arc.write() = code;
            
            Ok(WaitOutput {
                completed: true,
                exit_code: code,
                error: None,
            })
        }
        Ok(Err(e)) => {
            Ok(WaitOutput {
                completed: false,
                exit_code: None,
                error: Some(format!("Process error: {}", e)),
            })
        }
        Err(_) => {
            Ok(WaitOutput {
                completed: false,
                exit_code: None,
                error: Some("Timeout waiting for process".to_string()),
            })
        }
    }
}

/// Get Process Output.
pub async fn get_output(
    process_id: u32,
    stream: Option<&str>,
) -> Result<OutputOutput, String> {
    let stream = stream.unwrap_or("stdout");
    
    let tracked = PROCESSES
        .get(&process_id)
        .ok_or_else(|| format!("Process not found: {}", process_id))?;
    
    let mut child = tracked.child.lock().await;
    
    let output = match stream {
        "stderr" => {
            if let Some(ref mut stderr) = child.stderr {
                let mut buffer = String::new();
                // Try to read available data (non-blocking)
                let mut buf = [0u8; 4096];
                match timeout(Duration::from_millis(100), stderr.read(&mut buf)).await {
                    Ok(Ok(n)) if n > 0 => {
                        buffer.push_str(&String::from_utf8_lossy(&buf[..n]));
                    }
                    _ => {}
                }
                let mut existing = tracked.stderr_buffer.write();
                existing.push_str(&buffer);
                existing.clone()
            } else {
                tracked.stderr_buffer.read().clone()
            }
        }
        _ => {
            // stdout
            if let Some(ref mut stdout) = child.stdout {
                let mut buffer = String::new();
                let mut buf = [0u8; 4096];
                match timeout(Duration::from_millis(100), stdout.read(&mut buf)).await {
                    Ok(Ok(n)) if n > 0 => {
                        buffer.push_str(&String::from_utf8_lossy(&buf[..n]));
                    }
                    _ => {}
                }
                let mut existing = tracked.stdout_buffer.write();
                existing.push_str(&buffer);
                existing.clone()
            } else {
                tracked.stdout_buffer.read().clone()
            }
        }
    };
    
    Ok(OutputOutput {
        output,
        error: None,
    })
}

#[cfg(test)]
mod tests;
