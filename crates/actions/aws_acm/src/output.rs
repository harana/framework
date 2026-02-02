//! Harana Actions - AWS ACM Module Output Types
//!
//! Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Output for request_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCertificateOutput {
    pub certificate_arn: String,
    pub domain_validation_options: Vec<DomainValidationOption>,
    pub success: bool,
}

/// Output for import_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportCertificateOutput {
    pub certificate_arn: String,
    pub success: bool,
}

/// Output for delete_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCertificateOutput {
    pub success: bool,
}

/// Output for describe_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeCertificateOutput {
    pub certificate: Certificate,
}

/// Output for list_certificates action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCertificatesOutput {
    pub certificates: Vec<CertificateSummary>,
    pub next_token: Option<String>,
}

/// Output for get_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCertificateOutput {
    pub certificate: String,
    pub certificate_chain: Option<String>,
}

/// Output for add_tags action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTagsOutput {
    pub success: bool,
}

/// Output for remove_tags action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveTagsOutput {
    pub success: bool,
}

/// Output for list_tags action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTagsOutput {
    pub tags: AcmTags,
}

/// Output for renew_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenewCertificateOutput {
    pub success: bool,
}

/// Output for export_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportCertificateOutput {
    pub certificate: String,
    pub certificate_chain: Option<String>,
    pub private_key: String,
}

/// Output for resend_validation_email action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendValidationEmailOutput {
    pub success: bool,
}

/// Output for update_certificate_options action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCertificateOptionsOutput {
    pub success: bool,
}

/// Output for get_account_configuration action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountConfigurationOutput {
    pub configuration: AccountConfiguration,
}

/// Output for put_account_configuration action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutAccountConfigurationOutput {
    pub success: bool,
}

/// Certificate options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateOptions {
    pub certificate_transparency_logging_preference: Option<String>,
}

/// Account configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfiguration {
    pub expiry_events: Option<ExpiryEventsConfiguration>,
}

/// Expiry events configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiryEventsConfiguration {
    pub days_before_expiry: Option<i32>,
}

/// Detailed certificate information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub certificate_arn: String,
    pub domain_name: String,
    pub subject_alternative_names: Vec<String>,
    pub status: String,
    pub certificate_type: String,
    pub issuer: Option<String>,
    pub created_at: Option<String>,
    pub issued_at: Option<String>,
    pub not_before: Option<String>,
    pub not_after: Option<String>,
    pub key_algorithm: Option<String>,
    pub signature_algorithm: Option<String>,
    pub in_use_by: Vec<String>,
    pub domain_validation_options: Vec<DomainValidationOption>,
    pub renewal_eligibility: Option<String>,
}

/// Summary information about a certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateSummary {
    pub certificate_arn: String,
    pub domain_name: String,
    pub status: Option<String>,
    pub certificate_type: Option<String>,
    pub key_algorithm: Option<String>,
    pub in_use: Option<bool>,
}

/// Domain validation option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainValidationOption {
    pub domain_name: String,
    pub validation_domain: Option<String>,
    pub validation_status: Option<String>,
    pub validation_method: Option<String>,
    pub resource_record: Option<ResourceRecord>,
}

/// DNS resource record for domain validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRecord {
    pub name: String,
    pub record_type: String,
    pub value: String,
}

/// Tags for ACM certificates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcmTags {
    pub tags: HashMap<String, String>,
}

/// Filters for listing certificates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateFilters {
    pub extended_key_usage: Option<Vec<String>>,
    pub key_usage: Option<Vec<String>>,
    pub key_types: Option<Vec<String>>,
}
