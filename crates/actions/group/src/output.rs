// Harana Actions - Group Module Output Types
// Auto-generated output structs for Group action methods.

use serde::{Deserialize, Serialize};

// create_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupOutput {
    pub group_id: String,
    pub success: bool,
}

// update_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGroupOutput {
    pub success: bool,
}

// delete_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteGroupOutput {
    pub success: bool,
}

// get_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroupOutput {
    pub description: String,
    pub member_count: i32,
    pub name: String,
    pub roles: Vec<serde_json::Value>,
}

// list_groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupsOutput {
    pub groups: Vec<serde_json::Value>,
    pub total: i32,
}

// add_user_to_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUserToGroupOutput {
    pub success: bool,
}

// remove_user_from_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveUserFromGroupOutput {
    pub success: bool,
}

// list_group_members
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupMembersOutput {
    pub members: Vec<serde_json::Value>,
    pub total: i32,
}

// assign_role_to_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleToGroupOutput {
    pub success: bool,
}

// remove_role_from_group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRoleFromGroupOutput {
    pub success: bool,
}

// list_group_roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupRolesOutput {
    pub roles: Vec<serde_json::Value>,
    pub total: i32,
}

// check_group_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckGroupPermissionOutput {
    pub allowed: bool,
    pub reason: String,
}

// create
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOutput {
    pub group_id: String,
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

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub roles: Vec<HashMap<String, Value>>,
    pub name: String,
    pub member_count: i32,
    pub description: String
}

// lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListsOutput {
    pub total: i32,
    pub groups: Vec<HashMap<String, Value>>
}

// add_user_to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUserToOutput {
    pub success: bool
}

// remove_user_from
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveUserFromOutput {
    pub success: bool
}

// list_members
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMembersOutput {
    pub total: i32,
    pub members: Vec<HashMap<String, Value>>
}

// assign_role_to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleToOutput {
    pub success: bool
}

// remove_role_from
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRoleFromOutput {
    pub success: bool
}

// list_roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesOutput {
    pub roles: Vec<HashMap<String, Value>>,
    pub total: i32
}

// check_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPermissionOutput {
    pub reason: String,
    pub allowed: bool
}