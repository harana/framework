#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::Arc;

    // ========================================================================
    // Schedule Entity Tests
    // ========================================================================

    #[test]
    fn test_create_cron_schedule() {
        let schedule = Schedule::cron("test-1", "My Schedule", "0 */5 * * * *")
            .with_description("Runs every 5 minutes")
            .with_timezone("America/New_York");

        assert_eq!(schedule.id, "test-1");
        assert_eq!(schedule.name, "My Schedule");
        assert_eq!(schedule.schedule_type, ScheduleType::Cron);
        assert_eq!(schedule.cron_expression, Some("0 */5 * * * *".to_string()));
        assert_eq!(schedule.timezone, "America/New_York");
        assert_eq!(schedule.status, ScheduleStatus::Active);
    }

    #[test]
    fn test_create_interval_schedule() {
        let schedule = Schedule::interval("test-2", "Interval Schedule", 3600)
            .with_max_executions(10);

        assert_eq!(schedule.schedule_type, ScheduleType::Interval);
        assert_eq!(schedule.interval_seconds, Some(3600));
        assert_eq!(schedule.max_executions, Some(10));
    }

    #[test]
    fn test_create_one_time_schedule() {
        let run_at = Utc::now() + Duration::hours(1);
        let schedule = Schedule::one_time("test-3", "One Time Schedule", run_at);

        assert_eq!(schedule.schedule_type, ScheduleType::OneTime);
        assert_eq!(schedule.run_at, Some(run_at));
        assert_eq!(schedule.next_run_at, Some(run_at));
    }

    #[test]
    fn test_schedule_is_runnable() {
        let mut schedule = Schedule::cron("test", "Test", "* * * * *");
        schedule.next_run_at = Some(Utc::now() + Duration::hours(1));
        
        // Active schedule should be runnable
        assert!(schedule.is_runnable());

        // Disabled schedule should not be runnable
        schedule.status = ScheduleStatus::Disabled;
        assert!(!schedule.is_runnable());

        // Paused schedule should not be runnable
        schedule.status = ScheduleStatus::Paused;
        assert!(!schedule.is_runnable());

        // Schedule past end date should not be runnable
        schedule.status = ScheduleStatus::Active;
        schedule.end_at = Some(Utc::now() - Duration::hours(1));
        assert!(!schedule.is_runnable());

        // Schedule that exceeded max executions should not be runnable
        schedule.end_at = None;
        schedule.max_executions = Some(5);
        schedule.execution_count = 5;
        assert!(!schedule.is_runnable());
    }

    #[test]
    fn test_schedule_is_due() {
        let mut schedule = Schedule::cron("test", "Test", "* * * * *");
        
        // Not due if next_run_at is in the future
        schedule.next_run_at = Some(Utc::now() + Duration::hours(1));
        assert!(!schedule.is_due());

        // Due if next_run_at is in the past
        schedule.next_run_at = Some(Utc::now() - Duration::seconds(1));
        assert!(schedule.is_due());

        // Not due if disabled
        schedule.status = ScheduleStatus::Disabled;
        assert!(!schedule.is_due());
    }

    // ========================================================================
    // Job Entity Tests
    // ========================================================================

    #[test]
    fn test_create_job_from_schedule() {
        let schedule = Schedule::cron("sched-1", "Test Schedule", "* * * * *")
            .with_action("http_request".to_string(), {
                let mut config = HashMap::new();
                config.insert("url".to_string(), json!("https://api.example.com"));
                config
            });

        let scheduled_at = Utc::now();
        let job = Job::new(&schedule, scheduled_at);

        assert_eq!(job.schedule_id, "sched-1");
        assert_eq!(job.schedule_name, "Test Schedule");
        assert_eq!(job.status, JobStatus::Pending);
        assert_eq!(job.scheduled_at, scheduled_at);
        assert_eq!(job.action_type, "http_request");
        assert!(job.action_config.contains_key("url"));
    }

    #[test]
    fn test_job_lifecycle() {
        let schedule = Schedule::cron("sched-1", "Test", "* * * * *");
        let mut job = Job::new(&schedule, Utc::now());

        // Start job
        job.start(Some("worker-1".to_string()));
        assert_eq!(job.status, JobStatus::Running);
        assert!(job.started_at.is_some());
        assert_eq!(job.worker_id, Some("worker-1".to_string()));

        // Complete job
        job.complete(Some(json!({ "success": true })));
        assert_eq!(job.status, JobStatus::Completed);
        assert!(job.completed_at.is_some());
        assert!(job.duration_ms.is_some());
        assert!(job.result.is_some());
    }

    #[test]
    fn test_job_failure_and_retry() {
        let mut schedule = Schedule::cron("sched-1", "Test", "* * * * *");
        schedule.retry_config.max_retries = 3;

        let mut job = Job::new(&schedule, Utc::now());
        job.max_retries = 3;

        // First attempt fails
        job.start(None);
        job.fail("Connection timeout".to_string(), None);
        assert_eq!(job.status, JobStatus::Failed);

        // Can retry after first failure
        assert!(job.can_retry());
        
        let retry_at = Utc::now() + Duration::seconds(10);
        job.retry(retry_at);
        assert_eq!(job.status, JobStatus::Retrying);
        assert_eq!(job.retry_attempt, 1);
        assert_eq!(job.retry_at, Some(retry_at));

        // Retry until max
        job.retry(Utc::now() + Duration::seconds(20));
        assert_eq!(job.retry_attempt, 2);
        
        job.retry(Utc::now() + Duration::seconds(30));
        assert_eq!(job.retry_attempt, 3);

        // Can't retry anymore
        assert!(!job.can_retry());
    }

    #[test]
    fn test_job_cancellation() {
        let schedule = Schedule::cron("sched-1", "Test", "* * * * *");
        let mut job = Job::new(&schedule, Utc::now());

        job.cancel();
        assert_eq!(job.status, JobStatus::Cancelled);
        assert!(job.completed_at.is_some());
        assert!(job.status.is_terminal());
    }

    // ========================================================================
    // Retry Configuration Tests
    // ========================================================================

    #[test]
    fn test_retry_delay_fixed() {
        let config = RetryConfig {
            max_retries: 3,
            initial_delay_secs: 10,
            max_delay_secs: 100,
            multiplier: 2.0,
            strategy: BackoffStrategy::Fixed,
            jitter: false,
        };

        assert_eq!(config.delay_for_attempt(0).as_secs(), 10);
        assert_eq!(config.delay_for_attempt(1).as_secs(), 10);
        assert_eq!(config.delay_for_attempt(2).as_secs(), 10);
    }

    #[test]
    fn test_retry_delay_exponential() {
        let config = RetryConfig {
            max_retries: 5,
            initial_delay_secs: 10,
            max_delay_secs: 1000,
            multiplier: 2.0,
            strategy: BackoffStrategy::Exponential,
            jitter: false,
        };

        assert_eq!(config.delay_for_attempt(0).as_secs(), 10);
        assert_eq!(config.delay_for_attempt(1).as_secs(), 20);
        assert_eq!(config.delay_for_attempt(2).as_secs(), 40);
        assert_eq!(config.delay_for_attempt(3).as_secs(), 80);
    }

    #[test]
    fn test_retry_delay_max_cap() {
        let config = RetryConfig {
            max_retries: 10,
            initial_delay_secs: 10,
            max_delay_secs: 60,
            multiplier: 2.0,
            strategy: BackoffStrategy::Exponential,
            jitter: false,
        };

        // Should be capped at max_delay_secs
        assert_eq!(config.delay_for_attempt(5).as_secs(), 60);
        assert_eq!(config.delay_for_attempt(10).as_secs(), 60);
    }

    // ========================================================================
    // In-Memory Service Tests
    // ========================================================================

    #[tokio::test]
    async fn test_inmemory_store_schedule_crud() {
        let store = InMemoryScheduleService::new();

        // Create
        let schedule = Schedule::cron("test-1", "Test Schedule", "0 * * * *");
        store.create_schedule(&schedule).await.unwrap();

        // Read
        let retrieved = store.get_schedule("test-1").await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Schedule");

        // Update
        let mut updated = schedule.clone();
        updated.name = "Updated Name".to_string();
        store.update_schedule(&updated).await.unwrap();

        let retrieved = store.get_schedule("test-1").await.unwrap().unwrap();
        assert_eq!(retrieved.name, "Updated Name");

        // Delete
        let deleted = store.delete_schedule("test-1").await.unwrap();
        assert!(deleted);

        let retrieved = store.get_schedule("test-1").await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_inmemory_store_query_schedules() {
        let store = InMemoryScheduleService::new();

        // Create multiple schedules
        for i in 0..10 {
            let mut schedule = Schedule::cron(
                format!("test-{}", i),
                format!("Schedule {}", i),
                "0 * * * *",
            );
            if i % 2 == 0 {
                schedule.status = ScheduleStatus::Active;
            } else {
                schedule.status = ScheduleStatus::Disabled;
            }
            schedule.next_run_at = Some(Utc::now() + Duration::minutes(i as i64));
            store.create_schedule(&schedule).await.unwrap();
        }

        // Query all
        let all = store.query_schedules(ScheduleQuery::new()).await.unwrap();
        assert_eq!(all.len(), 10);

        // Query active only
        let active = store.query_schedules(ScheduleQuery::active()).await.unwrap();
        assert_eq!(active.len(), 5);

        // Query with limit
        let limited = store.query_schedules(
            ScheduleQuery::new().with_limit(3)
        ).await.unwrap();
        assert_eq!(limited.len(), 3);

        // Query with offset
        let offset = store.query_schedules(
            ScheduleQuery::new().with_offset(5).with_limit(10)
        ).await.unwrap();
        assert_eq!(offset.len(), 5);
    }

    #[tokio::test]
    async fn test_inmemory_store_job_crud() {
        let store = InMemoryScheduleService::new();

        // Create schedule first
        let schedule = Schedule::cron("sched-1", "Test", "* * * * *");
        store.create_schedule(&schedule).await.unwrap();

        // Create job
        let job = Job::new(&schedule, Utc::now());
        let job_id = job.id.clone();
        store.create_job(&job).await.unwrap();

        // Read
        let retrieved = store.get_job(&job_id).await.unwrap();
        assert!(retrieved.is_some());

        // Update
        let mut updated = job.clone();
        updated.status = JobStatus::Running;
        store.update_job(&updated).await.unwrap();

        let retrieved = store.get_job(&job_id).await.unwrap().unwrap();
        assert_eq!(retrieved.status, JobStatus::Running);

        // Delete
        let deleted = store.delete_job(&job_id).await.unwrap();
        assert!(deleted);
    }

    #[tokio::test]
    async fn test_inmemory_store_job_locking() {
        let store = InMemoryScheduleService::new();

        let schedule = Schedule::cron("sched-1", "Test", "* * * * *");
        store.create_schedule(&schedule).await.unwrap();

        let job = Job::new(&schedule, Utc::now());
        let job_id = job.id.clone();
        store.create_job(&job).await.unwrap();

        // First worker acquires lock
        let token1 = store.try_lock_job(&job_id, "worker-1", 60).await.unwrap();
        assert!(token1.is_some());
        let token1 = token1.unwrap();

        // Second worker can't acquire
        let token2 = store.try_lock_job(&job_id, "worker-2", 60).await.unwrap();
        assert!(token2.is_none());

        // First worker can extend
        let extended = store.extend_job_lock(&job_id, &token1, 120).await.unwrap();
        assert!(extended);

        // First worker releases
        let released = store.release_job_lock(&job_id, &token1).await.unwrap();
        assert!(released);

        // Now second worker can acquire
        let token3 = store.try_lock_job(&job_id, "worker-2", 60).await.unwrap();
        assert!(token3.is_some());
    }

    #[tokio::test]
    async fn test_inmemory_store_execution_history() {
        let store = InMemoryScheduleService::new();

        let schedule = Schedule::cron("sched-1", "Test", "* * * * *");
        store.create_schedule(&schedule).await.unwrap();

        // Record some history
        for i in 0..5 {
            let mut job = Job::new(&schedule, Utc::now() - Duration::hours(5 - i));
            job.complete(None);

            let history = ExecutionHistory::from_job(&job);
            store.record_execution(&history).await.unwrap();
        }

        // Get history
        let history = store.get_execution_history("sched-1", Some(10)).await.unwrap();
        assert_eq!(history.len(), 5);

        // Clean up old history
        let cutoff = Utc::now() - Duration::hours(3);
        let deleted = store.cleanup_history(cutoff).await.unwrap();
        assert!(deleted > 0);

        let remaining = store.get_execution_history("sched-1", None).await.unwrap();
        assert!(remaining.len() < 5);
    }

    // ========================================================================
    // Cron Validation Tests
    // ========================================================================

    #[test]
    fn test_validate_cron_valid() {
        assert!(validate_cron("0 * * * *").is_ok());
        assert!(validate_cron("0 0 * * *").is_ok());
        assert!(validate_cron("0 0 1 * *").is_ok());
        assert!(validate_cron("*/5 * * * *").is_ok());
        assert!(validate_cron("0 9-17 * * MON-FRI").is_ok());
    }

    #[test]
    fn test_validate_cron_invalid() {
        assert!(validate_cron("invalid").is_err());
        assert!(validate_cron("* * * *").is_err()); // Missing field
        assert!(validate_cron("60 * * * *").is_err()); // Invalid minute
    }

    #[test]
    fn test_validate_timezone() {
        assert!(validate_timezone("UTC").is_ok());
        assert!(validate_timezone("America/New_York").is_ok());
        assert!(validate_timezone("Europe/London").is_ok());
        assert!(validate_timezone("Invalid/Zone").is_err());
    }

    #[test]
    fn test_get_next_cron_runs() {
        let runs = get_next_cron_runs("0 * * * *", "UTC", 5).unwrap();
        assert_eq!(runs.len(), 5);

        // Verify runs are in ascending order
        for i in 1..runs.len() {
            assert!(runs[i] > runs[i - 1]);
        }
    }

    // ========================================================================
    // Scheduler Config Tests
    // ========================================================================

    #[test]
    fn test_scheduler_config_default() {
        let config = SchedulerConfig::default();
        
        assert!(!config.worker_id.is_empty());
        assert_eq!(config.poll_interval_secs, 10);
        assert_eq!(config.batch_size, 100);
        assert_eq!(config.max_concurrent_jobs, 10);
    }

    #[test]
    fn test_scheduler_config_builder() {
        let config = SchedulerConfig::new("my-worker")
            .with_poll_interval(5)
            .with_batch_size(50)
            .with_max_concurrent_jobs(20);

        assert_eq!(config.worker_id, "my-worker");
        assert_eq!(config.poll_interval_secs, 5);
        assert_eq!(config.batch_size, 50);
        assert_eq!(config.max_concurrent_jobs, 20);
    }
}
