// Harana Actions - Policy Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// attach_to_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachToRoleOutput {
    pub success: bool
}

// attach_to_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachToUserOutput {
    pub success: bool
}

// create
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOutput {
    pub success: bool,
    pub policy_id: String
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub success: bool
}

// detach_from_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachFromRoleOutput {
    pub success: bool
}

// detach_from_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachFromUserOutput {
    pub success: bool
}

// evaluate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateOutput {
    pub reason: String,
    pub allowed: bool,
    pub evaluated_conditions: HashMap<String, Value>
}

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub description: String,
    pub name: String,
    pub actions: Vec<String>,
    pub conditions: HashMap<String, Value>,
    pub resources: Vec<String>,
    pub effect: String
}

// list_policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPoliciesOutput {
    pub policies: Vec<HashMap<String, Value>>,
    pub total: i32
}

// update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOutput {
    pub success: bool
}
