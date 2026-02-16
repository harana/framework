// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignArtifactOutput {
    pub signature: String,
    pub signature_format: String,
    pub algorithm: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyArtifactOutput {
    pub valid: bool,
    pub signer: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub algorithm: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignCodeOutput {
    pub signed_artifact: String,
    pub signature: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyDetachedOutput {
    pub valid: bool,
    pub signer: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignContainerOutput {
    pub signature: String,
    pub signature_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyContainerOutput {
    pub valid: bool,
    pub signer: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyJarOutput {
    pub valid: bool,
    pub signers: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimestampOutput {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub timestamp_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignatureMetadata {
    pub signer: String,
    pub organization: String,
    pub email: String,
    pub description: String,
    pub url: String,
    pub custom_attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignedArtifact {
    pub artifact: String,
    pub signature: String,
    pub algorithm: String,
    pub format: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub signer: String,
    pub certificate_chain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SigningKey {
    pub algorithm: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    pub key_id: String,
    pub public_key_fingerprint: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Signature {
    pub algorithm: String,
    pub artifact_hash: String,
    pub artifact_path: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub format: String,
    #[serde(default)]
    pub is_valid: bool,
    pub key_id: String,
    pub signer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignatureVerificationLog {
    pub error_message: String,
    pub ip_address: String,
    #[serde(default)]
    pub is_valid: bool,
    pub signature_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub verified_at: chrono::DateTime<chrono::Utc>,
    pub verified_by: String,
}

#[async_trait]
pub trait SignAction {
    async fn sign_artifact(&self, algorithm: String, artifact: String, artifact_path: String, format: String, private_key: String, metadata: String) -> Result<SignArtifactOutput, Box<dyn std::error::Error>>;
    async fn verify_artifact(&self, algorithm: String, artifact: String, artifact_path: String, format: String, public_key: String, signature: String) -> Result<VerifyArtifactOutput, Box<dyn std::error::Error>>;
    async fn sign_code(&self, artifact: String, artifact_path: String, certificate: String, certificate_chain: Vec<String>, description: String, format: String, private_key: String, timestamp_authority: String) -> Result<SignCodeOutput, Box<dyn std::error::Error>>;
    async fn create_detached(&self, algorithm: String, artifact: String, format: String, private_key: String, armor: bool) -> Result<String, Box<dyn std::error::Error>>;
    async fn verify_detached(&self, algorithm: String, artifact: String, format: String, public_key: String, signature: String) -> Result<VerifyDetachedOutput, Box<dyn std::error::Error>>;
    async fn sign_container(&self, digest: String, format: String, image: String, private_key: String, registry: String, tag: String) -> Result<SignContainerOutput, Box<dyn std::error::Error>>;
    async fn verify_container(&self, format: String, image: String, public_key: String, registry: String, signature: String, tag: String) -> Result<VerifyContainerOutput, Box<dyn std::error::Error>>;
    async fn sign_jar(&self, alias: String, jar_path: String, keystore: String, keystore_password: String, private_key_password: String, signature_algorithm: String, timestamp_authority: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn verify_jar(&self, jar_path: String, certificate: String) -> Result<VerifyJarOutput, Box<dyn std::error::Error>>;
    async fn timestamp(&self, authority_url: String, digest: String, hash_algorithm: String) -> Result<TimestampOutput, Box<dyn std::error::Error>>;
}
