// Harana Actions - Secret Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSecretOutput {
    pub value: String,
    pub version: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSecretOutput {
    pub success: bool,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSecretOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSecretsOutput {
    pub secrets: Vec<SecretInfo>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateSecretOutput {
    pub success: bool,
    pub old_version: String,
    pub new_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSecretMetadataOutput {
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub expires_at: Option<String>,
    pub version_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretExistsOutput {
    pub exists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSecretVersionsOutput {
    pub versions: Vec<SecretVersion>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreSecretVersionOutput {
    pub success: bool,
    pub new_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateSecretOutput {
    pub success: bool,
    pub value: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretInfo {
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretVersion {
    pub version: String,
    pub created_at: String,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub name: String,
    pub value: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub expires_at: Option<String>,
}
