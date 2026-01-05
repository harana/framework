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

// TODO: Add remaining output types - see core/schema/actions/aws_sts.yml
