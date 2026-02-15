use super::*;
use harana_components_cache::MemoryCacheService;

#[tokio::test]
async fn test_acquire_and_release() {
    let cache = MemoryCacheService::new();
    let manager = DistributedLockManager::new(cache, LockConfig::default());

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
    let cache = MemoryCacheService::new();
    let manager = DistributedLockManager::new(cache, LockConfig::default());

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
    let cache = MemoryCacheService::new();
    let config = LockConfig::default().with_lock_ordering(true);
    let manager = DistributedLockManager::new(cache, config);

    // Acquire lock on "b"
    manager.try_acquire("b", "worker1", Some(30)).await.unwrap();

    // Try to acquire lock on "a" (should fail due to ordering)
    let result = manager.try_acquire("a", "worker1", Some(30)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fencing_tokens_increase() {
    let cache = MemoryCacheService::new();
    let manager = DistributedLockManager::new(cache, LockConfig::default());

    let token1 = manager.try_acquire("r1", "w1", Some(30)).await.unwrap().unwrap();
    manager.release_lock("r1", "w1").await.unwrap();
    
    let token2 = manager.try_acquire("r1", "w1", Some(30)).await.unwrap().unwrap();
    
    assert!(token2 > token1);
}

#[tokio::test]
async fn test_extend_lock() {
    let cache = MemoryCacheService::new();
    let manager = DistributedLockManager::new(cache, LockConfig::default());

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
