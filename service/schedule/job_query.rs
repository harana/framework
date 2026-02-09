use chrono::{DateTime, Utc};

use crate::JobStatus;

#[derive(Debug, Clone, Default)]
pub struct JobQuery {
    pub schedule_id: Option<String>,
    pub status: Option<JobStatus>,
    pub scheduled_after: Option<DateTime<Utc>>,
    pub scheduled_before: Option<DateTime<Utc>>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl JobQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn for_schedule(schedule_id: impl Into<String>) -> Self {
        Self {
            schedule_id: Some(schedule_id.into()),
            ..Default::default()
        }
    }

    pub fn pending() -> Self {
        Self {
            status: Some(JobStatus::Pending),
            ..Default::default()
        }
    }

    pub fn runnable() -> Self {
        Self {
            status: Some(JobStatus::Pending),
            scheduled_before: Some(Utc::now()),
            ..Default::default()
        }
    }

    pub fn with_status(mut self, status: JobStatus) -> Self {
        self.status = Some(status);
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
