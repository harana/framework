// Harana Actions - Aws Sts Module Output Types
// Auto-generated output structs for action methods.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub expiration: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumedRoleUser {
    pub assumed_role_id: String,
    pub arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedUser {
    pub federated_user_id: String,
    pub arn: String,
}

// assume_role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumeRoleOutput {
    pub credentials: Option<Credentials>,
    pub assumed_role_user: Option<AssumedRoleUser>,
    pub packed_policy_size: Option<i32>,
    pub source_identity: Option<String>,
    pub success: bool,
}

// assume_role_with_saml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumeRoleWithSamlOutput {
    pub credentials: Option<Credentials>,
    pub assumed_role_user: Option<AssumedRoleUser>,
    pub packed_policy_size: Option<i32>,
    pub subject: Option<String>,
    pub subject_type: Option<String>,
    pub issuer: Option<String>,
    pub audience: Option<String>,
    pub name_qualifier: Option<String>,
    pub source_identity: Option<String>,
    pub success: bool,
}

// assume_role_with_web_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssumeRoleWithWebIdentityOutput {
    pub credentials: Option<Credentials>,
    pub assumed_role_user: Option<AssumedRoleUser>,
    pub packed_policy_size: Option<i32>,
    pub provider: Option<String>,
    pub audience: Option<String>,
    pub subject_from_web_identity_token: Option<String>,
    pub source_identity: Option<String>,
    pub success: bool,
}

// decode_authorization_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodeAuthorizationMessageOutput {
    pub decoded_message: Option<String>,
    pub success: bool,
}

// get_access_key_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccessKeyInfoOutput {
    pub account: Option<String>,
    pub success: bool,
}

// get_caller_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCallerIdentityOutput {
    pub user_id: Option<String>,
    pub account: Option<String>,
    pub arn: Option<String>,
    pub success: bool,
}

// get_federation_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFederationTokenOutput {
    pub credentials: Option<Credentials>,
    pub federated_user: Option<FederatedUser>,
    pub packed_policy_size: Option<i32>,
    pub success: bool,
}

// get_session_token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSessionTokenOutput {
    pub credentials: Option<Credentials>,
    pub success: bool,
}
