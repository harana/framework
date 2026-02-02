// Harana Components - Task Entity Types

use chrono::{DateTime, Utc};
use harana_components_storage::Entity;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============================================================================
// Task Priority
// ============================================================================

/// Priority level for task execution
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    /// Low priority - executed when resources are available
    Low = 0,
    /// Normal priority - default level
    Normal = 1,
    /// High priority - executed before normal tasks
    High = 2,
    /// Critical priority - executed immediately
    Critical = 3,
}

impl Default for TaskPriority {
    fn default() -> Self {
        Self::Normal
    }
}

impl TaskPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

// ============================================================================
// Task Status
// ============================================================================

/// Status of a task
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// Task is queued and waiting for execution
    Pending,
    /// Task is scheduled for future execution
    Scheduled,
    /// Task is currently running
    Running,
    /// Task completed successfully
    Completed,
    /// Task failed
    Failed,
    /// Task was cancelled
    Cancelled,
    /// Task timed out
    TimedOut,
    /// Task is being retried
    Retrying,
    /// Task is paused
    Paused,
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Scheduled => "scheduled",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Retrying => "retrying",
            Self::Paused => "paused",
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled | Self::TimedOut)
    }

    pub fn is_runnable(&self) -> bool {
        matches!(self, Self::Pending | Self::Scheduled | Self::Retrying)
    }
}

// ============================================================================
// Retry Configuration
// ============================================================================

/// Backoff strategy for retries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BackoffStrategy {
    /// Fixed delay between retries
    Fixed,
    /// Linear increase: delay * attempt
    Linear,
    /// Exponential increase: delay * 2^attempt
    Exponential,
}

impl Default for BackoffStrategy {
    fn default() -> Self {
        Self::Exponential
    }
}

/// Configuration for task retries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
        pub max_retries: u32,
        pub retry_delay_secs: u64,
        pub max_delay_secs: u64,
        pub backoff_strategy: BackoffStrategy,
        pub retry_on_errors: Option<Vec<String>>,
        pub no_retry_on_errors: Option<Vec<String>>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay_secs: 5,
            max_delay_secs: 3600,
            backoff_strategy: BackoffStrategy::Exponential,
            retry_on_errors: None,
            no_retry_on_errors: None,
        }
    }
}

impl RetryConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn no_retries() -> Self {
        Self {
            max_retries: 0,
            ..Default::default()
        }
    }

    pub fn with_max_retries(mut self, max: u32) -> Self {
        self.max_retries = max;
        self
    }

    pub fn with_delay(mut self, secs: u64) -> Self {
        self.retry_delay_secs = secs;
        self
    }

    pub fn with_max_delay(mut self, secs: u64) -> Self {
        self.max_delay_secs = secs;
        self
    }

    pub fn with_backoff(mut self, strategy: BackoffStrategy) -> Self {
        self.backoff_strategy = strategy;
        self
    }

    /// Calculate the delay for a given retry attempt
    pub fn calculate_delay(&self, attempt: u32) -> u64 {
        let base_delay = self.retry_delay_secs;
        let delay = match self.backoff_strategy {
            BackoffStrategy::Fixed => base_delay,
            BackoffStrategy::Linear => base_delay * (attempt as u64 + 1),
            BackoffStrategy::Exponential => base_delay * 2u64.pow(attempt),
        };
        delay.min(self.max_delay_secs)
    }
}

// ============================================================================
// Task Entity
// ============================================================================

/// A durable task that can be queued, executed, and tracked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
        pub id: String,
        pub name: String,
        pub description: String,
        pub queue: String,
        pub task_type: String,
        pub priority: TaskPriority,
        pub status: TaskStatus,
        pub payload: HashMap<String, Value>,
        pub result: Option<Value>,
        pub error: Option<String>,
        pub error_details: Option<String>,
        pub retry_config: RetryConfig,
        pub retry_attempt: u32,
        pub retry_at: Option<DateTime<Utc>>,
        pub scheduled_at: Option<DateTime<Utc>>,
        pub started_at: Option<DateTime<Utc>>,
        pub completed_at: Option<DateTime<Utc>>,
        pub timeout_secs: u64,
        pub duration_ms: Option<i64>,
        pub worker_id: Option<String>,
        pub lock_token: Option<String>,
        pub lock_expires_at: Option<DateTime<Utc>>,
        pub progress: Option<u8>,
        pub progress_message: Option<String>,
        pub parent_task_id: Option<String>,
        pub schedule_id: Option<String>,
        pub correlation_id: Option<String>,
        pub metadata: HashMap<String, Value>,
        pub tags: Vec<String>,
        pub owner_id: Option<String>,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
        pub version: u64,
}

impl Task {
    /// Create a new task
    pub fn new(name: impl Into<String>, task_type: impl Into<String>, queue: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            description: String::new(),
            queue: queue.into(),
            task_type: task_type.into(),
            priority: TaskPriority::Normal,
            status: TaskStatus::Pending,
            payload: HashMap::new(),
            result: None,
            error: None,
            error_details: None,
            retry_config: RetryConfig::default(),
            retry_attempt: 0,
            retry_at: None,
            scheduled_at: None,
            started_at: None,
            completed_at: None,
            timeout_secs: 300, // 5 minutes default
            duration_ms: None,
            worker_id: None,
            lock_token: None,
            lock_expires_at: None,
            progress: None,
            progress_message: None,
            parent_task_id: None,
            schedule_id: None,
            correlation_id: None,
            metadata: HashMap::new(),
            tags: Vec::new(),
            owner_id: None,
            created_at: now,
            updated_at: now,
            version: 1,
        }
    }

    /// Create a task with a specific ID
    pub fn with_id(
        id: impl Into<String>,
        name: impl Into<String>,
        task_type: impl Into<String>,
        queue: impl Into<String>,
    ) -> Self {
        let mut task = Self::new(name, task_type, queue);
        task.id = id.into();
        task
    }

    // Builder methods
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_payload(mut self, payload: HashMap<String, Value>) -> Self {
        self.payload = payload;
        self
    }

    pub fn with_payload_value(mut self, key: impl Into<String>, value: Value) -> Self {
        self.payload.insert(key.into(), value);
        self
    }

    pub fn with_retry(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    pub fn with_scheduled_at(mut self, scheduled_at: DateTime<Utc>) -> Self {
        self.scheduled_at = Some(scheduled_at);
        self.status = TaskStatus::Scheduled;
        self
    }

    pub fn with_delay(mut self, delay_secs: i64) -> Self {
        self.scheduled_at = Some(Utc::now() + chrono::Duration::seconds(delay_secs));
        self.status = TaskStatus::Scheduled;
        self
    }

    pub fn with_parent(mut self, parent_task_id: impl Into<String>) -> Self {
        self.parent_task_id = Some(parent_task_id.into());
        self
    }

    pub fn with_schedule(mut self, schedule_id: impl Into<String>) -> Self {
        self.schedule_id = Some(schedule_id.into());
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_owner(mut self, owner_id: impl Into<String>) -> Self {
        self.owner_id = Some(owner_id.into());
        self
    }

    /// Check if the task is ready to run
    pub fn is_runnable(&self) -> bool {
        if !self.status.is_runnable() {
            return false;
        }

        // Check scheduled time
        if let Some(scheduled_at) = self.scheduled_at {
            if Utc::now() < scheduled_at {
                return false;
            }
        }

        // Check retry time
        if let Some(retry_at) = self.retry_at {
            if Utc::now() < retry_at {
                return false;
            }
        }

        true
    }

    /// Check if the task has timed out
    pub fn is_timed_out(&self) -> bool {
        if self.status != TaskStatus::Running {
            return false;
        }

        if let Some(started_at) = self.started_at {
            let deadline = started_at + chrono::Duration::seconds(self.timeout_secs as i64);
            Utc::now() > deadline
        } else {
            false
        }
    }

    /// Check if the task can be retried
    pub fn can_retry(&self) -> bool {
        self.retry_attempt < self.retry_config.max_retries
    }

    /// Mark the task as started
    pub fn start(&mut self, worker_id: impl Into<String>, lock_token: impl Into<String>, lock_duration_secs: u64) {
        let now = Utc::now();
        self.status = TaskStatus::Running;
        self.started_at = Some(now);
        self.worker_id = Some(worker_id.into());
        self.lock_token = Some(lock_token.into());
        self.lock_expires_at = Some(now + chrono::Duration::seconds(lock_duration_secs as i64));
        self.updated_at = now;
        self.version += 1;
    }

    /// Mark the task as completed
    pub fn complete(&mut self, result: Option<Value>) {
        let now = Utc::now();
        self.status = TaskStatus::Completed;
        self.completed_at = Some(now);
        self.result = result;
        self.progress = Some(100);

        if let Some(started_at) = self.started_at {
            self.duration_ms = Some((now - started_at).num_milliseconds());
        }

        self.worker_id = None;
        self.lock_token = None;
        self.lock_expires_at = None;
        self.updated_at = now;
        self.version += 1;
    }

    /// Mark the task as failed
    pub fn fail(&mut self, error: impl Into<String>, error_details: Option<String>) {
        let now = Utc::now();
        self.status = TaskStatus::Failed;
        self.completed_at = Some(now);
        self.error = Some(error.into());
        self.error_details = error_details;

        if let Some(started_at) = self.started_at {
            self.duration_ms = Some((now - started_at).num_milliseconds());
        }

        self.worker_id = None;
        self.lock_token = None;
        self.lock_expires_at = None;
        self.updated_at = now;
        self.version += 1;
    }

    /// Schedule a retry
    pub fn schedule_retry(&mut self) {
        let now = Utc::now();
        self.retry_attempt += 1;
        self.status = TaskStatus::Retrying;

        let delay = self.retry_config.calculate_delay(self.retry_attempt - 1);
        self.retry_at = Some(now + chrono::Duration::seconds(delay as i64));

        self.started_at = None;
        self.worker_id = None;
        self.lock_token = None;
        self.lock_expires_at = None;
        self.error = None;
        self.error_details = None;
        self.updated_at = now;
        self.version += 1;
    }

    /// Mark the task as cancelled
    pub fn cancel(&mut self) {
        let now = Utc::now();
        self.status = TaskStatus::Cancelled;
        self.completed_at = Some(now);
        self.worker_id = None;
        self.lock_token = None;
        self.lock_expires_at = None;
        self.updated_at = now;
        self.version += 1;
    }

    /// Mark the task as timed out
    pub fn timeout(&mut self) {
        let now = Utc::now();
        self.status = TaskStatus::TimedOut;
        self.completed_at = Some(now);

        if let Some(started_at) = self.started_at {
            self.duration_ms = Some((now - started_at).num_milliseconds());
        }

        self.worker_id = None;
        self.lock_token = None;
        self.lock_expires_at = None;
        self.updated_at = now;
        self.version += 1;
    }

    /// Update progress
    pub fn update_progress(&mut self, progress: u8, message: Option<String>) {
        self.progress = Some(progress.min(100));
        self.progress_message = message;
        self.updated_at = Utc::now();
        self.version += 1;
    }

    /// Extend the lock
    pub fn extend_lock(&mut self, extension_secs: u64) {
        let now = Utc::now();
        self.lock_expires_at = Some(now + chrono::Duration::seconds(extension_secs as i64));
        self.updated_at = now;
        self.version += 1;
    }
}

impl Entity for Task {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type() -> &'static str {
        "task"
    }
}

// ============================================================================
// Task Execution History
// ============================================================================

/// Historical record of a task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskExecutionHistory {
        pub id: String,
        pub task_id: String,
        pub task_name: String,
        pub queue: String,
        pub task_type: String,
        pub status: TaskStatus,
        pub attempt: u32,
        pub worker_id: Option<String>,
        pub started_at: Option<DateTime<Utc>>,
        pub completed_at: Option<DateTime<Utc>>,
        pub duration_ms: Option<i64>,
        pub result: Option<Value>,
        pub error: Option<String>,
        pub error_details: Option<String>,
        pub correlation_id: Option<String>,
        pub schedule_id: Option<String>,
        pub created_at: DateTime<Utc>,
}

impl TaskExecutionHistory {
    /// Create history record from a completed task
    pub fn from_task(task: &Task) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            task_id: task.id.clone(),
            task_name: task.name.clone(),
            queue: task.queue.clone(),
            task_type: task.task_type.clone(),
            status: task.status.clone(),
            attempt: task.retry_attempt,
            worker_id: task.worker_id.clone(),
            started_at: task.started_at,
            completed_at: task.completed_at,
            duration_ms: task.duration_ms,
            result: task.result.clone(),
            error: task.error.clone(),
            error_details: task.error_details.clone(),
            correlation_id: task.correlation_id.clone(),
            schedule_id: task.schedule_id.clone(),
            created_at: Utc::now(),
        }
    }
}

impl Entity for TaskExecutionHistory {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type() -> &'static str {
        "task_execution_history"
    }
}

// ============================================================================
// Resource Lock Helpers
// ============================================================================

/// Generate a lock resource ID for a task
pub fn task_lock_resource(task_id: &str) -> String {
    format!("task:{}", task_id)
}

/// Generate a lock resource ID for a task queue
pub fn queue_lock_resource(queue_name: &str) -> String {
    format!("task_queue:{}", queue_name)
}
