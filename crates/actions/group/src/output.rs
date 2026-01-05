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
