use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Default)]
pub struct ScheduleStats {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub average_duration_ms: Option<f64>,
    pub last_execution_at: Option<DateTime<Utc>>,
    pub next_execution_at: Option<DateTime<Utc>>,
}
