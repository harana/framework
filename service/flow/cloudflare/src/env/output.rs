// Harana Actions - Cloudflare Environment Module Output Types

use serde::{Deserialize, Serialize};

// get_var
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVarOutput {
    pub found: bool,
    pub value: String,
}

// get_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSecretOutput {
    pub value: String,
}

// get_store_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStoreSecretOutput {
    pub value: String,
}

// list_vars
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListVarsOutput {
    pub variables: Vec<EnvVariable>,
}

// get_version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVersionOutput {
    pub id: String,
    pub message: String,
    pub tag: String,
    pub timestamp: String,
}

// Helper structs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVariable {
    pub name: String,
    pub value: String,
}
