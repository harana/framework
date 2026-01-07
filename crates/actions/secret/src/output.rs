// Harana Actions - Secret Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// delete_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSecretOutput {
    pub success: bool
}

// generate_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateSecretOutput {
    pub version: String,
    pub value: String,
    pub success: bool
}

// get_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSecretOutput {
    pub value: String,
    pub version: String,
    pub created_at: String
}

// get_secret_metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSecretMetadataOutput {
    pub expires_at: String,
    pub created_at: String,
    pub description: String,
    pub version_count: i32,
    pub updated_at: String
}

// list_secret_versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSecretVersionsOutput {
    pub total: i32,
    pub versions: Vec<HashMap<String, Value>>
}

// list_secrets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSecretsOutput {
    pub total: i32,
    pub secrets: Vec<HashMap<String, Value>>
}

// restore_secret_version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreSecretVersionOutput {
    pub new_version: String,
    pub success: bool
}

// rotate_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateSecretOutput {
    pub new_version: String,
    pub old_version: String,
    pub success: bool
}

// secret_exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretExistsOutput {
    pub exists: bool
}

// set_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSecretOutput {
    pub success: bool,
    pub version: String
}
