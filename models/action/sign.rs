// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignArtifactInput {
    pub algorithm: String,
    pub artifact: String,
    pub artifact_path: String,
    pub format: String,
    pub private_key: String,
    pub metadata: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignArtifactOutput {
    pub signature: String,
    pub signature_format: String,
    pub algorithm: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyArtifactInput {
    pub algorithm: String,
    pub artifact: String,
    pub artifact_path: String,
    pub format: String,
    pub public_key: String,
    pub signature: String,
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
pub struct SignCodeInput {
    pub artifact: String,
    pub artifact_path: String,
    pub certificate: String,
    pub certificate_chain: Vec<String>,
    pub description: String,
    pub format: String,
    pub private_key: String,
    pub timestamp_authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignCodeOutput {
    pub signed_artifact: String,
    pub signature: String,
    pub success: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDetachedInput {
    pub algorithm: String,
    pub artifact: String,
    pub format: String,
    pub private_key: String,
    #[serde(default)]
    pub armor: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDetachedOutput {
    pub signature: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyDetachedInput {
    pub algorithm: String,
    pub artifact: String,
    pub format: String,
    pub public_key: String,
    pub signature: String,
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
pub struct SignContainerInput {
    pub digest: String,
    pub format: String,
    pub image: String,
    pub private_key: String,
    pub registry: String,
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignContainerOutput {
    pub signature: String,
    pub signature_digest: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyContainerInput {
    pub format: String,
    pub image: String,
    pub public_key: String,
    pub registry: String,
    pub signature: String,
    pub tag: String,
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
pub struct SignJarInput {
    pub alias: String,
    pub jar_path: String,
    pub keystore: String,
    pub keystore_password: String,
    pub private_key_password: String,
    pub signature_algorithm: String,
    pub timestamp_authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignJarOutput {
    pub signed_jar_path: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyJarInput {
    pub jar_path: String,
    pub certificate: String,
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
pub struct TimestampInput {
    pub authority_url: String,
    pub digest: String,
    pub hash_algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimestampOutput {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub timestamp_token: String,
    pub success: bool,
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

#[async_trait]
pub trait SignAction {
    async fn sign_artifact(&self, input: SignArtifactInput) -> Result<SignArtifactOutput, Box<dyn std::error::Error>>;
    async fn verify_artifact(&self, input: VerifyArtifactInput) -> Result<VerifyArtifactOutput, Box<dyn std::error::Error>>;
    async fn sign_code(&self, input: SignCodeInput) -> Result<SignCodeOutput, Box<dyn std::error::Error>>;
    async fn create_detached(&self, input: CreateDetachedInput) -> Result<CreateDetachedOutput, Box<dyn std::error::Error>>;
    async fn verify_detached(&self, input: VerifyDetachedInput) -> Result<VerifyDetachedOutput, Box<dyn std::error::Error>>;
    async fn sign_container(&self, input: SignContainerInput) -> Result<SignContainerOutput, Box<dyn std::error::Error>>;
    async fn verify_container(&self, input: VerifyContainerInput) -> Result<VerifyContainerOutput, Box<dyn std::error::Error>>;
    async fn sign_jar(&self, input: SignJarInput) -> Result<SignJarOutput, Box<dyn std::error::Error>>;
    async fn verify_jar(&self, input: VerifyJarInput) -> Result<VerifyJarOutput, Box<dyn std::error::Error>>;
    async fn timestamp(&self, input: TimestampInput) -> Result<TimestampOutput, Box<dyn std::error::Error>>;
}
