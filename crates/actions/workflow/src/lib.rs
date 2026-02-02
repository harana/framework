// Harana Actions - Workflow Module
// This module provides workflow orchestration actions and functionality.

pub mod output;

#[cfg(test)]
mod tests;

use chrono::Utc;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use output::*;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

/// Internal storage for workflow executions
static EXECUTIONS: Lazy<DashMap<String, WorkflowExecution>> = Lazy::new(DashMap::new);
/// Storage for signals sent to workflows
static SIGNALS: Lazy<DashMap<String, Vec<(String, Option<Value>)>>> = Lazy::new(DashMap::new);
/// Storage for workflow history events
static HISTORY: Lazy<DashMap<String, Vec<WorkflowHistoryEvent>>> = Lazy::new(DashMap::new);

/// Start Workflow Execution
///
/// Starts a new workflow execution.
///
pub async fn start(
    workflow_id: &str,
    input: Option<Value>,
    context: Option<WorkflowContext>,
) -> Result<StartOutput, String> {
    let execution_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let execution = WorkflowExecution {
        execution_id: execution_id.clone(),
        workflow_id: workflow_id.to_string(),
        status: WorkflowStatus::Running.as_str().to_string(),
        current_step: Some("start".to_string()),
        input,
        output: None,
        started_at: now.clone(),
        completed_at: None,
        error: None,
        context: context.unwrap_or_default(),
        progress: 0.0,
    };

    EXECUTIONS.insert(execution_id.clone(), execution);

    // Record history event
    let event = WorkflowHistoryEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: "WorkflowStarted".to_string(),
        step_id: None,
        timestamp: now,
        data: None,
    };
    HISTORY.entry(execution_id.clone()).or_default().push(event);

    Ok(StartOutput {
        success: true,
        execution_id,
        status: WorkflowStatus::Running.as_str().to_string(),
    })
}

/// Pause Workflow Execution
///
/// Pauses a running workflow execution.
///
pub async fn pause(
    execution_id: &str,
    reason: Option<&str>,
) -> Result<PauseOutput, String> {
    let mut execution = EXECUTIONS
        .get_mut(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;

    if execution.status != WorkflowStatus::Running.as_str() {
        return Err(format!("Cannot pause execution in status: {}", execution.status));
    }

    execution.status = WorkflowStatus::Paused.as_str().to_string();

    // Record history event
    let event = WorkflowHistoryEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: "WorkflowPaused".to_string(),
        step_id: None,
        timestamp: Utc::now().to_rfc3339(),
        data: reason.map(|r| serde_json::json!({ "reason": r })),
    };
    HISTORY.entry(execution_id.to_string()).or_default().push(event);

    Ok(PauseOutput { success: true })
}

/// Resume Paused Workflow
///
/// Resumes a paused workflow execution.
///
pub async fn resume(
    execution_id: &str,
    input: Option<Value>,
) -> Result<ResumeOutput, String> {
    let mut execution = EXECUTIONS
        .get_mut(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;

    if execution.status != WorkflowStatus::Paused.as_str() {
        return Err(format!("Cannot resume execution in status: {}", execution.status));
    }

    execution.status = WorkflowStatus::Running.as_str().to_string();

    // Record history event
    let event = WorkflowHistoryEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: "WorkflowResumed".to_string(),
        step_id: None,
        timestamp: Utc::now().to_rfc3339(),
        data: input,
    };
    HISTORY.entry(execution_id.to_string()).or_default().push(event);

    Ok(ResumeOutput { success: true })
}

/// Cancel Workflow Execution
///
/// Cancels a workflow execution.
///
pub async fn cancel(
    execution_id: &str,
    reason: Option<&str>,
) -> Result<CancelOutput, String> {
    let mut execution = EXECUTIONS
        .get_mut(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;

    let valid_statuses = [
        WorkflowStatus::Running.as_str(),
        WorkflowStatus::Paused.as_str(),
        WorkflowStatus::Pending.as_str(),
    ];

    if !valid_statuses.contains(&execution.status.as_str()) {
        return Err(format!("Cannot cancel execution in status: {}", execution.status));
    }

    execution.status = WorkflowStatus::Cancelled.as_str().to_string();
    execution.completed_at = Some(Utc::now().to_rfc3339());

    // Record history event
    let event = WorkflowHistoryEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: "WorkflowCancelled".to_string(),
        step_id: None,
        timestamp: Utc::now().to_rfc3339(),
        data: reason.map(|r| serde_json::json!({ "reason": r })),
    };
    HISTORY.entry(execution_id.to_string()).or_default().push(event);

    Ok(CancelOutput { success: true })
}

/// Get Workflow Status
///
/// Retrieves the current status of a workflow execution.
///
pub async fn get_status(
    execution_id: &str,
) -> Result<GetStatusOutput, String> {
    let execution = EXECUTIONS
        .get(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;

    Ok(GetStatusOutput {
        status: execution.status.clone(),
        current_step: execution.current_step.clone(),
        progress: Some(execution.progress),
        started_at: Some(execution.started_at.clone()),
        completed_at: execution.completed_at.clone(),
        error: execution.error.clone(),
    })
}

/// Get Workflow Result
///
/// Retrieves the result of a completed workflow execution.
///
pub async fn get_result(
    execution_id: &str,
) -> Result<GetResultOutput, String> {
    let execution = EXECUTIONS
        .get(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;

    let completed = execution.status == WorkflowStatus::Completed.as_str()
        || execution.status == WorkflowStatus::Failed.as_str()
        || execution.status == WorkflowStatus::Cancelled.as_str();

    Ok(GetResultOutput {
        completed,
        output: execution.output.clone(),
        error: execution.error.clone(),
    })
}

/// List Workflow Executions
///
/// Lists workflow executions with optional filtering.
///
pub async fn list_executions(
    workflow_id: Option<&str>,
    status: Option<&str>,
    limit: Option<i32>,
    offset: Option<i32>,
    _start_date: Option<&str>,
    _end_date: Option<&str>,
) -> Result<ListExecutionsOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    let offset = offset.unwrap_or(0) as usize;

    let executions: Vec<WorkflowExecutionInfo> = EXECUTIONS
        .iter()
        .filter(|entry| {
            let matches_workflow = workflow_id
                .map(|wid| entry.workflow_id == wid)
                .unwrap_or(true);
            let matches_status = status
                .map(|s| entry.status.to_lowercase() == s.to_lowercase())
                .unwrap_or(true);
            matches_workflow && matches_status
        })
        .skip(offset)
        .take(limit)
        .map(|entry| WorkflowExecutionInfo {
            execution_id: entry.execution_id.clone(),
            workflow_id: entry.workflow_id.clone(),
            status: entry.status.clone(),
            started_at: Some(entry.started_at.clone()),
            completed_at: entry.completed_at.clone(),
        })
        .collect();

    let total = executions.len() as i32;

    Ok(ListExecutionsOutput { executions, total })
}

/// Send Signal To Workflow
///
/// Sends a signal to a running workflow execution.
///
pub async fn signal(
    execution_id: &str,
    signal_name: &str,
    payload: Option<Value>,
) -> Result<SignalOutput, String> {
    // Verify execution exists and is running
    let execution = EXECUTIONS
        .get(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;

    if execution.status != WorkflowStatus::Running.as_str() 
        && execution.status != WorkflowStatus::Paused.as_str() {
        return Err(format!("Cannot signal execution in status: {}", execution.status));
    }

    // Store the signal
    SIGNALS
        .entry(execution_id.to_string())
        .or_default()
        .push((signal_name.to_string(), payload.clone()));

    // Record history event
    let event = WorkflowHistoryEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: "SignalReceived".to_string(),
        step_id: None,
        timestamp: Utc::now().to_rfc3339(),
        data: Some(serde_json::json!({
            "signal_name": signal_name,
            "payload": payload,
        })),
    };
    HISTORY.entry(execution_id.to_string()).or_default().push(event);

    Ok(SignalOutput { success: true })
}

/// Wait For External Event
///
/// Waits for an external event to be received.
///
pub async fn wait_for_event(
    execution_id: &str,
    event_name: &str,
    timeout: Option<i32>,
) -> Result<WaitForEventOutput, String> {
    let _timeout = timeout.unwrap_or(3600);

    // Check if the event (signal) has already been received
    if let Some(signals) = SIGNALS.get(execution_id) {
        for (name, payload) in signals.iter() {
            if name == event_name {
                return Ok(WaitForEventOutput {
                    received: true,
                    timed_out: false,
                    payload: payload.clone(),
                });
            }
        }
    }

    // In a real implementation, this would block until the event is received or timeout
    // For now, we return not received
    Ok(WaitForEventOutput {
        received: false,
        timed_out: false,
        payload: None,
    })
}

/// Get Workflow History
///
/// Retrieves the history of events for a workflow execution.
///
pub async fn history(
    execution_id: &str,
) -> Result<HistoryOutput, String> {
    // Verify execution exists
    if !EXECUTIONS.contains_key(execution_id) {
        return Err(format!("Execution not found: {}", execution_id));
    }

    let events = HISTORY
        .get(execution_id)
        .map(|h| h.clone())
        .unwrap_or_default();

    let total = events.len() as i32;

    Ok(HistoryOutput { events, total })
}

/// Retry Failed Step
///
/// Retries a failed step in a workflow execution.
///
pub async fn retry_step(
    execution_id: &str,
    step_id: &str,
) -> Result<RetryStepOutput, String> {
    let mut execution = EXECUTIONS
        .get_mut(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;

    if execution.status != WorkflowStatus::Failed.as_str() {
        return Err(format!("Can only retry steps in failed executions"));
    }

    // Reset status to running and set current step
    execution.status = WorkflowStatus::Running.as_str().to_string();
    execution.current_step = Some(step_id.to_string());
    execution.error = None;

    // Record history event
    let event = WorkflowHistoryEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: "StepRetried".to_string(),
        step_id: Some(step_id.to_string()),
        timestamp: Utc::now().to_rfc3339(),
        data: None,
    };
    HISTORY.entry(execution_id.to_string()).or_default().push(event);

    Ok(RetryStepOutput { success: true })
}

/// Skip Workflow Step
///
/// Skips a step in a workflow execution.
///
pub async fn skip_step(
    execution_id: &str,
    step_id: &str,
    output: Option<Value>,
) -> Result<SkipStepOutput, String> {
    let execution = EXECUTIONS
        .get(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;

    if execution.status != WorkflowStatus::Running.as_str()
        && execution.status != WorkflowStatus::Paused.as_str() {
        return Err(format!("Cannot skip step in execution with status: {}", execution.status));
    }

    // Record history event
    let event = WorkflowHistoryEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: "StepSkipped".to_string(),
        step_id: Some(step_id.to_string()),
        timestamp: Utc::now().to_rfc3339(),
        data: output,
    };
    HISTORY.entry(execution_id.to_string()).or_default().push(event);

    Ok(SkipStepOutput { success: true })
}

/// Terminate All Executions
///
/// Terminates all executions of a specific workflow.
///
pub async fn terminate_all(
    workflow_id: &str,
    reason: Option<&str>,
) -> Result<TerminateAllOutput, String> {
    let mut terminated_count = 0;
    let now = Utc::now().to_rfc3339();

    let terminable_statuses = [
        WorkflowStatus::Running.as_str(),
        WorkflowStatus::Paused.as_str(),
        WorkflowStatus::Pending.as_str(),
    ];

    for mut entry in EXECUTIONS.iter_mut() {
        if entry.workflow_id == workflow_id 
            && terminable_statuses.contains(&entry.status.as_str()) {
            entry.status = WorkflowStatus::Cancelled.as_str().to_string();
            entry.completed_at = Some(now.clone());
            terminated_count += 1;

            // Record history event
            let event = WorkflowHistoryEvent {
                event_id: Uuid::new_v4().to_string(),
                event_type: "WorkflowTerminated".to_string(),
                step_id: None,
                timestamp: now.clone(),
                data: reason.map(|r| serde_json::json!({ "reason": r })),
            };
            HISTORY.entry(entry.execution_id.clone()).or_default().push(event);
        }
    }

    Ok(TerminateAllOutput {
        success: true,
        terminated_count,
    })
}

// Helper function to complete a workflow (for internal use)
#[allow(dead_code)]
pub(crate) async fn complete_workflow(
    execution_id: &str,
    output: Option<Value>,
) -> Result<(), String> {
    let mut execution = EXECUTIONS
        .get_mut(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;

    execution.status = WorkflowStatus::Completed.as_str().to_string();
    execution.completed_at = Some(Utc::now().to_rfc3339());
    execution.output = output;
    execution.progress = 100.0;

    Ok(())
}

// Helper function to fail a workflow (for internal use)
#[allow(dead_code)]
pub(crate) async fn fail_workflow(
    execution_id: &str,
    error: &str,
) -> Result<(), String> {
    let mut execution = EXECUTIONS
        .get_mut(execution_id)
        .ok_or_else(|| format!("Execution not found: {}", execution_id))?;

    execution.status = WorkflowStatus::Failed.as_str().to_string();
    execution.completed_at = Some(Utc::now().to_rfc3339());
    execution.error = Some(error.to_string());

    Ok(())
}
