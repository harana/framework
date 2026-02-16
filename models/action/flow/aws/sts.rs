// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
pub struct GetSessionTokenOutput {
    pub access_key_id: String,
    pub credentials: String,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub secret_access_key: String,
    pub session_token: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsStsAssumedRoleSession {
    pub access_key_id: String,
    pub account_id: String,
    pub assumed_role_arn: String,
    pub assumed_role_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub method: String,
    pub packed_policy_size: i64,
    pub region: String,
    pub role_arn: String,
    pub role_session_name: String,
    pub source_identity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsStsFederationToken {
    pub access_key_id: String,
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub federated_user_arn: String,
    pub federated_user_id: String,
    pub name: String,
    pub packed_policy_size: i64,
    pub region: String,
}

#[async_trait]
pub trait StsAction {
    async fn assume_role(&self, duration_seconds: i64, external_id: String, policy: String, policy_arns: Vec<String>, region: String, role_arn: String, role_session_name: String, serial_number: String, source_identity: String, tags: String, token_code: String, transitive_tag_keys: Vec<String>) -> Result<AssumeRoleOutput, Box<dyn std::error::Error>>;
    async fn assume_role_with_saml(&self, duration_seconds: i64, policy: String, policy_arns: Vec<String>, principal_arn: String, region: String, role_arn: String, saml_assertion: String) -> Result<AssumeRoleWithSamlOutput, Box<dyn std::error::Error>>;
    async fn assume_role_with_web_identity(&self, duration_seconds: i64, policy: String, policy_arns: Vec<String>, provider_id: String, region: String, role_arn: String, role_session_name: String, web_identity_token: String) -> Result<AssumeRoleWithWebIdentityOutput, Box<dyn std::error::Error>>;
    async fn decode_authorization_message(&self, encoded_message: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_caller_identity(&self, region: String) -> Result<GetCallerIdentityOutput, Box<dyn std::error::Error>>;
    async fn get_session_token(&self, duration_seconds: i64, region: String, serial_number: String, token_code: String) -> Result<GetSessionTokenOutput, Box<dyn std::error::Error>>;
    async fn get_federation_token(&self, duration_seconds: i64, name: String, policy: String, policy_arns: Vec<String>, region: String, tags: String) -> Result<GetFederationTokenOutput, Box<dyn std::error::Error>>;
    async fn get_access_key_info(&self, access_key_id: String, region: String) -> Result<String, Box<dyn std::error::Error>>;
}
