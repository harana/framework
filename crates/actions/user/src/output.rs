// Harana Actions - User Module Output Types
// Auto-generated output structs for User action methods.

use serde::{Deserialize, Serialize};

// check_user_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckUserPermissionOutput {
    pub allowed: bool,
    pub reason: String,
}

// grant_user_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantUserPermissionOutput {
    pub permission_id: String,
    pub success: bool,
}

// revoke_user_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeUserPermissionOutput {
    pub success: bool,
}

// list_user_permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUserPermissionsOutput {
    pub permissions: Vec<serde_json::Value>,
    pub total: i32,
}

// assign_user_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignUserRoleOutput {
    pub success: bool,
}

// remove_user_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveUserRoleOutput {
    pub success: bool,
}

// list_user_roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUserRolesOutput {
    pub roles: Vec<serde_json::Value>,
    pub total: i32,
}

// get_user_effective_permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserEffectivePermissionsOutput {
    pub permissions: Vec<serde_json::Value>,
    pub sources: Vec<serde_json::Value>,
}
