#[derive(Debug, Clone)]
pub enum SchedulerEvent {
    Started {
        worker_id: String,
    },
    Stopped {
        worker_id: String,
    },
    ScheduleCreated {
        schedule_id: String,
    },
    ScheduleUpdated {
        schedule_id: String,
    },
    ScheduleDeleted {
        schedule_id: String,
    },
    JobCreated {
        job_id: String,
        schedule_id: String,
    },
    JobStarted {
        job_id: String,
        schedule_id: String,
    },
    JobCompleted {
        job_id: String,
        schedule_id: String,
        duration_ms: i64,
    },
    JobFailed {
        job_id: String,
        schedule_id: String,
        error: String,
    },
    JobRetrying {
        job_id: String,
        schedule_id: String,
        attempt: u32,
    },
    StaleJobRecovered {
        job_id: String,
    },
}
