// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSesIdentity {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub dkim_enabled: bool,
    #[serde(default)]
    pub feedback_forwarding_enabled: bool,
    pub identity: String,
    pub identity_type: String,
    pub region: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub verification_status: String,
    pub verification_token: Option<String>,
}

impl AwsSesIdentity {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSesTemplate {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub html_part: Option<String>,
    pub region: Option<String>,
    pub subject_part: String,
    pub template_name: String,
    pub text_part: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsSesTemplate {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSesConfigurationSet {
    pub account_id: String,
    pub configuration_set_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub region: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsSesConfigurationSet {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSesSendLog {
    pub bcc_addresses: Option<String>,
    pub cc_addresses: Option<String>,
    pub configuration_set_name: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub from_address: String,
    pub identity_id: Option<String>,
    pub message_id: String,
    pub status: String,
    pub subject: Option<String>,
    pub template_name: Option<String>,
    pub to_addresses: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AwsSesSendLog {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

