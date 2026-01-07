// Harana Actions - Aws Sts Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// assume_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumeRoleOutput {
    pub credentials: HashMap<String, Value>,
    pub secret_access_key: String,
    pub packed_policy_size: i32,
    pub expiration: String,
    pub success: bool,
    pub assumed_role_user: HashMap<String, Value>,
    pub session_token: String,
    pub access_key_id: String,
    pub source_identity: String
}

// assume_role_with_saml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumeRoleWithSamlOutput {
    pub subject_type: String,
    pub audience: String,
    pub expiration: String,
    pub access_key_id: String,
    pub issuer: String,
    pub assumed_role_user: HashMap<String, Value>,
    pub packed_policy_size: i32,
    pub secret_access_key: String,
    pub subject: String,
    pub name_qualifier: String,
    pub session_token: String,
    pub credentials: HashMap<String, Value>,
    pub success: bool
}

// assume_role_with_web_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumeRoleWithWebIdentityOutput {
    pub assumed_role_user: HashMap<String, Value>,
    pub audience: String,
    pub access_key_id: String,
    pub packed_policy_size: i32,
    pub provider: String,
    pub source_identity: String,
    pub expiration: String,
    pub session_token: String,
    pub credentials: HashMap<String, Value>,
    pub secret_access_key: String,
    pub subject_from_web_identity_token: String,
    pub success: bool
}

// decode_authorization_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodeAuthorizationMessageOutput {
    pub success: bool,
    pub decoded_message: String
}

// get_access_key_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccessKeyInfoOutput {
    pub account: String,
    pub success: bool
}

// get_caller_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCallerIdentityOutput {
    pub user_id: String,
    pub arn: String,
    pub account: String
}

// get_federation_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFederationTokenOutput {
    pub expiration: String,
    pub federated_user: HashMap<String, Value>,
    pub credentials: HashMap<String, Value>,
    pub packed_policy_size: i32,
    pub access_key_id: String,
    pub session_token: String,
    pub success: bool,
    pub secret_access_key: String
}

// get_session_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSessionTokenOutput {
    pub access_key_id: String,
    pub credentials: HashMap<String, Value>,
    pub secret_access_key: String,
    pub session_token: String,
    pub expiration: String,
    pub success: bool
}
