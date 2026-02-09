#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub worker_id: String,
    pub queues: Vec<String>,
    pub poll_interval_ms: u64,
    pub batch_size: usize,
    pub max_concurrent_tasks: usize,
    pub lock_duration_secs: u64,
    pub stale_check_interval_secs: u64,
    pub stale_threshold_secs: u64,
    pub cleanup_interval_secs: u64,
    pub history_retention_days: u32,
    pub auto_retry: bool,
    pub process_scheduled: bool,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            worker_id: uuid::Uuid::new_v4().to_string(),
            queues: vec![],
            poll_interval_ms: 1000,
            batch_size: 10,
            max_concurrent_tasks: 10,
            lock_duration_secs: 300,
            stale_check_interval_secs: 60,
            stale_threshold_secs: 600,
            cleanup_interval_secs: 3600,
            history_retention_days: 30,
            auto_retry: true,
            process_scheduled: true,
        }
    }
}

impl WorkerConfig {
    pub fn new(worker_id: impl Into<String>) -> Self {
        Self {
            worker_id: worker_id.into(),
            ..Default::default()
        }
    }

    pub fn with_queues(mut self, queues: Vec<String>) -> Self {
        self.queues = queues;
        self
    }

    pub fn with_poll_interval(mut self, ms: u64) -> Self {
        self.poll_interval_ms = ms;
        self
    }

    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    pub fn with_max_concurrent_tasks(mut self, max: usize) -> Self {
        self.max_concurrent_tasks = max;
        self
    }

    pub fn with_lock_duration(mut self, secs: u64) -> Self {
        self.lock_duration_secs = secs;
        self
    }
}
