use serde::{Deserialize, Serialize};

/// Current status of a task in its lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Scheduled,
    Running,
    Completed,
    Failed,
    Cancelled,
    TimedOut,
    Retrying,
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
