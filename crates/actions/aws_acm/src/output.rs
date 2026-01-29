//! Harana Actions - AWS ACM Module Output Types
//!
//! Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Output for request_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCertificateOutput {
    /// The ARN of the newly requested certificate
    pub certificate_arn: String,
    /// Domain validation options for the certificate
    pub domain_validation_options: Vec<DomainValidationOption>,
    /// Whether the operation succeeded
    pub success: bool,
}

/// Output for import_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportCertificateOutput {
    /// The ARN of the imported certificate
    pub certificate_arn: String,
    /// Whether the operation succeeded
    pub success: bool,
}

/// Output for delete_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCertificateOutput {
    /// Whether the operation succeeded
    pub success: bool,
}

/// Output for describe_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeCertificateOutput {
    /// Detailed certificate information
    pub certificate: Certificate,
}

/// Output for list_certificates action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCertificatesOutput {
    /// List of certificate summaries
    pub certificates: Vec<CertificateSummary>,
    /// Token for pagination
    pub next_token: Option<String>,
}

/// Output for get_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCertificateOutput {
    /// The certificate in PEM format
    pub certificate: String,
    /// The certificate chain in PEM format
    pub certificate_chain: Option<String>,
}

/// Output for add_tags action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTagsOutput {
    /// Whether the operation succeeded
    pub success: bool,
}

/// Output for remove_tags action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveTagsOutput {
    /// Whether the operation succeeded
    pub success: bool,
}

/// Output for list_tags action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTagsOutput {
    /// Tags associated with the certificate
    pub tags: AcmTags,
}

/// Output for renew_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenewCertificateOutput {
    /// Whether the operation succeeded
    pub success: bool,
}

/// Output for export_certificate action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportCertificateOutput {
    /// The certificate in PEM format
    pub certificate: String,
    /// The certificate chain in PEM format
    pub certificate_chain: Option<String>,
    /// The encrypted private key in PEM format
    pub private_key: String,
}

/// Output for resend_validation_email action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendValidationEmailOutput {
    /// Whether the operation succeeded
    pub success: bool,
}

/// Detailed certificate information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    /// The ARN of the certificate
    pub certificate_arn: String,
    /// The fully qualified domain name
    pub domain_name: String,
    /// Subject alternative names
    pub subject_alternative_names: Vec<String>,
    /// Certificate status
    pub status: String,
    /// Type of certificate (AMAZON_ISSUED, IMPORTED, PRIVATE)
    pub certificate_type: String,
    /// Certificate issuer
    pub issuer: Option<String>,
    /// When the certificate was created
    pub created_at: Option<String>,
    /// When the certificate was issued
    pub issued_at: Option<String>,
    /// Start of certificate validity
    pub not_before: Option<String>,
    /// End of certificate validity
    pub not_after: Option<String>,
    /// Key algorithm used
    pub key_algorithm: Option<String>,
    /// Signature algorithm used
    pub signature_algorithm: Option<String>,
    /// AWS resources using this certificate
    pub in_use_by: Vec<String>,
    /// Domain validation options
    pub domain_validation_options: Vec<DomainValidationOption>,
    /// Renewal eligibility status
    pub renewal_eligibility: Option<String>,
}

/// Summary information about a certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateSummary {
    /// The ARN of the certificate
    pub certificate_arn: String,
    /// The fully qualified domain name
    pub domain_name: String,
    /// Certificate status
    pub status: Option<String>,
    /// Type of certificate
    pub certificate_type: Option<String>,
    /// Key algorithm used
    pub key_algorithm: Option<String>,
    /// Whether the certificate is in use
    pub in_use: Option<bool>,
}

/// Domain validation option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainValidationOption {
    /// The domain name to validate
    pub domain_name: String,
    /// The validation domain
    pub validation_domain: Option<String>,
    /// Validation status
    pub validation_status: Option<String>,
    /// Validation method (DNS or EMAIL)
    pub validation_method: Option<String>,
    /// DNS resource record for DNS validation
    pub resource_record: Option<ResourceRecord>,
}

/// DNS resource record for domain validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRecord {
    /// DNS record name
    pub name: String,
    /// DNS record type (CNAME)
    pub record_type: String,
    /// DNS record value
    pub value: String,
}

/// Tags for ACM certificates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcmTags {
    /// Map of tag key-value pairs
    pub tags: HashMap<String, String>,
}

/// Filters for listing certificates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateFilters {
    /// Filter by extended key usage
    pub extended_key_usage: Option<Vec<String>>,
    /// Filter by key usage
    pub key_usage: Option<Vec<String>>,
    /// Filter by key types
    pub key_types: Option<Vec<String>>,
}
