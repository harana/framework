// Harana Actions - AWS IAM Module Output Types
// Auto-generated output structs for AWS IAM action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// create_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserOutput {
    pub arn: String,
    pub create_date: String,
    pub success: bool,
    pub user_id: String,
    pub user_name: String,
}

// delete_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserOutput {
    pub success: bool,
}

// get_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserOutput {
    pub arn: String,
    pub create_date: String,
    pub password_last_used: String,
    pub path: String,
    pub permissions_boundary: HashMap<String, Value>,
    pub tags: Vec<HashMap<String, Value>>,
    pub user_id: String,
    pub user_name: String,
}

// list_users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersOutput {
    pub is_truncated: bool,
    pub marker: String,
    pub users: Vec<HashMap<String, Value>>,
}

// update_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserOutput {
    pub success: bool,
}

// TODO: Add remaining output types - see core/schema/actions/aws_iam.yml
