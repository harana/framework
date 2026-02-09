use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use cron::Schedule as CronSchedule;
use parking_lot::RwLock;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

use crate::{
    ExecutionHistory, Job, JobExecutor, JobStatus, Schedule, ScheduleError, ScheduleQuery, ScheduleResult,
    ScheduleStats, ScheduleStatus, ScheduleStore, ScheduleType, SchedulerConfig, SchedulerEvent,
};

// ============================================================================
// Durable Scheduler
// ============================================================================

struct SchedulerState {
    running: bool,
    active_jobs: usize,
}

pub struct DurableScheduler<S: ScheduleStore> {
    store: Arc<S>,
    config: SchedulerConfig,
    executors: RwLock<Vec<Arc<dyn JobExecutor>>>,
    state: RwLock<SchedulerState>,
    event_sender: broadcast::Sender<SchedulerEvent>,
    shutdown_sender: Option<mpsc::Sender<()>>,
    tasks: RwLock<Vec<JoinHandle<()>>>,
}

impl<S: ScheduleStore + 'static> DurableScheduler<S> {
    pub fn new(store: S, config: SchedulerConfig) -> Self {
        let (event_sender, _) = broadcast::channel(1000);

        Self {
            store: Arc::new(store),
            config,
            executors: RwLock::new(Vec::new()),
            state: RwLock::new(SchedulerState {
                running: false,
                active_jobs: 0,
            }),
            event_sender,
            shutdown_sender: None,
            tasks: RwLock::new(Vec::new()),
        }
    }
    pub fn register_executor(&self, executor: Arc<dyn JobExecutor>) {
        self.executors.write().push(executor);
    }
    pub fn subscribe(&self) -> broadcast::Receiver<SchedulerEvent> {
        self.event_sender.subscribe()
    }
    pub fn store(&self) -> &S {
        &self.store
    }
    pub fn is_running(&self) -> bool {
        self.state.read().running
    }
    fn emit(&self, event: SchedulerEvent) {
        let _ = self.event_sender.send(event);
    }

    // ========================================================================
    // Schedule Management
    // ========================================================================
    pub async fn create_schedule(&self, mut schedule: Schedule) -> ScheduleResult<Schedule> {
        // Validate and calculate next run time
        self.calculate_next_run(&mut schedule)?;

        // Store schedule
        self.store.create_schedule(&schedule).await?;

        self.emit(SchedulerEvent::ScheduleCreated {
            schedule_id: schedule.id.clone(),
        });

        info!(schedule_id = %schedule.id, name = %schedule.name, "Schedule created");

        Ok(schedule)
    }
    pub async fn get_schedule(&self, schedule_id: &str) -> ScheduleResult<Option<Schedule>> {
        self.store.get_schedule(schedule_id).await
    }
    pub async fn update_schedule(&self, mut schedule: Schedule) -> ScheduleResult<Schedule> {
        // Recalculate next run time
        self.calculate_next_run(&mut schedule)?;
        schedule.updated_at = Utc::now();
        schedule.version += 1;

        self.store.update_schedule(&schedule).await?;

        self.emit(SchedulerEvent::ScheduleUpdated {
            schedule_id: schedule.id.clone(),
        });

        info!(schedule_id = %schedule.id, "Schedule updated");

        Ok(schedule)
    }
    pub async fn delete_schedule(&self, schedule_id: &str) -> ScheduleResult<bool> {
        let deleted = self.store.delete_schedule(schedule_id).await?;

        if deleted {
            self.emit(SchedulerEvent::ScheduleDeleted {
                schedule_id: schedule_id.to_string(),
            });
            info!(schedule_id = %schedule_id, "Schedule deleted");
        }

        Ok(deleted)
    }
    pub async fn list_schedules(&self, query: ScheduleQuery) -> ScheduleResult<Vec<Schedule>> {
        self.store.query_schedules(query).await
    }
    pub async fn enable_schedule(&self, schedule_id: &str) -> ScheduleResult<Schedule> {
        let mut schedule =
            self.store
                .get_schedule(schedule_id)
                .await?
                .ok_or_else(|| ScheduleError::ScheduleNotFound {
                    schedule_id: schedule_id.to_string(),
                })?;

        schedule.status = ScheduleStatus::Active;
        self.calculate_next_run(&mut schedule)?;

        self.update_schedule(schedule).await
    }
    pub async fn disable_schedule(&self, schedule_id: &str) -> ScheduleResult<Schedule> {
        let mut schedule =
            self.store
                .get_schedule(schedule_id)
                .await?
                .ok_or_else(|| ScheduleError::ScheduleNotFound {
                    schedule_id: schedule_id.to_string(),
                })?;

        schedule.status = ScheduleStatus::Disabled;
        schedule.next_run_at = None;

        self.update_schedule(schedule).await
    }
    pub async fn pause_schedule(
        &self,
        schedule_id: &str,
        resume_at: Option<DateTime<Utc>>,
    ) -> ScheduleResult<Schedule> {
        let mut schedule =
            self.store
                .get_schedule(schedule_id)
                .await?
                .ok_or_else(|| ScheduleError::ScheduleNotFound {
                    schedule_id: schedule_id.to_string(),
                })?;

        schedule.status = ScheduleStatus::Paused;
        schedule.resume_at = resume_at;
        schedule.next_run_at = None;

        self.update_schedule(schedule).await
    }
    pub async fn resume_schedule(&self, schedule_id: &str) -> ScheduleResult<Schedule> {
        let mut schedule =
            self.store
                .get_schedule(schedule_id)
                .await?
                .ok_or_else(|| ScheduleError::ScheduleNotFound {
                    schedule_id: schedule_id.to_string(),
                })?;

        if schedule.status != ScheduleStatus::Paused {
            return Err(ScheduleError::Internal("Schedule is not paused".to_string()));
        }

        schedule.status = ScheduleStatus::Active;
        schedule.resume_at = None;
        self.calculate_next_run(&mut schedule)?;

        self.update_schedule(schedule).await
    }
    pub async fn trigger_now(&self, schedule_id: &str) -> ScheduleResult<Job> {
        let schedule = self
            .store
            .get_schedule(schedule_id)
            .await?
            .ok_or_else(|| ScheduleError::ScheduleNotFound {
                schedule_id: schedule_id.to_string(),
            })?;

        let job = Job::new(&schedule, Utc::now());
        self.store.create_job(&job).await?;

        self.emit(SchedulerEvent::JobCreated {
            job_id: job.id.clone(),
            schedule_id: schedule_id.to_string(),
        });

        info!(
            job_id = %job.id,
            schedule_id = %schedule_id,
            "Manual trigger - job created"
        );

        Ok(job)
    }
    pub async fn get_stats(&self, schedule_id: &str) -> ScheduleResult<ScheduleStats> {
        self.store.get_schedule_stats(schedule_id).await
    }

    // ========================================================================
    // Job Management
    // ========================================================================
    pub async fn get_job(&self, job_id: &str) -> ScheduleResult<Option<Job>> {
        self.store.get_job(job_id).await
    }
    pub async fn cancel_job(&self, job_id: &str) -> ScheduleResult<Job> {
        let mut job = self
            .store
            .get_job(job_id)
            .await?
            .ok_or_else(|| ScheduleError::JobNotFound {
                job_id: job_id.to_string(),
            })?;

        if job.status.is_terminal() {
            return Err(ScheduleError::ExecutionAlreadyCompleted {
                execution_id: job_id.to_string(),
            });
        }

        job.cancel();
        self.store.update_job(&job).await?;

        // Record in history
        let history = ExecutionHistory::from_job(&job);
        self.store.record_execution(&history).await?;

        info!(job_id = %job_id, "Job cancelled");

        Ok(job)
    }
    pub async fn get_history(&self, schedule_id: &str, limit: Option<usize>) -> ScheduleResult<Vec<ExecutionHistory>> {
        self.store.get_execution_history(schedule_id, limit).await
    }

    // ========================================================================
    // Schedule Calculation
    // ========================================================================
    fn calculate_next_run(&self, schedule: &mut Schedule) -> ScheduleResult<()> {
        if schedule.status != ScheduleStatus::Active {
            schedule.next_run_at = None;
            return Ok(());
        }

        let now = Utc::now();

        // Check if schedule has expired
        if let Some(end) = schedule.end_at {
            if now > end {
                schedule.status = ScheduleStatus::Expired;
                schedule.next_run_at = None;
                return Ok(());
            }
        }

        // Check max executions
        if let Some(max) = schedule.max_executions {
            if schedule.execution_count >= max {
                schedule.status = ScheduleStatus::Completed;
                schedule.next_run_at = None;
                return Ok(());
            }
        }

        let next_run = match schedule.schedule_type {
            ScheduleType::Cron => {
                let cron_expr =
                    schedule
                        .cron_expression
                        .as_ref()
                        .ok_or_else(|| ScheduleError::InvalidCronExpression {
                            expression: "".to_string(),
                            reason: "No cron expression specified".to_string(),
                        })?;

                self.calculate_next_cron_run(cron_expr, &schedule.timezone)?
            }
            ScheduleType::Interval => {
                let interval = schedule
                    .interval_seconds
                    .ok_or_else(|| ScheduleError::InvalidInterval {
                        reason: "No interval specified".to_string(),
                    })?;

                let base = schedule.last_run_at.unwrap_or(now);
                base + chrono::Duration::seconds(interval)
            }
            ScheduleType::OneTime => schedule
                .run_at
                .ok_or_else(|| ScheduleError::Internal("No run_at time specified for one-time schedule".to_string()))?,
        };

        // Ensure next run is after start_at
        let next_run = if let Some(start) = schedule.start_at {
            if next_run < start { start } else { next_run }
        } else {
            next_run
        };

        schedule.next_run_at = Some(next_run);
        Ok(())
    }
    fn calculate_next_cron_run(&self, cron_expr: &str, timezone: &str) -> ScheduleResult<DateTime<Utc>> {
        let cron_schedule = CronSchedule::from_str(cron_expr).map_err(|e| ScheduleError::InvalidCronExpression {
            expression: cron_expr.to_string(),
            reason: e.to_string(),
        })?;

        let tz: Tz = timezone.parse().map_err(|_| ScheduleError::InvalidTimezone {
            timezone: timezone.to_string(),
        })?;

        let now_tz = Utc::now().with_timezone(&tz);

        cron_schedule
            .upcoming(tz)
            .next()
            .map(|dt| dt.with_timezone(&Utc))
            .ok_or_else(|| ScheduleError::InvalidCronExpression {
                expression: cron_expr.to_string(),
                reason: "Could not calculate next run time".to_string(),
            })
    }
    pub fn get_next_runs(&self, schedule: &Schedule, count: usize) -> ScheduleResult<Vec<DateTime<Utc>>> {
        match schedule.schedule_type {
            ScheduleType::Cron => {
                let cron_expr =
                    schedule
                        .cron_expression
                        .as_ref()
                        .ok_or_else(|| ScheduleError::InvalidCronExpression {
                            expression: "".to_string(),
                            reason: "No cron expression specified".to_string(),
                        })?;

                let cron_schedule =
                    CronSchedule::from_str(cron_expr).map_err(|e| ScheduleError::InvalidCronExpression {
                        expression: cron_expr.to_string(),
                        reason: e.to_string(),
                    })?;

                let tz: Tz = schedule.timezone.parse().map_err(|_| ScheduleError::InvalidTimezone {
                    timezone: schedule.timezone.clone(),
                })?;

                Ok(cron_schedule
                    .upcoming(tz)
                    .take(count)
                    .map(|dt| dt.with_timezone(&Utc))
                    .collect())
            }
            ScheduleType::Interval => {
                let interval = schedule
                    .interval_seconds
                    .ok_or_else(|| ScheduleError::InvalidInterval {
                        reason: "No interval specified".to_string(),
                    })?;

                let mut runs = Vec::with_capacity(count);
                let mut next = schedule.next_run_at.unwrap_or_else(Utc::now);

                for _ in 0..count {
                    runs.push(next);
                    next = next + chrono::Duration::seconds(interval);
                }

                Ok(runs)
            }
            ScheduleType::OneTime => Ok(schedule.run_at.map(|t| vec![t]).unwrap_or_default()),
        }
    }

    // ========================================================================
    // Scheduler Loop
    // ========================================================================
    pub async fn start(&mut self) -> ScheduleResult<()> {
        {
            let mut state = self.state.write();
            if state.running {
                return Ok(());
            }
            state.running = true;
        }

        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        self.shutdown_sender = Some(shutdown_tx);

        // Start the main scheduler loop
        let scheduler = self.clone_for_task();
        let poll_task = tokio::spawn(async move {
            scheduler.scheduler_loop(shutdown_rx).await;
        });

        // Start the stale job recovery task
        let scheduler = self.clone_for_task();
        let (_, stale_shutdown_rx) = mpsc::channel(1);
        let stale_task = tokio::spawn(async move {
            scheduler.stale_recovery_loop(stale_shutdown_rx).await;
        });

        // Start the cleanup task
        let scheduler = self.clone_for_task();
        let (_, cleanup_shutdown_rx) = mpsc::channel(1);
        let cleanup_task = tokio::spawn(async move {
            scheduler.cleanup_loop(cleanup_shutdown_rx).await;
        });

        {
            let mut tasks = self.tasks.write();
            tasks.push(poll_task);
            tasks.push(stale_task);
            tasks.push(cleanup_task);
        }

        self.emit(SchedulerEvent::Started {
            worker_id: self.config.worker_id.clone(),
        });

        info!(worker_id = %self.config.worker_id, "Scheduler started");

        Ok(())
    }
    pub async fn stop(&self) -> ScheduleResult<()> {
        {
            let mut state = self.state.write();
            if !state.running {
                return Ok(());
            }
            state.running = false;
        }

        // Signal shutdown (shutdown_sender will be dropped)

        self.emit(SchedulerEvent::Stopped {
            worker_id: self.config.worker_id.clone(),
        });

        info!(worker_id = %self.config.worker_id, "Scheduler stopped");

        Ok(())
    }
    fn clone_for_task(&self) -> SchedulerTaskHandle<S> {
        SchedulerTaskHandle {
            store: Arc::clone(&self.store),
            config: self.config.clone(),
            executors: self.executors.read().clone(),
            event_sender: self.event_sender.clone(),
            state: Arc::new(RwLock::new(true)),
        }
    }
}

struct SchedulerTaskHandle<S: ScheduleStore> {
    store: Arc<S>,
    config: SchedulerConfig,
    executors: Vec<Arc<dyn JobExecutor>>,
    event_sender: broadcast::Sender<SchedulerEvent>,
    state: Arc<RwLock<bool>>,
}

impl<S: ScheduleStore + 'static> SchedulerTaskHandle<S> {
    fn emit(&self, event: SchedulerEvent) {
        let _ = self.event_sender.send(event);
    }

    fn is_running(&self) -> bool {
        *self.state.read()
    }

    async fn scheduler_loop(&self, mut shutdown: mpsc::Receiver<()>) {
        let poll_interval = Duration::from_secs(self.config.poll_interval_secs);

        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("Scheduler loop shutting down");
                    break;
                }
                _ = tokio::time::sleep(poll_interval) => {
                    if let Err(e) = self.process_due_schedules().await {
                        error!(error = %e, "Error processing due schedules");
                    }

                    if let Err(e) = self.process_runnable_jobs().await {
                        error!(error = %e, "Error processing runnable jobs");
                    }
                }
            }
        }
    }

    async fn process_due_schedules(&self) -> ScheduleResult<()> {
        if !self.config.auto_create_jobs {
            return Ok(());
        }

        let due_schedules = self.store.get_due_schedules(self.config.batch_size).await?;

        for schedule in due_schedules {
            if let Err(e) = self.create_job_for_schedule(&schedule).await {
                error!(
                    schedule_id = %schedule.id,
                    error = %e,
                    "Failed to create job for schedule"
                );
            }
        }

        Ok(())
    }

    async fn create_job_for_schedule(&self, schedule: &Schedule) -> ScheduleResult<()> {
        let scheduled_at = schedule.next_run_at.unwrap_or_else(Utc::now);
        let job = Job::new(schedule, scheduled_at);

        self.store.create_job(&job).await?;

        // Update schedule
        let mut updated_schedule = schedule.clone();
        updated_schedule.last_run_at = Some(scheduled_at);
        updated_schedule.execution_count += 1;

        // Calculate next run
        match schedule.schedule_type {
            ScheduleType::Cron => {
                if let Some(cron_expr) = &schedule.cron_expression {
                    let cron_schedule =
                        CronSchedule::from_str(cron_expr).map_err(|e| ScheduleError::InvalidCronExpression {
                            expression: cron_expr.to_string(),
                            reason: e.to_string(),
                        })?;

                    let tz: Tz = schedule.timezone.parse().map_err(|_| ScheduleError::InvalidTimezone {
                        timezone: schedule.timezone.clone(),
                    })?;

                    updated_schedule.next_run_at = cron_schedule.upcoming(tz).next().map(|dt| dt.with_timezone(&Utc));
                }
            }
            ScheduleType::Interval => {
                if let Some(interval) = schedule.interval_seconds {
                    updated_schedule.next_run_at = Some(scheduled_at + chrono::Duration::seconds(interval));
                }
            }
            ScheduleType::OneTime => {
                updated_schedule.status = ScheduleStatus::Completed;
                updated_schedule.next_run_at = None;
            }
        }

        updated_schedule.updated_at = Utc::now();
        self.store.update_schedule(&updated_schedule).await?;

        self.emit(SchedulerEvent::JobCreated {
            job_id: job.id.clone(),
            schedule_id: schedule.id.clone(),
        });

        debug!(
            job_id = %job.id,
            schedule_id = %schedule.id,
            "Job created for schedule"
        );

        Ok(())
    }

    async fn process_runnable_jobs(&self) -> ScheduleResult<()> {
        let runnable_jobs = self.store.get_runnable_jobs(self.config.batch_size).await?;

        for job in runnable_jobs {
            // Try to acquire lock
            let lock_token = self
                .store
                .try_lock_job(&job.id, &self.config.worker_id, self.config.lock_duration_secs)
                .await?;

            if let Some(token) = lock_token {
                // Execute job in background
                let job_id = job.id.clone();
                let schedule_id = job.schedule_id.clone();

                if let Err(e) = self.execute_job(job, token).await {
                    error!(
                        job_id = %job_id,
                        schedule_id = %schedule_id,
                        error = %e,
                        "Failed to execute job"
                    );
                }
            }
        }

        Ok(())
    }

    async fn execute_job(&self, mut job: Job, lock_token: String) -> ScheduleResult<()> {
        let executor = self.executors.iter().find(|e| e.can_handle(&job.action_type)).cloned();

        let executor = match executor {
            Some(e) => e,
            None => {
                warn!(
                    job_id = %job.id,
                    action_type = %job.action_type,
                    "No executor found for action type"
                );
                job.fail(format!("No executor for action type: {}", job.action_type), None);
                self.store.update_job(&job).await?;
                self.store.release_job_lock(&job.id, &lock_token).await?;
                return Ok(());
            }
        };

        // Start job
        job.start(Some(self.config.worker_id.clone()));
        self.store.update_job(&job).await?;

        self.emit(SchedulerEvent::JobStarted {
            job_id: job.id.clone(),
            schedule_id: job.schedule_id.clone(),
        });

        // Execute
        let result = executor.execute(&job).await;

        // Update job status
        match result {
            Ok(result_value) => {
                job.complete(result_value);
                self.store.update_job(&job).await?;

                self.emit(SchedulerEvent::JobCompleted {
                    job_id: job.id.clone(),
                    schedule_id: job.schedule_id.clone(),
                    duration_ms: job.duration_ms.unwrap_or(0),
                });

                info!(
                    job_id = %job.id,
                    duration_ms = ?job.duration_ms,
                    "Job completed successfully"
                );
            }
            Err(error) => {
                if job.can_retry() {
                    let retry_delay = job.max_retries as u64 * 10; // Simple backoff
                    let retry_at = Utc::now() + chrono::Duration::seconds(retry_delay as i64);
                    job.retry(retry_at);
                    self.store.update_job(&job).await?;

                    self.emit(SchedulerEvent::JobRetrying {
                        job_id: job.id.clone(),
                        schedule_id: job.schedule_id.clone(),
                        attempt: job.retry_attempt,
                    });

                    warn!(
                        job_id = %job.id,
                        attempt = job.retry_attempt,
                        error = %error,
                        "Job failed, will retry"
                    );
                } else {
                    job.fail(error.clone(), None);
                    self.store.update_job(&job).await?;

                    self.emit(SchedulerEvent::JobFailed {
                        job_id: job.id.clone(),
                        schedule_id: job.schedule_id.clone(),
                        error,
                    });

                    error!(job_id = %job.id, "Job failed permanently");
                }
            }
        }

        // Record history
        let history = ExecutionHistory::from_job(&job);
        self.store.record_execution(&history).await?;

        // Release lock
        self.store.release_job_lock(&job.id, &lock_token).await?;

        Ok(())
    }

    async fn stale_recovery_loop(&self, mut shutdown: mpsc::Receiver<()>) {
        let interval = Duration::from_secs(self.config.stale_check_interval_secs);

        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("Stale recovery loop shutting down");
                    break;
                }
                _ = tokio::time::sleep(interval) => {
                    if let Err(e) = self.recover_stale_jobs().await {
                        error!(error = %e, "Error recovering stale jobs");
                    }
                }
            }
        }
    }

    async fn recover_stale_jobs(&self) -> ScheduleResult<()> {
        let threshold = Utc::now() - chrono::Duration::seconds(self.config.stale_threshold_secs as i64);
        let stale_jobs = self.store.get_stale_jobs(threshold).await?;

        for mut job in stale_jobs {
            warn!(job_id = %job.id, "Recovering stale job");

            if job.can_retry() {
                let retry_at = Utc::now() + chrono::Duration::seconds(10);
                job.retry(retry_at);
                job.lock_token = None;
                job.lock_expires_at = None;
                self.store.update_job(&job).await?;

                self.emit(SchedulerEvent::StaleJobRecovered { job_id: job.id.clone() });
            } else {
                job.fail("Job timed out".to_string(), None);
                self.store.update_job(&job).await?;

                self.emit(SchedulerEvent::JobFailed {
                    job_id: job.id.clone(),
                    schedule_id: job.schedule_id.clone(),
                    error: "Job timed out".to_string(),
                });
            }
        }

        Ok(())
    }

    async fn cleanup_loop(&self, mut shutdown: mpsc::Receiver<()>) {
        let interval = Duration::from_secs(self.config.cleanup_interval_secs);

        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("Cleanup loop shutting down");
                    break;
                }
                _ = tokio::time::sleep(interval) => {
                    if let Err(e) = self.cleanup_old_history().await {
                        error!(error = %e, "Error cleaning up history");
                    }
                }
            }
        }
    }

    async fn cleanup_old_history(&self) -> ScheduleResult<()> {
        let cutoff = Utc::now() - chrono::Duration::days(self.config.history_retention_days as i64);
        let deleted = self.store.cleanup_history(cutoff).await?;

        if deleted > 0 {
            info!(deleted = deleted, "Cleaned up old execution history");
        }

        Ok(())
    }
}

// ============================================================================
// Helper Functions
// ============================================================================
pub fn validate_cron(expression: &str) -> Result<(), String> {
    CronSchedule::from_str(expression)
        .map(|_| ())
        .map_err(|e| e.to_string())
}
pub fn validate_timezone(timezone: &str) -> Result<(), String> {
    timezone
        .parse::<Tz>()
        .map(|_| ())
        .map_err(|_| format!("Invalid timezone: {}", timezone))
}
pub fn get_next_cron_runs(expression: &str, timezone: &str, count: usize) -> Result<Vec<DateTime<Utc>>, String> {
    let schedule = CronSchedule::from_str(expression).map_err(|e| e.to_string())?;

    let tz: Tz = timezone
        .parse()
        .map_err(|_| format!("Invalid timezone: {}", timezone))?;

    Ok(schedule
        .upcoming(tz)
        .take(count)
        .map(|dt| dt.with_timezone(&Utc))
        .collect())
}
