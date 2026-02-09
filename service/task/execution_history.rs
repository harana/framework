use chrono::{DateTime, Utc};
use harana_components_storage::Entity;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::status::TaskStatus;
use crate::task::Task;

/// Historical record of a task execution attempt.
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
