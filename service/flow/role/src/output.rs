// Harana Actions - Role Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// add_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPermissionOutput {
    pub success: bool
}

// check_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPermissionOutput {
    pub allowed: bool
}

// clone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneOutput {
    pub success: bool,
    pub role_id: String
}

// create
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOutput {
    pub success: bool,
    pub role_id: String
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub success: bool
}

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub user_count: i32,
    pub permissions: Vec<HashMap<String, Value>>,
    pub description: String,
    pub name: String
}

// list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOutput {
    pub roles: Vec<HashMap<String, Value>>,
    pub total: i32
}

// remove_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePermissionOutput {
    pub success: bool
}

// update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOutput {
    pub success: bool
}
