// Harana Components - Schedule Entity Types

use chrono::{DateTime, Utc};
use harana_components_storage::Entity;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============================================================================
// Schedule Entity
// ============================================================================

/// Type of schedule trigger
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleType {
    /// Cron-based schedule
    Cron,
    /// Fixed interval schedule
    Interval,
    /// One-time execution
    OneTime,
}

impl Default for ScheduleType {
    fn default() -> Self {
        Self::Cron
    }
}

/// Status of a schedule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleStatus {
    /// Schedule is active and will trigger
    Active,
    /// Schedule is paused (won't trigger until resumed)
    Paused,
    /// Schedule is disabled
    Disabled,
    /// Schedule completed (for one-time schedules)
    Completed,
    /// Schedule expired
    Expired,
}

impl Default for ScheduleStatus {
    fn default() -> Self {
        Self::Active
    }
}

impl ScheduleStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Disabled => "disabled",
            Self::Completed => "completed",
            Self::Expired => "expired",
        }
    }
}

/// A durable schedule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
        pub id: String,
        pub name: String,
        pub description: String,
        pub schedule_type: ScheduleType,
        pub cron_expression: Option<String>,
        pub interval_seconds: Option<i64>,
        pub run_at: Option<DateTime<Utc>>,
        pub timezone: String,
        pub status: ScheduleStatus,
        pub resume_at: Option<DateTime<Utc>>,
        pub start_at: Option<DateTime<Utc>>,
        pub end_at: Option<DateTime<Utc>>,
        pub max_executions: Option<u64>,
        pub execution_count: u64,
        pub last_run_at: Option<DateTime<Utc>>,
        pub next_run_at: Option<DateTime<Utc>>,
        pub action_type: String,
        pub action_config: HashMap<String, Value>,
        pub retry_config: RetryConfig,
        pub metadata: HashMap<String, Value>,
        pub tags: Vec<String>,
        pub owner_id: Option<String>,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
        pub version: u64,
}

impl Schedule {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            schedule_type: ScheduleType::Cron,
            cron_expression: None,
            interval_seconds: None,
            run_at: None,
            timezone: "UTC".to_string(),
            status: ScheduleStatus::Active,
            resume_at: None,
            start_at: None,
            end_at: None,
            max_executions: None,
            execution_count: 0,
            last_run_at: None,
            next_run_at: None,
            action_type: String::new(),
            action_config: HashMap::new(),
            retry_config: RetryConfig::default(),
            metadata: HashMap::new(),
            tags: Vec::new(),
            owner_id: None,
            created_at: now,
            updated_at: now,
            version: 1,
        }
    }

    /// Create a cron-based schedule
    pub fn cron(id: impl Into<String>, name: impl Into<String>, expression: impl Into<String>) -> Self {
        let mut schedule = Self::new(id, name);
        schedule.schedule_type = ScheduleType::Cron;
        schedule.cron_expression = Some(expression.into());
        schedule
    }

    /// Create an interval-based schedule
    pub fn interval(id: impl Into<String>, name: impl Into<String>, seconds: i64) -> Self {
        let mut schedule = Self::new(id, name);
        schedule.schedule_type = ScheduleType::Interval;
        schedule.interval_seconds = Some(seconds);
        schedule
    }

    /// Create a one-time schedule
    pub fn one_time(id: impl Into<String>, name: impl Into<String>, run_at: DateTime<Utc>) -> Self {
        let mut schedule = Self::new(id, name);
        schedule.schedule_type = ScheduleType::OneTime;
        schedule.run_at = Some(run_at);
        schedule.next_run_at = Some(run_at);
        schedule
    }

    /// Check if the schedule is active and should be evaluated
    pub fn is_runnable(&self) -> bool {
        if self.status != ScheduleStatus::Active {
            return false;
        }

        let now = Utc::now();

        // Check start time
        if let Some(start) = self.start_at {
            if now < start {
                return false;
            }
        }

        // Check end time
        if let Some(end) = self.end_at {
            if now > end {
                return false;
            }
        }

        // Check max executions
        if let Some(max) = self.max_executions {
            if self.execution_count >= max {
                return false;
            }
        }

        true
    }

    /// Check if this schedule should run now
    pub fn is_due(&self) -> bool {
        if !self.is_runnable() {
            return false;
        }

        if let Some(next_run) = self.next_run_at {
            Utc::now() >= next_run
        } else {
            false
        }
    }

    // Builder methods
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = timezone.into();
        self
    }

    pub fn with_action(mut self, action_type: impl Into<String>, config: HashMap<String, Value>) -> Self {
        self.action_type = action_type.into();
        self.action_config = config;
        self
    }

    pub fn with_retry(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    pub fn with_start_at(mut self, start_at: DateTime<Utc>) -> Self {
        self.start_at = Some(start_at);
        self
    }

    pub fn with_end_at(mut self, end_at: DateTime<Utc>) -> Self {
        self.end_at = Some(end_at);
        self
    }

    pub fn with_max_executions(mut self, max: u64) -> Self {
        self.max_executions = Some(max);
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

    pub fn with_metadata(mut self, key: impl Into<String>, value: Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }
}

impl Entity for Schedule {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type() -> &'static str {
        "schedule"
    }
}

// ============================================================================
// Job Entity (an instance of a scheduled execution)
// ============================================================================

/// Status of a scheduled job
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    /// Job is pending execution
    Pending,
    /// Job is currently running
    Running,
    /// Job completed successfully
    Completed,
    /// Job failed
    Failed,
    /// Job was cancelled
    Cancelled,
    /// Job timed out
    TimedOut,
    /// Job is being retried
    Retrying,
}

impl Default for JobStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl JobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Retrying => "retrying",
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled | Self::TimedOut)
    }
}

/// A scheduled job instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
        pub id: String,
        pub schedule_id: String,
        pub schedule_name: String,
        pub status: JobStatus,
        pub scheduled_at: DateTime<Utc>,
        pub started_at: Option<DateTime<Utc>>,
        pub completed_at: Option<DateTime<Utc>>,
        pub action_type: String,
        pub action_config: HashMap<String, Value>,
        pub result: Option<Value>,
        pub error: Option<String>,
        pub error_details: Option<String>,
        pub retry_attempt: u32,
        pub max_retries: u32,
        pub retry_at: Option<DateTime<Utc>>,
        pub duration_ms: Option<i64>,
        pub worker_id: Option<String>,
        pub lock_token: Option<String>,
        pub lock_expires_at: Option<DateTime<Utc>>,
        pub metadata: HashMap<String, Value>,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
}

impl Job {
    pub fn new(schedule: &Schedule, scheduled_at: DateTime<Utc>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            schedule_id: schedule.id.clone(),
            schedule_name: schedule.name.clone(),
            status: JobStatus::Pending,
            scheduled_at,
            started_at: None,
            completed_at: None,
            action_type: schedule.action_type.clone(),
            action_config: schedule.action_config.clone(),
            result: None,
            error: None,
            error_details: None,
            retry_attempt: 0,
            max_retries: schedule.retry_config.max_retries,
            retry_at: None,
            duration_ms: None,
            worker_id: None,
            lock_token: None,
            lock_expires_at: None,
            metadata: schedule.metadata.clone(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Start the job execution
    pub fn start(&mut self, worker_id: Option<String>) {
        self.status = JobStatus::Running;
        self.started_at = Some(Utc::now());
        self.worker_id = worker_id;
        self.updated_at = Utc::now();
    }

    /// Complete the job successfully
    pub fn complete(&mut self, result: Option<Value>) {
        let now = Utc::now();
        self.status = JobStatus::Completed;
        self.completed_at = Some(now);
        self.result = result;
        if let Some(started) = self.started_at {
            self.duration_ms = Some((now - started).num_milliseconds());
        }
        self.updated_at = now;
    }

    /// Fail the job
    pub fn fail(&mut self, error: String, details: Option<String>) {
        let now = Utc::now();
        self.status = JobStatus::Failed;
        self.completed_at = Some(now);
        self.error = Some(error);
        self.error_details = details;
        if let Some(started) = self.started_at {
            self.duration_ms = Some((now - started).num_milliseconds());
        }
        self.updated_at = now;
    }

    /// Mark job for retry
    pub fn retry(&mut self, retry_at: DateTime<Utc>) {
        self.status = JobStatus::Retrying;
        self.retry_attempt += 1;
        self.retry_at = Some(retry_at);
        self.started_at = None;
        self.updated_at = Utc::now();
    }

    /// Cancel the job
    pub fn cancel(&mut self) {
        self.status = JobStatus::Cancelled;
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Check if job can be retried
    pub fn can_retry(&self) -> bool {
        self.retry_attempt < self.max_retries && !self.status.is_terminal()
    }
}

impl Entity for Job {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type() -> &'static str {
        "job"
    }
}

// ============================================================================
// Retry Configuration
// ============================================================================

/// Retry backoff strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BackoffStrategy {
    /// Fixed delay between retries
    Fixed,
    /// Linear increase in delay
    Linear,
    /// Exponential increase in delay
    Exponential,
}

impl Default for BackoffStrategy {
    fn default() -> Self {
        Self::Exponential
    }
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
        pub max_retries: u32,
        pub initial_delay_secs: u64,
        pub max_delay_secs: u64,
        pub multiplier: f64,
        pub strategy: BackoffStrategy,
        pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_secs: 10,
            max_delay_secs: 3600,
            multiplier: 2.0,
            strategy: BackoffStrategy::Exponential,
            jitter: true,
        }
    }
}

impl RetryConfig {
    pub fn no_retry() -> Self {
        Self {
            max_retries: 0,
            ..Default::default()
        }
    }

    /// Calculate delay for a given retry attempt
    pub fn delay_for_attempt(&self, attempt: u32) -> std::time::Duration {
        if attempt == 0 {
            return std::time::Duration::from_secs(self.initial_delay_secs);
        }

        let base_delay = match self.strategy {
            BackoffStrategy::Fixed => self.initial_delay_secs as f64,
            BackoffStrategy::Linear => self.initial_delay_secs as f64 + (attempt as f64 * self.multiplier),
            BackoffStrategy::Exponential => self.initial_delay_secs as f64 * self.multiplier.powi(attempt as i32),
        };

        let delay_secs = base_delay.min(self.max_delay_secs as f64);

        let final_delay = if self.jitter {
            // Add up to 25% jitter
            let jitter_factor = 1.0 + (rand_simple() * 0.25);
            delay_secs * jitter_factor
        } else {
            delay_secs
        };

        std::time::Duration::from_secs_f64(final_delay)
    }
}

/// Simple pseudo-random for jitter (avoids external dependency)
fn rand_simple() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (nanos % 1000) as f64 / 1000.0
}

// ============================================================================
// Execution History Entry
// ============================================================================

/// A historical record of a job execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionHistory {
        pub id: String,
        pub schedule_id: String,
        pub job_id: String,
        pub status: JobStatus,
        pub scheduled_at: DateTime<Utc>,
        pub started_at: Option<DateTime<Utc>>,
        pub completed_at: Option<DateTime<Utc>>,
        pub duration_ms: Option<i64>,
        pub retry_attempt: u32,
        pub error: Option<String>,
        pub worker_id: Option<String>,
        pub created_at: DateTime<Utc>,
}

impl ExecutionHistory {
    pub fn from_job(job: &Job) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            schedule_id: job.schedule_id.clone(),
            job_id: job.id.clone(),
            status: job.status.clone(),
            scheduled_at: job.scheduled_at,
            started_at: job.started_at,
            completed_at: job.completed_at,
            duration_ms: job.duration_ms,
            retry_attempt: job.retry_attempt,
            error: job.error.clone(),
            worker_id: job.worker_id.clone(),
            created_at: Utc::now(),
        }
    }
}

impl Entity for ExecutionHistory {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type() -> &'static str {
        "execution_history"
    }
}
