use async_trait::async_trait;
use chrono::{Duration, Utc};
use harana_components_cache::{CacheService, ListOptions, PutOptions};
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration as TokioDuration};

use crate::{DistributedLock, LockConfig, LockError, LockResult};

/// Cache key prefix for lock entries
const LOCK_PREFIX: &str = "lock:";

/// Cache key prefix for owner-to-locks mapping
const OWNER_PREFIX: &str = "lock_owner:";

/// Build cache key for a lock resource
fn lock_key(resource_id: &str) -> String {
    format!("{}{}", LOCK_PREFIX, resource_id)
}

/// Build cache key for an owner's lock set
fn owner_key(owner_id: &str) -> String {
    format!("{}{}", OWNER_PREFIX, owner_id)
}

// ============================================================================
// Lock Manager Trait
// ============================================================================

/// Trait for distributed lock operations
#[async_trait]
pub trait LockManager: Send + Sync {
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
    async fn cleanup_expired_locks(&self) -> LockResult<u64>;

    /// Force release a lock (admin operation)
    async fn force_release(&self, resource_id: &str) -> LockResult<bool>;
}

// ============================================================================
// Lock Handle (RAII guard)
// ============================================================================

pub struct LockHandle<C>
where
    C: CacheService + 'static,
{
    resource_id: String,
    owner_id: String,
    fencing_token: u64,
    lock_manager: Arc<DistributedLockManager<C>>,
    released: bool,
}

impl<C> LockHandle<C>
where
    C: CacheService,
{
    fn new(
        resource_id: String,
        owner_id: String,
        fencing_token: u64,
        lock_manager: Arc<DistributedLockManager<C>>,
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

impl<C> Drop for LockHandle<C>
where
    C: CacheService + 'static,
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
// Multi-Lock Guard (for acquiring multiple locks safely)
// ============================================================================

pub struct MultiLockGuard<C>
where
    C: CacheService + 'static,
{
    resource_ids: Vec<String>,
    owner_id: String,
    fencing_tokens: Vec<u64>,
    lock_manager: Arc<DistributedLockManager<C>>,
    released: bool,
}

impl<C> MultiLockGuard<C>
where
    C: CacheService + 'static,
{
    pub fn new(
        resource_ids: Vec<String>,
        owner_id: String,
        fencing_tokens: Vec<u64>,
        lock_manager: Arc<DistributedLockManager<C>>,
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

impl<C> Drop for MultiLockGuard<C>
where
    C: CacheService + 'static,
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
// Distributed Lock Manager Implementation
// ============================================================================

pub struct DistributedLockManager<C>
where
    C: CacheService,
{
    cache: C,
    config: LockConfig,
        fencing_counter: AtomicU64,
        owner_locks: RwLock<std::collections::HashMap<String, BTreeSet<String>>>,
}

impl<C> DistributedLockManager<C>
where
    C: CacheService + 'static,
{
    pub fn new(cache: C, config: LockConfig) -> Self {
        Self {
            cache,
            config,
            fencing_counter: AtomicU64::new(0),
            owner_locks: RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub fn with_default_config(cache: C) -> Self {
        Self::new(cache, LockConfig::default())
    }

    /// Get the next fencing token
    fn next_fencing_token(&self) -> u64 {
        self.fencing_counter.fetch_add(1, AtomicOrdering::SeqCst) + 1
    }
    pub async fn initialize(&self) -> LockResult<()> {
        // Scan existing locks in cache to find the highest fencing token
        let list_result = self.cache
            .list(ListOptions::new().with_prefix(LOCK_PREFIX).with_limit(10000))
            .await
            .map_err(|e| LockError::CacheError(e.to_string()))?;

        let mut max_token: u64 = 0;
        for key_entry in &list_result.keys {
            if let Ok(Some(lock)) = self.cache.get_json::<DistributedLock>(&key_entry.name).await {
                if lock.fencing_token > max_token {
                    max_token = lock.fencing_token;
                }
            }
        }

        if max_token > 0 {
            self.fencing_counter.store(max_token, AtomicOrdering::SeqCst);
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

    async fn store_lock(&self, lock: &DistributedLock) -> LockResult<()> {
        let ttl_seconds = (lock.expires_at - Utc::now()).num_seconds().max(1) as u64;
        let options = PutOptions::new().with_ttl(ttl_seconds);
        self.cache
            .put_json(&lock_key(&lock.id), lock, options)
            .await
            .map_err(|e| LockError::CacheError(e.to_string()))
    }

    async fn persist_owner_locks(&self, owner_id: &str) -> LockResult<()> {
        let owner_locks = self.owner_locks.read().await;
        if let Some(locks) = owner_locks.get(owner_id) {
            let resource_ids: Vec<&String> = locks.iter().collect();
            // Owner mapping doesn't expire – it is maintained alongside lock operations
            self.cache
                .put_json(&owner_key(owner_id), &resource_ids, PutOptions::new())
                .await
                .map_err(|e| LockError::CacheError(e.to_string()))?;
        } else {
            // No locks held, remove the owner key
            let _ = self.cache.delete(&owner_key(owner_id)).await;
        }
        Ok(())
    }

    pub async fn acquire_handle(
        self: &Arc<Self>,
        resource_id: &str,
        owner_id: &str,
        ttl_seconds: Option<u64>,
    ) -> LockResult<LockHandle<C>> {
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
impl<C> LockManager for DistributedLockManager<C>
where
    C: CacheService + Send + Sync + 'static,
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

        // Try to get existing lock from cache
        let existing: Option<DistributedLock> = self.cache
            .get_json(&lock_key(resource_id))
            .await
            .map_err(|e| LockError::CacheError(e.to_string()))?;

        match existing {
            Some(lock) => {
                if lock.is_expired() {
                    // Lock expired (should be rare since cache TTL handles this),
                    // but handle gracefully – take it over
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

                    self.store_lock(&new_lock).await?;
                    self.track_lock(resource_id, owner_id).await;
                    self.persist_owner_locks(owner_id).await?;
                    Ok(Some(fencing_token))
                } else if lock.owner_id == owner_id {
                    // Already held by this owner, extend it
                    let mut updated_lock = lock;
                    updated_lock.extend(ttl);

                    self.store_lock(&updated_lock).await?;
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

                self.store_lock(&new_lock).await?;
                self.track_lock(resource_id, owner_id).await;
                self.persist_owner_locks(owner_id).await?;
                Ok(Some(fencing_token))
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
        let existing: Option<DistributedLock> = self.cache
            .get_json(&lock_key(resource_id))
            .await
            .map_err(|e| LockError::CacheError(e.to_string()))?;

        match existing {
            Some(lock) if lock.owner_id == owner_id => {
                self.cache
                    .delete(&lock_key(resource_id))
                    .await
                    .map_err(|e| LockError::CacheError(e.to_string()))?;

                self.untrack_lock(resource_id, owner_id).await;
                self.persist_owner_locks(owner_id).await?;
                Ok(true)
            }
            Some(_) => {
                // Not owned by this owner
                Ok(false)
            }
            None => {
                // Lock doesn't exist (possibly expired via cache TTL)
                self.untrack_lock(resource_id, owner_id).await;
                self.persist_owner_locks(owner_id).await?;
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
        let existing: Option<DistributedLock> = self.cache
            .get_json(&lock_key(resource_id))
            .await
            .map_err(|e| LockError::CacheError(e.to_string()))?;

        match existing {
            Some(mut lock) if lock.owner_id == owner_id && !lock.is_expired() => {
                lock.extend(extension_seconds);
                self.store_lock(&lock).await?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    async fn is_locked(&self, resource_id: &str) -> LockResult<bool> {
        let existing: Option<DistributedLock> = self.cache
            .get_json(&lock_key(resource_id))
            .await
            .map_err(|e| LockError::CacheError(e.to_string()))?;

        Ok(existing.map(|l| !l.is_expired()).unwrap_or(false))
    }

    async fn get_lock_info(&self, resource_id: &str) -> LockResult<Option<DistributedLock>> {
        self.cache
            .get_json(&lock_key(resource_id))
            .await
            .map_err(|e| LockError::CacheError(e.to_string()))
    }

    async fn get_locks_by_owner(&self, owner_id: &str) -> LockResult<Vec<DistributedLock>> {
        // Use in-memory owner tracking to find the owner's locks
        let owner_locks = self.owner_locks.read().await;
        let resource_ids: Vec<String> = match owner_locks.get(owner_id) {
            Some(ids) => ids.iter().cloned().collect(),
            None => return Ok(Vec::new()),
        };
        drop(owner_locks);

        let mut locks = Vec::new();
        for resource_id in &resource_ids {
            if let Some(lock) = self.cache
                .get_json::<DistributedLock>(&lock_key(resource_id))
                .await
                .map_err(|e| LockError::CacheError(e.to_string()))?
            {
                if lock.owner_id == owner_id && !lock.is_expired() {
                    locks.push(lock);
                }
            }
        }
        Ok(locks)
    }

    async fn cleanup_expired_locks(&self) -> LockResult<u64> {
        // With cache TTL, expired locks are automatically evicted.
        // This method cleans up stale owner tracking entries.
        let mut cleaned = 0u64;
        let owner_ids: Vec<String> = {
            let owner_locks = self.owner_locks.read().await;
            owner_locks.keys().cloned().collect()
        };

        for owner_id in owner_ids {
            let mut needs_update = false;
            let stale_resources: Vec<String> = {
                let owner_locks = self.owner_locks.read().await;
                if let Some(resources) = owner_locks.get(&owner_id) {
                    let mut stale = Vec::new();
                    for resource_id in resources {
                        let exists: Option<DistributedLock> = self.cache
                            .get_json(&lock_key(resource_id))
                            .await
                            .map_err(|e| LockError::CacheError(e.to_string()))?;
                        if exists.is_none() {
                            stale.push(resource_id.clone());
                        }
                    }
                    stale
                } else {
                    Vec::new()
                }
            };

            for resource_id in &stale_resources {
                self.untrack_lock(resource_id, &owner_id).await;
                needs_update = true;
                cleaned += 1;
            }

            if needs_update {
                self.persist_owner_locks(&owner_id).await?;
            }
        }

        Ok(cleaned)
    }

    async fn force_release(&self, resource_id: &str) -> LockResult<bool> {
        let existing: Option<DistributedLock> = self.cache
            .get_json(&lock_key(resource_id))
            .await
            .map_err(|e| LockError::CacheError(e.to_string()))?;

        if let Some(lock) = existing {
            self.cache
                .delete(&lock_key(resource_id))
                .await
                .map_err(|e| LockError::CacheError(e.to_string()))?;

            self.untrack_lock(resource_id, &lock.owner_id).await;
            self.persist_owner_locks(&lock.owner_id).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
