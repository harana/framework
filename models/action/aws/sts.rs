// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssumeRoleInput {
    pub duration_seconds: i64,
    pub external_id: String,
    pub policy: String,
    pub policy_arns: Vec<String>,
    pub region: String,
    pub role_arn: String,
    pub role_session_name: String,
    pub serial_number: String,
    pub source_identity: String,
    pub tags: String,
    pub token_code: String,
    pub transitive_tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssumeRoleOutput {
    pub access_key_id: String,
    pub assumed_role_user: String,
    pub credentials: String,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub packed_policy_size: i64,
    pub secret_access_key: String,
    pub session_token: String,
    pub source_identity: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssumeRoleWithSamlInput {
    pub duration_seconds: i64,
    pub policy: String,
    pub policy_arns: Vec<String>,
    pub principal_arn: String,
    pub region: String,
    pub role_arn: String,
    pub saml_assertion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssumeRoleWithSamlOutput {
    pub access_key_id: String,
    pub assumed_role_user: String,
    pub audience: String,
    pub credentials: String,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub issuer: String,
    pub name_qualifier: String,
    pub packed_policy_size: i64,
    pub secret_access_key: String,
    pub session_token: String,
    pub subject: String,
    pub subject_type: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssumeRoleWithWebIdentityInput {
    pub duration_seconds: i64,
    pub policy: String,
    pub policy_arns: Vec<String>,
    pub provider_id: String,
    pub region: String,
    pub role_arn: String,
    pub role_session_name: String,
    pub web_identity_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AssumeRoleWithWebIdentityOutput {
    pub access_key_id: String,
    pub assumed_role_user: String,
    pub audience: String,
    pub credentials: String,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub packed_policy_size: i64,
    pub provider: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub source_identity: String,
    pub subject_from_web_identity_token: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecodeAuthorizationMessageInput {
    pub encoded_message: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DecodeAuthorizationMessageOutput {
    pub decoded_message: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCallerIdentityInput {
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCallerIdentityOutput {
    pub account: String,
    pub arn: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSessionTokenInput {
    pub duration_seconds: i64,
    pub region: String,
    pub serial_number: String,
    pub token_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSessionTokenOutput {
    pub access_key_id: String,
    pub credentials: String,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub secret_access_key: String,
    pub session_token: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFederationTokenInput {
    pub duration_seconds: i64,
    pub name: String,
    pub policy: String,
    pub policy_arns: Vec<String>,
    pub region: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetFederationTokenOutput {
    pub access_key_id: String,
    pub credentials: String,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub federated_user: String,
    pub packed_policy_size: i64,
    pub secret_access_key: String,
    pub session_token: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAccessKeyInfoInput {
    pub access_key_id: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAccessKeyInfoOutput {
    pub account: String,
    pub success: bool,
}

#[async_trait]
pub trait StsAction {
    async fn assume_role(&self, input: AssumeRoleInput) -> Result<AssumeRoleOutput, Box<dyn std::error::Error>>;
    async fn assume_role_with_saml(&self, input: AssumeRoleWithSamlInput) -> Result<AssumeRoleWithSamlOutput, Box<dyn std::error::Error>>;
    async fn assume_role_with_web_identity(&self, input: AssumeRoleWithWebIdentityInput) -> Result<AssumeRoleWithWebIdentityOutput, Box<dyn std::error::Error>>;
    async fn decode_authorization_message(&self, input: DecodeAuthorizationMessageInput) -> Result<DecodeAuthorizationMessageOutput, Box<dyn std::error::Error>>;
    async fn get_caller_identity(&self, input: GetCallerIdentityInput) -> Result<GetCallerIdentityOutput, Box<dyn std::error::Error>>;
    async fn get_session_token(&self, input: GetSessionTokenInput) -> Result<GetSessionTokenOutput, Box<dyn std::error::Error>>;
    async fn get_federation_token(&self, input: GetFederationTokenInput) -> Result<GetFederationTokenOutput, Box<dyn std::error::Error>>;
    async fn get_access_key_info(&self, input: GetAccessKeyInfoInput) -> Result<GetAccessKeyInfoOutput, Box<dyn std::error::Error>>;
}
