// Harana Actions - Role Module Output Types
// Auto-generated output structs for Role action methods.

use serde::{Deserialize, Serialize};

// create_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleOutput {
    pub role_id: String,
    pub success: bool,
}

// update_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleOutput {
    pub success: bool,
}

// delete_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoleOutput {
    pub success: bool,
}

// list_roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesOutput {
    pub roles: Vec<serde_json::Value>,
    pub total: i32,
}

// get_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoleOutput {
    pub description: String,
    pub name: String,
    pub permissions: Vec<serde_json::Value>,
    pub user_count: i32,
}

// check_role_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckRolePermissionOutput {
    pub allowed: bool,
}

// add_role_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRolePermissionOutput {
    pub success: bool,
}

// remove_role_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRolePermissionOutput {
    pub success: bool,
}

// clone_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneRoleOutput {
    pub role_id: String,
    pub success: bool,
}

// create
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOutput {
    pub role_id: String,
    pub success: bool
}

// update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOutput {
    pub success: bool
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub success: bool
}

// list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOutput {
    pub total: i32,
    pub roles: Vec<HashMap<String, Value>>
}

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub description: String,
    pub name: String,
    pub user_count: i32,
    pub permissions: Vec<HashMap<String, Value>>
}

// check_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPermissionOutput {
    pub allowed: bool
}

// add_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPermissionOutput {
    pub success: bool
}

// remove_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePermissionOutput {
    pub success: bool
}

// clone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneOutput {
    pub role_id: String,
    pub success: bool
}