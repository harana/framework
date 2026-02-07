// Harana Actions - Session Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// create
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOutput {
    pub expires_at: String,
    pub session_id: String
}

// delete_value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteValueOutput {
    pub success: bool
}

// destroy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestroyOutput {
    pub success: bool
}

// destroy_users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestroyUsersOutput {
    pub destroyed_count: i32,
    pub success: bool
}

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub found: bool,
    pub expires_at: String,
    pub created_at: String,
    pub data: HashMap<String, Value>,
    pub user_id: String
}

// get_value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetValueOutput {
    pub found: bool,
    pub value: String
}

// list_users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersOutput {
    pub sessions: Vec<HashMap<String, Value>>,
    pub total: i32
}

// refresh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshOutput {
    pub success: bool,
    pub expires_at: String
}

// set_value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetValueOutput {
    pub success: bool
}

// update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOutput {
    pub success: bool
}

// validate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub user_id: String,
    pub expires_at: String,
    pub valid: bool
}
