use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::{ExecutionHistory, Job, JobQuery, Schedule, ScheduleQuery, ScheduleResult, ScheduleStats};

#[async_trait]
pub trait ScheduleStore: Send + Sync {

    // ========== Schedule Operations ==========

    async fn create_schedule(&self, schedule: &Schedule) -> ScheduleResult<()>;
    async fn get_schedule(&self, schedule_id: &str) -> ScheduleResult<Option<Schedule>>;
    async fn update_schedule(&self, schedule: &Schedule) -> ScheduleResult<()>;
    async fn delete_schedule(&self, schedule_id: &str) -> ScheduleResult<bool>;
    async fn query_schedules(&self, query: ScheduleQuery) -> ScheduleResult<Vec<Schedule>>;
    async fn count_schedules(&self, query: ScheduleQuery) -> ScheduleResult<u64>;
    async fn get_due_schedules(&self, limit: usize) -> ScheduleResult<Vec<Schedule>>;

    // ========== Job Operations ==========

    async fn create_job(&self, job: &Job) -> ScheduleResult<()>;
    async fn get_job(&self, job_id: &str) -> ScheduleResult<Option<Job>>;
    async fn update_job(&self, job: &Job) -> ScheduleResult<()>;
    async fn delete_job(&self, job_id: &str) -> ScheduleResult<bool>;
    async fn query_jobs(&self, query: JobQuery) -> ScheduleResult<Vec<Job>>;
    async fn count_jobs(&self, query: JobQuery) -> ScheduleResult<u64>;
    async fn get_runnable_jobs(&self, limit: usize) -> ScheduleResult<Vec<Job>>;

    async fn try_lock_job(
        &self,
        job_id: &str,
        worker_id: &str,
        lock_duration_secs: u64,
    ) -> ScheduleResult<Option<String>>; // Returns lock token if acquired
    async fn release_job_lock(&self, job_id: &str, lock_token: &str) -> ScheduleResult<bool>;

    async fn extend_job_lock(&self, job_id: &str, lock_token: &str, extension_secs: u64) -> ScheduleResult<bool>;

    async fn get_stale_jobs(&self, stale_threshold: DateTime<Utc>) -> ScheduleResult<Vec<Job>>;

    // ========== Execution History Operations ==========

    async fn record_execution(&self, history: &ExecutionHistory) -> ScheduleResult<()>;

    async fn get_execution_history(
        &self,
        schedule_id: &str,
        limit: Option<usize>,
    ) -> ScheduleResult<Vec<ExecutionHistory>>;

    async fn cleanup_history(&self, before: DateTime<Utc>) -> ScheduleResult<u64>;

    // ========== Statistics ==========

    async fn get_schedule_stats(&self, schedule_id: &str) -> ScheduleResult<ScheduleStats>;
}
