// Harana Actions - AWS ACM Module
//
// This module provides AWS Certificate Manager (ACM) actions for managing
// SSL/TLS certificates.

pub mod output;

use aws_config::BehaviorVersion;
use aws_sdk_acm::{
    Client,
    config::Region,
    primitives::Blob,
    types::{
        CertificateStatus, DomainValidationOption as AwsDomainValidationOption, ExtendedKeyUsageName, Filters,
        KeyAlgorithm, KeyUsageName, Tag as AcmTag, ValidationMethod,
    },
};
use output::*;

/// Creates an ACM client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let acm_config = if let Some(region_str) = region {
        aws_sdk_acm::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_acm::config::Builder::from(&config).build()
    };

    Ok(Client::from_conf(acm_config))
}

/// Converts AcmTags to AWS Tag vector
fn tags_to_aws_tags(tags: Option<AcmTags>) -> Option<Vec<AcmTag>> {
    tags.map(|t| {
        t.tags
            .iter()
            .filter_map(|(k, v)| AcmTag::builder().key(k).value(v).build().ok())
            .collect()
    })
}

/// Converts AWS tags to output AcmTags format
fn convert_tags(tags: &[aws_sdk_acm::types::Tag]) -> AcmTags {
    let mut map = std::collections::HashMap::new();
    for tag in tags {
        let key = tag.key();
        let value = tag.value().unwrap_or_default();
        map.insert(key.to_string(), value.to_string());
    }
    AcmTags { tags: map }
}

/// Converts DomainValidationOption to AWS format
fn convert_domain_validation_options(
    options: Option<Vec<DomainValidationOption>>,
) -> Option<Vec<AwsDomainValidationOption>> {
    options.map(|opts| {
        opts.iter()
            .filter_map(|opt| {
                let mut builder = AwsDomainValidationOption::builder().domain_name(&opt.domain_name);

                if let Some(ref validation_domain) = opt.validation_domain {
                    builder = builder.validation_domain(validation_domain);
                }

                Some(builder.build().ok()?)
            })
            .collect()
    })
}

/// Converts AWS DomainValidation to output format
fn convert_domain_validation(validations: &[aws_sdk_acm::types::DomainValidation]) -> Vec<DomainValidationOption> {
    validations
        .iter()
        .map(|v| {
            let resource_record = v.resource_record().map(|rr| ResourceRecord {
                name: rr.name().to_string(),
                record_type: rr.r#type().as_str().to_string(),
                value: rr.value().to_string(),
            });

            DomainValidationOption {
                domain_name: v.domain_name().to_string(),
                validation_domain: v.validation_domain().map(|s| s.to_string()),
                validation_status: v.validation_status().map(|s| s.as_str().to_string()),
                validation_method: v.validation_method().map(|m| m.as_str().to_string()),
                resource_record,
            }
        })
        .collect()
}

/// Converts certificate statuses to AWS format
fn convert_statuses(statuses: Option<Vec<String>>) -> Option<Vec<CertificateStatus>> {
    statuses.map(|s| {
        s.iter()
            .filter_map(|status| match status.to_uppercase().as_str() {
                "PENDING_VALIDATION" => Some(CertificateStatus::PendingValidation),
                "ISSUED" => Some(CertificateStatus::Issued),
                "INACTIVE" => Some(CertificateStatus::Inactive),
                "EXPIRED" => Some(CertificateStatus::Expired),
                "VALIDATION_TIMED_OUT" => Some(CertificateStatus::ValidationTimedOut),
                "REVOKED" => Some(CertificateStatus::Revoked),
                "FAILED" => Some(CertificateStatus::Failed),
                _ => None,
            })
            .collect()
    })
}

/// Converts CertificateFilters to AWS Filters format
fn convert_filters(filters: Option<CertificateFilters>) -> Option<Filters> {
    filters.map(|f| {
        let mut builder = Filters::builder();

        if let Some(extended_key_usage) = f.extended_key_usage {
            for usage in extended_key_usage {
                if let Some(eku) = match usage.to_uppercase().as_str() {
                    "TLS_WEB_SERVER_AUTHENTICATION" => Some(ExtendedKeyUsageName::TlsWebServerAuthentication),
                    "TLS_WEB_CLIENT_AUTHENTICATION" => Some(ExtendedKeyUsageName::TlsWebClientAuthentication),
                    "CODE_SIGNING" => Some(ExtendedKeyUsageName::CodeSigning),
                    "EMAIL_PROTECTION" => Some(ExtendedKeyUsageName::EmailProtection),
                    "TIME_STAMPING" => Some(ExtendedKeyUsageName::TimeStamping),
                    "OCSP_SIGNING" => Some(ExtendedKeyUsageName::OcspSigning),
                    "IPSEC_END_SYSTEM" => Some(ExtendedKeyUsageName::IpsecEndSystem),
                    "IPSEC_TUNNEL" => Some(ExtendedKeyUsageName::IpsecTunnel),
                    "IPSEC_USER" => Some(ExtendedKeyUsageName::IpsecUser),
                    "ANY" => Some(ExtendedKeyUsageName::Any),
                    "NONE" => Some(ExtendedKeyUsageName::None),
                    "CUSTOM" => Some(ExtendedKeyUsageName::Custom),
                    _ => None,
                } {
                    builder = builder.extended_key_usage(eku);
                }
            }
        }

        if let Some(key_usage) = f.key_usage {
            for usage in key_usage {
                if let Some(ku) = match usage.to_uppercase().as_str() {
                    "DIGITAL_SIGNATURE" => Some(KeyUsageName::DigitalSignature),
                    "NON_REPUDIATION" => Some(KeyUsageName::NonRepudation),
                    "KEY_ENCIPHERMENT" => Some(KeyUsageName::KeyEncipherment),
                    "DATA_ENCIPHERMENT" => Some(KeyUsageName::DataEncipherment),
                    "KEY_AGREEMENT" => Some(KeyUsageName::KeyAgreement),
                    "CERTIFICATE_SIGNING" => Some(KeyUsageName::CertificateSigning),
                    "CRL_SIGNING" => Some(KeyUsageName::CrlSigning),
                    "ENCIPHER_ONLY" => Some(KeyUsageName::DecipherOnly),
                    "DECIPHER_ONLY" => Some(KeyUsageName::DecipherOnly),
                    "ANY" => Some(KeyUsageName::Any),
                    "CUSTOM" => Some(KeyUsageName::Custom),
                    _ => None,
                } {
                    builder = builder.key_usage(ku);
                }
            }
        }

        if let Some(key_types) = f.key_types {
            for key_type in key_types {
                if let Some(kt) = match key_type.to_uppercase().as_str() {
                    "RSA_1024" => Some(KeyAlgorithm::Rsa1024),
                    "RSA_2048" => Some(KeyAlgorithm::Rsa2048),
                    "RSA_3072" => Some(KeyAlgorithm::Rsa3072),
                    "RSA_4096" => Some(KeyAlgorithm::Rsa4096),
                    "EC_PRIME256V1" => Some(KeyAlgorithm::EcPrime256v1),
                    "EC_SECP384R1" => Some(KeyAlgorithm::EcSecp384r1),
                    "EC_SECP521R1" => Some(KeyAlgorithm::EcSecp521r1),
                    _ => None,
                } {
                    builder = builder.key_types(kt);
                }
            }
        }

        builder.build()
    })
}

/// Request Certificate
///
/// Requests an ACM certificate for use with other Amazon Web Services services.
///
pub async fn request_certificate(
    domain_name: &str,
    certificate_authority_arn: Option<&str>,
    domain_validation_options: Option<Vec<DomainValidationOption>>,
    subject_alternative_names: Option<Vec<String>>,
    tags: Option<AcmTags>,
    validation_method: Option<&str>,
    region: Option<&str>,
) -> Result<RequestCertificateOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.request_certificate().domain_name(domain_name);

    if let Some(ca_arn) = certificate_authority_arn {
        request = request.certificate_authority_arn(ca_arn);
    }

    if let Some(opts) = convert_domain_validation_options(domain_validation_options) {
        request = request.set_domain_validation_options(Some(opts));
    }

    if let Some(sans) = subject_alternative_names {
        request = request.set_subject_alternative_names(Some(sans));
    }

    if let Some(aws_tags) = tags_to_aws_tags(tags) {
        request = request.set_tags(Some(aws_tags));
    }

    if let Some(method) = validation_method {
        let vm = match method.to_uppercase().as_str() {
            "EMAIL" => ValidationMethod::Email,
            "DNS" => ValidationMethod::Dns,
            _ => ValidationMethod::Dns,
        };
        request = request.validation_method(vm);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to request certificate: {}", e))?;

    let certificate_arn = response
        .certificate_arn()
        .ok_or("No certificate ARN in response")?
        .to_string();

    // Get the domain validation options from the certificate description
    let describe_response = client
        .describe_certificate()
        .certificate_arn(&certificate_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to describe certificate: {}", e))?;

    let domain_validation_options = describe_response
        .certificate()
        .map(|cert| convert_domain_validation(cert.domain_validation_options()))
        .unwrap_or_default();

    Ok(RequestCertificateOutput {
        certificate_arn,
        domain_validation_options,
        success: true,
    })
}

/// Import Certificate
///
/// Imports a certificate into AWS Certificate Manager (ACM).
///
pub async fn import_certificate(
    certificate: &str,
    private_key: &str,
    certificate_arn: Option<&str>,
    certificate_chain: Option<&str>,
    tags: Option<AcmTags>,
    region: Option<&str>,
) -> Result<ImportCertificateOutput, String> {
    let client = create_client(region).await?;

    let mut request = client
        .import_certificate()
        .certificate(Blob::new(certificate.as_bytes()))
        .private_key(Blob::new(private_key.as_bytes()));

    if let Some(arn) = certificate_arn {
        request = request.certificate_arn(arn);
    }

    if let Some(chain) = certificate_chain {
        request = request.certificate_chain(Blob::new(chain.as_bytes()));
    }

    if let Some(aws_tags) = tags_to_aws_tags(tags) {
        request = request.set_tags(Some(aws_tags));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to import certificate: {}", e))?;

    Ok(ImportCertificateOutput {
        certificate_arn: response.certificate_arn().unwrap_or_default().to_string(),
        success: true,
    })
}

/// Delete Certificate
///
/// Deletes a certificate and its associated private key.
///
pub async fn delete_certificate(
    certificate_arn: &str,
    region: Option<&str>,
) -> Result<DeleteCertificateOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_certificate()
        .certificate_arn(certificate_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to delete certificate: {}", e))?;

    Ok(DeleteCertificateOutput { success: true })
}

/// Describe Certificate
///
/// Returns detailed metadata about the specified ACM certificate.
///
pub async fn describe_certificate(
    certificate_arn: &str,
    region: Option<&str>,
) -> Result<DescribeCertificateOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .describe_certificate()
        .certificate_arn(certificate_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to describe certificate: {}", e))?;

    let cert = response.certificate().ok_or("No certificate in response")?;

    let certificate = Certificate {
        certificate_arn: cert.certificate_arn().unwrap_or_default().to_string(),
        domain_name: cert.domain_name().unwrap_or_default().to_string(),
        subject_alternative_names: cert.subject_alternative_names().iter().map(|s| s.to_string()).collect(),
        status: cert.status().map(|s| s.as_str().to_string()).unwrap_or_default(),
        certificate_type: cert.r#type().map(|t| t.as_str().to_string()).unwrap_or_default(),
        issuer: cert.issuer().map(|s| s.to_string()),
        created_at: cert.created_at().map(|dt| dt.to_string()),
        issued_at: cert.issued_at().map(|dt| dt.to_string()),
        not_before: cert.not_before().map(|dt| dt.to_string()),
        not_after: cert.not_after().map(|dt| dt.to_string()),
        key_algorithm: cert.key_algorithm().map(|k| k.as_str().to_string()),
        signature_algorithm: cert.signature_algorithm().map(|s| s.to_string()),
        in_use_by: cert.in_use_by().iter().map(|s| s.to_string()).collect(),
        domain_validation_options: convert_domain_validation(cert.domain_validation_options()),
        renewal_eligibility: cert.renewal_eligibility().map(|r| r.as_str().to_string()),
    };

    Ok(DescribeCertificateOutput { certificate })
}

/// List Certificates
///
/// Retrieves a list of certificate ARNs and domain names.
///
pub async fn list_certificates(
    certificate_statuses: Option<Vec<String>>,
    includes: Option<CertificateFilters>,
    max_items: Option<i32>,
    next_token: Option<&str>,
    region: Option<&str>,
) -> Result<ListCertificatesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_certificates();

    if let Some(statuses) = convert_statuses(certificate_statuses) {
        request = request.set_certificate_statuses(Some(statuses));
    }

    if let Some(filters) = convert_filters(includes) {
        request = request.includes(filters);
    }

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list certificates: {}", e))?;

    let certificates: Vec<CertificateSummary> = response
        .certificate_summary_list()
        .iter()
        .map(|c| CertificateSummary {
            certificate_arn: c.certificate_arn().unwrap_or_default().to_string(),
            domain_name: c.domain_name().unwrap_or_default().to_string(),
            status: c.status().map(|s| s.as_str().to_string()),
            certificate_type: c.r#type().map(|t| t.as_str().to_string()),
            key_algorithm: c.key_algorithm().map(|k| k.as_str().to_string()),
            in_use: c.in_use(),
        })
        .collect();

    Ok(ListCertificatesOutput {
        certificates,
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

/// Get Certificate
///
/// Retrieves an ACM certificate and the certificate chain.
///
pub async fn get_certificate(certificate_arn: &str, region: Option<&str>) -> Result<GetCertificateOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_certificate()
        .certificate_arn(certificate_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to get certificate: {}", e))?;

    Ok(GetCertificateOutput {
        certificate: response.certificate().unwrap_or_default().to_string(),
        certificate_chain: response.certificate_chain().map(|s| s.to_string()),
    })
}

/// Add Tags To Certificate
///
/// Adds one or more tags to an ACM certificate.
///
pub async fn add_tags(certificate_arn: &str, tags: AcmTags, region: Option<&str>) -> Result<AddTagsOutput, String> {
    let client = create_client(region).await?;

    let aws_tags = tags_to_aws_tags(Some(tags)).unwrap_or_default();

    client
        .add_tags_to_certificate()
        .certificate_arn(certificate_arn)
        .set_tags(Some(aws_tags))
        .send()
        .await
        .map_err(|e| format!("Failed to add tags: {}", e))?;

    Ok(AddTagsOutput { success: true })
}

/// Remove Tags From Certificate
///
/// Removes one or more tags from an ACM certificate.
///
pub async fn remove_tags(
    certificate_arn: &str,
    tag_keys: Vec<String>,
    region: Option<&str>,
) -> Result<RemoveTagsOutput, String> {
    let client = create_client(region).await?;

    let tags: Vec<AcmTag> = tag_keys
        .iter()
        .filter_map(|k| AcmTag::builder().key(k).build().ok())
        .collect();

    client
        .remove_tags_from_certificate()
        .certificate_arn(certificate_arn)
        .set_tags(Some(tags))
        .send()
        .await
        .map_err(|e| format!("Failed to remove tags: {}", e))?;

    Ok(RemoveTagsOutput { success: true })
}

/// List Tags For Certificate
///
/// Lists the tags that have been applied to an ACM certificate.
///
pub async fn list_tags(certificate_arn: &str, region: Option<&str>) -> Result<ListTagsOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .list_tags_for_certificate()
        .certificate_arn(certificate_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to list tags: {}", e))?;

    let tags = convert_tags(response.tags());

    Ok(ListTagsOutput { tags })
}

/// Renew Certificate
///
/// Renews an eligible ACM certificate.
///
pub async fn renew_certificate(certificate_arn: &str, region: Option<&str>) -> Result<RenewCertificateOutput, String> {
    let client = create_client(region).await?;

    client
        .renew_certificate()
        .certificate_arn(certificate_arn)
        .send()
        .await
        .map_err(|e| format!("Failed to renew certificate: {}", e))?;

    Ok(RenewCertificateOutput { success: true })
}

/// Export Certificate
///
/// Exports a private certificate issued by a private certificate authority (CA).
///
pub async fn export_certificate(
    certificate_arn: &str,
    passphrase: &[u8],
    region: Option<&str>,
) -> Result<ExportCertificateOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .export_certificate()
        .certificate_arn(certificate_arn)
        .passphrase(Blob::new(passphrase))
        .send()
        .await
        .map_err(|e| format!("Failed to export certificate: {}", e))?;

    Ok(ExportCertificateOutput {
        certificate: response.certificate().unwrap_or_default().to_string(),
        certificate_chain: response.certificate_chain().map(|s| s.to_string()),
        private_key: response.private_key().unwrap_or_default().to_string(),
    })
}

/// Resend Validation Email
///
/// Resends the email that requests domain ownership validation.
///
pub async fn resend_validation_email(
    certificate_arn: &str,
    domain: &str,
    validation_domain: &str,
    region: Option<&str>,
) -> Result<ResendValidationEmailOutput, String> {
    let client = create_client(region).await?;

    client
        .resend_validation_email()
        .certificate_arn(certificate_arn)
        .domain(domain)
        .validation_domain(validation_domain)
        .send()
        .await
        .map_err(|e| format!("Failed to resend validation email: {}", e))?;

    Ok(ResendValidationEmailOutput { success: true })
}

/// Update Certificate Options
///
/// Updates a certificate's options, such as certificate transparency logging.
///
pub async fn update_certificate_options(
    certificate_arn: &str,
    options: CertificateOptions,
    region: Option<&str>,
) -> Result<UpdateCertificateOptionsOutput, String> {
    let client = create_client(region).await?;

    let mut aws_options = aws_sdk_acm::types::CertificateOptions::builder();

    if let Some(transparency_logging) = options.certificate_transparency_logging_preference {
        let preference = match transparency_logging.to_uppercase().as_str() {
            "ENABLED" => aws_sdk_acm::types::CertificateTransparencyLoggingPreference::Enabled,
            "DISABLED" => aws_sdk_acm::types::CertificateTransparencyLoggingPreference::Disabled,
            _ => aws_sdk_acm::types::CertificateTransparencyLoggingPreference::Enabled,
        };
        aws_options = aws_options.certificate_transparency_logging_preference(preference);
    }

    client
        .update_certificate_options()
        .certificate_arn(certificate_arn)
        .options(aws_options.build())
        .send()
        .await
        .map_err(|e| format!("Failed to update certificate options: {}", e))?;

    Ok(UpdateCertificateOptionsOutput { success: true })
}

/// Get Account Configuration
///
/// Returns the account configuration options associated with an AWS account.
///
pub async fn get_account_configuration(region: Option<&str>) -> Result<GetAccountConfigurationOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_account_configuration()
        .send()
        .await
        .map_err(|e| format!("Failed to get account configuration: {}", e))?;

    let configuration = response
        .expiry_events()
        .map(|ee| AccountConfiguration {
            expiry_events: Some(ExpiryEventsConfiguration {
                days_before_expiry: ee.days_before_expiry(),
            }),
        })
        .unwrap_or(AccountConfiguration { expiry_events: None });

    Ok(GetAccountConfigurationOutput { configuration })
}

/// Put Account Configuration
///
/// Adds or modifies account-level configurations in ACM.
///
pub async fn put_account_configuration(
    expiry_events: ExpiryEventsConfiguration,
    idempotency_token: Option<&str>,
    region: Option<&str>,
) -> Result<PutAccountConfigurationOutput, String> {
    let client = create_client(region).await?;

    let mut aws_expiry_events = aws_sdk_acm::types::ExpiryEventsConfiguration::builder();

    if let Some(days) = expiry_events.days_before_expiry {
        aws_expiry_events = aws_expiry_events.days_before_expiry(days);
    }

    let mut request = client
        .put_account_configuration()
        .expiry_events(aws_expiry_events.build());

    if let Some(token) = idempotency_token {
        request = request.idempotency_token(token);
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to put account configuration: {}", e))?;

    Ok(PutAccountConfigurationOutput { success: true })
}
