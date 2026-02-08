// Harana Actions - Session Module
// This module provides in-memory session management actions.

pub mod output;

use output::*;
use chrono::{DateTime, Duration, Utc};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Default session TTL in seconds (1 hour).
const DEFAULT_TTL_SECONDS: i64 = 3600;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Session {
    session_id: String,
    user_id: String,
    data: HashMap<String, Value>,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

/// Thread-safe session store: session_id -> Session
type SessionStore = Arc<RwLock<HashMap<String, Session>>>;

/// Thread-safe user sessions index: user_id -> Vec<session_id>
type UserIndex = Arc<RwLock<HashMap<String, Vec<String>>>>;

/// Global session storage.
static SESSION_STORE: Lazy<SessionStore> = Lazy::new(|| {
    Arc::new(RwLock::new(HashMap::new()))
});

/// Global user-to-sessions index.
static USER_INDEX: Lazy<UserIndex> = Lazy::new(|| {
    Arc::new(RwLock::new(HashMap::new()))
});

/// Create New Session.
pub async fn create(
    user_id: &str,
    ttl: Option<i32>,
    data: Option<HashMap<String, Value>>,
) -> Result<CreateOutput, String> {
    let session_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let ttl_seconds = ttl.map(|t| t as i64).unwrap_or(DEFAULT_TTL_SECONDS);
    let expires_at = now + Duration::seconds(ttl_seconds);
    
    let session = Session {
        session_id: session_id.clone(),
        user_id: user_id.to_string(),
        data: data.unwrap_or_default(),
        created_at: now,
        expires_at,
    };
    
    // Store session
    {
        let mut store = SESSION_STORE.write();
        store.insert(session_id.clone(), session);
    }
    
    // Update user index
    {
        let mut index = USER_INDEX.write();
        index
            .entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .push(session_id.clone());
    }
    
    Ok(CreateOutput {
        session_id,
        expires_at: expires_at.to_rfc3339(),
    })
}

/// Get Session Data.
pub async fn get(
    session_id: &str,
) -> Result<GetOutput, String> {
    let store = SESSION_STORE.read();
    
    match store.get(session_id) {
        Some(session) => {
            // Check if expired
            if Utc::now() > session.expires_at {
                drop(store);
                // Clean up expired session
                let _ = destroy(session_id).await;
                return Ok(GetOutput {
                    found: false,
                    user_id: String::new(),
                    data: HashMap::new(),
                    created_at: String::new(),
                    expires_at: String::new(),
                });
            }
            
            Ok(GetOutput {
                found: true,
                user_id: session.user_id.clone(),
                data: session.data.clone(),
                created_at: session.created_at.to_rfc3339(),
                expires_at: session.expires_at.to_rfc3339(),
            })
        }
        None => Ok(GetOutput {
            found: false,
            user_id: String::new(),
            data: HashMap::new(),
            created_at: String::new(),
            expires_at: String::new(),
        }),
    }
}

/// Update Session Data.
pub async fn update(
    session_id: &str,
    data: HashMap<String, Value>,
    merge: Option<bool>,
) -> Result<UpdateOutput, String> {
    let mut store = SESSION_STORE.write();
    
    match store.get_mut(session_id) {
        Some(session) => {
            // Check if expired
            if Utc::now() > session.expires_at {
                return Ok(UpdateOutput { success: false });
            }
            
            let merge = merge.unwrap_or(true);
            if merge {
                // Merge new data with existing
                for (key, value) in data {
                    session.data.insert(key, value);
                }
            } else {
                // Replace all data
                session.data = data;
            }
            
            Ok(UpdateOutput { success: true })
        }
        None => Ok(UpdateOutput { success: false }),
    }
}

/// Destroy Session.
pub async fn destroy(
    session_id: &str,
) -> Result<DestroyOutput, String> {
    let user_id = {
        let mut store = SESSION_STORE.write();
        match store.remove(session_id) {
            Some(session) => Some(session.user_id),
            None => None,
        }
    };
    
    // Update user index
    if let Some(uid) = user_id {
        let mut index = USER_INDEX.write();
        if let Some(sessions) = index.get_mut(&uid) {
            sessions.retain(|id| id != session_id);
            if sessions.is_empty() {
                index.remove(&uid);
            }
        }
    }
    
    Ok(DestroyOutput { success: true })
}

/// Refresh Session Expiry.
pub async fn refresh(
    session_id: &str,
    ttl: Option<i32>,
) -> Result<RefreshOutput, String> {
    let mut store = SESSION_STORE.write();
    
    match store.get_mut(session_id) {
        Some(session) => {
            let ttl_seconds = ttl.map(|t| t as i64).unwrap_or(DEFAULT_TTL_SECONDS);
            let new_expires_at = Utc::now() + Duration::seconds(ttl_seconds);
            session.expires_at = new_expires_at;
            
            Ok(RefreshOutput {
                success: true,
                expires_at: new_expires_at.to_rfc3339(),
            })
        }
        None => Ok(RefreshOutput {
            success: false,
            expires_at: String::new(),
        }),
    }
}

/// Get Session Value.
pub async fn get_value(
    session_id: &str,
    key: &str,
) -> Result<GetValueOutput, String> {
    let store = SESSION_STORE.read();
    
    match store.get(session_id) {
        Some(session) => {
            // Check if expired
            if Utc::now() > session.expires_at {
                return Ok(GetValueOutput {
                    found: false,
                    value: String::new(),
                });
            }
            
            match session.data.get(key) {
                Some(value) => Ok(GetValueOutput {
                    found: true,
                    value: value.to_string(),
                }),
                None => Ok(GetValueOutput {
                    found: false,
                    value: String::new(),
                }),
            }
        }
        None => Ok(GetValueOutput {
            found: false,
            value: String::new(),
        }),
    }
}

/// Set Session Value.
pub async fn set_value(
    session_id: &str,
    key: &str,
    value: &str,
) -> Result<SetValueOutput, String> {
    let mut store = SESSION_STORE.write();
    
    match store.get_mut(session_id) {
        Some(session) => {
            // Check if expired
            if Utc::now() > session.expires_at {
                return Ok(SetValueOutput { success: false });
            }
            
            let parsed_value: Value = serde_json::from_str(value)
                .unwrap_or_else(|_| Value::String(value.to_string()));
            
            session.data.insert(key.to_string(), parsed_value);
            
            Ok(SetValueOutput { success: true })
        }
        None => Ok(SetValueOutput { success: false }),
    }
}

/// Delete Session Value.
pub async fn delete_value(
    session_id: &str,
    key: &str,
) -> Result<DeleteValueOutput, String> {
    let mut store = SESSION_STORE.write();
    
    match store.get_mut(session_id) {
        Some(session) => {
            session.data.remove(key);
            Ok(DeleteValueOutput { success: true })
        }
        None => Ok(DeleteValueOutput { success: false }),
    }
}

/// List User Sessions.
pub async fn list_users(
    user_id: &str,
) -> Result<ListUsersOutput, String> {
    let index = USER_INDEX.read();
    let store = SESSION_STORE.read();
    
    let session_ids = index.get(user_id);
    
    match session_ids {
        Some(ids) => {
            let now = Utc::now();
            let mut sessions: Vec<HashMap<String, Value>> = Vec::new();
            
            for session_id in ids {
                if let Some(session) = store.get(session_id) {
                    // Skip expired sessions
                    if now > session.expires_at {
                        continue;
                    }
                    
                    let mut info = HashMap::new();
                    info.insert("session_id".to_string(), Value::String(session.session_id.clone()));
                    info.insert("user_id".to_string(), Value::String(session.user_id.clone()));
                    info.insert("created_at".to_string(), Value::String(session.created_at.to_rfc3339()));
                    info.insert("expires_at".to_string(), Value::String(session.expires_at.to_rfc3339()));
                    sessions.push(info);
                }
            }
            
            let total = sessions.len() as i32;
            Ok(ListUsersOutput { sessions, total })
        }
        None => Ok(ListUsersOutput {
            sessions: Vec::new(),
            total: 0,
        }),
    }
}

/// Destroy All User Sessions.
pub async fn destroy_users(
    user_id: &str,
    except_id: Option<&str>,
) -> Result<DestroyUsersOutput, String> {
    let session_ids: Vec<String> = {
        let index = USER_INDEX.read();
        match index.get(user_id) {
            Some(ids) => ids.clone(),
            None => Vec::new(),
        }
    };
    
    let mut destroyed_count = 0;
    
    for session_id in session_ids {
        // Skip the exception if provided
        if let Some(except) = except_id {
            if session_id == except {
                continue;
            }
        }
        
        let _ = destroy(&session_id).await;
        destroyed_count += 1;
    }
    
    Ok(DestroyUsersOutput {
        success: true,
        destroyed_count,
    })
}

/// Check Session Valid.
pub async fn validate(
    session_id: &str,
) -> Result<ValidateOutput, String> {
    let store = SESSION_STORE.read();
    
    match store.get(session_id) {
        Some(session) => {
            let now = Utc::now();
            let is_valid = now <= session.expires_at;
            
            Ok(ValidateOutput {
                valid: is_valid,
                user_id: if is_valid { session.user_id.clone() } else { String::new() },
                expires_at: if is_valid { session.expires_at.to_rfc3339() } else { String::new() },
            })
        }
        None => Ok(ValidateOutput {
            valid: false,
            user_id: String::new(),
            expires_at: String::new(),
        }),
    }
}

#[cfg(test)]
mod tests;
