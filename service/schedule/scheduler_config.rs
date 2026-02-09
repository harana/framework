#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    pub worker_id: String,
    pub poll_interval_secs: u64,
    pub batch_size: usize,
    pub lock_duration_secs: u64,
    pub stale_check_interval_secs: u64,
    pub stale_threshold_secs: u64,
    pub cleanup_interval_secs: u64,
    pub history_retention_days: u32,
    pub max_concurrent_jobs: usize,
    pub auto_create_jobs: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            worker_id: uuid::Uuid::new_v4().to_string(),
            poll_interval_secs: 10,
            batch_size: 100,
            lock_duration_secs: 300,
            stale_check_interval_secs: 60,
            stale_threshold_secs: 600,
            cleanup_interval_secs: 3600,
            history_retention_days: 30,
            max_concurrent_jobs: 10,
            auto_create_jobs: true,
        }
    }
}

impl SchedulerConfig {
    pub fn new(worker_id: impl Into<String>) -> Self {
        Self {
            worker_id: worker_id.into(),
            ..Default::default()
        }
    }

    pub fn with_poll_interval(mut self, secs: u64) -> Self {
        self.poll_interval_secs = secs;
        self
    }

    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    pub fn with_max_concurrent_jobs(mut self, max: usize) -> Self {
        self.max_concurrent_jobs = max;
        self
    }
}
