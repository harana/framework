// Harana Actions - Aws Sts Module
// This module provides aws sts actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Assume STS Role
pub async fn assume_role(
    role_session_name: &str,
    role_arn: &str,
    policy: Option<&str>,
    serial_number: Option<&str>,
    source_identity: Option<&str>,
    tags: Option<HashMap<String, Value>>,
    external_id: Option<&str>,
    policy_arns: Option<Vec<HashMap<String, Value>>>,
    transitive_tag_keys: Option<Vec<String>>,
    token_code: Option<&str>,
    duration_seconds: Option<i32>,
    region: Option<&str>,
) -> Result<AssumeRoleOutput, String> {
    unimplemented!("assume_role")
}

/// Assume STS Role With SAML
pub async fn assume_role_with_saml(
    role_arn: &str,
    saml_assertion: &str,
    principal_arn: &str,
    region: Option<&str>,
    policy_arns: Option<Vec<HashMap<String, Value>>>,
    duration_seconds: Option<i32>,
    policy: Option<&str>,
) -> Result<AssumeRoleWithSamlOutput, String> {
    unimplemented!("assume_role_with_saml")
}

/// Assume STS Role With Web Identity
pub async fn assume_role_with_web_identity(
    role_session_name: &str,
    role_arn: &str,
    web_identity_token: &str,
    duration_seconds: Option<i32>,
    policy_arns: Option<Vec<HashMap<String, Value>>>,
    provider_id: Option<&str>,
    policy: Option<&str>,
    region: Option<&str>,
) -> Result<AssumeRoleWithWebIdentityOutput, String> {
    unimplemented!("assume_role_with_web_identity")
}

/// Decode STS Authorization Message
pub async fn decode_authorization_message(
    encoded_message: &str,
    region: Option<&str>,
) -> Result<DecodeAuthorizationMessageOutput, String> {
    unimplemented!("decode_authorization_message")
}

/// Get STS Access Key Info
pub async fn get_access_key_info(
    access_key_id: &str,
    region: Option<&str>,
) -> Result<GetAccessKeyInfoOutput, String> {
    unimplemented!("get_access_key_info")
}

/// Get STS Caller Identity
pub async fn get_caller_identity(
    region: Option<&str>,
) -> Result<GetCallerIdentityOutput, String> {
    unimplemented!("get_caller_identity")
}

/// Get STS Federation Token
pub async fn get_federation_token(
    name: &str,
    tags: Option<HashMap<String, Value>>,
    policy: Option<&str>,
    region: Option<&str>,
    policy_arns: Option<Vec<HashMap<String, Value>>>,
    duration_seconds: Option<i32>,
) -> Result<GetFederationTokenOutput, String> {
    unimplemented!("get_federation_token")
}

/// Get STS Session Token
pub async fn get_session_token(
    token_code: Option<&str>,
    serial_number: Option<&str>,
    region: Option<&str>,
    duration_seconds: Option<i32>,
) -> Result<GetSessionTokenOutput, String> {
    unimplemented!("get_session_token")
}
