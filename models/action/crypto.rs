// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HashInput {
    pub algorithm: String,
    pub data: String,
    pub encoding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HashOutput {
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HmacInput {
    pub algorithm: String,
    pub data: String,
    pub encoding: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HmacOutput {
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EncryptInput {
    pub algorithm: String,
    pub data: String,
    pub iv: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EncryptOutput {
    pub auth_tag: String,
    pub ciphertext: String,
    pub iv: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecryptInput {
    pub algorithm: String,
    pub auth_tag: String,
    pub ciphertext: String,
    pub iv: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecryptOutput {
    pub plaintext: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateKeyInput {
    pub encoding: String,
    pub length: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateKeyOutput {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignInput {
    pub algorithm: String,
    pub data: String,
    pub private_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SignOutput {
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyInput {
    pub algorithm: String,
    pub data: String,
    pub public_key: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyOutput {
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateKeypairInput {
    pub algorithm: String,
    pub key_size: i64,
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

#[async_trait]
pub trait CryptoAction {
    async fn hash(&self, input: HashInput) -> Result<HashOutput, Box<dyn std::error::Error>>;
    async fn hmac(&self, input: HmacInput) -> Result<HmacOutput, Box<dyn std::error::Error>>;
    async fn encrypt(&self, input: EncryptInput) -> Result<EncryptOutput, Box<dyn std::error::Error>>;
    async fn decrypt(&self, input: DecryptInput) -> Result<DecryptOutput, Box<dyn std::error::Error>>;
    async fn generate_key(&self, input: GenerateKeyInput) -> Result<GenerateKeyOutput, Box<dyn std::error::Error>>;
    async fn sign(&self, input: SignInput) -> Result<SignOutput, Box<dyn std::error::Error>>;
    async fn verify(&self, input: VerifyInput) -> Result<VerifyOutput, Box<dyn std::error::Error>>;
    async fn generate_keypair(&self, input: GenerateKeypairInput) -> Result<GenerateKeypairOutput, Box<dyn std::error::Error>>;
}
