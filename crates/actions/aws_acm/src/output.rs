// Harana Actions - AWS ACM Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// request_certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCertificateOutput {
    pub certificate_arn: String,
    pub domain_validation_options: Vec<DomainValidationOption>,
    pub success: bool,
}

// import_certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportCertificateOutput {
    pub certificate_arn: String,
    pub success: bool,
}

// delete_certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCertificateOutput {
    pub success: bool,
}

// describe_certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeCertificateOutput {
    pub certificate: Certificate,
}

// list_certificates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCertificatesOutput {
    pub certificates: Vec<CertificateSummary>,
    pub next_token: Option<String>,
}

// get_certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCertificateOutput {
    pub certificate: String,
    pub certificate_chain: Option<String>,
}

// add_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTagsOutput {
    pub success: bool,
}

// remove_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveTagsOutput {
    pub success: bool,
}

// list_tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTagsOutput {
    pub tags: AcmTags,
}

// renew_certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenewCertificateOutput {
    pub success: bool,
}

// export_certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportCertificateOutput {
    pub certificate: String,
    pub certificate_chain: Option<String>,
    pub private_key: String,
}

// resend_validation_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendValidationEmailOutput {
    pub success: bool,
}

// Helper structs
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateSummary {
    pub certificate_arn: String,
    pub domain_name: String,
    pub status: Option<String>,
    pub certificate_type: Option<String>,
    pub key_algorithm: Option<String>,
    pub in_use: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainValidationOption {
    pub domain_name: String,
    pub validation_domain: Option<String>,
    pub validation_status: Option<String>,
    pub validation_method: Option<String>,
    pub resource_record: Option<ResourceRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRecord {
    pub name: String,
    pub record_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcmTags {
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateFilters {
    pub extended_key_usage: Option<Vec<String>>,
    pub key_usage: Option<Vec<String>>,
    pub key_types: Option<Vec<String>>,
}
