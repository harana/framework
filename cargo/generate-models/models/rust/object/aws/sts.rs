// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_sts_assumed_role_session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsStsAssumedRoleSession {
    pub access_key_id: Option<String>,
    pub account_id: String,
    pub assumed_role_arn: Option<String>,
    pub assumed_role_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub method: String,
    pub packed_policy_size: Option<i64>,
    pub region: Option<String>,
    pub role_arn: String,
    pub role_session_name: String,
    pub source_identity: Option<String>,
}

impl AwsStsAssumedRoleSession {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_sts_federation_token
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsStsFederationToken {
    pub access_key_id: Option<String>,
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expiration: chrono::DateTime<chrono::Utc>,
    pub federated_user_arn: String,
    pub federated_user_id: Option<String>,
    pub name: String,
    pub packed_policy_size: Option<i64>,
    pub region: Option<String>,
}

impl AwsStsFederationToken {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

