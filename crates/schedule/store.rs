// Harana Components - Schedule Store Trait and Implementations

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::VecDeque;
use std::sync::Arc;

use crate::{
    ExecutionHistory, Job, JobStatus, Schedule, ScheduleError, 
    ScheduleResult, ScheduleStatus, ScheduleType,
};

// ============================================================================
// Query Types
// ============================================================================

/// Query options for listing schedules
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

/// Query options for listing jobs
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

// ============================================================================
// Schedule Store Trait
// ============================================================================

/// Store trait for persisting and querying schedules, jobs, and execution history
#[async_trait]
pub trait ScheduleStore: Send + Sync {
    // ========== Schedule Operations ==========

    /// Create a new schedule
    async fn create_schedule(&self, schedule: &Schedule) -> ScheduleResult<()>;

    /// Get a schedule by ID
    async fn get_schedule(&self, schedule_id: &str) -> ScheduleResult<Option<Schedule>>;

    /// Update a schedule
    async fn update_schedule(&self, schedule: &Schedule) -> ScheduleResult<()>;

    /// Delete a schedule and all its jobs
    async fn delete_schedule(&self, schedule_id: &str) -> ScheduleResult<bool>;

    /// Query schedules
    async fn query_schedules(&self, query: ScheduleQuery) -> ScheduleResult<Vec<Schedule>>;

    /// Count schedules matching query
    async fn count_schedules(&self, query: ScheduleQuery) -> ScheduleResult<u64>;

    /// Get schedules that are due to run
    async fn get_due_schedules(&self, limit: usize) -> ScheduleResult<Vec<Schedule>>;

    // ========== Job Operations ==========

    /// Create a new job
    async fn create_job(&self, job: &Job) -> ScheduleResult<()>;

    /// Get a job by ID
    async fn get_job(&self, job_id: &str) -> ScheduleResult<Option<Job>>;

    /// Update a job
    async fn update_job(&self, job: &Job) -> ScheduleResult<()>;

    /// Delete a job
    async fn delete_job(&self, job_id: &str) -> ScheduleResult<bool>;

    /// Query jobs
    async fn query_jobs(&self, query: JobQuery) -> ScheduleResult<Vec<Job>>;

    /// Count jobs matching query
    async fn count_jobs(&self, query: JobQuery) -> ScheduleResult<u64>;

    /// Get runnable jobs (pending and due)
    async fn get_runnable_jobs(&self, limit: usize) -> ScheduleResult<Vec<Job>>;

    /// Try to acquire a lock on a job for execution
    async fn try_lock_job(
        &self,
        job_id: &str,
        worker_id: &str,
        lock_duration_secs: u64,
    ) -> ScheduleResult<Option<String>>; // Returns lock token if acquired

    /// Release a job lock
    async fn release_job_lock(&self, job_id: &str, lock_token: &str) -> ScheduleResult<bool>;

    /// Extend a job lock
    async fn extend_job_lock(
        &self,
        job_id: &str,
        lock_token: &str,
        extension_secs: u64,
    ) -> ScheduleResult<bool>;

    /// Get jobs with expired locks (for recovery)
    async fn get_stale_jobs(&self, stale_threshold: DateTime<Utc>) -> ScheduleResult<Vec<Job>>;

    // ========== Execution History Operations ==========

    /// Record execution history
    async fn record_execution(&self, history: &ExecutionHistory) -> ScheduleResult<()>;

    /// Get execution history for a schedule
    async fn get_execution_history(
        &self,
        schedule_id: &str,
        limit: Option<usize>,
    ) -> ScheduleResult<Vec<ExecutionHistory>>;

    /// Delete old execution history
    async fn cleanup_history(&self, before: DateTime<Utc>) -> ScheduleResult<u64>;

    // ========== Statistics ==========

    /// Get schedule statistics
    async fn get_schedule_stats(&self, schedule_id: &str) -> ScheduleResult<ScheduleStats>;
}

/// Statistics for a schedule
#[derive(Debug, Clone, Default)]
pub struct ScheduleStats {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub average_duration_ms: Option<f64>,
    pub last_execution_at: Option<DateTime<Utc>>,
    pub next_execution_at: Option<DateTime<Utc>>,
}

// ============================================================================
// In-Memory Store Implementation
// ============================================================================

/// In-memory schedule store (for testing or single-node deployments)
pub struct InMemoryScheduleStore {
    schedules: DashMap<String, Schedule>,
    jobs: DashMap<String, Job>,
    history: Arc<RwLock<VecDeque<ExecutionHistory>>>,
    max_history_size: usize,
}

impl InMemoryScheduleStore {
    pub fn new() -> Self {
        Self {
            schedules: DashMap::new(),
            jobs: DashMap::new(),
            history: Arc::new(RwLock::new(VecDeque::new())),
            max_history_size: 10000,
        }
    }

    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history_size = max;
        self
    }
}

impl Default for InMemoryScheduleStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ScheduleStore for InMemoryScheduleStore {
    // ========== Schedule Operations ==========

    async fn create_schedule(&self, schedule: &Schedule) -> ScheduleResult<()> {
        if self.schedules.contains_key(&schedule.id) {
            return Err(ScheduleError::ScheduleAlreadyExists {
                schedule_id: schedule.id.clone(),
            });
        }
        self.schedules.insert(schedule.id.clone(), schedule.clone());
        Ok(())
    }

    async fn get_schedule(&self, schedule_id: &str) -> ScheduleResult<Option<Schedule>> {
        Ok(self.schedules.get(schedule_id).map(|s| s.clone()))
    }

    async fn update_schedule(&self, schedule: &Schedule) -> ScheduleResult<()> {
        if !self.schedules.contains_key(&schedule.id) {
            return Err(ScheduleError::ScheduleNotFound {
                schedule_id: schedule.id.clone(),
            });
        }
        self.schedules.insert(schedule.id.clone(), schedule.clone());
        Ok(())
    }

    async fn delete_schedule(&self, schedule_id: &str) -> ScheduleResult<bool> {
        // Delete associated jobs
        let job_ids: Vec<String> = self
            .jobs
            .iter()
            .filter(|j| j.schedule_id == schedule_id)
            .map(|j| j.id.clone())
            .collect();

        for job_id in job_ids {
            self.jobs.remove(&job_id);
        }

        Ok(self.schedules.remove(schedule_id).is_some())
    }

    async fn query_schedules(&self, query: ScheduleQuery) -> ScheduleResult<Vec<Schedule>> {
        let mut schedules: Vec<Schedule> = self
            .schedules
            .iter()
            .filter(|s| {
                if let Some(status) = &query.status {
                    if &s.status != status {
                        return false;
                    }
                }
                if let Some(stype) = &query.schedule_type {
                    if &s.schedule_type != stype {
                        return false;
                    }
                }
                if let Some(owner) = &query.owner_id {
                    if s.owner_id.as_ref() != Some(owner) {
                        return false;
                    }
                }
                if let Some(tags) = &query.tags {
                    if !tags.iter().any(|t| s.tags.contains(t)) {
                        return false;
                    }
                }
                if let Some(due_before) = query.due_before {
                    if let Some(next_run) = s.next_run_at {
                        if next_run > due_before {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                if let Some(search) = &query.search {
                    let search_lower = search.to_lowercase();
                    if !s.name.to_lowercase().contains(&search_lower)
                        && !s.description.to_lowercase().contains(&search_lower)
                    {
                        return false;
                    }
                }
                true
            })
            .map(|s| s.clone())
            .collect();

        // Sort by next_run_at
        schedules.sort_by(|a, b| a.next_run_at.cmp(&b.next_run_at));

        // Apply pagination
        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(usize::MAX);

        Ok(schedules.into_iter().skip(offset).take(limit).collect())
    }

    async fn count_schedules(&self, query: ScheduleQuery) -> ScheduleResult<u64> {
        let schedules = self.query_schedules(ScheduleQuery {
            limit: None,
            offset: None,
            ..query
        }).await?;
        Ok(schedules.len() as u64)
    }

    async fn get_due_schedules(&self, limit: usize) -> ScheduleResult<Vec<Schedule>> {
        self.query_schedules(ScheduleQuery::due_now().with_limit(limit)).await
    }

    // ========== Job Operations ==========

    async fn create_job(&self, job: &Job) -> ScheduleResult<()> {
        self.jobs.insert(job.id.clone(), job.clone());
        Ok(())
    }

    async fn get_job(&self, job_id: &str) -> ScheduleResult<Option<Job>> {
        Ok(self.jobs.get(job_id).map(|j| j.clone()))
    }

    async fn update_job(&self, job: &Job) -> ScheduleResult<()> {
        if !self.jobs.contains_key(&job.id) {
            return Err(ScheduleError::JobNotFound {
                job_id: job.id.clone(),
            });
        }
        self.jobs.insert(job.id.clone(), job.clone());
        Ok(())
    }

    async fn delete_job(&self, job_id: &str) -> ScheduleResult<bool> {
        Ok(self.jobs.remove(job_id).is_some())
    }

    async fn query_jobs(&self, query: JobQuery) -> ScheduleResult<Vec<Job>> {
        let mut jobs: Vec<Job> = self
            .jobs
            .iter()
            .filter(|j| {
                if let Some(schedule_id) = &query.schedule_id {
                    if &j.schedule_id != schedule_id {
                        return false;
                    }
                }
                if let Some(status) = &query.status {
                    if &j.status != status {
                        return false;
                    }
                }
                if let Some(after) = query.scheduled_after {
                    if j.scheduled_at < after {
                        return false;
                    }
                }
                if let Some(before) = query.scheduled_before {
                    if j.scheduled_at > before {
                        return false;
                    }
                }
                true
            })
            .map(|j| j.clone())
            .collect();

        // Sort by scheduled_at
        jobs.sort_by(|a, b| a.scheduled_at.cmp(&b.scheduled_at));

        // Apply pagination
        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(usize::MAX);

        Ok(jobs.into_iter().skip(offset).take(limit).collect())
    }

    async fn count_jobs(&self, query: JobQuery) -> ScheduleResult<u64> {
        let jobs = self.query_jobs(JobQuery {
            limit: None,
            offset: None,
            ..query
        }).await?;
        Ok(jobs.len() as u64)
    }

    async fn get_runnable_jobs(&self, limit: usize) -> ScheduleResult<Vec<Job>> {
        self.query_jobs(JobQuery::runnable().with_limit(limit)).await
    }

    async fn try_lock_job(
        &self,
        job_id: &str,
        worker_id: &str,
        lock_duration_secs: u64,
    ) -> ScheduleResult<Option<String>> {
        let mut job = match self.jobs.get_mut(job_id) {
            Some(j) => j,
            None => return Err(ScheduleError::JobNotFound { job_id: job_id.to_string() }),
        };

        let now = Utc::now();

        // Check if already locked by someone else
        if let Some(expires) = job.lock_expires_at {
            if expires > now && job.worker_id.as_deref() != Some(worker_id) {
                return Ok(None); // Already locked
            }
        }

        // Acquire lock
        let lock_token = uuid::Uuid::new_v4().to_string();
        job.lock_token = Some(lock_token.clone());
        job.lock_expires_at = Some(now + chrono::Duration::seconds(lock_duration_secs as i64));
        job.worker_id = Some(worker_id.to_string());
        job.updated_at = now;

        Ok(Some(lock_token))
    }

    async fn release_job_lock(&self, job_id: &str, lock_token: &str) -> ScheduleResult<bool> {
        let mut job = match self.jobs.get_mut(job_id) {
            Some(j) => j,
            None => return Err(ScheduleError::JobNotFound { job_id: job_id.to_string() }),
        };

        if job.lock_token.as_deref() != Some(lock_token) {
            return Ok(false);
        }

        job.lock_token = None;
        job.lock_expires_at = None;
        job.updated_at = Utc::now();

        Ok(true)
    }

    async fn extend_job_lock(
        &self,
        job_id: &str,
        lock_token: &str,
        extension_secs: u64,
    ) -> ScheduleResult<bool> {
        let mut job = match self.jobs.get_mut(job_id) {
            Some(j) => j,
            None => return Err(ScheduleError::JobNotFound { job_id: job_id.to_string() }),
        };

        if job.lock_token.as_deref() != Some(lock_token) {
            return Ok(false);
        }

        let now = Utc::now();
        job.lock_expires_at = Some(now + chrono::Duration::seconds(extension_secs as i64));
        job.updated_at = now;

        Ok(true)
    }

    async fn get_stale_jobs(&self, stale_threshold: DateTime<Utc>) -> ScheduleResult<Vec<Job>> {
        Ok(self
            .jobs
            .iter()
            .filter(|j| {
                j.status == JobStatus::Running
                    && j.lock_expires_at.map(|e| e < stale_threshold).unwrap_or(true)
            })
            .map(|j| j.clone())
            .collect())
    }

    // ========== Execution History Operations ==========

    async fn record_execution(&self, history: &ExecutionHistory) -> ScheduleResult<()> {
        let mut hist = self.history.write();
        hist.push_back(history.clone());

        // Enforce max size
        while hist.len() > self.max_history_size {
            hist.pop_front();
        }

        Ok(())
    }

    async fn get_execution_history(
        &self,
        schedule_id: &str,
        limit: Option<usize>,
    ) -> ScheduleResult<Vec<ExecutionHistory>> {
        let hist = self.history.read();
        let limit = limit.unwrap_or(100);

        Ok(hist
            .iter()
            .filter(|h| h.schedule_id == schedule_id)
            .rev()
            .take(limit)
            .cloned()
            .collect())
    }

    async fn cleanup_history(&self, before: DateTime<Utc>) -> ScheduleResult<u64> {
        let mut hist = self.history.write();
        let initial_len = hist.len();

        hist.retain(|h| h.created_at >= before);

        Ok((initial_len - hist.len()) as u64)
    }

    // ========== Statistics ==========

    async fn get_schedule_stats(&self, schedule_id: &str) -> ScheduleResult<ScheduleStats> {
        let hist = self.history.read();
        let schedule = self.schedules.get(schedule_id);

        let executions: Vec<_> = hist
            .iter()
            .filter(|h| h.schedule_id == schedule_id)
            .collect();

        let total = executions.len() as u64;
        let successful = executions
            .iter()
            .filter(|h| h.status == JobStatus::Completed)
            .count() as u64;
        let failed = executions
            .iter()
            .filter(|h| h.status == JobStatus::Failed)
            .count() as u64;

        let durations: Vec<i64> = executions
            .iter()
            .filter_map(|h| h.duration_ms)
            .collect();

        let average_duration_ms = if durations.is_empty() {
            None
        } else {
            Some(durations.iter().sum::<i64>() as f64 / durations.len() as f64)
        };

        let last_execution_at = executions
            .iter()
            .filter_map(|h| h.completed_at)
            .max();

        let next_execution_at = schedule.and_then(|s| s.next_run_at);

        Ok(ScheduleStats {
            total_executions: total,
            successful_executions: successful,
            failed_executions: failed,
            average_duration_ms,
            last_execution_at,
            next_execution_at,
        })
    }
}

// ============================================================================
// Persistent Store (uses harana-components-storage)
// ============================================================================

use harana_components_storage::{FilterCondition, QueryOptions, Store};
use std::marker::PhantomData;

use harana_components_lock::{DistributedLock, DistributedLockManager, LockConfig, LockManager, job_lock_resource};

/// Persistent schedule store that uses the storage component.
/// This store delegates all persistence operations to an underlying `Store` implementation
/// from harana-components-storage, supporting SQL (Postgres, MySQL, SQLite) and MongoDB backends.
///
/// Includes integrated distributed locking with deadlock prevention:
/// - Lock timeouts (TTL) prevent indefinite holding
/// - Lock ordering prevents circular waits  
/// - Wait timeouts prevent indefinite waiting
/// - Fencing tokens prevent stale lock holders from making changes
pub struct PersistentScheduleStore<S>
where
    S: Store<Schedule> + Store<Job> + Store<ExecutionHistory> + Store<DistributedLock>,
{
    store: S,
    lock_manager: Arc<DistributedLockManager<S>>,
    _phantom: PhantomData<(Schedule, Job, ExecutionHistory)>,
}

impl<S> PersistentScheduleStore<S>
where
    S: Store<Schedule> + Store<Job> + Store<ExecutionHistory> + Store<DistributedLock> + Clone + 'static,
{
    /// Create a new persistent schedule store with the given storage backend.
    ///
    /// # Example
    /// ```ignore
    /// use harana_components_storage::sql::postgres::PgBackend;
    /// use harana_components_schedule::PersistentScheduleStore;
    ///
    /// let pg_backend = PgBackend::new(pool).await?;
    /// let store = PersistentScheduleStore::new(pg_backend);
    /// ```
    pub fn new(store: S) -> Self {
        let lock_manager = Arc::new(DistributedLockManager::new(
            store.clone(),
            LockConfig::default(),
        ));
        Self {
            store,
            lock_manager,
            _phantom: PhantomData,
        }
    }

    /// Create a new persistent schedule store with custom lock configuration.
    ///
    /// # Example
    /// ```ignore
    /// use harana_components_schedule::{PersistentScheduleStore, LockConfig};
    ///
    /// let config = LockConfig::new()
    ///     .with_ttl(60)
    ///     .with_wait_timeout(30000)
    ///     .with_lock_ordering(true);
    /// let store = PersistentScheduleStore::with_lock_config(backend, config);
    /// ```
    pub fn with_lock_config(store: S, lock_config: LockConfig) -> Self {
        let lock_manager = Arc::new(DistributedLockManager::new(
            store.clone(),
            lock_config,
        ));
        Self {
            store,
            lock_manager,
            _phantom: PhantomData,
        }
    }

    /// Get a reference to the underlying storage backend.
    pub fn inner(&self) -> &S {
        &self.store
    }

    /// Get a reference to the lock manager for advanced locking operations.
    pub fn lock_manager(&self) -> &Arc<DistributedLockManager<S>> {
        &self.lock_manager
    }

    /// Initialize the store (call on startup to sync fencing tokens).
    pub async fn initialize(&self) -> ScheduleResult<()> {
        self.lock_manager.initialize().await.map_err(|e| e.into())
    }

    /// Clean up expired locks (call periodically for housekeeping).
    pub async fn cleanup_expired_locks(&self) -> ScheduleResult<u64> {
        self.lock_manager.cleanup_expired_locks().await.map_err(|e| e.into())
    }
}

#[async_trait]
impl<S> ScheduleStore for PersistentScheduleStore<S>
where
    S: Store<Schedule> + Store<Job> + Store<ExecutionHistory> + Store<DistributedLock> + Send + Sync + Clone + 'static,
{
    async fn create_schedule(&self, schedule: &Schedule) -> ScheduleResult<()> {
        Store::<Schedule>::create(&self.store, schedule)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn get_schedule(&self, schedule_id: &str) -> ScheduleResult<Option<Schedule>> {
        Store::<Schedule>::find_by_id(&self.store, schedule_id)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn update_schedule(&self, schedule: &Schedule) -> ScheduleResult<()> {
        Store::<Schedule>::update(&self.store, schedule)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn delete_schedule(&self, schedule_id: &str) -> ScheduleResult<bool> {
        // First delete all jobs for this schedule
        let _ = Store::<Job>::delete_many(&self.store, FilterCondition::eq("schedule_id", schedule_id)).await;

        Store::<Schedule>::delete(&self.store, schedule_id)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn query_schedules(&self, query: ScheduleQuery) -> ScheduleResult<Vec<Schedule>> {
        let mut conditions = Vec::new();

        if let Some(status) = query.status {
            conditions.push(FilterCondition::eq("status", status.as_str()));
        }

        if let Some(owner) = query.owner_id {
            conditions.push(FilterCondition::eq("owner_id", owner));
        }

        if let Some(due_before) = query.due_before {
            conditions.push(FilterCondition::lte("next_run_at", due_before.to_rfc3339()));
        }

        let filter = if conditions.is_empty() {
            None
        } else if conditions.len() == 1 {
            Some(conditions.remove(0))
        } else {
            Some(FilterCondition::And(conditions))
        };

        let mut options = QueryOptions::new()
            .with_sort("next_run_at", false);

        if let Some(limit) = query.limit {
            options = options.with_limit(limit as u32);
        }
        if let Some(offset) = query.offset {
            options = options.with_offset(offset as u32);
        }

        Store::<Schedule>::find_many(&self.store, filter, options)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn count_schedules(&self, query: ScheduleQuery) -> ScheduleResult<u64> {
        let mut conditions = Vec::new();

        if let Some(status) = query.status {
            conditions.push(FilterCondition::eq("status", status.as_str()));
        }

        let filter = if conditions.is_empty() {
            None
        } else if conditions.len() == 1 {
            Some(conditions.remove(0))
        } else {
            Some(FilterCondition::And(conditions))
        };

        Store::<Schedule>::count(&self.store, filter)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn get_due_schedules(&self, limit: usize) -> ScheduleResult<Vec<Schedule>> {
        self.query_schedules(ScheduleQuery::due_now().with_limit(limit)).await
    }

    async fn create_job(&self, job: &Job) -> ScheduleResult<()> {
        Store::<Job>::create(&self.store, job)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn get_job(&self, job_id: &str) -> ScheduleResult<Option<Job>> {
        Store::<Job>::find_by_id(&self.store, job_id)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn update_job(&self, job: &Job) -> ScheduleResult<()> {
        Store::<Job>::update(&self.store, job)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn delete_job(&self, job_id: &str) -> ScheduleResult<bool> {
        Store::<Job>::delete(&self.store, job_id)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn query_jobs(&self, query: JobQuery) -> ScheduleResult<Vec<Job>> {
        let mut conditions = Vec::new();

        if let Some(schedule_id) = query.schedule_id {
            conditions.push(FilterCondition::eq("schedule_id", schedule_id));
        }

        if let Some(status) = query.status {
            conditions.push(FilterCondition::eq("status", status.as_str()));
        }

        if let Some(after) = query.scheduled_after {
            conditions.push(FilterCondition::gte("scheduled_at", after.to_rfc3339()));
        }

        if let Some(before) = query.scheduled_before {
            conditions.push(FilterCondition::lte("scheduled_at", before.to_rfc3339()));
        }

        let filter = if conditions.is_empty() {
            None
        } else if conditions.len() == 1 {
            Some(conditions.remove(0))
        } else {
            Some(FilterCondition::And(conditions))
        };

        let mut options = QueryOptions::new()
            .with_sort("scheduled_at", false);

        if let Some(limit) = query.limit {
            options = options.with_limit(limit as u32);
        }
        if let Some(offset) = query.offset {
            options = options.with_offset(offset as u32);
        }

        Store::<Job>::find_many(&self.store, filter, options)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn count_jobs(&self, query: JobQuery) -> ScheduleResult<u64> {
        let mut conditions = Vec::new();

        if let Some(schedule_id) = query.schedule_id {
            conditions.push(FilterCondition::eq("schedule_id", schedule_id));
        }

        if let Some(status) = query.status {
            conditions.push(FilterCondition::eq("status", status.as_str()));
        }

        let filter = if conditions.is_empty() {
            None
        } else if conditions.len() == 1 {
            Some(conditions.remove(0))
        } else {
            Some(FilterCondition::And(conditions))
        };

        Store::<Job>::count(&self.store, filter)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn get_runnable_jobs(&self, limit: usize) -> ScheduleResult<Vec<Job>> {
        self.query_jobs(JobQuery::runnable().with_limit(limit)).await
    }

    async fn try_lock_job(
        &self,
        job_id: &str,
        worker_id: &str,
        lock_duration_secs: u64,
    ) -> ScheduleResult<Option<String>> {
        // Use the distributed lock manager with deadlock prevention
        let resource_id = job_lock_resource(job_id);
        
        match self.lock_manager.try_acquire(&resource_id, worker_id, Some(lock_duration_secs)).await? {
            Some(fencing_token) => {
                // Update the job record with lock information
                let job = Store::<Job>::find_by_id(&self.store, job_id)
                    .await
                    .map_err(|e| ScheduleError::StorageError(e.to_string()))?
                    .ok_or_else(|| ScheduleError::JobNotFound { job_id: job_id.to_string() })?;

                let now = Utc::now();
                let lock_token = format!("{}:{}", fencing_token, uuid::Uuid::new_v4());
                
                let mut updated_job = job;
                updated_job.lock_token = Some(lock_token.clone());
                updated_job.lock_expires_at = Some(now + chrono::Duration::seconds(lock_duration_secs as i64));
                updated_job.worker_id = Some(worker_id.to_string());
                updated_job.updated_at = now;

                Store::<Job>::update(&self.store, &updated_job)
                    .await
                    .map_err(|e| ScheduleError::StorageError(e.to_string()))?;

                Ok(Some(lock_token))
            }
            None => Ok(None),
        }
    }

    async fn release_job_lock(&self, job_id: &str, lock_token: &str) -> ScheduleResult<bool> {
        let job = Store::<Job>::find_by_id(&self.store, job_id)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))?
            .ok_or_else(|| ScheduleError::JobNotFound { job_id: job_id.to_string() })?;

        // Verify the lock token matches
        if job.lock_token.as_deref() != Some(lock_token) {
            return Ok(false);
        }

        // Extract worker_id from the job to release the distributed lock
        let worker_id = job.worker_id.clone().unwrap_or_default();
        let resource_id = job_lock_resource(job_id);

        // Release the distributed lock
        self.lock_manager.release_lock(&resource_id, &worker_id).await?;

        // Update the job record
        let mut updated_job = job;
        updated_job.lock_token = None;
        updated_job.lock_expires_at = None;
        updated_job.updated_at = Utc::now();

        Store::<Job>::update(&self.store, &updated_job)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))?;

        Ok(true)
    }

    async fn extend_job_lock(
        &self,
        job_id: &str,
        lock_token: &str,
        extension_secs: u64,
    ) -> ScheduleResult<bool> {
        let job = Store::<Job>::find_by_id(&self.store, job_id)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))?
            .ok_or_else(|| ScheduleError::JobNotFound { job_id: job_id.to_string() })?;

        // Verify the lock token matches
        if job.lock_token.as_deref() != Some(lock_token) {
            return Ok(false);
        }

        // Extend the distributed lock
        let worker_id = job.worker_id.clone().unwrap_or_default();
        let resource_id = job_lock_resource(job_id);
        
        if !self.lock_manager.extend_lock(&resource_id, &worker_id, extension_secs).await? {
            return Ok(false);
        }

        // Update the job record
        let now = Utc::now();
        let mut updated_job = job;
        updated_job.lock_expires_at = Some(now + chrono::Duration::seconds(extension_secs as i64));
        updated_job.updated_at = now;

        Store::<Job>::update(&self.store, &updated_job)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))?;

        Ok(true)
    }

    async fn get_stale_jobs(&self, stale_threshold: DateTime<Utc>) -> ScheduleResult<Vec<Job>> {
        let filter = FilterCondition::And(vec![
            FilterCondition::eq("status", JobStatus::Running.as_str()),
            FilterCondition::lt("lock_expires_at", stale_threshold.to_rfc3339()),
        ]);

        Store::<Job>::find_many(&self.store, Some(filter), QueryOptions::new())
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn record_execution(&self, history: &ExecutionHistory) -> ScheduleResult<()> {
        Store::<ExecutionHistory>::create(&self.store, history)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn get_execution_history(
        &self,
        schedule_id: &str,
        limit: Option<usize>,
    ) -> ScheduleResult<Vec<ExecutionHistory>> {
        let filter = FilterCondition::eq("schedule_id", schedule_id);
        let mut options = QueryOptions::new()
            .with_sort("created_at", true);

        if let Some(limit) = limit {
            options = options.with_limit(limit as u32);
        }

        Store::<ExecutionHistory>::find_many(&self.store, Some(filter), options)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn cleanup_history(&self, before: DateTime<Utc>) -> ScheduleResult<u64> {
        let filter = FilterCondition::lt("created_at", before.to_rfc3339());

        Store::<ExecutionHistory>::delete_many(&self.store, filter)
            .await
            .map_err(|e| ScheduleError::StorageError(e.to_string()))
    }

    async fn get_schedule_stats(&self, schedule_id: &str) -> ScheduleResult<ScheduleStats> {
        let history = self.get_execution_history(schedule_id, Some(1000)).await?;
        let schedule = self.get_schedule(schedule_id).await?;

        let total = history.len() as u64;
        let successful = history.iter().filter(|h| h.status == JobStatus::Completed).count() as u64;
        let failed = history.iter().filter(|h| h.status == JobStatus::Failed).count() as u64;

        let durations: Vec<i64> = history.iter().filter_map(|h| h.duration_ms).collect();
        let average_duration_ms = if durations.is_empty() {
            None
        } else {
            Some(durations.iter().sum::<i64>() as f64 / durations.len() as f64)
        };

        let last_execution_at = history.iter().filter_map(|h| h.completed_at).max();
        let next_execution_at = schedule.and_then(|s| s.next_run_at);

        Ok(ScheduleStats {
            total_executions: total,
            successful_executions: successful,
            failed_executions: failed,
            average_duration_ms,
            last_execution_at,
            next_execution_at,
        })
    }
}
