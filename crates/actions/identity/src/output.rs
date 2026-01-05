// Harana Actions - Identity Module Output Types
// Auto-generated output structs for Identity action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// authenticate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticateOutput {
    pub access_token: String,
    pub expires_at: String, // datetime
    pub refresh_token: String,
    pub success: bool,
    pub user_id: String,
}

// verify_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyTokenOutput {
    pub claims: HashMap<String, Value>,
    pub expires_at: String, // datetime
    pub user_id: String,
    pub valid: bool,
}

// refresh_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenOutput {
    pub access_token: String,
    pub expires_at: String, // datetime
    pub refresh_token: String,
}

// revoke_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeTokenOutput {
    pub success: bool,
}

// get_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserOutput {
    pub created_at: String, // datetime
    pub email: String,
    pub metadata: HashMap<String, Value>,
    pub roles: Vec<String>,
    pub user_id: String,
    pub username: String,
}

// create_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserOutput {
    pub success: bool,
    pub user_id: String,
}

// update_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserOutput {
    pub success: bool,
}

// delete_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserOutput {
    pub success: bool,
}

// change_password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordOutput {
    pub success: bool,
}

// request_password_reset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestPasswordResetOutput {
    pub reset_token: String,
    pub success: bool,
}

// reset_password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordOutput {
    pub success: bool,
}

// list_users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersOutput {
    pub total: i32,
    pub users: Vec<HashMap<String, Value>>,
}
