use chrono::{DateTime, Utc};
use harana_components_storage::Entity;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::priority::TaskPriority;
use crate::retry_config::RetryConfig;
use crate::status::TaskStatus;

/// Main task entity representing a unit of work to be executed.
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
