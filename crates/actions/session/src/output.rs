// Harana Actions - Session Module Output Types
// Auto-generated output structs for Session action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// create_session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionOutput {
    pub expires_at: String, // datetime
    pub session_id: String,
}

// get_session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSessionOutput {
    pub created_at: Option<String>, // datetime
    pub data: Option<HashMap<String, Value>>,
    pub expires_at: Option<String>, // datetime
    pub found: bool,
    pub user_id: Option<String>,
}

// update_session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSessionOutput {
    pub success: bool,
}

// destroy_session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestroySessionOutput {
    pub success: bool,
}

// refresh_session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshSessionOutput {
    pub expires_at: String, // datetime
    pub success: bool,
}

// get_session_value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSessionValueOutput {
    pub found: bool,
    pub value: Option<Value>,
}

// set_session_value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSessionValueOutput {
    pub success: bool,
}

// delete_session_value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSessionValueOutput {
    pub success: bool,
}

// list_user_sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUserSessionsOutput {
    pub sessions: Vec<HashMap<String, Value>>,
    pub total: i32,
}

// destroy_user_sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestroyUserSessionsOutput {
    pub destroyed_count: i32,
    pub success: bool,
}

// validate_session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateSessionOutput {
    pub expires_at: Option<String>, // datetime
    pub user_id: Option<String>,
    pub valid: bool,
}
