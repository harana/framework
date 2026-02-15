// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsAcmCertificate {
    pub account_id: String,
    pub certificate_arn: String,
    pub certificate_authority_arn: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub domain_name: String,
    pub failure_reason: Option<String>,
    pub imported_at: Option<chrono::DateTime<chrono::Utc>>,
    pub in_use_by: Option<String>,
    pub issued_at: Option<chrono::DateTime<chrono::Utc>>,
    pub issuer: Option<String>,
    pub key_algorithm: String,
    pub not_after: Option<chrono::DateTime<chrono::Utc>>,
    pub not_before: Option<chrono::DateTime<chrono::Utc>>,
    pub region: Option<String>,
    pub renewal_eligibility: String,
    pub revocation_reason: Option<String>,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
    pub serial: Option<String>,
    pub status: String,
    pub subject_alternative_names: Option<String>,
    pub tags: Option<String>,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub validation_method: String,
}

impl AwsAcmCertificate {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsAcmDomainValidation {
    pub certificate_id: String,
    pub domain_name: String,
    pub resource_record_name: Option<String>,
    pub resource_record_type: Option<String>,
    pub resource_record_value: Option<String>,
    pub validation_domain: Option<String>,
    pub validation_method: String,
    pub validation_status: String,
}

impl AwsAcmDomainValidation {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

