// Harana Actions - Group Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// add_user_to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUserToOutput {
    pub success: bool
}

// assign_role_to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleToOutput {
    pub success: bool
}

// check_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPermissionOutput {
    pub reason: String,
    pub allowed: bool
}

// create
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOutput {
    pub success: bool,
    pub group_id: String
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub success: bool
}

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub description: String,
    pub member_count: i32,
    pub roles: Vec<HashMap<String, Value>>,
    pub name: String
}

// list_members
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMembersOutput {
    pub members: Vec<HashMap<String, Value>>,
    pub total: i32
}

// list_roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesOutput {
    pub roles: Vec<HashMap<String, Value>>,
    pub total: i32
}

// lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListsOutput {
    pub groups: Vec<HashMap<String, Value>>,
    pub total: i32
}

// remove_role_from
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRoleFromOutput {
    pub success: bool
}

// remove_user_from
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveUserFromOutput {
    pub success: bool
}

// update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOutput {
    pub success: bool
}
