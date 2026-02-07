// Harana Actions - Workflow Module
// This module provides workflow actions and functionality.

pub mod output;

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use output::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WorkflowExecution {
    execution_id: String,
    workflow_id: String,
    status: String,
    current_step: String,
    input: Value,
    output: Value,
    started_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    error: String,
    progress: f64,
    context: HashMap<String, Value>,
    pause_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WorkflowHistoryEvent {
    event_id: String,
    event_type: String,
    step_id: String,
    timestamp: DateTime<Utc>,
    data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WorkflowSignal {
    signal_name: String,
    payload: Value,
    received_at: DateTime<Utc>,
}

/// Global workflow execution storage
static EXECUTIONS: Lazy<DashMap<String, WorkflowExecution>> = Lazy::new(DashMap::new);

/// Global workflow history storage
static HISTORY: Lazy<DashMap<String, Vec<WorkflowHistoryEvent>>> = Lazy::new(DashMap::new);

/// Global workflow signals storage
static SIGNALS: Lazy<DashMap<String, Vec<WorkflowSignal>>> = Lazy::new(DashMap::new);

/// Global workflow event waiters
static EVENT_WAITERS: Lazy<Arc<RwLock<HashMap<String, Arc<Mutex<Option<Value>>>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

fn add_history_event(
    execution_id: &str,
    event_type: &str,
    step_id: &str,
    data: Value,
) {
    let event = WorkflowHistoryEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: event_type.to_string(),
        step_id: step_id.to_string(),
        timestamp: Utc::now(),
        data,
    };
    
    HISTORY
        .entry(execution_id.to_string())
        .or_insert_with(Vec::new)
        .push(event);
}

/// Start Workflow Execution
pub async fn start(
    workflow_id: &str,
    context: Option<HashMap<String, Value>>,
    input: Option<&str>,
) -> Result<StartOutput, String> {
    let execution_id = Uuid::new_v4().to_string();
    let input_value = if let Some(inp) = input {
        serde_json::from_str(inp).unwrap_or(Value::String(inp.to_string()))
    } else {
        Value::Null
    };
    
    let execution = WorkflowExecution {
        execution_id: execution_id.clone(),
        workflow_id: workflow_id.to_string(),
        status: "pending".to_string(),
        current_step: "start".to_string(),
        input: input_value.clone(),
        output: Value::Null,
        started_at: Utc::now(),
        completed_at: None,
        error: String::new(),
        progress: 0.0,
        context: context.unwrap_or_default(),
        pause_reason: None,
    };
    
    EXECUTIONS.insert(execution_id.clone(), execution);
    
    add_history_event(
        &execution_id,
        "workflow_started",
        "start",
        serde_json::json!({
            "workflow_id": workflow_id,
            "input": input_value
        }),
    );
    
    // Simulate workflow starting
    if let Some(mut exec) = EXECUTIONS.get_mut(&execution_id) {
        exec.status = "running".to_string();
        exec.progress = 0.1;
    }
    
    Ok(StartOutput {
        execution_id,
        status: "running".to_string(),
        success: true,
    })
}

/// Pause Workflow Execution
pub async fn pause(
    execution_id: &str,
    reason: Option<&str>,
) -> Result<PauseOutput, String> {
    let mut exec = EXECUTIONS
        .get_mut(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;
    
    if exec.status != "running" {
        return Err(format!("Cannot pause workflow in status: {}", exec.status));
    }
    
    exec.status = "paused".to_string();
    exec.pause_reason = reason.map(|s| s.to_string());
    
    add_history_event(
        execution_id,
        "workflow_paused",
        &exec.current_step,
        serde_json::json!({
            "reason": reason.unwrap_or("")
        }),
    );
    
    Ok(PauseOutput { success: true })
}

/// Resume Paused Workflow
pub async fn resume(
    execution_id: &str,
    input: Option<&str>,
) -> Result<ResumeOutput, String> {
    let mut exec = EXECUTIONS
        .get_mut(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;
    
    if exec.status != "paused" {
        return Err(format!("Cannot resume workflow in status: {}", exec.status));
    }
    
    exec.status = "running".to_string();
    exec.pause_reason = None;
    
    if let Some(inp) = input {
        if let Ok(value) = serde_json::from_str::<Value>(inp) {
            exec.input = value;
        }
    }
    
    add_history_event(
        execution_id,
        "workflow_resumed",
        &exec.current_step,
        serde_json::json!({
            "input": input.unwrap_or("")
        }),
    );
    
    Ok(ResumeOutput { success: true })
}

/// Cancel Workflow Execution
pub async fn cancel(
    execution_id: &str,
    reason: Option<&str>,
) -> Result<CancelOutput, String> {
    let mut exec = EXECUTIONS
        .get_mut(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;
    
    if exec.status == "completed" || exec.status == "cancelled" {
        return Err(format!("Cannot cancel workflow in status: {}", exec.status));
    }
    
    exec.status = "cancelled".to_string();
    exec.completed_at = Some(Utc::now());
    exec.error = reason.unwrap_or("Cancelled by user").to_string();
    
    add_history_event(
        execution_id,
        "workflow_cancelled",
        &exec.current_step,
        serde_json::json!({
            "reason": reason.unwrap_or("Cancelled by user")
        }),
    );
    
    Ok(CancelOutput { success: true })
}

/// Get Workflow Status
pub async fn get_status(
    execution_id: &str,
) -> Result<GetStatusOutput, String> {
    let exec = EXECUTIONS
        .get(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;
    
    Ok(GetStatusOutput {
        status: exec.status.clone(),
        current_step: exec.current_step.clone(),
        progress: exec.progress,
        error: exec.error.clone(),
        started_at: exec.started_at.to_rfc3339(),
        completed_at: exec
            .completed_at
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_default(),
    })
}

/// Get Workflow Result
pub async fn get_result(
    execution_id: &str,
) -> Result<GetResultOutput, String> {
    let exec = EXECUTIONS
        .get(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;
    
    let completed = exec.status == "completed" || exec.status == "failed" || exec.status == "cancelled";
    
    Ok(GetResultOutput {
        completed,
        output: serde_json::to_string(&exec.output).unwrap_or_default(),
        error: exec.error.clone(),
    })
}

/// List Workflow Executions
pub async fn list_executions(
    status: Option<&str>,
    workflow_id: Option<&str>,
    start_date: Option<&str>,
    limit: Option<i32>,
    end_date: Option<&str>,
    offset: Option<i32>,
) -> Result<ListExecutionsOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    let offset = offset.unwrap_or(0) as usize;
    
    let start_dt = start_date
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    
    let end_dt = end_date
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    
    let executions: Vec<HashMap<String, Value>> = EXECUTIONS
        .iter()
        .filter(|entry| {
            let exec = entry.value();
            
            if let Some(s) = status {
                if exec.status != s {
                    return false;
                }
            }
            
            if let Some(wid) = workflow_id {
                if exec.workflow_id != wid {
                    return false;
                }
            }
            
            if let Some(start) = start_dt {
                if exec.started_at < start {
                    return false;
                }
            }
            
            if let Some(end) = end_dt {
                if exec.started_at > end {
                    return false;
                }
            }
            
            true
        })
        .skip(offset)
        .take(limit)
        .map(|entry| {
            let exec = entry.value();
            let mut map = HashMap::new();
            map.insert("execution_id".to_string(), Value::String(exec.execution_id.clone()));
            map.insert("workflow_id".to_string(), Value::String(exec.workflow_id.clone()));
            map.insert("status".to_string(), Value::String(exec.status.clone()));
            map.insert("started_at".to_string(), Value::String(exec.started_at.to_rfc3339()));
            if let Some(completed) = exec.completed_at {
                map.insert("completed_at".to_string(), Value::String(completed.to_rfc3339()));
            }
            map
        })
        .collect();
    
    let total = executions.len() as i32;
    
    Ok(ListExecutionsOutput { executions, total })
}

/// Get Workflow History
pub async fn history(
    execution_id: &str,
) -> Result<HistoryOutput, String> {
    if !EXECUTIONS.contains_key(execution_id) {
        return Err(format!("Execution not found: {}", execution_id));
    }
    
    let events: Vec<HashMap<String, Value>> = HISTORY
        .get(execution_id)
        .map(|entry| {
            entry
                .value()
                .iter()
                .map(|event| {
                    let mut map = HashMap::new();
                    map.insert("event_id".to_string(), Value::String(event.event_id.clone()));
                    map.insert("event_type".to_string(), Value::String(event.event_type.clone()));
                    map.insert("step_id".to_string(), Value::String(event.step_id.clone()));
                    map.insert("timestamp".to_string(), Value::String(event.timestamp.to_rfc3339()));
                    map.insert("data".to_string(), event.data.clone());
                    map
                })
                .collect()
        })
        .unwrap_or_default();
    
    let total = events.len() as i32;
    
    Ok(HistoryOutput { events, total })
}

/// Send Signal To Workflow
pub async fn signal(
    signal_name: &str,
    execution_id: &str,
    payload: Option<&str>,
) -> Result<SignalOutput, String> {
    if !EXECUTIONS.contains_key(execution_id) {
        return Err(format!("Execution not found: {}", execution_id));
    }
    
    let payload_value = if let Some(p) = payload {
        serde_json::from_str(p).unwrap_or(Value::String(p.to_string()))
    } else {
        Value::Null
    };
    
    let signal = WorkflowSignal {
        signal_name: signal_name.to_string(),
        payload: payload_value.clone(),
        received_at: Utc::now(),
    };
    
    SIGNALS
        .entry(execution_id.to_string())
        .or_insert_with(Vec::new)
        .push(signal);
    
    add_history_event(
        execution_id,
        "signal_received",
        "signal",
        serde_json::json!({
            "signal_name": signal_name,
            "payload": payload_value
        }),
    );
    
    Ok(SignalOutput { success: true })
}

/// Wait For External Event
pub async fn wait_for_event(
    event_name: &str,
    execution_id: &str,
    timeout: Option<i32>,
) -> Result<WaitForEventOutput, String> {
    if !EXECUTIONS.contains_key(execution_id) {
        return Err(format!("Execution not found: {}", execution_id));
    }
    
    let timeout_secs = timeout.unwrap_or(3600);
    let key = format!("{}:{}", execution_id, event_name);
    
    // Create a waiter
    let waiter = Arc::new(Mutex::new(None));
    {
        let mut waiters = EVENT_WAITERS.write().await;
        waiters.insert(key.clone(), waiter.clone());
    }
    
    // Wait for event with timeout
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(timeout_secs as u64),
        async {
            loop {
                let lock = waiter.lock().await;
                if lock.is_some() {
                    return lock.clone();
                }
                drop(lock);
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        },
    )
    .await;
    
    // Clean up waiter
    {
        let mut waiters = EVENT_WAITERS.write().await;
        waiters.remove(&key);
    }
    
    match result {
        Ok(Some(payload)) => Ok(WaitForEventOutput {
            received: true,
            timed_out: false,
            payload: serde_json::to_string(&payload).unwrap_or_default(),
        }),
        Ok(None) => Ok(WaitForEventOutput {
            received: false,
            timed_out: true,
            payload: String::new(),
        }),
        Err(_) => Ok(WaitForEventOutput {
            received: false,
            timed_out: true,
            payload: String::new(),
        }),
    }
}

/// Retry Failed Step
pub async fn retry_step(
    execution_id: &str,
    step_id: &str,
) -> Result<RetryStepOutput, String> {
    let exec = EXECUTIONS
        .get(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;
    
    if exec.status != "failed" {
        return Err(format!("Can only retry failed workflows, current status: {}", exec.status));
    }
    
    drop(exec);
    
    if let Some(mut exec) = EXECUTIONS.get_mut(execution_id) {
        exec.status = "running".to_string();
        exec.error = String::new();
        exec.current_step = step_id.to_string();
    }
    
    add_history_event(
        execution_id,
        "step_retried",
        step_id,
        serde_json::json!({
            "step_id": step_id
        }),
    );
    
    Ok(RetryStepOutput { success: true })
}

/// Skip Workflow Step
pub async fn skip_step(
    execution_id: &str,
    step_id: &str,
    output: Option<&str>,
) -> Result<SkipStepOutput, String> {
    let mut exec = EXECUTIONS
        .get_mut(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;
    
    if exec.status != "running" && exec.status != "paused" {
        return Err(format!("Cannot skip step in status: {}", exec.status));
    }
    
    let output_value = if let Some(out) = output {
        serde_json::from_str(out).unwrap_or(Value::String(out.to_string()))
    } else {
        Value::Null
    };
    
    exec.output = output_value.clone();
    
    add_history_event(
        execution_id,
        "step_skipped",
        step_id,
        serde_json::json!({
            "step_id": step_id,
            "output": output_value
        }),
    );
    
    Ok(SkipStepOutput { success: true })
}

/// Terminate All Executions
pub async fn terminate_all(
    workflow_id: &str,
    reason: Option<&str>,
) -> Result<TerminateAllOutput, String> {
    let mut terminated_count = 0;
    let reason_str = reason.unwrap_or("Terminated by system");
    
    for mut entry in EXECUTIONS.iter_mut() {
        let exec = entry.value_mut();
        if exec.workflow_id == workflow_id && (exec.status == "running" || exec.status == "paused" || exec.status == "pending") {
            exec.status = "cancelled".to_string();
            exec.completed_at = Some(Utc::now());
            exec.error = reason_str.to_string();
            terminated_count += 1;
            
            add_history_event(
                &exec.execution_id,
                "workflow_terminated",
                &exec.current_step,
                serde_json::json!({
                    "reason": reason_str
                }),
            );
        }
    }
    
    Ok(TerminateAllOutput {
        terminated_count,
        success: true,
    })
}

#[cfg(test)]
mod tests;
