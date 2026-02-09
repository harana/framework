use chrono::{DateTime, Utc};

use crate::{ScheduleStatus, ScheduleType};

#[derive(Debug, Clone, Default)]
pub struct ScheduleQuery {
    pub status: Option<ScheduleStatus>,
    pub schedule_type: Option<ScheduleType>,
    pub tags: Option<Vec<String>>,
    pub owner_id: Option<String>,
    pub due_before: Option<DateTime<Utc>>,
    pub search: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl ScheduleQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn active() -> Self {
        Self {
            status: Some(ScheduleStatus::Active),
            ..Default::default()
        }
    }

    pub fn due_now() -> Self {
        Self {
            status: Some(ScheduleStatus::Active),
            due_before: Some(Utc::now()),
            ..Default::default()
        }
    }

    pub fn with_status(mut self, status: ScheduleStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_type(mut self, schedule_type: ScheduleType) -> Self {
        self.schedule_type = Some(schedule_type);
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
