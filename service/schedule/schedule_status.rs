use serde::{Deserialize, Serialize};

/// Current status of a schedule.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleStatus {
    Active,
    Paused,
    Disabled,
    Completed,
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
