// Harana Actions - Sign Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};

// sign_artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignArtifactOutput {
    pub signature: String,
    pub signature_format: String,
    pub algorithm: String,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
}

// verify_artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyArtifactOutput {
    pub valid: bool,
    pub signer: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
    pub algorithm: Option<String>,
    pub error: Option<String>,
}

// sign_code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignCodeOutput {
    pub signed_artifact: Vec<u8>,
    pub signature: String,
    pub success: bool,
    pub timestamp: Option<DateTime<Utc>>,
}

// create_detached
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDetachedOutput {
    pub signature: String,
    pub success: bool,
}

// verify_detached
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyDetachedOutput {
    pub valid: bool,
    pub signer: Option<String>,
    pub error: Option<String>,
}

// sign_container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignContainerOutput {
    pub signature: String,
    pub signature_digest: String,
    pub success: bool,
}

// verify_container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyContainerOutput {
    pub valid: bool,
    pub signer: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

// sign_jar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignJarOutput {
    pub signed_jar_path: String,
    pub success: bool,
}

// verify_jar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyJarOutput {
    pub valid: bool,
    pub signers: Vec<String>,
    pub timestamp: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

// timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampOutput {
    pub timestamp: DateTime<Utc>,
    pub timestamp_token: String,
    pub success: bool,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureMetadata {
    pub signer_name: Option<String>,
    pub signer_email: Option<String>,
    pub description: Option<String>,
    pub custom_attributes: Option<Value>,
}
