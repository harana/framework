#[derive(Debug, Clone)]
pub enum WorkerEvent {
    Started {
        worker_id: String,
    },
    Stopped {
        worker_id: String,
    },
    TaskPickedUp {
        task_id: String,
        queue: String,
    },
    TaskStarted {
        task_id: String,
        queue: String,
    },
    TaskCompleted {
        task_id: String,
        queue: String,
        duration_ms: i64,
    },
    TaskFailed {
        task_id: String,
        queue: String,
        error: String,
    },
    TaskRetrying {
        task_id: String,
        queue: String,
        attempt: u32,
    },
    TaskTimedOut {
        task_id: String,
        queue: String,
    },
    StaleTaskRecovered {
        task_id: String,
    },
    LockExtended {
        task_id: String,
    },
}
