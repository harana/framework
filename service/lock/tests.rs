use super::*;
use async_trait::async_trait;
use harana_components_storage::{FilterCondition, QueueMessage, QueryOptions, StorageError, StorageResult, Store};
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::RwLock;

// Mock store for testing
struct MockLockStore {
    locks: RwLock<HashMap<String, DistributedLock>>,
}

impl MockLockStore {
    fn new() -> Self {
        Self {
            locks: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Store<DistributedLock> for MockLockStore {
    type Filter = FilterCondition;

    async fn create(&self, entity: &DistributedLock) -> StorageResult<()> {
        let mut locks = self.locks.write().await;
        if locks.contains_key(&entity.id) {
            return Err(StorageError::DuplicateKey {
                entity_type: "distributed_lock".to_string(),
                id: entity.id.clone(),
            });
        }
        locks.insert(entity.id.clone(), entity.clone());
        Ok(())
    }

    async fn create_many(&self, _entities: &[DistributedLock]) -> StorageResult<usize> {
        unimplemented!()
    }

    async fn find_by_id(&self, id: &str) -> StorageResult<Option<DistributedLock>> {
        let locks = self.locks.read().await;
        Ok(locks.get(id).cloned())
    }

    async fn find_many(
        &self,
        filter: Option<FilterCondition>,
        _options: QueryOptions,
    ) -> StorageResult<Vec<DistributedLock>> {
        let locks = self.locks.read().await;
        let all: Vec<DistributedLock> = locks.values().cloned().collect();
        
        // Simple filter implementation for testing
        if let Some(FilterCondition::Eq(field, value)) = filter {
            if field == "owner_id" {
                let owner = value.as_str().unwrap_or("");
                return Ok(all.into_iter().filter(|l| l.owner_id == owner).collect());
            }
        }
        
        Ok(all)
    }

    async fn count(&self, _filter: Option<FilterCondition>) -> StorageResult<u64> {
        let locks = self.locks.read().await;
        Ok(locks.len() as u64)
    }

    async fn update(&self, entity: &DistributedLock) -> StorageResult<()> {
        let mut locks = self.locks.write().await;
        locks.insert(entity.id.clone(), entity.clone());
        Ok(())
    }

    async fn upsert(&self, entity: &DistributedLock) -> StorageResult<()> {
        self.update(entity).await
    }

    async fn update_many(
        &self,
        _filter: FilterCondition,
        _updates: serde_json::Map<String, serde_json::Value>,
    ) -> StorageResult<u64> {
        unimplemented!()
    }

    async fn delete(&self, id: &str) -> StorageResult<bool> {
        let mut locks = self.locks.write().await;
        Ok(locks.remove(id).is_some())
    }

    async fn delete_many(&self, filter: FilterCondition) -> StorageResult<u64> {
        let mut locks = self.locks.write().await;
        
        // Simple implementation for lt(expires_at, ...) filter
        if let FilterCondition::Lt(field, _value) = filter {
            if field == "expires_at" {
                let before_count = locks.len();
                locks.retain(|_, v| !v.is_expired());
                return Ok((before_count - locks.len()) as u64);
            }
        }
        
        Ok(0)
    }

    async fn delete_all(&self) -> StorageResult<u64> {
        let mut locks = self.locks.write().await;
        let count = locks.len();
        locks.clear();
        Ok(count as u64)
    }

    async fn text_search(
        &self,
        _text: &str,
        _limit: Option<i64>,
        _offset: Option<u64>,
    ) -> StorageResult<Vec<DistributedLock>> {
        unimplemented!()
    }

    async fn text_search_with_filter(
        &self,
        _text: &str,
        _filter: Self::Filter,
        _limit: Option<i64>,
        _offset: Option<u64>,
    ) -> StorageResult<Vec<DistributedLock>> {
        unimplemented!()
    }

    async fn create_queue(&self, _queue_name: &str) -> StorageResult<()> {
        unimplemented!()
    }

    async fn queue_add<Q: Serialize + Send + Sync>(
        &self,
        _queue_name: &str,
        _items: &[Q],
        _delay_secs: Option<i64>,
    ) -> StorageResult<()> {
        unimplemented!()
    }

    async fn queue_get<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _visibility_secs: i64,
    ) -> StorageResult<Option<QueueMessage<Q>>> {
        unimplemented!()
    }

    async fn queue_ack<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _ack_id: &str,
    ) -> StorageResult<Option<Q>> {
        unimplemented!()
    }

    async fn queue_ping<Q: serde::de::DeserializeOwned + Send>(
        &self,
        _queue_name: &str,
        _ack_id: &str,
        _visibility_secs: i64,
    ) -> StorageResult<Option<Q>> {
        unimplemented!()
    }

    async fn queue_waiting_count(&self, _queue_name: &str) -> StorageResult<i64> {
        unimplemented!()
    }

    async fn queue_in_flight_count(&self, _queue_name: &str) -> StorageResult<i64> {
        unimplemented!()
    }

    async fn queue_completed_count(&self, _queue_name: &str) -> StorageResult<i64> {
        unimplemented!()
    }

    async fn queue_total_count(&self, _queue_name: &str) -> StorageResult<i64> {
        unimplemented!()
    }

    async fn queue_purge(&self, _queue_name: &str) -> StorageResult<u64> {
        unimplemented!()
    }
}

#[tokio::test]
async fn test_acquire_and_release() {
    let store = MockLockStore::new();
    let manager = DistributedLockManager::new(store, LockConfig::default());

    // Acquire lock
    let token = manager.try_acquire("resource1", "worker1", Some(30)).await.unwrap();
    assert!(token.is_some());

    // Verify lock is held
    assert!(manager.is_locked("resource1").await.unwrap());

    // Release lock
    let released = manager.release_lock("resource1", "worker1").await.unwrap();
    assert!(released);

    // Verify lock is released
    assert!(!manager.is_locked("resource1").await.unwrap());
}

#[tokio::test]
async fn test_lock_contention() {
    let store = MockLockStore::new();
    let manager = DistributedLockManager::new(store, LockConfig::default());

    // Worker 1 acquires lock
    let token1 = manager.try_acquire("resource1", "worker1", Some(30)).await.unwrap();
    assert!(token1.is_some());

    // Worker 2 cannot acquire same lock
    let token2 = manager.try_acquire("resource1", "worker2", Some(30)).await.unwrap();
    assert!(token2.is_none());

    // Worker 1 releases
    manager.release_lock("resource1", "worker1").await.unwrap();

    // Now worker 2 can acquire
    let token2 = manager.try_acquire("resource1", "worker2", Some(30)).await.unwrap();
    assert!(token2.is_some());
}

#[tokio::test]
async fn test_lock_ordering_prevents_deadlock() {
    let store = MockLockStore::new();
    let config = LockConfig::default().with_lock_ordering(true);
    let manager = DistributedLockManager::new(store, config);

    // Acquire lock on "b"
    manager.try_acquire("b", "worker1", Some(30)).await.unwrap();

    // Try to acquire lock on "a" (should fail due to ordering)
    let result = manager.try_acquire("a", "worker1", Some(30)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fencing_tokens_increase() {
    let store = MockLockStore::new();
    let manager = DistributedLockManager::new(store, LockConfig::default());

    let token1 = manager.try_acquire("r1", "w1", Some(30)).await.unwrap().unwrap();
    manager.release_lock("r1", "w1").await.unwrap();
    
    let token2 = manager.try_acquire("r1", "w1", Some(30)).await.unwrap().unwrap();
    
    assert!(token2 > token1);
}

#[tokio::test]
async fn test_extend_lock() {
    let store = MockLockStore::new();
    let manager = DistributedLockManager::new(store, LockConfig::default());

    manager.try_acquire("resource1", "worker1", Some(30)).await.unwrap();
    
    let extended = manager.extend_lock("resource1", "worker1", 60).await.unwrap();
    assert!(extended);
    
    // Cannot extend lock owned by someone else
    let extended = manager.extend_lock("resource1", "worker2", 60).await.unwrap();
    assert!(!extended);
}

#[tokio::test]
async fn test_helper_functions() {
    assert_eq!(job_lock_resource("abc"), "job:abc");
    assert_eq!(schedule_lock_resource("xyz"), "schedule:xyz");
    assert_eq!(worker_lock_resource("w1"), "worker:w1");
    assert_eq!(event_lock_resource("e1"), "event:e1");
    assert_eq!(channel_lock_resource("c1"), "channel:c1");
    assert_eq!(subscription_lock_resource("s1"), "subscription:s1");
}
