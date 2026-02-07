// Harana Actions - Crypto Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};

// decrypt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptOutput {
    pub plaintext: String,
}

// encrypt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptOutput {
    pub ciphertext: String,
    pub iv: String,
    pub auth_tag: String,
}

// generate_key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateKeyOutput {
    pub key: String,
}

// generate_keypair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateKeypairOutput {
    pub private_key: String,
    pub public_key: String,
}

// hash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashOutput {
    pub digest: String,
}

// hmac
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HmacOutput {
    pub digest: String,
}

// sign
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignOutput {
    pub signature: String,
}

// verify
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyOutput {
    pub valid: bool,
}
