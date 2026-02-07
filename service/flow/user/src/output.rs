// Harana Actions - User Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// assign_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleOutput {
    pub success: bool
}

// check_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPermissionOutput {
    pub allowed: bool,
    pub reason: String
}

// get_effective_permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEffectivePermissionsOutput {
    pub sources: Vec<HashMap<String, Value>>,
    pub permissions: Vec<HashMap<String, Value>>
}

// grant_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantPermissionOutput {
    pub permission_id: String,
    pub success: bool
}

// list_permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPermissionsOutput {
    pub total: i32,
    pub permissions: Vec<HashMap<String, Value>>
}

// list_roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesOutput {
    pub total: i32,
    pub roles: Vec<HashMap<String, Value>>
}

// remove_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRoleOutput {
    pub success: bool
}

// revoke_permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokePermissionOutput {
    pub success: bool
}
