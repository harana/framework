// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// aws_secrets_manager_secret
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSecretsManagerSecret {
    pub account_id: String,
    pub arn: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub description: Option<String>,
    pub kms_key_id: Option<String>,
    pub last_changed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_rotated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub name: String,
    pub region: Option<String>,
    #[serde(default)]
    pub rotation_enabled: bool,
    pub rotation_lambda_arn: Option<String>,
    pub rotation_rules: Option<String>,
    pub tags: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsSecretsManagerSecret {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_secrets_manager_secret_version
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSecretsManagerSecretVersion {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: aws_secrets_manager_secret.id
    pub secret_id: String,
    pub version_id: String,
    pub version_stages: Option<String>,
}

impl AwsSecretsManagerSecretVersion {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_secrets_manager_secret_replication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSecretsManagerSecretReplication {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub kms_key_id: Option<String>,
    pub last_accessed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub region: String,
    /// Reference: aws_secrets_manager_secret.id
    pub secret_id: String,
    pub status: String,
    pub status_message: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsSecretsManagerSecretReplication {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// aws_secrets_manager_access_log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSecretsManagerAccessLog {
    #[serde(default = "chrono::Utc::now")]
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    pub action: String,
    pub ip_address: Option<String>,
    /// Reference: aws_secrets_manager_secret.id
    pub secret_id: String,
    #[serde(default)]
    pub success: bool,
    pub user_arn: Option<String>,
}

impl AwsSecretsManagerAccessLog {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

