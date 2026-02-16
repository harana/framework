// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EncryptOutput {
    pub auth_tag: String,
    pub ciphertext: String,
    pub iv: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateKeypairOutput {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeyPair {
    pub algorithm: String,
    pub private_key: String,
    pub public_key: String,
    pub key_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EncryptionKey {
    pub algorithm: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub is_active: bool,
    pub key_id: String,
    pub key_size: i64,
    pub key_type: String,
    pub last_rotated_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CryptoOperationLog {
    pub algorithm: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub key_id: String,
    pub operation: String,
    pub status: String,
    pub user_id: String,
}

#[async_trait]
pub trait CryptoAction {
    async fn hash(&self, algorithm: String, data: String, encoding: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn hmac(&self, algorithm: String, data: String, encoding: String, key: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn encrypt(&self, algorithm: String, data: String, iv: String, key: String) -> Result<EncryptOutput, Box<dyn std::error::Error>>;
    async fn decrypt(&self, algorithm: String, auth_tag: String, ciphertext: String, iv: String, key: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn generate_key(&self, encoding: String, length: i64) -> Result<String, Box<dyn std::error::Error>>;
    async fn sign(&self, algorithm: String, data: String, private_key: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn verify(&self, algorithm: String, data: String, public_key: String, signature: String) -> Result<bool, Box<dyn std::error::Error>>;
    async fn generate_keypair(&self, algorithm: String, key_size: i64) -> Result<GenerateKeypairOutput, Box<dyn std::error::Error>>;
}
