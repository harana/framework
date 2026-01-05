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
