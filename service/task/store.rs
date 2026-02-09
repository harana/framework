use chrono::{DateTime, Utc};

use harana_components_storage::{FilterCondition, QueryOptions, Store};

use crate::{
    Task, TaskError, TaskExecutionHistory, TaskPriority, TaskResult, TaskStatus,
};

// ============================================================================
// Query Types
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct TaskQuery {
    pub queue: Option<String>,
    pub task_type: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub worker_id: Option<String>,
    pub parent_task_id: Option<String>,
    pub schedule_id: Option<String>,
    pub correlation_id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub owner_id: Option<String>,
    pub scheduled_before: Option<DateTime<Utc>>,
    pub scheduled_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub created_after: Option<DateTime<Utc>>,
    pub search: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl TaskQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pending() -> Self {
        Self {
            status: Some(TaskStatus::Pending),
            ..Default::default()
        }
    }

    pub fn running() -> Self {
        Self {
            status: Some(TaskStatus::Running),
            ..Default::default()
        }
    }

    pub fn for_queue(queue: impl Into<String>) -> Self {
        Self {
            queue: Some(queue.into()),
            ..Default::default()
        }
    }

    pub fn for_worker(worker_id: impl Into<String>) -> Self {
        Self {
            worker_id: Some(worker_id.into()),
            ..Default::default()
        }
    }

    pub fn for_schedule(schedule_id: impl Into<String>) -> Self {
        Self {
            schedule_id: Some(schedule_id.into()),
            ..Default::default()
        }
    }

    pub fn with_queue(mut self, queue: impl Into<String>) -> Self {
        self.queue = Some(queue.into());
        self
    }

    pub fn with_task_type(mut self, task_type: impl Into<String>) -> Self {
        self.task_type = Some(task_type.into());
        self
    }

    pub fn with_status(mut self, status: TaskStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn with_owner(mut self, owner_id: impl Into<String>) -> Self {
        self.owner_id = Some(owner_id.into());
        self
    }

    pub fn with_search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

// ============================================================================
// Task Statistics
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct QueueStats {
    pub pending: u64,
    pub scheduled: u64,
    pub running: u64,
    pub completed: u64,
    pub failed: u64,
    pub cancelled: u64,
    pub timed_out: u64,
    pub retrying: u64,
    pub total: u64,
    pub average_duration_ms: Option<f64>,
    pub average_wait_time_ms: Option<f64>,
}

// ============================================================================
// Filter Conversion
// ============================================================================

impl TaskQuery {
    /// Convert TaskQuery to FilterCondition for storage
    pub fn to_filter(&self) -> Option<FilterCondition> {
        let mut conditions = Vec::new();

        if let Some(queue) = &self.queue {
            conditions.push(FilterCondition::eq("queue", queue.clone()));
        }
        if let Some(task_type) = &self.task_type {
            conditions.push(FilterCondition::eq("task_type", task_type.clone()));
        }
        if let Some(status) = &self.status {
            conditions.push(FilterCondition::eq("status", status.as_str()));
        }
        if let Some(priority) = &self.priority {
            conditions.push(FilterCondition::eq("priority", priority.as_str()));
        }
        if let Some(worker_id) = &self.worker_id {
            conditions.push(FilterCondition::eq("worker_id", worker_id.clone()));
        }
        if let Some(parent_id) = &self.parent_task_id {
            conditions.push(FilterCondition::eq("parent_task_id", parent_id.clone()));
        }
        if let Some(schedule_id) = &self.schedule_id {
            conditions.push(FilterCondition::eq("schedule_id", schedule_id.clone()));
        }
        if let Some(correlation_id) = &self.correlation_id {
            conditions.push(FilterCondition::eq("correlation_id", correlation_id.clone()));
        }
        if let Some(owner_id) = &self.owner_id {
            conditions.push(FilterCondition::eq("owner_id", owner_id.clone()));
        }
        if let Some(before) = self.scheduled_before {
            conditions.push(FilterCondition::lte("scheduled_at", before.to_rfc3339()));
        }
        if let Some(after) = self.scheduled_after {
            conditions.push(FilterCondition::gte("scheduled_at", after.to_rfc3339()));
        }
        if let Some(before) = self.created_before {
            conditions.push(FilterCondition::lte("created_at", before.to_rfc3339()));
        }
        if let Some(after) = self.created_after {
            conditions.push(FilterCondition::gte("created_at", after.to_rfc3339()));
        }

        if conditions.is_empty() {
            None
        } else if conditions.len() == 1 {
            Some(conditions.remove(0))
        } else {
            Some(FilterCondition::and(conditions))
        }
    }

    /// Convert TaskQuery to QueryOptions for storage
    pub fn to_query_options(&self) -> QueryOptions {
        let mut options = QueryOptions::new()
            .with_sort("priority", true); // Sort by priority descending

        if let Some(limit) = self.limit {
            options = options.with_limit(limit as u32);
        }
        if let Some(offset) = self.offset {
            options = options.with_offset(offset as u32);
        }

        options
    }
}

// ============================================================================
// Task Store Operations
// ============================================================================
pub async fn create_task<S>(store: &S, task: &Task) -> TaskResult<()>
where
    S: Store<Task>,
{
    // Check if task already exists
    if store.find_by_id(&task.id).await.map_err(map_storage_error)?.is_some() {
        return Err(TaskError::TaskAlreadyExists {
            task_id: task.id.clone(),
        });
    }
    store.create(task).await.map_err(map_storage_error)
}
pub async fn get_task<S>(store: &S, task_id: &str) -> TaskResult<Option<Task>>
where
    S: Store<Task>,
{
    store.find_by_id(task_id).await.map_err(map_storage_error)
}
pub async fn update_task<S>(store: &S, task: &Task) -> TaskResult<()>
where
    S: Store<Task>,
{
    // Verify task exists
    if store.find_by_id(&task.id).await.map_err(map_storage_error)?.is_none() {
        return Err(TaskError::TaskNotFound {
            task_id: task.id.clone(),
        });
    }
    store.update(task).await.map_err(map_storage_error)
}
pub async fn delete_task<S>(store: &S, task_id: &str) -> TaskResult<bool>
where
    S: Store<Task>,
{
    store.delete(task_id).await.map_err(map_storage_error)
}
pub async fn query_tasks<S>(store: &S, query: TaskQuery) -> TaskResult<Vec<Task>>
where
    S: Store<Task>,
{
    let filter = query.to_filter();
    let options = query.to_query_options();

    let mut tasks = store.find_many(filter, options).await.map_err(map_storage_error)?;

    // Apply text search filter if specified (done in memory for now)
    if let Some(search) = &query.search {
        let search_lower = search.to_lowercase();
        tasks.retain(|t| {
            t.name.to_lowercase().contains(&search_lower)
                || t.description.to_lowercase().contains(&search_lower)
        });
    }

    // Apply tag filter if specified (done in memory for now)
    if let Some(tags) = &query.tags {
        tasks.retain(|t| tags.iter().any(|tag| t.tags.contains(tag)));
    }

    Ok(tasks)
}
/// Count tasks matching query
pub async fn count_tasks<S>(store: &S, query: TaskQuery) -> TaskResult<u64>
where
    S: Store<Task>,
{
    let filter = query.to_filter();
    store.count(filter).await.map_err(map_storage_error)
}
pub async fn get_runnable_tasks<S>(store: &S, queue: &str, limit: usize) -> TaskResult<Vec<Task>>
where
    S: Store<Task>,
{
    // Build filter for runnable tasks
    let status_filter = FilterCondition::or(vec![
        FilterCondition::eq("status", TaskStatus::Pending.as_str()),
        FilterCondition::eq("status", TaskStatus::Scheduled.as_str()),
        FilterCondition::eq("status", TaskStatus::Retrying.as_str()),
    ]);

    let queue_filter = FilterCondition::eq("queue", queue.to_string());
    let unlocked_filter = FilterCondition::is_null("lock_token");

    let filter = FilterCondition::and(vec![queue_filter, status_filter, unlocked_filter]);

    let options = QueryOptions::new()
        .with_sort("priority", true) // Sort by priority descending
        .with_limit(limit as u32 * 2); // Fetch extra to filter by time

    let tasks = store.find_many(Some(filter), options).await.map_err(map_storage_error)?;

    // Filter by runnable time and take limit
    let runnable: Vec<Task> = tasks
        .into_iter()
        .filter(|t| t.is_runnable())
        .take(limit)
        .collect();

    Ok(runnable)
}
/// Try to acquire a lock on a task for execution
pub async fn try_lock_task<S>(
    store: &S,
    task_id: &str,
    worker_id: &str,
    lock_duration_secs: u64,
) -> TaskResult<Option<String>>
where
    S: Store<Task>,
{
    let task = store.find_by_id(task_id).await.map_err(map_storage_error)?;

    let mut task = match task {
        Some(t) => t,
        None => {
            return Err(TaskError::TaskNotFound {
                task_id: task_id.to_string(),
            })
        }
    };

    // Check if already locked
    if let Some(expires_at) = task.lock_expires_at {
        if expires_at > Utc::now() {
            // Lock is still valid
            return Ok(None);
        }
    }

    // Acquire lock
    let lock_token = uuid::Uuid::new_v4().to_string();
    task.start(worker_id, &lock_token, lock_duration_secs);

    store.update(&task).await.map_err(map_storage_error)?;

    Ok(Some(lock_token))
}
pub async fn release_task_lock<S>(
    store: &S,
    task_id: &str,
    lock_token: &str,
) -> TaskResult<bool>
where
    S: Store<Task>,
{
    let task = store.find_by_id(task_id).await.map_err(map_storage_error)?;

    let mut task = match task {
        Some(t) => t,
        None => return Ok(false),
    };

    if task.lock_token.as_ref() != Some(&lock_token.to_string()) {
        return Ok(false);
    }

    task.lock_token = None;
    task.lock_expires_at = None;
    task.worker_id = None;
    task.updated_at = Utc::now();
    task.version += 1;

    store.update(&task).await.map_err(map_storage_error)?;

    Ok(true)
}
/// Extend a task lock
pub async fn extend_task_lock<S>(
    store: &S,
    task_id: &str,
    lock_token: &str,
    extension_secs: u64,
) -> TaskResult<bool>
where
    S: Store<Task>,
{
    let task = store.find_by_id(task_id).await.map_err(map_storage_error)?;

    let mut task = match task {
        Some(t) => t,
        None => return Ok(false),
    };

    if task.lock_token.as_ref() != Some(&lock_token.to_string()) {
        return Ok(false);
    }

    task.extend_lock(extension_secs);
    store.update(&task).await.map_err(map_storage_error)?;

    Ok(true)
}
pub async fn get_stale_tasks<S>(store: &S, stale_threshold: DateTime<Utc>) -> TaskResult<Vec<Task>>
where
    S: Store<Task>,
{
    let filter = FilterCondition::and(vec![
        FilterCondition::eq("status", TaskStatus::Running.as_str()),
        FilterCondition::lt("lock_expires_at", stale_threshold.to_rfc3339()),
    ]);

    store
        .find_many(Some(filter), QueryOptions::new())
        .await
        .map_err(map_storage_error)
}

// ============================================================================
// History Operations
// ============================================================================
/// Record execution history
pub async fn record_history<S>(store: &S, history: &TaskExecutionHistory) -> TaskResult<()>
where
    S: Store<TaskExecutionHistory>,
{
    store.create(history).await.map_err(map_storage_error)
}
pub async fn get_task_history<S>(
    store: &S,
    task_id: &str,
    limit: Option<usize>,
) -> TaskResult<Vec<TaskExecutionHistory>>
where
    S: Store<TaskExecutionHistory>,
{
    let filter = FilterCondition::eq("task_id", task_id.to_string());
    let options = QueryOptions::new()
        .with_sort("created_at", true)
        .with_limit(limit.unwrap_or(100) as u32);

    store
        .find_many(Some(filter), options)
        .await
        .map_err(map_storage_error)
}
pub async fn get_queue_history<S>(
    store: &S,
    queue: &str,
    limit: Option<usize>,
) -> TaskResult<Vec<TaskExecutionHistory>>
where
    S: Store<TaskExecutionHistory>,
{
    let filter = FilterCondition::eq("queue", queue.to_string());
    let options = QueryOptions::new()
        .with_sort("created_at", true)
        .with_limit(limit.unwrap_or(100) as u32);

    store
        .find_many(Some(filter), options)
        .await
        .map_err(map_storage_error)
}

/// Delete old execution history
pub async fn cleanup_history<S>(store: &S, before: DateTime<Utc>) -> TaskResult<u64>
where
    S: Store<TaskExecutionHistory>,
{
    let filter = FilterCondition::lt("created_at", before.to_rfc3339());
    store.delete_many(filter).await.map_err(map_storage_error)
}

// ============================================================================
// Statistics Operations
// ============================================================================
pub async fn get_queue_stats<S>(store: &S, queue: &str) -> TaskResult<QueueStats>
where
    S: Store<Task>,
{
    let queue_filter = FilterCondition::eq("queue", queue.to_string());

    // Count by status
    let pending = store
        .count(Some(FilterCondition::and(vec![
            queue_filter.clone(),
            FilterCondition::eq("status", TaskStatus::Pending.as_str()),
        ])))
        .await
        .map_err(map_storage_error)?;

    let scheduled = store
        .count(Some(FilterCondition::and(vec![
            queue_filter.clone(),
            FilterCondition::eq("status", TaskStatus::Scheduled.as_str()),
        ])))
        .await
        .map_err(map_storage_error)?;

    let running = store
        .count(Some(FilterCondition::and(vec![
            queue_filter.clone(),
            FilterCondition::eq("status", TaskStatus::Running.as_str()),
        ])))
        .await
        .map_err(map_storage_error)?;

    let completed = store
        .count(Some(FilterCondition::and(vec![
            queue_filter.clone(),
            FilterCondition::eq("status", TaskStatus::Completed.as_str()),
        ])))
        .await
        .map_err(map_storage_error)?;

    let failed = store
        .count(Some(FilterCondition::and(vec![
            queue_filter.clone(),
            FilterCondition::eq("status", TaskStatus::Failed.as_str()),
        ])))
        .await
        .map_err(map_storage_error)?;

    let cancelled = store
        .count(Some(FilterCondition::and(vec![
            queue_filter.clone(),
            FilterCondition::eq("status", TaskStatus::Cancelled.as_str()),
        ])))
        .await
        .map_err(map_storage_error)?;

    let timed_out = store
        .count(Some(FilterCondition::and(vec![
            queue_filter.clone(),
            FilterCondition::eq("status", TaskStatus::TimedOut.as_str()),
        ])))
        .await
        .map_err(map_storage_error)?;

    let retrying = store
        .count(Some(FilterCondition::and(vec![
            queue_filter.clone(),
            FilterCondition::eq("status", TaskStatus::Retrying.as_str()),
        ])))
        .await
        .map_err(map_storage_error)?;

    let total = store.count(Some(queue_filter)).await.map_err(map_storage_error)?;

    // Calculate average duration from completed tasks
    let completed_tasks = store
        .find_many(
            Some(FilterCondition::and(vec![
                FilterCondition::eq("queue", queue.to_string()),
                FilterCondition::eq("status", TaskStatus::Completed.as_str()),
            ])),
            QueryOptions::new().with_limit(1000),
        )
        .await
        .map_err(map_storage_error)?;

    let durations: Vec<i64> = completed_tasks
        .iter()
        .filter_map(|t| t.duration_ms)
        .collect();

    let average_duration_ms = if !durations.is_empty() {
        let sum: i64 = durations.iter().sum();
        Some(sum as f64 / durations.len() as f64)
    } else {
        None
    };

    Ok(QueueStats {
        pending,
        scheduled,
        running,
        completed,
        failed,
        cancelled,
        timed_out,
        retrying,
        total,
        average_duration_ms,
        average_wait_time_ms: None,
    })
}
pub async fn get_queues<S>(store: &S) -> TaskResult<Vec<String>>
where
    S: Store<Task>,
{
    // Fetch all tasks and extract unique queues
    // Note: This could be optimized with a distinct query in the storage backend
    let tasks = store
        .find_many(None, QueryOptions::new())
        .await
        .map_err(map_storage_error)?;

    let mut queues: Vec<String> = tasks
        .iter()
        .map(|t| t.queue.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    queues.sort();
    Ok(queues)
}

// ============================================================================
// Error Mapping
// ============================================================================

fn map_storage_error(err: harana_components_storage::StorageError) -> TaskError {
    TaskError::StoreError {
        reason: err.to_string(),
    }
}

