use serde::{Deserialize, Serialize};

/// Type of schedule execution pattern.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleType {
    Cron,
    Interval,
    OneTime,
}

impl Default for ScheduleType {
    fn default() -> Self {
        Self::Cron
    }
}
