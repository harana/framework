// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendEmailInput {
    pub bcc_addresses: Vec<String>,
    pub cc_addresses: Vec<String>,
    pub configuration_set_name: String,
    pub from_address: String,
    pub html_body: String,
    pub region: String,
    pub reply_to_addresses: Vec<String>,
    pub return_path: String,
    pub subject: String,
    pub tags: String,
    pub text_body: String,
    pub to_addresses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendEmailOutput {
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendRawEmailInput {
    pub configuration_set_name: String,
    pub destinations: Vec<String>,
    pub from_arn: String,
    pub raw_message: String,
    pub region: String,
    pub return_path_arn: String,
    pub source: String,
    pub source_arn: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendRawEmailOutput {
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendTemplatedEmailInput {
    pub bcc_addresses: Vec<String>,
    pub cc_addresses: Vec<String>,
    pub configuration_set_name: String,
    pub from_address: String,
    pub region: String,
    pub reply_to_addresses: Vec<String>,
    pub return_path: String,
    pub tags: String,
    pub template_data: String,
    pub template_name: String,
    pub to_addresses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendTemplatedEmailOutput {
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendBulkTemplatedEmailInput {
    pub configuration_set_name: String,
    pub default_tags: String,
    pub default_template_data: String,
    pub destinations: Vec<String>,
    pub from_address: String,
    pub region: String,
    pub reply_to_addresses: Vec<String>,
    pub return_path: String,
    pub template_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendBulkTemplatedEmailOutput {
    pub message_ids: Vec<String>,
    pub status: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyEmailIdentityInput {
    pub email_address: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyEmailIdentityOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyDomainIdentityInput {
    pub domain: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerifyDomainIdentityOutput {
    pub success: bool,
    pub verification_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteIdentityInput {
    pub identity: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteIdentityOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListIdentitiesInput {
    pub identity_type: String,
    pub max_items: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListIdentitiesOutput {
    pub identities: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetIdentityVerificationAttributesInput {
    pub identities: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetIdentityVerificationAttributesOutput {
    pub verification_attributes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetIdentityNotificationTopicInput {
    pub identity: String,
    pub notification_type: String,
    pub region: String,
    pub sns_topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetIdentityNotificationTopicOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTemplateInput {
    pub html_part: String,
    pub region: String,
    pub subject_part: String,
    pub template_name: String,
    pub text_part: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTemplateOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateTemplateInput {
    pub html_part: String,
    pub region: String,
    pub subject_part: String,
    pub template_name: String,
    pub text_part: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateTemplateOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTemplateInput {
    pub region: String,
    pub template_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteTemplateOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTemplateInput {
    pub region: String,
    pub template_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTemplateOutput {
    pub html_part: String,
    pub subject_part: String,
    pub template_name: String,
    pub text_part: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTemplatesInput {
    pub max_items: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTemplatesOutput {
    pub next_token: String,
    pub templates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSendQuotaInput {
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSendQuotaOutput {
    pub max_24_hour_send: f64,
    pub max_send_rate: f64,
    pub sent_last_24_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSendStatisticsInput {
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSendStatisticsOutput {
    pub send_data_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutConfigurationSetInput {
    pub configuration_set_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutConfigurationSetOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteConfigurationSetInput {
    pub configuration_set_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteConfigurationSetOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListConfigurationSetsInput {
    pub max_items: i64,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListConfigurationSetsOutput {
    pub configuration_sets: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutIdentityPolicyInput {
    pub identity: String,
    pub policy: String,
    pub policy_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutIdentityPolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetIdentityPoliciesInput {
    pub identity: String,
    pub policy_names: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetIdentityPoliciesOutput {
    pub policies: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteIdentityPolicyInput {
    pub identity: String,
    pub policy_name: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteIdentityPolicyOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetIdentityFeedbackForwardingInput {
    pub forwarding_enabled: bool,
    pub identity: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetIdentityFeedbackForwardingOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetIdentityDkimEnabledInput {
    pub dkim_enabled: bool,
    pub identity: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetIdentityDkimEnabledOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetIdentityDkimAttributesInput {
    pub identities: Vec<String>,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetIdentityDkimAttributesOutput {
    pub dkim_attributes: String,
}

#[async_trait]
pub trait SesAction {
    async fn send_email(&self, input: SendEmailInput) -> Result<SendEmailOutput, Box<dyn std::error::Error>>;
    async fn send_raw_email(&self, input: SendRawEmailInput) -> Result<SendRawEmailOutput, Box<dyn std::error::Error>>;
    async fn send_templated_email(&self, input: SendTemplatedEmailInput) -> Result<SendTemplatedEmailOutput, Box<dyn std::error::Error>>;
    async fn send_bulk_templated_email(&self, input: SendBulkTemplatedEmailInput) -> Result<SendBulkTemplatedEmailOutput, Box<dyn std::error::Error>>;
    async fn verify_email_identity(&self, input: VerifyEmailIdentityInput) -> Result<VerifyEmailIdentityOutput, Box<dyn std::error::Error>>;
    async fn verify_domain_identity(&self, input: VerifyDomainIdentityInput) -> Result<VerifyDomainIdentityOutput, Box<dyn std::error::Error>>;
    async fn delete_identity(&self, input: DeleteIdentityInput) -> Result<DeleteIdentityOutput, Box<dyn std::error::Error>>;
    async fn list_identities(&self, input: ListIdentitiesInput) -> Result<ListIdentitiesOutput, Box<dyn std::error::Error>>;
    async fn get_identity_verification_attributes(&self, input: GetIdentityVerificationAttributesInput) -> Result<GetIdentityVerificationAttributesOutput, Box<dyn std::error::Error>>;
    async fn set_identity_notification_topic(&self, input: SetIdentityNotificationTopicInput) -> Result<SetIdentityNotificationTopicOutput, Box<dyn std::error::Error>>;
    async fn create_template(&self, input: CreateTemplateInput) -> Result<CreateTemplateOutput, Box<dyn std::error::Error>>;
    async fn update_template(&self, input: UpdateTemplateInput) -> Result<UpdateTemplateOutput, Box<dyn std::error::Error>>;
    async fn delete_template(&self, input: DeleteTemplateInput) -> Result<DeleteTemplateOutput, Box<dyn std::error::Error>>;
    async fn get_template(&self, input: GetTemplateInput) -> Result<GetTemplateOutput, Box<dyn std::error::Error>>;
    async fn list_templates(&self, input: ListTemplatesInput) -> Result<ListTemplatesOutput, Box<dyn std::error::Error>>;
    async fn get_send_quota(&self, input: GetSendQuotaInput) -> Result<GetSendQuotaOutput, Box<dyn std::error::Error>>;
    async fn get_send_statistics(&self, input: GetSendStatisticsInput) -> Result<GetSendStatisticsOutput, Box<dyn std::error::Error>>;
    async fn put_configuration_set(&self, input: PutConfigurationSetInput) -> Result<PutConfigurationSetOutput, Box<dyn std::error::Error>>;
    async fn delete_configuration_set(&self, input: DeleteConfigurationSetInput) -> Result<DeleteConfigurationSetOutput, Box<dyn std::error::Error>>;
    async fn list_configuration_sets(&self, input: ListConfigurationSetsInput) -> Result<ListConfigurationSetsOutput, Box<dyn std::error::Error>>;
    async fn put_identity_policy(&self, input: PutIdentityPolicyInput) -> Result<PutIdentityPolicyOutput, Box<dyn std::error::Error>>;
    async fn get_identity_policies(&self, input: GetIdentityPoliciesInput) -> Result<GetIdentityPoliciesOutput, Box<dyn std::error::Error>>;
    async fn delete_identity_policy(&self, input: DeleteIdentityPolicyInput) -> Result<DeleteIdentityPolicyOutput, Box<dyn std::error::Error>>;
    async fn set_identity_feedback_forwarding(&self, input: SetIdentityFeedbackForwardingInput) -> Result<SetIdentityFeedbackForwardingOutput, Box<dyn std::error::Error>>;
    async fn set_identity_dkim_enabled(&self, input: SetIdentityDkimEnabledInput) -> Result<SetIdentityDkimEnabledOutput, Box<dyn std::error::Error>>;
    async fn get_identity_dkim_attributes(&self, input: GetIdentityDkimAttributesInput) -> Result<GetIdentityDkimAttributesOutput, Box<dyn std::error::Error>>;
}
