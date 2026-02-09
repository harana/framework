use chrono::{DateTime, Utc};
use harana_components_storage::Entity;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::retry_config::RetryConfig;
use crate::schedule_status::ScheduleStatus;
use crate::schedule_type::ScheduleType;

/// Main schedule entity representing a scheduled task configuration.
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
