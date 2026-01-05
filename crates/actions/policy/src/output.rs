// Harana Actions - Policy Module Output Types
// Auto-generated output structs for Policy action methods.

use serde::{Deserialize, Serialize};

// create_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicyOutput {
    pub policy_id: String,
    pub success: bool,
}

// update_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePolicyOutput {
    pub success: bool,
}

// delete_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePolicyOutput {
    pub success: bool,
}

// get_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPolicyOutput {
    pub actions: Vec<String>,
    pub conditions: serde_json::Value,
    pub description: String,
    pub effect: String,
    pub name: String,
    pub resources: Vec<String>,
}

// list_policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPoliciesOutput {
    pub policies: Vec<serde_json::Value>,
    pub total: i32,
}

// attach_policy_to_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachPolicyToUserOutput {
    pub success: bool,
}

// detach_policy_from_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachPolicyFromUserOutput {
    pub success: bool,
}

// attach_policy_to_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachPolicyToRoleOutput {
    pub success: bool,
}

// detach_policy_from_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachPolicyFromRoleOutput {
    pub success: bool,
}

// evaluate_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluatePolicyOutput {
    pub allowed: bool,
    pub evaluated_conditions: serde_json::Value,
    pub reason: String,
}
