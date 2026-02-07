// Harana Components - Distributed Locking with Deadlock Prevention
//
// This module provides a distributed locking mechanism that uses the storage component
// for persistence. It includes deadlock prevention through:
// 1. Lock timeouts (TTL) - prevents indefinite holding
// 2. Lock ordering - prevents circular waits
// 3. Wait timeout - prevents indefinite waiting
// 4. Fencing tokens - prevents stale lock holders from making changes

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use harana_components_storage::{Entity, FilterCondition, QueryOptions, Store, StorageError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration as TokioDuration};

use crate::{LockError, LockResult};

// ============================================================================
// Lock Entity
// ============================================================================

/// A distributed lock record stored in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedLock {
        pub id: String,
        pub owner_id: String,
        pub fencing_token: u64,
        pub acquired_at: DateTime<Utc>,
        pub expires_at: DateTime<Utc>,
        pub version: u64,
        pub metadata: Option<String>,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
}

impl Entity for DistributedLock {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type() -> &'static str {
        "distributed_lock"
    }
}

impl DistributedLock {
    pub fn new(
        resource_id: impl Into<String>,
        owner_id: impl Into<String>,
        ttl_seconds: u64,
        fencing_token: u64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: resource_id.into(),
            owner_id: owner_id.into(),
            fencing_token,
            acquired_at: now,
            expires_at: now + Duration::seconds(ttl_seconds as i64),
            version: 1,
            metadata: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if the lock has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Check if this lock is held by the given owner
    pub fn is_held_by(&self, owner_id: &str) -> bool {
        self.owner_id == owner_id && !self.is_expired()
    }

    /// Extend the lock's TTL
    pub fn extend(&mut self, extension_seconds: u64) {
        let now = Utc::now();
        self.expires_at = now + Duration::seconds(extension_seconds as i64);
        self.updated_at = now;
        self.version += 1;
    }
}

// ============================================================================
// Lock Handle (RAII guard)
// ============================================================================

/// A handle to an acquired lock that automatically releases on drop
pub struct LockHandle<S>
where
    S: Store<DistributedLock> + 'static,
{
    resource_id: String,
    owner_id: String,
    fencing_token: u64,
    lock_manager: Arc<DistributedLockManager<S>>,
    released: bool,
}

impl<S> LockHandle<S>
where
    S: Store<DistributedLock>,
{
    fn new(
        resource_id: String,
        owner_id: String,
        fencing_token: u64,
        lock_manager: Arc<DistributedLockManager<S>>,
    ) -> Self {
        Self {
            resource_id,
            owner_id,
            fencing_token,
            lock_manager,
            released: false,
        }
    }

    /// Get the fencing token for this lock acquisition
    pub fn fencing_token(&self) -> u64 {
        self.fencing_token
    }

    /// Get the resource ID
    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }

    /// Get the owner ID
    pub fn owner_id(&self) -> &str {
        &self.owner_id
    }

    /// Extend the lock's TTL
    pub async fn extend(&self, extension_seconds: u64) -> LockResult<bool> {
        self.lock_manager
            .extend_lock(&self.resource_id, &self.owner_id, extension_seconds)
            .await
    }

    /// Explicitly release the lock
    pub async fn release(mut self) -> LockResult<bool> {
        self.released = true;
        self.lock_manager
            .release_lock(&self.resource_id, &self.owner_id)
            .await
    }
}

impl<S> Drop for LockHandle<S>
where
    S: Store<DistributedLock> + 'static,
{
    fn drop(&mut self) {
        if !self.released {
            // Best effort release - we can't do async in drop
            // The lock will expire via TTL if this fails
            let resource_id = self.resource_id.clone();
            let owner_id = self.owner_id.clone();
            let manager = Arc::clone(&self.lock_manager);
            
            tokio::spawn(async move {
                let _ = manager.release_lock(&resource_id, &owner_id).await;
            });
        }
    }
}

// ============================================================================
// Lock Configuration
// ============================================================================

/// Configuration for the distributed lock manager
#[derive(Debug, Clone)]
pub struct LockConfig {
        pub default_ttl_seconds: u64,
        pub wait_timeout_ms: u64,
        pub retry_interval_ms: u64,
        pub max_locks_per_owner: usize,
        pub enable_lock_ordering: bool,
        pub stale_threshold_seconds: u64,
}

impl Default for LockConfig {
    fn default() -> Self {
        Self {
            default_ttl_seconds: 30,
            wait_timeout_ms: 10000,
            retry_interval_ms: 100,
            max_locks_per_owner: 10,
            enable_lock_ordering: true,
            stale_threshold_seconds: 300,
        }
    }
}

impl LockConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_ttl(mut self, seconds: u64) -> Self {
        self.default_ttl_seconds = seconds;
        self
    }

    pub fn with_wait_timeout(mut self, ms: u64) -> Self {
        self.wait_timeout_ms = ms;
        self
    }

    pub fn with_retry_interval(mut self, ms: u64) -> Self {
        self.retry_interval_ms = ms;
        self
    }

    pub fn with_max_locks_per_owner(mut self, max: usize) -> Self {
        self.max_locks_per_owner = max;
        self
    }

    pub fn with_lock_ordering(mut self, enabled: bool) -> Self {
        self.enable_lock_ordering = enabled;
        self
    }
}

// ============================================================================
// Lock Manager Trait
// ============================================================================

/// Trait for distributed lock operations
#[async_trait]
pub trait LockManager: Send + Sync {
    /// Try to acquire a lock without waiting
    async fn try_acquire(
        &self,
        resource_id: &str,
        owner_id: &str,
        ttl_seconds: Option<u64>,
    ) -> LockResult<Option<u64>>; // Returns fencing token if acquired

    /// Acquire a lock, waiting up to the configured timeout
    async fn acquire(
        &self,
        resource_id: &str,
        owner_id: &str,
        ttl_seconds: Option<u64>,
    ) -> LockResult<u64>; // Returns fencing token

    /// Release a lock
    async fn release_lock(&self, resource_id: &str, owner_id: &str) -> LockResult<bool>;

    /// Extend a lock's TTL
    async fn extend_lock(
        &self,
        resource_id: &str,
        owner_id: &str,
        extension_seconds: u64,
    ) -> LockResult<bool>;

    /// Check if a resource is locked
    async fn is_locked(&self, resource_id: &str) -> LockResult<bool>;

    /// Get lock info for a resource
    async fn get_lock_info(&self, resource_id: &str) -> LockResult<Option<DistributedLock>>;

    /// Get all locks held by an owner
    async fn get_locks_by_owner(&self, owner_id: &str) -> LockResult<Vec<DistributedLock>>;

    /// Clean up expired locks
    async fn cleanup_expired_locks(&self) -> LockResult<u64>;

    /// Force release a lock (admin operation)
    async fn force_release(&self, resource_id: &str) -> LockResult<bool>;
}

// ============================================================================
// Distributed Lock Manager Implementation
// ============================================================================

/// Distributed lock manager using storage backend
pub struct DistributedLockManager<S>
where
    S: Store<DistributedLock>,
{
    store: S,
    config: LockConfig,
        fencing_counter: AtomicU64,
        owner_locks: RwLock<std::collections::HashMap<String, BTreeSet<String>>>,
}

impl<S> DistributedLockManager<S>
where
    S: Store<DistributedLock> + 'static,
{
    pub fn new(store: S, config: LockConfig) -> Self {
        Self {
            store,
            config,
            fencing_counter: AtomicU64::new(0),
            owner_locks: RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub fn with_default_config(store: S) -> Self {
        Self::new(store, LockConfig::default())
    }

    /// Get the next fencing token
    fn next_fencing_token(&self) -> u64 {
        self.fencing_counter.fetch_add(1, AtomicOrdering::SeqCst) + 1
    }

    /// Initialize fencing counter from database (call on startup)
    pub async fn initialize(&self) -> LockResult<()> {
        // Find the highest fencing token in the database
        let locks = self.store
            .find_many(None, QueryOptions::new().with_sort("fencing_token", true).with_limit(1))
            .await
            .map_err(|e| LockError::StorageError(e.to_string()))?;

        if let Some(lock) = locks.first() {
            self.fencing_counter.store(lock.fencing_token, AtomicOrdering::SeqCst);
        }

        Ok(())
    }

    /// Check lock ordering to prevent deadlocks (circular wait prevention)
    async fn check_lock_ordering(&self, resource_id: &str, owner_id: &str) -> LockResult<()> {
        if !self.config.enable_lock_ordering {
            return Ok(());
        }

        let owner_locks = self.owner_locks.read().await;
        if let Some(held_locks) = owner_locks.get(owner_id) {
            // Enforce that locks must be acquired in lexicographic order
            // This prevents circular waits (A holds X, wants Y; B holds Y, wants X)
            if let Some(highest_held) = held_locks.iter().next_back() {
                if resource_id < highest_held.as_str() {
                    return Err(LockError::OrderingViolation {
                        message: format!(
                            "Cannot acquire '{}' while holding '{}'. \
                             Locks must be acquired in ascending order to prevent deadlocks.",
                            resource_id, highest_held
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    /// Check if owner has reached max lock limit
    async fn check_max_locks(&self, owner_id: &str) -> LockResult<()> {
        let owner_locks = self.owner_locks.read().await;
        if let Some(held_locks) = owner_locks.get(owner_id) {
            if held_locks.len() >= self.config.max_locks_per_owner {
                return Err(LockError::MaxLocksExceeded {
                    max: self.config.max_locks_per_owner,
                });
            }
        }
        Ok(())
    }

    /// Track a newly acquired lock
    async fn track_lock(&self, resource_id: &str, owner_id: &str) {
        let mut owner_locks = self.owner_locks.write().await;
        owner_locks
            .entry(owner_id.to_string())
            .or_insert_with(BTreeSet::new)
            .insert(resource_id.to_string());
    }

    /// Untrack a released lock
    async fn untrack_lock(&self, resource_id: &str, owner_id: &str) {
        let mut owner_locks = self.owner_locks.write().await;
        if let Some(locks) = owner_locks.get_mut(owner_id) {
            locks.remove(resource_id);
            if locks.is_empty() {
                owner_locks.remove(owner_id);
            }
        }
    }

    /// Acquire a lock and return a handle for RAII-style management
    pub async fn acquire_handle(
        self: &Arc<Self>,
        resource_id: &str,
        owner_id: &str,
        ttl_seconds: Option<u64>,
    ) -> LockResult<LockHandle<S>> {
        let fencing_token = self.acquire(resource_id, owner_id, ttl_seconds).await?;
        Ok(LockHandle::new(
            resource_id.to_string(),
            owner_id.to_string(),
            fencing_token,
            Arc::clone(self),
        ))
    }

    /// Try to acquire multiple locks atomically (all-or-nothing)
    /// Resources are sorted to ensure consistent ordering and prevent deadlocks
    pub async fn acquire_multiple(
        &self,
        resource_ids: &[&str],
        owner_id: &str,
        ttl_seconds: Option<u64>,
    ) -> LockResult<Vec<u64>> {
        // Sort resource IDs to ensure consistent lock ordering
        let mut sorted_resources: Vec<&str> = resource_ids.to_vec();
        sorted_resources.sort();

        let mut acquired_tokens = Vec::new();
        let mut acquired_resources = Vec::new();

        for resource_id in sorted_resources {
            match self.try_acquire(resource_id, owner_id, ttl_seconds).await {
                Ok(Some(token)) => {
                    acquired_tokens.push(token);
                    acquired_resources.push(resource_id.to_string());
                }
                Ok(None) | Err(_) => {
                    // Rollback: release all acquired locks
                    for acquired in &acquired_resources {
                        let _ = self.release_lock(acquired, owner_id).await;
                    }
                    return Err(LockError::LockFailed {
                        resource: format!("Failed to acquire lock on '{}'", resource_id),
                    });
                }
            }
        }

        Ok(acquired_tokens)
    }

    /// Release multiple locks
    pub async fn release_multiple(&self, resource_ids: &[&str], owner_id: &str) -> LockResult<usize> {
        let mut released_count = 0;
        for resource_id in resource_ids {
            if self.release_lock(resource_id, owner_id).await? {
                released_count += 1;
            }
        }
        Ok(released_count)
    }
}

#[async_trait]
impl<S> LockManager for DistributedLockManager<S>
where
    S: Store<DistributedLock> + Send + Sync + 'static,
{
    async fn try_acquire(
        &self,
        resource_id: &str,
        owner_id: &str,
        ttl_seconds: Option<u64>,
    ) -> LockResult<Option<u64>> {
        // Check deadlock prevention constraints
        self.check_lock_ordering(resource_id, owner_id).await?;
        self.check_max_locks(owner_id).await?;

        let ttl = ttl_seconds.unwrap_or(self.config.default_ttl_seconds);
        let now = Utc::now();

        // Try to get existing lock
        let existing = self.store
            .find_by_id(resource_id)
            .await
            .map_err(|e| LockError::StorageError(e.to_string()))?;

        match existing {
            Some(lock) => {
                if lock.is_expired() {
                    // Lock expired, we can take it over
                    let fencing_token = self.next_fencing_token();
                    let new_lock = DistributedLock {
                        id: resource_id.to_string(),
                        owner_id: owner_id.to_string(),
                        fencing_token,
                        acquired_at: now,
                        expires_at: now + Duration::seconds(ttl as i64),
                        version: lock.version + 1,
                        metadata: None,
                        created_at: lock.created_at,
                        updated_at: now,
                    };

                    self.store
                        .update(&new_lock)
                        .await
                        .map_err(|e| LockError::StorageError(e.to_string()))?;

                    self.track_lock(resource_id, owner_id).await;
                    Ok(Some(fencing_token))
                } else if lock.owner_id == owner_id {
                    // Already held by this owner, extend it
                    let mut updated_lock = lock;
                    updated_lock.extend(ttl);

                    self.store
                        .update(&updated_lock)
                        .await
                        .map_err(|e| LockError::StorageError(e.to_string()))?;

                    Ok(Some(updated_lock.fencing_token))
                } else {
                    // Held by someone else
                    Ok(None)
                }
            }
            None => {
                // No existing lock, create new one
                let fencing_token = self.next_fencing_token();
                let new_lock = DistributedLock::new(resource_id, owner_id, ttl, fencing_token);

                match self.store.create(&new_lock).await {
                    Ok(()) => {
                        self.track_lock(resource_id, owner_id).await;
                        Ok(Some(fencing_token))
                    }
                    Err(StorageError::DuplicateKey { .. }) => {
                        // Race condition: someone else created the lock
                        Ok(None)
                    }
                    Err(e) => Err(LockError::StorageError(e.to_string())),
                }
            }
        }
    }

    async fn acquire(
        &self,
        resource_id: &str,
        owner_id: &str,
        ttl_seconds: Option<u64>,
    ) -> LockResult<u64> {
        let deadline = tokio::time::Instant::now()
            + TokioDuration::from_millis(self.config.wait_timeout_ms);

        loop {
            match self.try_acquire(resource_id, owner_id, ttl_seconds).await? {
                Some(token) => return Ok(token),
                None => {
                    if tokio::time::Instant::now() >= deadline {
                        return Err(LockError::LockFailed {
                            resource: format!(
                                "Timeout waiting for lock on '{}' after {}ms",
                                resource_id, self.config.wait_timeout_ms
                            ),
                        });
                    }
                    sleep(TokioDuration::from_millis(self.config.retry_interval_ms)).await;
                }
            }
        }
    }

    async fn release_lock(&self, resource_id: &str, owner_id: &str) -> LockResult<bool> {
        let existing = self.store
            .find_by_id(resource_id)
            .await
            .map_err(|e| LockError::StorageError(e.to_string()))?;

        match existing {
            Some(lock) if lock.owner_id == owner_id => {
                self.store
                    .delete(resource_id)
                    .await
                    .map_err(|e| LockError::StorageError(e.to_string()))?;

                self.untrack_lock(resource_id, owner_id).await;
                Ok(true)
            }
            Some(_) => {
                // Not owned by this owner
                Ok(false)
            }
            None => {
                // Lock doesn't exist
                self.untrack_lock(resource_id, owner_id).await;
                Ok(false)
            }
        }
    }

    async fn extend_lock(
        &self,
        resource_id: &str,
        owner_id: &str,
        extension_seconds: u64,
    ) -> LockResult<bool> {
        let existing = self.store
            .find_by_id(resource_id)
            .await
            .map_err(|e| LockError::StorageError(e.to_string()))?;

        match existing {
            Some(mut lock) if lock.owner_id == owner_id && !lock.is_expired() => {
                lock.extend(extension_seconds);

                self.store
                    .update(&lock)
                    .await
                    .map_err(|e| LockError::StorageError(e.to_string()))?;

                Ok(true)
            }
            _ => Ok(false),
        }
    }

    async fn is_locked(&self, resource_id: &str) -> LockResult<bool> {
        let existing = self.store
            .find_by_id(resource_id)
            .await
            .map_err(|e| LockError::StorageError(e.to_string()))?;

        Ok(existing.map(|l| !l.is_expired()).unwrap_or(false))
    }

    async fn get_lock_info(&self, resource_id: &str) -> LockResult<Option<DistributedLock>> {
        self.store
            .find_by_id(resource_id)
            .await
            .map_err(|e| LockError::StorageError(e.to_string()))
    }

    async fn get_locks_by_owner(&self, owner_id: &str) -> LockResult<Vec<DistributedLock>> {
        let filter = FilterCondition::eq("owner_id", owner_id);
        self.store
            .find_many(Some(filter), QueryOptions::new())
            .await
            .map_err(|e| LockError::StorageError(e.to_string()))
    }

    async fn cleanup_expired_locks(&self) -> LockResult<u64> {
        let now = Utc::now();
        let filter = FilterCondition::lt("expires_at", now.to_rfc3339());

        self.store
            .delete_many(filter)
            .await
            .map_err(|e| LockError::StorageError(e.to_string()))
    }

    async fn force_release(&self, resource_id: &str) -> LockResult<bool> {
        let existing = self.store
            .find_by_id(resource_id)
            .await
            .map_err(|e| LockError::StorageError(e.to_string()))?;

        if let Some(lock) = existing {
            self.store
                .delete(resource_id)
                .await
                .map_err(|e| LockError::StorageError(e.to_string()))?;

            self.untrack_lock(resource_id, &lock.owner_id).await;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// ============================================================================
// Multi-Lock Guard (for acquiring multiple locks safely)
// ============================================================================

/// A guard that holds multiple locks and releases them all on drop
pub struct MultiLockGuard<S>
where
    S: Store<DistributedLock> + 'static,
{
    resource_ids: Vec<String>,
    owner_id: String,
    fencing_tokens: Vec<u64>,
    lock_manager: Arc<DistributedLockManager<S>>,
    released: bool,
}

impl<S> MultiLockGuard<S>
where
    S: Store<DistributedLock> + 'static,
{
    /// Create a new multi-lock guard
    pub fn new(
        resource_ids: Vec<String>,
        owner_id: String,
        fencing_tokens: Vec<u64>,
        lock_manager: Arc<DistributedLockManager<S>>,
    ) -> Self {
        Self {
            resource_ids,
            owner_id,
            fencing_tokens,
            lock_manager,
            released: false,
        }
    }

    /// Get the fencing tokens
    pub fn fencing_tokens(&self) -> &[u64] {
        &self.fencing_tokens
    }

    /// Get the locked resource IDs
    pub fn resource_ids(&self) -> &[String] {
        &self.resource_ids
    }

    /// Explicitly release all locks
    pub async fn release(mut self) -> LockResult<usize> {
        self.released = true;
        let resource_refs: Vec<&str> = self.resource_ids.iter().map(|s| s.as_str()).collect();
        self.lock_manager
            .release_multiple(&resource_refs, &self.owner_id)
            .await
    }
}

impl<S> Drop for MultiLockGuard<S>
where
    S: Store<DistributedLock> + 'static,
{
    fn drop(&mut self) {
        if !self.released {
            let resource_ids = std::mem::take(&mut self.resource_ids);
            let owner_id = self.owner_id.clone();
            let manager = Arc::clone(&self.lock_manager);

            tokio::spawn(async move {
                let refs: Vec<&str> = resource_ids.iter().map(|s| s.as_str()).collect();
                let _ = manager.release_multiple(&refs, &owner_id).await;
            });
        }
    }
}

// ============================================================================
// Helper functions for resource locking
// ============================================================================

/// Generate a lock resource ID for a job
pub fn job_lock_resource(job_id: &str) -> String {
    format!("job:{}", job_id)
}

/// Generate a lock resource ID for a schedule
pub fn schedule_lock_resource(schedule_id: &str) -> String {
    format!("schedule:{}", schedule_id)
}

/// Generate a lock resource ID for a worker's heartbeat
pub fn worker_lock_resource(worker_id: &str) -> String {
    format!("worker:{}", worker_id)
}

/// Generate a lock resource ID for an event
pub fn event_lock_resource(event_id: &str) -> String {
    format!("event:{}", event_id)
}

/// Generate a lock resource ID for an event channel
pub fn channel_lock_resource(channel_id: &str) -> String {
    format!("channel:{}", channel_id)
}

/// Generate a lock resource ID for a subscription
pub fn subscription_lock_resource(subscription_id: &str) -> String {
    format!("subscription:{}", subscription_id)
}
