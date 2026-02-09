/// Generate a lock resource ID for a task
pub fn task_lock_resource(task_id: &str) -> String {
    format!("task:{}", task_id)
}

/// Generate a lock resource ID for a task queue
pub fn queue_lock_resource(queue_name: &str) -> String {
    format!("task_queue:{}", queue_name)
}
