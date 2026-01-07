// Harana Actions - Identity Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// authenticate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticateOutput {
    pub refresh_token: String,
    pub expires_at: String,
    pub success: bool,
    pub access_token: String,
    pub user_id: String
}

// change_password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordOutput {
    pub success: bool
}

// create_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserOutput {
    pub success: bool,
    pub user_id: String
}

// delete_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserOutput {
    pub success: bool
}

// get_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserOutput {
    pub email: String,
    pub username: String,
    pub metadata: HashMap<String, Value>,
    pub created_at: String,
    pub user_id: String,
    pub roles: Vec<String>
}

// list_users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersOutput {
    pub total: i32,
    pub users: Vec<HashMap<String, Value>>
}

// refresh_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenOutput {
    pub refresh_token: String,
    pub access_token: String,
    pub expires_at: String
}

// request_password_reset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestPasswordResetOutput {
    pub success: bool,
    pub reset_token: String
}

// reset_password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordOutput {
    pub success: bool
}

// revoke_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeTokenOutput {
    pub success: bool
}

// update_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserOutput {
    pub success: bool
}

// verify_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyTokenOutput {
    pub expires_at: String,
    pub user_id: String,
    pub claims: HashMap<String, Value>,
    pub valid: bool
}
