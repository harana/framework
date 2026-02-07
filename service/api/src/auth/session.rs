//! Session management

use crate::config::SessionConfig;
use crate::error::ServerError;
use chrono::{Duration, Utc};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Session store
static SESSION_STORE: Lazy<DashMap<String, Session>> = Lazy::new(DashMap::new);

/// User to sessions index
static USER_SESSIONS: Lazy<DashMap<String, Vec<String>>> = Lazy::new(DashMap::new);

/// Session data
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

/// Authentication method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthMethod {
    /// Password authentication
    Password,
    /// Passkey/WebAuthn authentication
    Passkey,
    /// OAuth authentication
    OAuth,
    /// OpenID Connect authentication
    Oidc,
    /// API key authentication
    ApiKey,
}

/// Session info response
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

/// Session Manager
pub struct SessionManager {
    config: SessionConfig,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(config: SessionConfig) -> Self {
        Self { config }
    }

    /// Create a new session
    pub fn create_session(
        &self,
        user_id: &str,
        email: Option<&str>,
        roles: Vec<String>,
        auth_method: AuthMethod,
    ) -> Session {
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

        // Store session
        SESSION_STORE.insert(session.id.clone(), session.clone());

        // Update user index
        USER_SESSIONS
            .entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .push(session.id.clone());

        session
    }

    /// Get session by ID
    pub fn get_session(&self, session_id: &str) -> Result<Session, ServerError> {
        let session = SESSION_STORE
            .get(session_id)
            .map(|s| s.clone())
            .ok_or(ServerError::SessionNotFound)?;

        // Check if expired
        if Utc::now() > session.expires_at {
            self.destroy_session(session_id);
            return Err(ServerError::SessionNotFound);
        }

        Ok(session)
    }

    /// Update session last activity
    pub fn touch_session(&self, session_id: &str) -> Result<(), ServerError> {
        let mut session = SESSION_STORE.get_mut(session_id).ok_or(ServerError::SessionNotFound)?;

        session.last_activity = Utc::now();
        Ok(())
    }

    /// Extend session expiration
    pub fn extend_session(&self, session_id: &str) -> Result<Session, ServerError> {
        let mut session = SESSION_STORE.get_mut(session_id).ok_or(ServerError::SessionNotFound)?;

        let now = Utc::now();
        session.last_activity = now;
        session.expires_at = now + Duration::seconds(self.config.ttl);

        Ok(session.clone())
    }

    /// Set session data
    pub fn set_session_data(&self, session_id: &str, key: &str, value: serde_json::Value) -> Result<(), ServerError> {
        let mut session = SESSION_STORE.get_mut(session_id).ok_or(ServerError::SessionNotFound)?;

        session.data.insert(key.to_string(), value);
        Ok(())
    }

    /// Get session data
    pub fn get_session_data(&self, session_id: &str, key: &str) -> Result<Option<serde_json::Value>, ServerError> {
        let session = SESSION_STORE.get(session_id).ok_or(ServerError::SessionNotFound)?;

        Ok(session.data.get(key).cloned())
    }

    /// Destroy session
    pub fn destroy_session(&self, session_id: &str) {
        if let Some((_, session)) = SESSION_STORE.remove(session_id) {
            // Remove from user index
            if let Some(mut sessions) = USER_SESSIONS.get_mut(&session.user_id) {
                sessions.retain(|id| id != session_id);
            }
        }
    }

    /// Destroy all sessions for a user
    pub fn destroy_user_sessions(&self, user_id: &str) {
        if let Some((_, session_ids)) = USER_SESSIONS.remove(user_id) {
            for session_id in session_ids {
                SESSION_STORE.remove(&session_id);
            }
        }
    }

    /// Get all sessions for a user
    pub fn get_user_sessions(&self, user_id: &str) -> Vec<Session> {
        USER_SESSIONS
            .get(user_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| SESSION_STORE.get(id).map(|s| s.clone()))
                    .filter(|s| Utc::now() <= s.expires_at)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get session info for response
    pub fn get_session_info(&self, session_id: Option<&str>) -> SessionInfo {
        match session_id.and_then(|id| self.get_session(id).ok()) {
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
mod tests {
    use super::*;

    #[test]
    fn test_create_and_get_session() {
        let config = SessionConfig::default();
        let manager = SessionManager::new(config);

        let session = manager.create_session(
            "user123",
            Some("test@example.com"),
            vec!["user".to_string()],
            AuthMethod::Password,
        );

        let retrieved = manager.get_session(&session.id).unwrap();
        assert_eq!(retrieved.user_id, "user123");
        assert_eq!(retrieved.email, Some("test@example.com".to_string()));
    }

    #[test]
    fn test_destroy_session() {
        let config = SessionConfig::default();
        let manager = SessionManager::new(config);

        let session = manager.create_session("user123", None, vec![], AuthMethod::Passkey);
        manager.destroy_session(&session.id);

        assert!(manager.get_session(&session.id).is_err());
    }
}
