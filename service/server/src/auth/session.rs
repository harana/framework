//! Session management
//!
//! Sessions and user-session indices are persisted via the generic
//! `CacheStore` backend (e.g. Cloudflare KV, Redis, etc.) instead of
//! in-process `DashMap` statics.

use crate::config::SessionConfig;
use crate::error::ServerError;
use chrono::{Duration, Utc};
use harana_components_cache::{CacheStore, PutOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
        pub id: String,
        pub user_id: String,
        pub email: Option<String>,
        pub roles: Vec<String>,
        pub auth_method: AuthMethod,
        pub created_at: chrono::DateTime<Utc>,
        pub expires_at: chrono::DateTime<Utc>,
        pub last_activity: chrono::DateTime<Utc>,
        pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthMethod {
    Password,
    Passkey,
    OAuth,
    Oidc,
    ApiKey,
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionInfo {
    pub authenticated: bool,
    pub session_id: Option<String>,
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub roles: Vec<String>,
    pub auth_method: Option<AuthMethod>,
    pub created_at: Option<String>,
    pub expires_at: Option<String>,
}

/// Cache key helpers
fn session_key(session_id: &str) -> String {
    format!("session:{session_id}")
}

fn user_sessions_key(user_id: &str) -> String {
    format!("user_sessions:{user_id}")
}

pub struct SessionManager {
    config: SessionConfig,
    cache: Arc<dyn CacheStore>,
}

impl SessionManager {
    /// Create a new session manager with the given cache backend.
    pub fn new(config: SessionConfig, cache: Arc<dyn CacheStore>) -> Self {
        Self { config, cache }
    }

    /// TTL expressed as seconds (u64) for the cache backend.
    fn ttl_secs(&self) -> u64 {
        self.config.ttl.max(0) as u64
    }

    /// Serialize a value to JSON and store it in the cache.
    async fn cache_put_json<T: Serialize>(&self, key: &str, value: &T, opts: PutOptions) -> Result<(), ServerError> {
        let json = serde_json::to_string(value)
            .map_err(|e| ServerError::Internal(format!("serialize: {e}")))?;
        self.cache
            .put(key, &json, opts)
            .await
            .map_err(|e| ServerError::Internal(format!("cache put: {e}")))
    }

    /// Read a JSON value from the cache.
    async fn cache_get_json<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>, ServerError> {
        match self.cache.get_text(key).await {
            Ok(Some(text)) => {
                let val = serde_json::from_str(&text)
                    .map_err(|e| ServerError::Internal(format!("deserialize: {e}")))?;
                Ok(Some(val))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(ServerError::Internal(format!("cache get: {e}"))),
        }
    }

    /// Create a new session
    pub async fn create_session(
        &self,
        user_id: &str,
        email: Option<&str>,
        roles: Vec<String>,
        auth_method: AuthMethod,
    ) -> Result<Session, ServerError> {
        let now = Utc::now();
        let session = Session {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            email: email.map(String::from),
            roles,
            auth_method,
            created_at: now,
            expires_at: now + Duration::seconds(self.config.ttl),
            last_activity: now,
            data: HashMap::new(),
        };

        let opts = PutOptions::new().with_ttl(self.ttl_secs());

        // Store session
        self.cache_put_json(&session_key(&session.id), &session, opts).await?;

        // Update user → sessions index
        let mut ids: Vec<String> = self
            .cache_get_json(&user_sessions_key(user_id))
            .await?
            .unwrap_or_default();
        ids.push(session.id.clone());

        let idx_opts = PutOptions::new(); // index has no TTL; entries expire naturally
        self.cache_put_json(&user_sessions_key(user_id), &ids, idx_opts).await?;

        Ok(session)
    }

    /// Get session by ID
    pub async fn get_session(&self, session_id: &str) -> Result<Session, ServerError> {
        let session: Session = self
            .cache_get_json(&session_key(session_id))
            .await?
            .ok_or(ServerError::SessionNotFound)?;

        // Check if expired (belt-and-suspenders – cache TTL should already handle this)
        if Utc::now() > session.expires_at {
            let _ = self.destroy_session(session_id).await;
            return Err(ServerError::SessionNotFound);
        }

        Ok(session)
    }

    /// Update session last activity
    pub async fn touch_session(&self, session_id: &str) -> Result<(), ServerError> {
        let mut session = self.get_session(session_id).await?;

        session.last_activity = Utc::now();

        // Re-store with remaining TTL
        let remaining = (session.expires_at - Utc::now()).num_seconds().max(1) as u64;
        let opts = PutOptions::new().with_ttl(remaining);
        self.cache_put_json(&session_key(session_id), &session, opts).await?;

        Ok(())
    }

    /// Extend session expiration
    pub async fn extend_session(&self, session_id: &str) -> Result<Session, ServerError> {
        let mut session = self.get_session(session_id).await?;

        let now = Utc::now();
        session.last_activity = now;
        session.expires_at = now + Duration::seconds(self.config.ttl);

        let opts = PutOptions::new().with_ttl(self.ttl_secs());
        self.cache_put_json(&session_key(session_id), &session, opts).await?;

        Ok(session)
    }

    /// Set session data
    pub async fn set_session_data(
        &self,
        session_id: &str,
        key: &str,
        value: serde_json::Value,
    ) -> Result<(), ServerError> {
        let mut session = self.get_session(session_id).await?;
        session.data.insert(key.to_string(), value);

        let remaining = (session.expires_at - Utc::now()).num_seconds().max(1) as u64;
        let opts = PutOptions::new().with_ttl(remaining);
        self.cache_put_json(&session_key(session_id), &session, opts).await?;

        Ok(())
    }

    /// Get session data
    pub async fn get_session_data(
        &self,
        session_id: &str,
        key: &str,
    ) -> Result<Option<serde_json::Value>, ServerError> {
        let session = self.get_session(session_id).await?;
        Ok(session.data.get(key).cloned())
    }

    /// Destroy session
    pub async fn destroy_session(&self, session_id: &str) {
        // Read session to find the owning user
        if let Ok(Some(session)) = self.cache_get_json::<Session>(&session_key(session_id)).await {
            // Remove session key
            let _ = self.cache.delete(&session_key(session_id)).await;

            // Update user index
            if let Ok(Some(mut ids)) = self
                .cache_get_json::<Vec<String>>(&user_sessions_key(&session.user_id))
                .await
            {
                ids.retain(|id| id != session_id);
                let _ = self
                    .cache_put_json(&user_sessions_key(&session.user_id), &ids, PutOptions::new())
                    .await;
            }
        } else {
            // Best-effort delete even if we cannot read the session
            let _ = self.cache.delete(&session_key(session_id)).await;
        }
    }

    /// Destroy all sessions for a user
    pub async fn destroy_user_sessions(&self, user_id: &str) {
        if let Ok(Some(ids)) = self
            .cache_get_json::<Vec<String>>(&user_sessions_key(user_id))
            .await
        {
            for id in &ids {
                let _ = self.cache.delete(&session_key(id)).await;
            }
            let _ = self.cache.delete(&user_sessions_key(user_id)).await;
        }
    }

    /// Get all sessions for a user
    pub async fn get_user_sessions(&self, user_id: &str) -> Vec<Session> {
        let ids: Vec<String> = self
            .cache_get_json(&user_sessions_key(user_id))
            .await
            .ok()
            .flatten()
            .unwrap_or_default();

        let mut sessions = Vec::new();
        for id in &ids {
            if let Ok(Some(s)) = self.cache_get_json::<Session>(&session_key(id)).await {
                if Utc::now() <= s.expires_at {
                    sessions.push(s);
                }
            }
        }
        sessions
    }

    /// Get session info for response
    pub async fn get_session_info(&self, session_id: Option<&str>) -> SessionInfo {
        let session = match session_id {
            Some(id) => self.get_session(id).await.ok(),
            None => None,
        };

        match session {
            Some(session) => SessionInfo {
                authenticated: true,
                session_id: Some(session.id),
                user_id: Some(session.user_id),
                email: session.email,
                roles: session.roles,
                auth_method: Some(session.auth_method),
                created_at: Some(session.created_at.to_rfc3339()),
                expires_at: Some(session.expires_at.to_rfc3339()),
            },
            None => SessionInfo {
                authenticated: false,
                session_id: None,
                user_id: None,
                email: None,
                roles: vec![],
                auth_method: None,
                created_at: None,
                expires_at: None,
            },
        }
    }

    /// Get cookie name
    pub fn cookie_name(&self) -> &str {
        &self.config.cookie_name
    }

    /// Check if secure cookies should be used
    pub fn is_secure(&self) -> bool {
        self.config.secure
    }

    /// Get cookie domain
    pub fn cookie_domain(&self) -> Option<&str> {
        self.config.domain.as_deref()
    }

    /// Get session TTL
    pub fn ttl(&self) -> i64 {
        self.config.ttl
    }
}

#[cfg(test)]
mod tests;
