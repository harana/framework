// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestCertificateInput {
    pub domain_name: String,
    pub subject_alternative_names: Vec<String>,
    pub validation_method: String,
    pub certificate_authority_arn: String,
    pub domain_validation_options: Vec<String>,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestCertificateOutput {
    pub certificate_arn: String,
    pub domain_validation_options: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImportCertificateInput {
    pub certificate: String,
    pub certificate_chain: String,
    pub private_key: String,
    pub certificate_arn: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImportCertificateOutput {
    pub certificate_arn: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteCertificateInput {
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteCertificateOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeCertificateInput {
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescribeCertificateOutput {
    pub certificate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCertificatesInput {
    pub certificate_statuses: Vec<String>,
    pub includes: String,
    pub max_items: i64,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCertificatesOutput {
    pub certificates: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCertificateInput {
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCertificateOutput {
    pub certificate: String,
    pub certificate_chain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddTagsInput {
    pub certificate_arn: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddTagsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveTagsInput {
    pub certificate_arn: String,
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveTagsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTagsInput {
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTagsOutput {
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenewCertificateInput {
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RenewCertificateOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExportCertificateInput {
    pub certificate_arn: String,
    pub passphrase: String,
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
pub struct ResendValidationEmailInput {
    pub certificate_arn: String,
    pub domain: String,
    pub validation_domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResendValidationEmailOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateCertificateOptionsInput {
    pub certificate_arn: String,
    pub options: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateCertificateOptionsOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAccountConfigurationInput {
    pub expiry_events: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetAccountConfigurationOutput {
    pub configuration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutAccountConfigurationInput {
    pub expiry_events: String,
    pub idempotency_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutAccountConfigurationOutput {
    pub success: bool,
}

#[async_trait]
pub trait AcmAction {
    async fn request_certificate(&self, input: RequestCertificateInput) -> Result<RequestCertificateOutput, Box<dyn std::error::Error>>;
    async fn import_certificate(&self, input: ImportCertificateInput) -> Result<ImportCertificateOutput, Box<dyn std::error::Error>>;
    async fn delete_certificate(&self, input: DeleteCertificateInput) -> Result<DeleteCertificateOutput, Box<dyn std::error::Error>>;
    async fn describe_certificate(&self, input: DescribeCertificateInput) -> Result<DescribeCertificateOutput, Box<dyn std::error::Error>>;
    async fn list_certificates(&self, input: ListCertificatesInput) -> Result<ListCertificatesOutput, Box<dyn std::error::Error>>;
    async fn get_certificate(&self, input: GetCertificateInput) -> Result<GetCertificateOutput, Box<dyn std::error::Error>>;
    async fn add_tags(&self, input: AddTagsInput) -> Result<AddTagsOutput, Box<dyn std::error::Error>>;
    async fn remove_tags(&self, input: RemoveTagsInput) -> Result<RemoveTagsOutput, Box<dyn std::error::Error>>;
    async fn list_tags(&self, input: ListTagsInput) -> Result<ListTagsOutput, Box<dyn std::error::Error>>;
    async fn renew_certificate(&self, input: RenewCertificateInput) -> Result<RenewCertificateOutput, Box<dyn std::error::Error>>;
    async fn export_certificate(&self, input: ExportCertificateInput) -> Result<ExportCertificateOutput, Box<dyn std::error::Error>>;
    async fn resend_validation_email(&self, input: ResendValidationEmailInput) -> Result<ResendValidationEmailOutput, Box<dyn std::error::Error>>;
    async fn update_certificate_options(&self, input: UpdateCertificateOptionsInput) -> Result<UpdateCertificateOptionsOutput, Box<dyn std::error::Error>>;
    async fn get_account_configuration(&self, input: GetAccountConfigurationInput) -> Result<GetAccountConfigurationOutput, Box<dyn std::error::Error>>;
    async fn put_account_configuration(&self, input: PutAccountConfigurationInput) -> Result<PutAccountConfigurationOutput, Box<dyn std::error::Error>>;
}
