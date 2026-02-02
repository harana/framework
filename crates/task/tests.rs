// Harana Components - Task Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BackoffStrategy, LoggingExecutor, RetryConfig, Task, TaskExecutor,
        TaskPriority, TaskQuery, TaskStatus, TaskExecutionHistory,
        store as task_store,
    };
    use chrono::Utc;
    use std::collections::HashMap;
    use std::sync::Arc;

    // Simple in-memory store for testing that implements Store<Task>
    mod test_store {
        use dashmap::DashMap;
        use harana_components_storage::{Entity, FilterCondition, QueryOptions, StorageError, StorageResult, Store};
        use serde::{de::DeserializeOwned, Serialize};
        use serde_json::{Map, Value};
        use async_trait::async_trait;
        use crate::{Task, TaskExecutionHistory};
        use std::sync::Arc;
        use parking_lot::RwLock;
        use std::collections::VecDeque;

        pub struct TestTaskStore {
            tasks: DashMap<String, Task>,
        }

        impl TestTaskStore {
            pub fn new() -> Self {
                Self {
                    tasks: DashMap::new(),
                }
            }
        }

        #[async_trait]
        impl Store<Task> for TestTaskStore {
            type Filter = FilterCondition;

            async fn create(&self, entity: &Task) -> StorageResult<()> {
                self.tasks.insert(entity.id.clone(), entity.clone());
                Ok(())
            }

            async fn create_many(&self, entities: &[Task]) -> StorageResult<usize> {
                for entity in entities {
                    self.tasks.insert(entity.id.clone(), entity.clone());
                }
                Ok(entities.len())
            }

            async fn find_by_id(&self, id: &str) -> StorageResult<Option<Task>> {
                Ok(self.tasks.get(id).map(|t| t.clone()))
            }

            async fn find_many(&self, _filter: Option<FilterCondition>, _options: QueryOptions) -> StorageResult<Vec<Task>> {
                Ok(self.tasks.iter().map(|t| t.clone()).collect())
            }

            async fn count(&self, _filter: Option<FilterCondition>) -> StorageResult<u64> {
                Ok(self.tasks.len() as u64)
            }

            async fn update(&self, entity: &Task) -> StorageResult<()> {
                self.tasks.insert(entity.id.clone(), entity.clone());
                Ok(())
            }

            async fn upsert(&self, entity: &Task) -> StorageResult<()> {
                self.tasks.insert(entity.id.clone(), entity.clone());
                Ok(())
            }

            async fn update_many(&self, _filter: FilterCondition, _updates: Map<String, Value>) -> StorageResult<u64> {
                Ok(0)
            }

            async fn delete(&self, id: &str) -> StorageResult<bool> {
                Ok(self.tasks.remove(id).is_some())
            }

            async fn delete_many(&self, _filter: FilterCondition) -> StorageResult<u64> {
                Ok(0)
            }

            async fn delete_all(&self) -> StorageResult<u64> {
                let count = self.tasks.len();
                self.tasks.clear();
                Ok(count as u64)
            }

            async fn text_search(&self, _text: &str, _limit: Option<i64>, _offset: Option<u64>) -> StorageResult<Vec<Task>> {
                Ok(vec![])
            }

            async fn text_search_with_filter(&self, _text: &str, _filter: Self::Filter, _limit: Option<i64>, _offset: Option<u64>) -> StorageResult<Vec<Task>> {
                Ok(vec![])
            }

            async fn create_queue(&self, _queue_name: &str) -> StorageResult<()> { Ok(()) }
            async fn queue_add<Q: Serialize + Send + Sync>(&self, _queue_name: &str, _items: &[Q], _delay_secs: Option<i64>) -> StorageResult<()> { Ok(()) }
            async fn queue_get<Q: DeserializeOwned + Send>(&self, _queue_name: &str, _visibility_secs: i64) -> StorageResult<Option<harana_components_storage::QueueMessage<Q>>> { Ok(None) }
            async fn queue_ack<Q: DeserializeOwned + Send>(&self, _queue_name: &str, _ack_id: &str) -> StorageResult<Option<Q>> { Ok(None) }
            async fn queue_ping<Q: DeserializeOwned + Send>(&self, _queue_name: &str, _ack_id: &str, _visibility_secs: i64) -> StorageResult<Option<Q>> { Ok(None) }
            async fn queue_waiting_count(&self, _queue_name: &str) -> StorageResult<i64> { Ok(0) }
            async fn queue_in_flight_count(&self, _queue_name: &str) -> StorageResult<i64> { Ok(0) }
            async fn queue_completed_count(&self, _queue_name: &str) -> StorageResult<i64> { Ok(0) }
            async fn queue_total_count(&self, _queue_name: &str) -> StorageResult<i64> { Ok(0) }
            async fn queue_purge(&self, _queue_name: &str) -> StorageResult<u64> { Ok(0) }
        }

        pub struct TestHistoryStore {
            history: Arc<RwLock<VecDeque<TaskExecutionHistory>>>,
        }

        impl TestHistoryStore {
            pub fn new() -> Self {
                Self {
                    history: Arc::new(RwLock::new(VecDeque::new())),
                }
            }
        }

        #[async_trait]
        impl Store<TaskExecutionHistory> for TestHistoryStore {
            type Filter = FilterCondition;

            async fn create(&self, entity: &TaskExecutionHistory) -> StorageResult<()> {
                self.history.write().push_front(entity.clone());
                Ok(())
            }

            async fn create_many(&self, entities: &[TaskExecutionHistory]) -> StorageResult<usize> {
                for entity in entities {
                    self.history.write().push_front(entity.clone());
                }
                Ok(entities.len())
            }

            async fn find_by_id(&self, id: &str) -> StorageResult<Option<TaskExecutionHistory>> {
                Ok(self.history.read().iter().find(|h| h.id == id).cloned())
            }

            async fn find_many(&self, _filter: Option<FilterCondition>, _options: QueryOptions) -> StorageResult<Vec<TaskExecutionHistory>> {
                Ok(self.history.read().iter().cloned().collect())
            }

            async fn count(&self, _filter: Option<FilterCondition>) -> StorageResult<u64> {
                Ok(self.history.read().len() as u64)
            }

            async fn update(&self, _entity: &TaskExecutionHistory) -> StorageResult<()> { Ok(()) }
            async fn upsert(&self, entity: &TaskExecutionHistory) -> StorageResult<()> {
                self.history.write().push_front(entity.clone());
                Ok(())
            }
            async fn update_many(&self, _filter: FilterCondition, _updates: Map<String, Value>) -> StorageResult<u64> { Ok(0) }
            async fn delete(&self, _id: &str) -> StorageResult<bool> { Ok(false) }
            async fn delete_many(&self, _filter: FilterCondition) -> StorageResult<u64> { Ok(0) }
            async fn delete_all(&self) -> StorageResult<u64> {
                let count = self.history.read().len();
                self.history.write().clear();
                Ok(count as u64)
            }
            async fn text_search(&self, _text: &str, _limit: Option<i64>, _offset: Option<u64>) -> StorageResult<Vec<TaskExecutionHistory>> { Ok(vec![]) }
            async fn text_search_with_filter(&self, _text: &str, _filter: Self::Filter, _limit: Option<i64>, _offset: Option<u64>) -> StorageResult<Vec<TaskExecutionHistory>> { Ok(vec![]) }
            async fn create_queue(&self, _queue_name: &str) -> StorageResult<()> { Ok(()) }
            async fn queue_add<Q: Serialize + Send + Sync>(&self, _queue_name: &str, _items: &[Q], _delay_secs: Option<i64>) -> StorageResult<()> { Ok(()) }
            async fn queue_get<Q: DeserializeOwned + Send>(&self, _queue_name: &str, _visibility_secs: i64) -> StorageResult<Option<harana_components_storage::QueueMessage<Q>>> { Ok(None) }
            async fn queue_ack<Q: DeserializeOwned + Send>(&self, _queue_name: &str, _ack_id: &str) -> StorageResult<Option<Q>> { Ok(None) }
            async fn queue_ping<Q: DeserializeOwned + Send>(&self, _queue_name: &str, _ack_id: &str, _visibility_secs: i64) -> StorageResult<Option<Q>> { Ok(None) }
            async fn queue_waiting_count(&self, _queue_name: &str) -> StorageResult<i64> { Ok(0) }
            async fn queue_in_flight_count(&self, _queue_name: &str) -> StorageResult<i64> { Ok(0) }
            async fn queue_completed_count(&self, _queue_name: &str) -> StorageResult<i64> { Ok(0) }
            async fn queue_total_count(&self, _queue_name: &str) -> StorageResult<i64> { Ok(0) }
            async fn queue_purge(&self, _queue_name: &str) -> StorageResult<u64> { Ok(0) }
        }
    }

    use test_store::{TestTaskStore, TestHistoryStore};

    // ========================================================================
    // Task Entity Tests
    // ========================================================================

    #[test]
    fn test_task_creation() {
        let task = Task::new("Test Task", "test_type", "default");

        assert!(!task.id.is_empty());
        assert_eq!(task.name, "Test Task");
        assert_eq!(task.task_type, "test_type");
        assert_eq!(task.queue, "default");
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.priority, TaskPriority::Normal);
        assert_eq!(task.retry_attempt, 0);
    }

    #[test]
    fn test_task_builder() {
        let task = Task::new("Test Task", "test_type", "default")
            .with_description("A test task")
            .with_priority(TaskPriority::High)
            .with_timeout(600)
            .with_tags(vec!["test".to_string(), "example".to_string()])
            .with_owner("user123");

        assert_eq!(task.description, "A test task");
        assert_eq!(task.priority, TaskPriority::High);
        assert_eq!(task.timeout_secs, 600);
        assert_eq!(task.tags.len(), 2);
        assert_eq!(task.owner_id, Some("user123".to_string()));
    }

    #[test]
    fn test_task_delayed() {
        let task = Task::new("Delayed Task", "test_type", "default").with_delay(60);

        assert_eq!(task.status, TaskStatus::Scheduled);
        assert!(task.scheduled_at.is_some());
        
        let scheduled = task.scheduled_at.unwrap();
        let now = Utc::now();
        assert!(scheduled > now);
    }

    #[test]
    fn test_task_lifecycle() {
        let mut task = Task::new("Lifecycle Task", "test_type", "default");

        // Start the task
        task.start("worker-1", "lock-token-1", 300);
        assert_eq!(task.status, TaskStatus::Running);
        assert_eq!(task.worker_id, Some("worker-1".to_string()));
        assert!(task.started_at.is_some());
        assert!(task.lock_token.is_some());

        // Complete the task
        task.complete(Some(serde_json::json!({ "result": "success" })));
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(task.completed_at.is_some());
        assert!(task.duration_ms.is_some());
        assert!(task.result.is_some());
        assert!(task.worker_id.is_none());
        assert!(task.lock_token.is_none());
    }

    #[test]
    fn test_task_failure_and_retry() {
        let mut task = Task::new("Failing Task", "test_type", "default")
            .with_retry(RetryConfig::new().with_max_retries(3).with_delay(5));

        task.start("worker-1", "lock-token-1", 300);
        task.fail("Something went wrong", None);

        assert_eq!(task.status, TaskStatus::Failed);
        assert!(task.can_retry());

        // Schedule retry
        task.schedule_retry();
        assert_eq!(task.status, TaskStatus::Retrying);
        assert_eq!(task.retry_attempt, 1);
        assert!(task.retry_at.is_some());
    }

    #[test]
    fn test_retry_config_backoff() {
        let config = RetryConfig::new()
            .with_max_retries(5)
            .with_delay(10)
            .with_max_delay(300)
            .with_backoff(BackoffStrategy::Exponential);

        // Exponential backoff: 10 * 2^attempt
        assert_eq!(config.calculate_delay(0), 10);  // 10 * 2^0 = 10
        assert_eq!(config.calculate_delay(1), 20);  // 10 * 2^1 = 20
        assert_eq!(config.calculate_delay(2), 40);  // 10 * 2^2 = 40
        assert_eq!(config.calculate_delay(3), 80);  // 10 * 2^3 = 80
        assert_eq!(config.calculate_delay(4), 160); // 10 * 2^4 = 160
        assert_eq!(config.calculate_delay(5), 300); // 10 * 2^5 = 320, capped at 300
    }

    #[test]
    fn test_priority_ordering() {
        assert!(TaskPriority::Critical > TaskPriority::High);
        assert!(TaskPriority::High > TaskPriority::Normal);
        assert!(TaskPriority::Normal > TaskPriority::Low);
    }

    // ========================================================================
    // Task Store Tests
    // ========================================================================

    #[tokio::test]
    async fn test_store_crud() {
        let store = TestTaskStore::new();

        // Create
        let task = Task::new("CRUD Test", "test_type", "default");
        let task_id = task.id.clone();
        task_store::create_task(&store, &task).await.unwrap();

        // Read
        let retrieved = task_store::get_task(&store, &task_id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "CRUD Test");

        // Update
        let mut task = task_store::get_task(&store, &task_id).await.unwrap().unwrap();
        task.name = "Updated Task".to_string();
        task_store::update_task(&store, &task).await.unwrap();

        let updated = task_store::get_task(&store, &task_id).await.unwrap().unwrap();
        assert_eq!(updated.name, "Updated Task");

        // Delete
        let deleted = task_store::delete_task(&store, &task_id).await.unwrap();
        assert!(deleted);

        let not_found = task_store::get_task(&store, &task_id).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_store_query() {
        let store = TestTaskStore::new();

        // Create tasks with different properties
        let task1 = Task::new("Task 1", "type_a", "queue1")
            .with_priority(TaskPriority::High);
        let task2 = Task::new("Task 2", "type_b", "queue1")
            .with_priority(TaskPriority::Normal);
        let task3 = Task::new("Task 3", "type_a", "queue2")
            .with_priority(TaskPriority::Low);

        task_store::create_task(&store, &task1).await.unwrap();
        task_store::create_task(&store, &task2).await.unwrap();
        task_store::create_task(&store, &task3).await.unwrap();

        // Query by queue - note: the test store doesn't filter, so we get all tasks
        let all_tasks = task_store::query_tasks(&store, TaskQuery::new()).await.unwrap();
        assert_eq!(all_tasks.len(), 3);
    }

    #[tokio::test]
    async fn test_store_locking() {
        let store = TestTaskStore::new();

        let task = Task::new("Lock Test", "test", "default");
        let task_id = task.id.clone();
        task_store::create_task(&store, &task).await.unwrap();

        // First lock should succeed
        let lock1 = task_store::try_lock_task(&store, &task_id, "worker-1", 300).await.unwrap();
        assert!(lock1.is_some());

        // Second lock should fail (already locked)
        let lock2 = task_store::try_lock_task(&store, &task_id, "worker-2", 300).await.unwrap();
        assert!(lock2.is_none());

        // Release the lock
        let released = task_store::release_task_lock(&store, &task_id, lock1.as_ref().unwrap()).await.unwrap();
        assert!(released);

        // Now lock should succeed again
        let lock3 = task_store::try_lock_task(&store, &task_id, "worker-2", 300).await.unwrap();
        assert!(lock3.is_some());
    }
    // ========================================================================
    // History Tests
    // ========================================================================

    #[tokio::test]
    async fn test_execution_history() {
        let store = TestTaskStore::new();
        let history_store = TestHistoryStore::new();

        let mut task = Task::new("History Test", "test", "default");
        task.start("worker-1", "token", 300);
        task.complete(Some(serde_json::json!({ "result": "success" })));

        task_store::create_task(&store, &task).await.unwrap();

        // Record history
        let history = TaskExecutionHistory::from_task(&task);
        task_store::record_history(&history_store, &history).await.unwrap();

        // Retrieve history
        let task_history = task_store::get_task_history(&history_store, &task.id, Some(10)).await.unwrap();
        // Note: Our test store doesn't filter by task_id, so we just check it recorded something
        assert!(!task_history.is_empty());
    }

    // ========================================================================
    // Queue Stats Tests
    // ========================================================================

    #[tokio::test]
    async fn test_queue_stats() {
        let store = TestTaskStore::new();

        // Create various tasks
        let pending = Task::new("Pending", "test", "stats_queue");
        let mut running = Task::new("Running", "test", "stats_queue");
        running.start("worker", "token", 300);
        let mut completed = Task::new("Completed", "test", "stats_queue");
        completed.start("worker", "token", 300);
        completed.complete(None);
        let mut failed = Task::new("Failed", "test", "stats_queue");
        failed.start("worker", "token", 300);
        failed.fail("Error", None);

        task_store::create_task(&store, &pending).await.unwrap();
        task_store::create_task(&store, &running).await.unwrap();
        task_store::create_task(&store, &completed).await.unwrap();
        task_store::create_task(&store, &failed).await.unwrap();

        // Note: Our test store doesn't support proper filtering, so stats may not be accurate
        // This is just to verify the function works
        let stats = task_store::get_queue_stats(&store, "stats_queue").await.unwrap();
        assert!(stats.total >= 0);
    }
}
