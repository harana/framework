// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestCertificateOutput {
    pub certificate_arn: String,
    pub domain_validation_options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCertificatesOutput {
    pub certificates: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCertificateOutput {
    pub certificate: String,
    pub certificate_chain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExportCertificateOutput {
    pub certificate: String,
    pub certificate_chain: String,
    pub private_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsAcmCertificate {
    pub account_id: String,
    pub certificate_arn: String,
    pub certificate_authority_arn: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub domain_name: String,
    pub failure_reason: String,
    pub imported_at: chrono::DateTime<chrono::Utc>,
    pub in_use_by: String,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub issuer: String,
    pub key_algorithm: String,
    pub not_after: chrono::DateTime<chrono::Utc>,
    pub not_before: chrono::DateTime<chrono::Utc>,
    pub region: String,
    pub renewal_eligibility: String,
    pub revocation_reason: String,
    pub revoked_at: chrono::DateTime<chrono::Utc>,
    pub serial: String,
    pub status: String,
    pub subject_alternative_names: String,
    pub tags: String,
    pub type: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub validation_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsAcmDomainValidation {
    pub certificate_id: String,
    pub domain_name: String,
    pub resource_record_name: String,
    pub resource_record_type: String,
    pub resource_record_value: String,
    pub validation_domain: String,
    pub validation_method: String,
    pub validation_status: String,
}

#[async_trait]
pub trait AcmAction {
    async fn request_certificate(&self, domain_name: String, subject_alternative_names: Vec<String>, validation_method: String, certificate_authority_arn: String, domain_validation_options: Vec<String>, tags: String) -> Result<RequestCertificateOutput, Box<dyn std::error::Error>>;
    async fn import_certificate(&self, certificate: String, certificate_chain: String, private_key: String, certificate_arn: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_certificate(&self, certificate_arn: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn describe_certificate(&self, certificate_arn: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn list_certificates(&self, certificate_statuses: Vec<String>, includes: String, max_items: i64, next_token: String) -> Result<ListCertificatesOutput, Box<dyn std::error::Error>>;
    async fn get_certificate(&self, certificate_arn: String) -> Result<GetCertificateOutput, Box<dyn std::error::Error>>;
    async fn add_tags(&self, certificate_arn: String, tags: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_tags(&self, certificate_arn: String, tag_keys: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_tags(&self, certificate_arn: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn renew_certificate(&self, certificate_arn: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn export_certificate(&self, certificate_arn: String, passphrase: String) -> Result<ExportCertificateOutput, Box<dyn std::error::Error>>;
    async fn resend_validation_email(&self, certificate_arn: String, domain: String, validation_domain: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn update_certificate_options(&self, certificate_arn: String, options: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_account_configuration(&self, expiry_events: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn put_account_configuration(&self, expiry_events: String, idempotency_token: String) -> Result<(), Box<dyn std::error::Error>>;
}
