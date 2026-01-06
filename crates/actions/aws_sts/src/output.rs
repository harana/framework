// Harana Actions - AWS STS Module Output Types
// Auto-generated output structs for AWS STS action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// assume_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumeRoleOutput {
    pub access_key_id: String,
    pub assumed_role_user: HashMap<String, Value>,
    pub credentials: HashMap<String, Value>,
    pub expiration: String,
    pub packed_policy_size: i32,
    pub secret_access_key: String,
    pub session_token: String,
    pub source_identity: String,
    pub success: bool,
}

// get_caller_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCallerIdentityOutput {
    pub account: String,
    pub arn: String,
    pub user_id: String,
}


// assume_role_with_saml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumeRoleWithSamlOutput {
    pub subject: String,
    pub success: bool,
    pub access_key_id: String,
    pub assumed_role_user: HashMap<String, Value>,
    pub credentials: HashMap<String, Value>,
    pub audience: String,
    pub session_token: String,
    pub issuer: String,
    pub subject_type: String,
    pub expiration: String,
    pub name_qualifier: String,
    pub packed_policy_size: i32,
    pub secret_access_key: String
}

// assume_role_with_web_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumeRoleWithWebIdentityOutput {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub success: bool,
    pub expiration: String,
    pub audience: String,
    pub assumed_role_user: HashMap<String, Value>,
    pub subject_from_web_identity_token: String,
    pub packed_policy_size: i32,
    pub provider: String,
    pub credentials: HashMap<String, Value>,
    pub source_identity: String
}

// decode_authorization_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodeAuthorizationMessageOutput {
    pub decoded_message: String,
    pub success: bool
}

// get_session_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSessionTokenOutput {
    pub secret_access_key: String,
    pub expiration: String,
    pub credentials: HashMap<String, Value>,
    pub success: bool,
    pub access_key_id: String,
    pub session_token: String
}

// get_federation_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFederationTokenOutput {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub expiration: String,
    pub federated_user: HashMap<String, Value>,
    pub packed_policy_size: i32,
    pub session_token: String,
    pub credentials: HashMap<String, Value>,
    pub success: bool
}

// get_access_key_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccessKeyInfoOutput {
    pub success: bool,
    pub account: String
}
// TODO: Add remaining output types - see core/schema/actions/aws_sts.yml
