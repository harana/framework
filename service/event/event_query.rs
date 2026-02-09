use chrono::{DateTime, Utc};

use crate::EventStatus;

#[derive(Debug, Clone, Default)]
pub struct EventQuery {
    pub channel: Option<String>,
    pub event_types: Option<Vec<String>>,
    pub status: Option<EventStatus>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub ascending: bool,
}

impl EventQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn for_channel(channel: impl Into<String>) -> Self {
        Self {
            channel: Some(channel.into()),
            ..Default::default()
        }
    }

    pub fn with_types(mut self, types: Vec<String>) -> Self {
        self.event_types = Some(types);
        self
    }

    pub fn with_status(mut self, status: EventStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_time_range(mut self, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> Self {
        self.start_time = start;
        self.end_time = end;
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

    pub fn ascending(mut self) -> Self {
        self.ascending = true;
        self
    }

    pub fn descending(mut self) -> Self {
        self.ascending = false;
        self
    }
}
