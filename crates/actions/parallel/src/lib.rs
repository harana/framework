pub mod output;

use output::*;
use futures::future::{join_all, select_all};
use serde_json::Value;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time::timeout as tokio_timeout;

/// Execute a task using its handler
/// This is a simplified implementation - in production, handlers would be registered
async fn execute_task(task: &ParallelTask) -> Result<Value, String> {
    // Simulate task execution based on handler name
    // In production, this would look up and call actual registered handlers
    match task.handler.as_str() {
        "echo" => Ok(task.input.clone().unwrap_or(Value::Null)),
        "double" => {
            if let Some(Value::Number(n)) = &task.input {
                if let Some(num) = n.as_i64() {
                    return Ok(Value::Number((num * 2).into()));
                }
                if let Some(num) = n.as_f64() {
                    return Ok(Value::Number(serde_json::Number::from_f64(num * 2.0).unwrap()));
                }
            }
            Err("Invalid input for double handler".to_string())
        }
        "fail" => Err("Intentional failure".to_string()),
        "slow" => {
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok(task.input.clone().unwrap_or(Value::Null))
        }
        "increment" => {
            if let Some(Value::Number(n)) = &task.input {
                if let Some(num) = n.as_i64() {
                    return Ok(Value::Number((num + 1).into()));
                }
            }
            Err("Invalid input for increment handler".to_string())
        }
        "is_even" => {
            if let Some(Value::Number(n)) = &task.input {
                if let Some(num) = n.as_i64() {
                    return Ok(Value::Bool(num % 2 == 0));
                }
            }
            Err("Invalid input for is_even handler".to_string())
        }
        "sum" => {
            // Expects input as array [accumulator, current]
            if let Some(Value::Array(arr)) = &task.input {
                if arr.len() == 2 {
                    if let (Some(a), Some(b)) = (arr[0].as_i64(), arr[1].as_i64()) {
                        return Ok(Value::Number((a + b).into()));
                    }
                }
            }
            Err("Invalid input for sum handler".to_string())
        }
        _ => {
            // Default: return input as-is
            Ok(task.input.clone().unwrap_or(Value::Null))
        }
    }
}

/// Execute Tasks In Parallel.
pub async fn all(
    tasks: Vec<ParallelTask>,
    fail_fast: Option<bool>,
    max_concurrency: Option<i32>,
    timeout_ms: Option<i64>,
) -> Result<AllOutput, String> {
    let fail_fast = fail_fast.unwrap_or(true);
    let max_concurrency = max_concurrency.unwrap_or(tasks.len() as i32) as usize;
    let semaphore = Arc::new(Semaphore::new(max_concurrency));
    
    let mut handles = Vec::new();
    
    for task in tasks {
        let sem = semaphore.clone();
        let task_timeout = timeout_ms.or(task.timeout_ms);
        
        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            let start = Instant::now();
            
            let result = if let Some(timeout) = task_timeout {
                match tokio_timeout(Duration::from_millis(timeout as u64), execute_task(&task)).await {
                    Ok(r) => r,
                    Err(_) => Err("Task timed out".to_string()),
                }
            } else {
                execute_task(&task).await
            };
            
            let duration_ms = start.elapsed().as_millis() as i64;
            
            match result {
                Ok(value) => ParallelResult {
                    task_id: task.id,
                    success: true,
                    result: Some(value),
                    error: None,
                    duration_ms,
                },
                Err(e) => ParallelResult {
                    task_id: task.id,
                    success: false,
                    result: None,
                    error: Some(e),
                    duration_ms,
                },
            }
        });
        
        handles.push(handle);
    }
    
    let mut results = Vec::new();
    let mut completed = 0;
    let mut failed = 0;
    
    for handle in handles {
        let result = handle.await.map_err(|e| e.to_string())?;
        if result.success {
            completed += 1;
        } else {
            failed += 1;
            if fail_fast {
                results.push(result);
                return Ok(AllOutput {
                    completed,
                    failed,
                    results,
                    success: false,
                });
            }
        }
        results.push(result);
    }
    
    Ok(AllOutput {
        completed,
        failed,
        results,
        success: failed == 0,
    })
}

/// Execute First Successful Task (race).
pub async fn race(
    tasks: Vec<ParallelTask>,
    timeout_ms: Option<i64>,
) -> Result<RaceOutput, String> {
    if tasks.is_empty() {
        return Err("No tasks provided".to_string());
    }
    
    let futures: Vec<_> = tasks
        .into_iter()
        .enumerate()
        .map(|(idx, task)| {
            Box::pin(async move {
                let result = execute_task(&task).await;
                (idx, result)
            })
        })
        .collect();
    
    let race_future = async {
        let (result, idx, _) = select_all(futures).await;
        (idx, result)
    };
    
    let (winner_index, (_, result)) = if let Some(timeout) = timeout_ms {
        match tokio_timeout(Duration::from_millis(timeout as u64), race_future).await {
            Ok(r) => r,
            Err(_) => {
                return Ok(RaceOutput {
                    result: None,
                    success: false,
                    winner_index: -1,
                });
            }
        }
    } else {
        race_future.await
    };
    
    match result {
        Ok(value) => Ok(RaceOutput {
            result: Some(value),
            success: true,
            winner_index: winner_index as i32,
        }),
        Err(_) => Ok(RaceOutput {
            result: None,
            success: false,
            winner_index: winner_index as i32,
        }),
    }
}

/// Execute Any Successful Task.
pub async fn any(
    tasks: Vec<ParallelTask>,
    timeout_ms: Option<i64>,
) -> Result<AnyOutput, String> {
    if tasks.is_empty() {
        return Err("No tasks provided".to_string());
    }
    
    let mut futures: Vec<_> = tasks
        .into_iter()
        .enumerate()
        .map(|(idx, task)| {
            Box::pin(async move {
                let result = execute_task(&task).await;
                (idx, result)
            })
        })
        .collect();
    
    let any_future = async {
        while !futures.is_empty() {
            let (result, _idx, remaining) = select_all(futures).await;
            futures = remaining;
            
            if let (original_idx, Ok(value)) = result {
                return Some((original_idx, value));
            }
        }
        None
    };
    
    let result = if let Some(timeout) = timeout_ms {
        match tokio_timeout(Duration::from_millis(timeout as u64), any_future).await {
            Ok(r) => r,
            Err(_) => None,
        }
    } else {
        any_future.await
    };
    
    match result {
        Some((idx, value)) => Ok(AnyOutput {
            result: Some(value),
            success: true,
            success_index: idx as i32,
        }),
        None => Ok(AnyOutput {
            result: None,
            success: false,
            success_index: -1,
        }),
    }
}

/// Execute All And Settle.
pub async fn all_settled(
    tasks: Vec<ParallelTask>,
    max_concurrency: Option<i32>,
    timeout_ms: Option<i64>,
) -> Result<AllSettledOutput, String> {
    let max_concurrency = max_concurrency.unwrap_or(tasks.len() as i32) as usize;
    let semaphore = Arc::new(Semaphore::new(max_concurrency));
    
    let futures: Vec<_> = tasks
        .into_iter()
        .map(|task| {
            let sem = semaphore.clone();
            let task_timeout = timeout_ms.or(task.timeout_ms);
            
            async move {
                let _permit = sem.acquire().await.unwrap();
                let start = Instant::now();
                
                let result = if let Some(timeout) = task_timeout {
                    match tokio_timeout(Duration::from_millis(timeout as u64), execute_task(&task)).await {
                        Ok(r) => r,
                        Err(_) => Err("Task timed out".to_string()),
                    }
                } else {
                    execute_task(&task).await
                };
                
                let duration_ms = start.elapsed().as_millis() as i64;
                
                match result {
                    Ok(value) => ParallelSettledResult {
                        task_id: task.id,
                        status: "fulfilled".to_string(),
                        value: Some(value),
                        reason: None,
                        duration_ms,
                    },
                    Err(e) => ParallelSettledResult {
                        task_id: task.id,
                        status: "rejected".to_string(),
                        value: None,
                        reason: Some(e),
                        duration_ms,
                    },
                }
            }
        })
        .collect();
    
    let results: Vec<ParallelSettledResult> = join_all(futures).await;
    
    let fulfilled = results.iter().filter(|r| r.status == "fulfilled").count() as i32;
    let rejected = results.iter().filter(|r| r.status == "rejected").count() as i32;
    
    Ok(AllSettledOutput {
        fulfilled,
        rejected,
        results,
    })
}

/// Map Items In Parallel.
pub async fn map(
    handler: &str,
    items: Vec<Value>,
    max_concurrency: Option<i32>,
) -> Result<MapOutput, String> {
    let max_concurrency = max_concurrency.unwrap_or(10) as usize;
    let semaphore = Arc::new(Semaphore::new(max_concurrency));
    let handler = handler.to_string();
    
    let futures: Vec<_> = items
        .into_iter()
        .enumerate()
        .map(|(idx, item)| {
            let sem = semaphore.clone();
            let h = handler.clone();
            
            async move {
                let _permit = sem.acquire().await.unwrap();
                let task = ParallelTask {
                    id: format!("map_{}", idx),
                    handler: h,
                    input: Some(item),
                    timeout_ms: None,
                };
                (idx, execute_task(&task).await)
            }
        })
        .collect();
    
    let results_raw: Vec<(usize, Result<Value, String>)> = join_all(futures).await;
    
    let mut results = Vec::new();
    let mut errors = Vec::new();
    
    for (idx, result) in results_raw {
        match result {
            Ok(value) => results.push(value),
            Err(e) => {
                errors.push(ParallelError {
                    task_id: format!("map_{}", idx),
                    index: idx as i32,
                    message: e,
                    code: "MAP_ERROR".to_string(),
                });
                results.push(Value::Null);
            }
        }
    }
    
    Ok(MapOutput {
        errors: errors.clone(),
        results,
        success: errors.is_empty(),
    })
}

/// Filter Items In Parallel.
pub async fn filter(
    handler: &str,
    items: Vec<Value>,
    max_concurrency: Option<i32>,
) -> Result<FilterOutput, String> {
    let max_concurrency = max_concurrency.unwrap_or(10) as usize;
    let semaphore = Arc::new(Semaphore::new(max_concurrency));
    let handler = handler.to_string();
    
    let futures: Vec<_> = items
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, item)| {
            let sem = semaphore.clone();
            let h = handler.clone();
            let item_clone = item.clone();
            
            async move {
                let _permit = sem.acquire().await.unwrap();
                let task = ParallelTask {
                    id: format!("filter_{}", idx),
                    handler: h,
                    input: Some(item_clone.clone()),
                    timeout_ms: None,
                };
                let result = execute_task(&task).await;
                (item_clone, result)
            }
        })
        .collect();
    
    let results_raw: Vec<(Value, Result<Value, String>)> = join_all(futures).await;
    
    let results: Vec<Value> = results_raw
        .into_iter()
        .filter_map(|(item, result)| {
            match result {
                Ok(Value::Bool(true)) => Some(item),
                _ => None,
            }
        })
        .collect();
    
    let count = results.len() as i32;
    
    Ok(FilterOutput { count, results })
}

/// Reduce Items In Parallel.
pub async fn reduce(
    handler: &str,
    items: Vec<Value>,
    initial_value: Option<Value>,
    _max_concurrency: Option<i32>,
) -> Result<ReduceOutput, String> {
    // Note: reduce is inherently sequential, but we use async for consistency
    let mut accumulator = initial_value.unwrap_or(Value::Number(0.into()));
    
    for (idx, item) in items.into_iter().enumerate() {
        let task = ParallelTask {
            id: format!("reduce_{}", idx),
            handler: handler.to_string(),
            input: Some(Value::Array(vec![accumulator.clone(), item])),
            timeout_ms: None,
        };
        
        accumulator = execute_task(&task).await?;
    }
    
    Ok(ReduceOutput { result: accumulator })
}

/// Execute With Retry.
pub async fn retry(
    task: ParallelTask,
    backoff_multiplier: Option<f64>,
    delay_ms: Option<i64>,
    max_attempts: Option<i32>,
) -> Result<RetryOutput, String> {
    let max_attempts = max_attempts.unwrap_or(3);
    let initial_delay = delay_ms.unwrap_or(1000) as u64;
    let multiplier = backoff_multiplier.unwrap_or(2.0);
    
    let mut attempts = 0;
    let mut current_delay = initial_delay;
    
    loop {
        attempts += 1;
        
        match execute_task(&task).await {
            Ok(value) => {
                return Ok(RetryOutput {
                    attempts,
                    result: Some(value),
                    success: true,
                });
            }
            Err(_e) => {
                if attempts >= max_attempts {
                    return Ok(RetryOutput {
                        attempts,
                        result: None,
                        success: false,
                    });
                }
                
                tokio::time::sleep(Duration::from_millis(current_delay)).await;
                current_delay = (current_delay as f64 * multiplier) as u64;
            }
        }
    }
}

/// Execute With Timeout.
pub async fn timeout(
    task: ParallelTask,
    timeout_ms: i64,
) -> Result<TimeoutOutput, String> {
    let result = tokio_timeout(
        Duration::from_millis(timeout_ms as u64),
        execute_task(&task),
    ).await;
    
    match result {
        Ok(Ok(value)) => Ok(TimeoutOutput {
            result: Some(value),
            success: true,
            timed_out: false,
        }),
        Ok(Err(_)) => Ok(TimeoutOutput {
            result: None,
            success: false,
            timed_out: false,
        }),
        Err(_) => Ok(TimeoutOutput {
            result: None,
            success: false,
            timed_out: true,
        }),
    }
}

/// Execute In Batches.
pub async fn batch(
    handler: &str,
    items: Vec<Value>,
    batch_size: Option<i32>,
    max_concurrency: Option<i32>,
) -> Result<BatchOutput, String> {
    let batch_size = batch_size.unwrap_or(10) as usize;
    let max_concurrency = max_concurrency.unwrap_or(1) as usize;
    let semaphore = Arc::new(Semaphore::new(max_concurrency));
    
    let batches: Vec<Vec<Value>> = items
        .chunks(batch_size)
        .map(|chunk| chunk.to_vec())
        .collect();
    
    let batch_count = batches.len() as i32;
    let handler = handler.to_string();
    
    let futures: Vec<_> = batches
        .into_iter()
        .enumerate()
        .map(|(batch_idx, batch_items)| {
            let sem = semaphore.clone();
            let h = handler.clone();
            
            async move {
                let _permit = sem.acquire().await.unwrap();
                
                let mut batch_results = Vec::new();
                for (item_idx, item) in batch_items.into_iter().enumerate() {
                    let task = ParallelTask {
                        id: format!("batch_{}_{}", batch_idx, item_idx),
                        handler: h.clone(),
                        input: Some(item),
                        timeout_ms: None,
                    };
                    
                    match execute_task(&task).await {
                        Ok(value) => batch_results.push(value),
                        Err(_) => batch_results.push(Value::Null),
                    }
                }
                
                batch_results
            }
        })
        .collect();
    
    let batch_results: Vec<Vec<Value>> = join_all(futures).await;
    let results: Vec<Value> = batch_results.into_iter().flatten().collect();
    
    Ok(BatchOutput {
        batch_count,
        results,
        success: true,
    })
}

/// Execute With Semaphore.
pub async fn semaphore(
    max_concurrent: i32,
    tasks: Vec<ParallelTask>,
) -> Result<SemaphoreOutput, String> {
    let semaphore = Arc::new(Semaphore::new(max_concurrent as usize));
    
    let futures: Vec<_> = tasks
        .into_iter()
        .map(|task| {
            let sem = semaphore.clone();
            
            async move {
                let _permit = sem.acquire().await.unwrap();
                let start = Instant::now();
                let result = execute_task(&task).await;
                let duration_ms = start.elapsed().as_millis() as i64;
                
                match result {
                    Ok(value) => ParallelResult {
                        task_id: task.id,
                        success: true,
                        result: Some(value),
                        error: None,
                        duration_ms,
                    },
                    Err(e) => ParallelResult {
                        task_id: task.id,
                        success: false,
                        result: None,
                        error: Some(e),
                        duration_ms,
                    },
                }
            }
        })
        .collect();
    
    let results: Vec<ParallelResult> = join_all(futures).await;
    
    let completed = results.iter().filter(|r| r.success).count() as i32;
    let failed = results.iter().filter(|r| !r.success).count() as i32;
    
    Ok(SemaphoreOutput {
        completed,
        failed,
        results,
    })
}

#[cfg(test)]
mod tests;
