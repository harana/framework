// Harana Actions - Schedule Module
// This module provides schedule actions and functionality.

pub mod output;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use serde_json::Value;
use output::*;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use cron::Schedule as CronSchedule;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct ScheduleData {
    schedule_id: String,
    name: String,
    cron_expression: Option<String>,
    interval_seconds: Option<i32>,
    one_time_run_at: Option<i64>,
    timezone: String,
    enabled: bool,
    description: String,
    metadata: HashMap<String, Value>,
    created_at: i64,
    last_run: Option<i64>,
    paused: bool,
    resume_at: Option<i64>,
    action_type: Option<String>,
    action_config: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
struct ScheduleExecution {
    execution_id: String,
    schedule_id: String,
    started_at: i64,
    completed_at: Option<i64>,
    status: String,
    duration_ms: Option<i64>,
    error: Option<String>,
}

static SCHEDULES: Lazy<DashMap<String, ScheduleData>> = Lazy::new(DashMap::new);
static EXECUTIONS: Lazy<RwLock<Vec<ScheduleExecution>>> = Lazy::new(|| RwLock::new(Vec::new()));

/// Helper to parse timezone
fn parse_timezone(tz_str: &str) -> Result<Tz, String> {
    Tz::from_str(tz_str).map_err(|_| format!("Invalid timezone: {}", tz_str))
}

/// Helper to get next run time for a cron expression
fn get_next_run_time(cron_expr: &str, tz_str: &str) -> Result<i64, String> {
    let schedule = CronSchedule::from_str(cron_expr)
        .map_err(|e| format!("Invalid cron expression: {}", e))?;
    let tz = parse_timezone(tz_str)?;
    let now = Utc::now().with_timezone(&tz);
    
    schedule
        .upcoming(tz)
        .next()
        .map(|dt| dt.timestamp())
        .ok_or_else(|| "Could not calculate next run time".to_string())
}

/// Helper to get multiple next run times
fn get_next_run_times(cron_expr: &str, tz_str: &str, count: usize) -> Result<Vec<i64>, String> {
    let schedule = CronSchedule::from_str(cron_expr)
        .map_err(|e| format!("Invalid cron expression: {}", e))?;
    let tz = parse_timezone(tz_str)?;
    
    Ok(schedule
        .upcoming(tz)
        .take(count)
        .map(|dt| dt.timestamp())
        .collect())
}

/// Bulk Disable Schedules
pub async fn bulk_disable(
    schedule_ids: Vec<String>,
) -> Result<BulkDisableOutput, String> {
    let mut successful = 0;
    let mut failed = 0;
    let mut results = Vec::new();
    
    for schedule_id in schedule_ids {
        match disable_schedule(&schedule_id).await {
            Ok(_) => {
                successful += 1;
                let mut result = HashMap::new();
                result.insert("schedule_id".to_string(), Value::String(schedule_id.clone()));
                result.insert("success".to_string(), Value::Bool(true));
                results.push(result);
            }
            Err(e) => {
                failed += 1;
                let mut result = HashMap::new();
                result.insert("schedule_id".to_string(), Value::String(schedule_id.clone()));
                result.insert("success".to_string(), Value::Bool(false));
                result.insert("error".to_string(), Value::String(e));
                results.push(result);
            }
        }
    }
    
    Ok(BulkDisableOutput {
        successful,
        failed,
        results,
    })
}

/// Bulk Enable Schedules
pub async fn bulk_enable(
    schedule_ids: Vec<String>,
) -> Result<BulkEnableOutput, String> {
    let mut successful = 0;
    let mut failed = 0;
    let mut results = Vec::new();
    
    for schedule_id in schedule_ids {
        match enable_schedule(&schedule_id).await {
            Ok(_) => {
                successful += 1;
                let mut result = HashMap::new();
                result.insert("schedule_id".to_string(), Value::String(schedule_id.clone()));
                result.insert("success".to_string(), Value::Bool(true));
                results.push(result);
            }
            Err(e) => {
                failed += 1;
                let mut result = HashMap::new();
                result.insert("schedule_id".to_string(), Value::String(schedule_id.clone()));
                result.insert("success".to_string(), Value::Bool(false));
                result.insert("error".to_string(), Value::String(e));
                results.push(result);
            }
        }
    }
    
    Ok(BulkEnableOutput {
        successful,
        failed,
        results,
    })
}

/// Create Interval Schedule
pub async fn create_interval(
    name: &str,
    interval_seconds: i32,
    end_time: Option<i32>,
    description: Option<&str>,
    start_time: Option<i32>,
    enabled: Option<bool>,
) -> Result<CreateIntervalOutput, String> {
    let schedule_id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();
    let start_time_val = start_time.map(|t| t as i64).unwrap_or(now);
    
    let schedule_data = ScheduleData {
        schedule_id: schedule_id.clone(),
        name: name.to_string(),
        cron_expression: None,
        interval_seconds: Some(interval_seconds),
        one_time_run_at: None,
        timezone: "UTC".to_string(),
        enabled: enabled.unwrap_or(true),
        description: description.unwrap_or("").to_string(),
        metadata: HashMap::new(),
        created_at: now,
        last_run: None,
        paused: false,
        resume_at: None,
        action_type: None,
        action_config: HashMap::new(),
    };
    
    SCHEDULES.insert(schedule_id.clone(), schedule_data);
    
    let next_run = start_time_val + interval_seconds as i64;
    
    Ok(CreateIntervalOutput {
        schedule_id,
        success: true,
        next_run: next_run as i32,
    })
}

/// Create One-Time Schedule
pub async fn create_one_time(
    name: &str,
    run_at: i32,
    metadata: Option<HashMap<String, Value>>,
    timezone: Option<&str>,
    description: Option<&str>,
) -> Result<CreateOneTimeOutput, String> {
    let schedule_id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();
    
    let schedule_data = ScheduleData {
        schedule_id: schedule_id.clone(),
        name: name.to_string(),
        cron_expression: None,
        interval_seconds: None,
        one_time_run_at: Some(run_at as i64),
        timezone: timezone.unwrap_or("UTC").to_string(),
        enabled: true,
        description: description.unwrap_or("").to_string(),
        metadata: metadata.unwrap_or_default(),
        created_at: now,
        last_run: None,
        paused: false,
        resume_at: None,
        action_type: None,
        action_config: HashMap::new(),
    };
    
    SCHEDULES.insert(schedule_id.clone(), schedule_data);
    
    Ok(CreateOneTimeOutput {
        schedule_id,
        success: true,
        run_at,
    })
}

/// Create Schedule
pub async fn create_schedule(
    cron_expression: &str,
    name: &str,
    metadata: Option<HashMap<String, Value>>,
    timezone: Option<&str>,
    enabled: Option<bool>,
    description: Option<&str>,
) -> Result<CreateScheduleOutput, String> {
    let tz = timezone.unwrap_or("UTC");
    
    // Validate cron expression
    let next_run = get_next_run_time(cron_expression, tz)?;
    
    let schedule_id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();
    
    let schedule_data = ScheduleData {
        schedule_id: schedule_id.clone(),
        name: name.to_string(),
        cron_expression: Some(cron_expression.to_string()),
        interval_seconds: None,
        one_time_run_at: None,
        timezone: tz.to_string(),
        enabled: enabled.unwrap_or(true),
        description: description.unwrap_or("").to_string(),
        metadata: metadata.unwrap_or_default(),
        created_at: now,
        last_run: None,
        paused: false,
        resume_at: None,
        action_type: None,
        action_config: HashMap::new(),
    };
    
    SCHEDULES.insert(schedule_id.clone(), schedule_data);
    
    Ok(CreateScheduleOutput {
        schedule_id,
        name: name.to_string(),
        next_run: next_run as i32,
        success: true,
    })
}

/// Delete Schedule
pub async fn delete_schedule(
    schedule_id: &str,
) -> Result<DeleteScheduleOutput, String> {
    SCHEDULES
        .remove(schedule_id)
        .ok_or_else(|| format!("Schedule not found: {}", schedule_id))?;
    
    Ok(DeleteScheduleOutput {
        schedule_id: schedule_id.to_string(),
        success: true,
    })
}

/// Disable Schedule
pub async fn disable_schedule(
    schedule_id: &str,
) -> Result<DisableScheduleOutput, String> {
    let mut schedule = SCHEDULES
        .get_mut(schedule_id)
        .ok_or_else(|| format!("Schedule not found: {}", schedule_id))?;
    
    schedule.enabled = false;
    
    Ok(DisableScheduleOutput {
        schedule_id: schedule_id.to_string(),
        enabled: false,
        success: true,
    })
}

/// Enable Schedule
pub async fn enable_schedule(
    schedule_id: &str,
) -> Result<EnableScheduleOutput, String> {
    let mut schedule = SCHEDULES
        .get_mut(schedule_id)
        .ok_or_else(|| format!("Schedule not found: {}", schedule_id))?;
    
    schedule.enabled = true;
    
    // Calculate next run time
    let next_run = if let Some(cron_expr) = &schedule.cron_expression {
        get_next_run_time(cron_expr, &schedule.timezone)? as i32
    } else if let Some(interval) = schedule.interval_seconds {
        (Utc::now().timestamp() + interval as i64) as i32
    } else if let Some(run_at) = schedule.one_time_run_at {
        run_at as i32
    } else {
        0
    };
    
    Ok(EnableScheduleOutput {
        schedule_id: schedule_id.to_string(),
        enabled: true,
        next_run,
        success: true,
    })
}

/// Get Next Run Time
pub async fn get_next_run(
    schedule_id: &str,
    count: Option<i32>,
) -> Result<GetNextRunOutput, String> {
    let schedule = SCHEDULES
        .get(schedule_id)
        .ok_or_else(|| format!("Schedule not found: {}", schedule_id))?;
    
    let count = count.unwrap_or(1).max(1) as usize;
    
    let next_runs: Vec<i32> = if let Some(cron_expr) = &schedule.cron_expression {
        get_next_run_times(cron_expr, &schedule.timezone, count)?
            .into_iter()
            .map(|t| t as i32)
            .collect()
    } else if let Some(interval) = schedule.interval_seconds {
        let mut runs = Vec::new();
        let mut next = Utc::now().timestamp();
        for _ in 0..count {
            next += interval as i64;
            runs.push(next as i32);
        }
        runs
    } else if let Some(run_at) = schedule.one_time_run_at {
        vec![run_at as i32]
    } else {
        Vec::new()
    };
    
    Ok(GetNextRunOutput {
        schedule_id: schedule_id.to_string(),
        next_runs,
    })
}

/// Get Schedule
pub async fn get_schedule(
    schedule_id: &str,
) -> Result<GetScheduleOutput, String> {
    let schedule = SCHEDULES
        .get(schedule_id)
        .ok_or_else(|| format!("Schedule not found: {}", schedule_id))?;
    
    let next_run = if schedule.enabled && !schedule.paused {
        if let Some(cron_expr) = &schedule.cron_expression {
            get_next_run_time(cron_expr, &schedule.timezone).unwrap_or(0) as i32
        } else if let Some(interval) = schedule.interval_seconds {
            let last = schedule.last_run.unwrap_or(schedule.created_at);
            (last + interval as i64) as i32
        } else if let Some(run_at) = schedule.one_time_run_at {
            run_at as i32
        } else {
            0
        }
    } else {
        0
    };
    
    Ok(GetScheduleOutput {
        schedule_id: schedule_id.to_string(),
        name: schedule.name.clone(),
        cron_expression: schedule.cron_expression.clone().unwrap_or_default(),
        timezone: schedule.timezone.clone(),
        enabled: schedule.enabled,
        metadata: schedule.metadata.clone(),
        created_at: schedule.created_at as i32,
        last_run: schedule.last_run.map(|t| t as i32).unwrap_or(0),
        next_run,
    })
}

/// Get Schedule History
pub async fn get_schedule_history(
    schedule_id: &str,
    start_date: Option<i32>,
    limit: Option<i32>,
    end_date: Option<i32>,
) -> Result<GetScheduleHistoryOutput, String> {
    let executions = EXECUTIONS.read();
    let limit = limit.unwrap_or(100).max(1) as usize;
    
    let filtered: Vec<HashMap<String, Value>> = executions
        .iter()
        .filter(|e| e.schedule_id == schedule_id)
        .filter(|e| {
            if let Some(start) = start_date {
                e.started_at >= start as i64
            } else {
                true
            }
        })
        .filter(|e| {
            if let Some(end) = end_date {
                e.started_at <= end as i64
            } else {
                true
            }
        })
        .take(limit)
        .map(|e| {
            let mut map = HashMap::new();
            map.insert("execution_id".to_string(), Value::String(e.execution_id.clone()));
            map.insert("schedule_id".to_string(), Value::String(e.schedule_id.clone()));
            map.insert("started_at".to_string(), Value::Number((e.started_at as i32).into()));
            if let Some(completed) = e.completed_at {
                map.insert("completed_at".to_string(), Value::Number((completed as i32).into()));
            }
            map.insert("status".to_string(), Value::String(e.status.clone()));
            if let Some(duration) = e.duration_ms {
                map.insert("duration_ms".to_string(), Value::Number(duration.into()));
            }
            if let Some(err) = &e.error {
                map.insert("error".to_string(), Value::String(err.clone()));
            }
            map
        })
        .collect();
    
    let total = filtered.len() as i32;
    
    Ok(GetScheduleHistoryOutput {
        schedule_id: schedule_id.to_string(),
        executions: filtered,
        total,
    })
}

/// Get Schedule Stats
pub async fn get_schedule_stats(
    schedule_id: &str,
    start_date: Option<i32>,
    end_date: Option<i32>,
) -> Result<GetScheduleStatsOutput, String> {
    let executions = EXECUTIONS.read();
    
    let filtered: Vec<&ScheduleExecution> = executions
        .iter()
        .filter(|e| e.schedule_id == schedule_id)
        .filter(|e| {
            if let Some(start) = start_date {
                e.started_at >= start as i64
            } else {
                true
            }
        })
        .filter(|e| {
            if let Some(end) = end_date {
                e.started_at <= end as i64
            } else {
                true
            }
        })
        .collect();
    
    let total_executions = filtered.len() as i32;
    let successful_executions = filtered.iter().filter(|e| e.status == "success").count() as i32;
    let failed_executions = filtered.iter().filter(|e| e.status == "failed").count() as i32;
    
    let total_duration: i64 = filtered
        .iter()
        .filter_map(|e| e.duration_ms)
        .sum();
    
    let average_duration = if total_executions > 0 {
        total_duration as f64 / total_executions as f64
    } else {
        0.0
    };
    
    let last_execution = filtered.last().map(|e| {
        let mut map = HashMap::new();
        map.insert("execution_id".to_string(), Value::String(e.execution_id.clone()));
        map.insert("schedule_id".to_string(), Value::String(e.schedule_id.clone()));
        map.insert("started_at".to_string(), Value::Number((e.started_at as i32).into()));
        if let Some(completed) = e.completed_at {
            map.insert("completed_at".to_string(), Value::Number((completed as i32).into()));
        }
        map.insert("status".to_string(), Value::String(e.status.clone()));
        if let Some(duration) = e.duration_ms {
            map.insert("duration_ms".to_string(), Value::Number(duration.into()));
        }
        map
    }).unwrap_or_default();
    
    Ok(GetScheduleStatsOutput {
        schedule_id: schedule_id.to_string(),
        total_executions,
        successful_executions,
        failed_executions,
        average_duration,
        last_execution,
    })
}

/// List Schedules
pub async fn list_schedules(
    offset: Option<i32>,
    search: Option<&str>,
    limit: Option<i32>,
    enabled: Option<bool>,
) -> Result<ListSchedulesOutput, String> {
    let offset = offset.unwrap_or(0).max(0) as usize;
    let limit = limit.unwrap_or(100).max(1) as usize;
    
    let mut all_schedules: Vec<_> = SCHEDULES
        .iter()
        .filter(|entry| {
            if let Some(en) = enabled {
                entry.value().enabled == en
            } else {
                true
            }
        })
        .filter(|entry| {
            if let Some(s) = search {
                entry.value().name.to_lowercase().contains(&s.to_lowercase())
            } else {
                true
            }
        })
        .map(|entry| {
            let schedule = entry.value();
            let mut map = HashMap::new();
            map.insert("schedule_id".to_string(), Value::String(schedule.schedule_id.clone()));
            map.insert("name".to_string(), Value::String(schedule.name.clone()));
            map.insert("cron_expression".to_string(), 
                Value::String(schedule.cron_expression.clone().unwrap_or_default()));
            map.insert("enabled".to_string(), Value::Bool(schedule.enabled));
            
            let next_run = if schedule.enabled && !schedule.paused {
                if let Some(cron_expr) = &schedule.cron_expression {
                    get_next_run_time(cron_expr, &schedule.timezone).unwrap_or(0) as i32
                } else if let Some(interval) = schedule.interval_seconds {
                    let last = schedule.last_run.unwrap_or(schedule.created_at);
                    (last + interval as i64) as i32
                } else if let Some(run_at) = schedule.one_time_run_at {
                    run_at as i32
                } else {
                    0
                }
            } else {
                0
            };
            map.insert("next_run".to_string(), Value::Number(next_run.into()));
            map.insert("last_run".to_string(), 
                Value::Number(schedule.last_run.map(|t| t as i32).unwrap_or(0).into()));
            map
        })
        .collect();
    
    let total = all_schedules.len() as i32;
    
    all_schedules.sort_by_key(|s| {
        s.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    });
    
    let schedules = all_schedules
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();
    
    Ok(ListSchedulesOutput {
        schedules,
        total,
    })
}

/// Pause Schedule
pub async fn pause_schedule(
    schedule_id: &str,
    resume_at: Option<i32>,
) -> Result<PauseScheduleOutput, String> {
    let mut schedule = SCHEDULES
        .get_mut(schedule_id)
        .ok_or_else(|| format!("Schedule not found: {}", schedule_id))?;
    
    schedule.paused = true;
    schedule.resume_at = resume_at.map(|t| t as i64);
    
    Ok(PauseScheduleOutput {
        schedule_id: schedule_id.to_string(),
        paused: true,
        resume_at: resume_at.unwrap_or(0),
        success: true,
    })
}

/// Resume Schedule
pub async fn resume_schedule(
    schedule_id: &str,
) -> Result<ResumeScheduleOutput, String> {
    let mut schedule = SCHEDULES
        .get_mut(schedule_id)
        .ok_or_else(|| format!("Schedule not found: {}", schedule_id))?;
    
    schedule.paused = false;
    schedule.resume_at = None;
    
    let next_run = if schedule.enabled {
        if let Some(cron_expr) = &schedule.cron_expression {
            get_next_run_time(cron_expr, &schedule.timezone).unwrap_or(0) as i32
        } else if let Some(interval) = schedule.interval_seconds {
            (Utc::now().timestamp() + interval as i64) as i32
        } else if let Some(run_at) = schedule.one_time_run_at {
            run_at as i32
        } else {
            0
        }
    } else {
        0
    };
    
    Ok(ResumeScheduleOutput {
        schedule_id: schedule_id.to_string(),
        paused: false,
        next_run,
        success: true,
    })
}

/// Trigger Schedule Immediately
pub async fn trigger_schedule(
    schedule_id: &str,
) -> Result<TriggerScheduleOutput, String> {
    let schedule = SCHEDULES
        .get(schedule_id)
        .ok_or_else(|| format!("Schedule not found: {}", schedule_id))?;
    
    let execution_id = Uuid::new_v4().to_string();
    let triggered_at = Utc::now().timestamp();
    
    // Create execution record
    let execution = ScheduleExecution {
        execution_id: execution_id.clone(),
        schedule_id: schedule_id.to_string(),
        started_at: triggered_at,
        completed_at: Some(triggered_at), // Simulate immediate completion
        status: "triggered".to_string(),
        duration_ms: Some(0),
        error: None,
    };
    
    EXECUTIONS.write().push(execution);
    
    Ok(TriggerScheduleOutput {
        execution_id,
        schedule_id: schedule_id.to_string(),
        triggered_at: triggered_at as i32,
        success: true,
    })
}

/// Update Schedule Action
pub async fn update_action(
    schedule_id: &str,
    action_config: HashMap<String, Value>,
    action_type: &str,
) -> Result<UpdateActionOutput, String> {
    let mut schedule = SCHEDULES
        .get_mut(schedule_id)
        .ok_or_else(|| format!("Schedule not found: {}", schedule_id))?;
    
    schedule.action_type = Some(action_type.to_string());
    schedule.action_config = action_config;
    
    Ok(UpdateActionOutput {
        schedule_id: schedule_id.to_string(),
        success: true,
    })
}

/// Update Schedule
pub async fn update_schedule(
    schedule_id: &str,
    cron_expression: Option<&str>,
    name: Option<&str>,
    timezone: Option<&str>,
    enabled: Option<bool>,
    metadata: Option<HashMap<String, Value>>,
    description: Option<&str>,
) -> Result<UpdateScheduleOutput, String> {
    let mut schedule = SCHEDULES
        .get_mut(schedule_id)
        .ok_or_else(|| format!("Schedule not found: {}", schedule_id))?;
    
    if let Some(cron) = cron_expression {
        // Validate cron expression before updating
        let tz = timezone.unwrap_or(&schedule.timezone);
        get_next_run_time(cron, tz)?;
        schedule.cron_expression = Some(cron.to_string());
    }
    
    if let Some(n) = name {
        schedule.name = n.to_string();
    }
    
    if let Some(tz) = timezone {
        parse_timezone(tz)?; // Validate timezone
        schedule.timezone = tz.to_string();
    }
    
    if let Some(en) = enabled {
        schedule.enabled = en;
    }
    
    if let Some(meta) = metadata {
        schedule.metadata = meta;
    }
    
    if let Some(desc) = description {
        schedule.description = desc.to_string();
    }
    
    let next_run = if schedule.enabled && !schedule.paused {
        if let Some(cron_expr) = &schedule.cron_expression {
            get_next_run_time(cron_expr, &schedule.timezone).unwrap_or(0) as i32
        } else {
            0
        }
    } else {
        0
    };
    
    Ok(UpdateScheduleOutput {
        schedule_id: schedule_id.to_string(),
        next_run,
        success: true,
    })
}

/// Validate Cron Expression
pub async fn validate_cron(
    cron_expression: &str,
    timezone: Option<&str>,
) -> Result<ValidateCronOutput, String> {
    let tz = timezone.unwrap_or("UTC");
    
    match get_next_run_time(cron_expression, tz) {
        Ok(next_run) => Ok(ValidateCronOutput {
            valid: true,
            error: String::new(),
            next_run: next_run as i32,
        }),
        Err(e) => Ok(ValidateCronOutput {
            valid: false,
            error: e,
            next_run: 0,
        }),
    }
}
