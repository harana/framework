// Harana Actions - Secret Module Output Types
// Auto-generated output structs for Secret action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// get_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSecretOutput {
    pub created_at: String, // datetime
    pub value: String,
    pub version: String,
}

// set_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSecretOutput {
    pub success: bool,
    pub version: String,
}

// delete_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSecretOutput {
    pub success: bool,
}

// list_secrets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSecretsOutput {
    pub secrets: Vec<HashMap<String, Value>>,
    pub total: i32,
}

// rotate_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateSecretOutput {
    pub new_version: String,
    pub old_version: String,
    pub success: bool,
}

// get_secret_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSecretMetadataOutput {
    pub created_at: String, // datetime
    pub description: String,
    pub expires_at: Option<String>, // datetime
    pub updated_at: String,         // datetime
    pub version_count: i32,
}

// secret_exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretExistsOutput {
    pub exists: bool,
}

// list_secret_versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSecretVersionsOutput {
    pub total: i32,
    pub versions: Vec<HashMap<String, Value>>,
}

// restore_secret_version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreSecretVersionOutput {
    pub new_version: String,
    pub success: bool,
}

// generate_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateSecretOutput {
    pub success: bool,
    pub value: String,
    pub version: String,
}
