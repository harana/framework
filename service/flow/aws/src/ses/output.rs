// Harana Actions - AWS SES Module Output Types
// Auto-generated output structs for action methods.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// create_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplateOutput {
    pub success: bool,
}

// delete_configuration_set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteConfigurationSetOutput {
    pub success: bool,
}

// delete_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteIdentityOutput {
    pub success: bool,
}

// delete_identity_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteIdentityPolicyOutput {
    pub success: bool,
}

// delete_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTemplateOutput {
    pub success: bool,
}

// get_identity_dkim_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIdentityDkimAttributesOutput {
    pub dkim_attributes: HashMap<String, DkimAttributes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DkimAttributes {
    pub dkim_enabled: bool,
    pub dkim_verification_status: String,
    pub dkim_tokens: Option<Vec<String>>,
}

// get_identity_policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIdentityPoliciesOutput {
    pub policies: HashMap<String, String>,
}

// get_identity_verification_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIdentityVerificationAttributesOutput {
    pub verification_attributes: HashMap<String, VerificationAttributes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationAttributes {
    pub verification_status: String,
    pub verification_token: Option<String>,
}

// get_send_quota
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSendQuotaOutput {
    pub max_send_rate: f64,
    pub sent_last_24_hours: f64,
    pub max_24_hour_send: f64,
}

// get_send_statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSendStatisticsOutput {
    pub send_data_points: Vec<SendDataPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendDataPoint {
    pub timestamp: Option<DateTime<Utc>>,
    pub delivery_attempts: i64,
    pub bounces: i64,
    pub complaints: i64,
    pub rejects: i64,
}

// get_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTemplateOutput {
    pub template_name: String,
    pub subject_part: String,
    pub html_part: Option<String>,
    pub text_part: Option<String>,
}

// list_configuration_sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListConfigurationSetsOutput {
    pub configuration_sets: Vec<ConfigurationSet>,
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSet {
    pub name: String,
}

// list_identities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListIdentitiesOutput {
    pub identities: Vec<String>,
    pub next_token: Option<String>,
}

// list_templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTemplatesOutput {
    pub templates: Vec<TemplateMetadata>,
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    pub name: String,
    pub created_timestamp: Option<DateTime<Utc>>,
}

// put_configuration_set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutConfigurationSetOutput {
    pub success: bool,
}

// put_identity_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutIdentityPolicyOutput {
    pub success: bool,
}

// send_bulk_templated_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBulkTemplatedEmailOutput {
    pub status: Vec<BulkEmailDestinationStatus>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEmailDestinationStatus {
    pub status: String,
    pub error: Option<String>,
    pub message_id: Option<String>,
}

// send_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailOutput {
    pub success: bool,
    pub message_id: String,
}

// send_raw_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendRawEmailOutput {
    pub success: bool,
    pub message_id: String,
}

// send_templated_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTemplatedEmailOutput {
    pub success: bool,
    pub message_id: String,
}

// set_identity_dkim_enabled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetIdentityDkimEnabledOutput {
    pub success: bool,
}

// set_identity_feedback_forwarding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetIdentityFeedbackForwardingOutput {
    pub success: bool,
}

// set_identity_notification_topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetIdentityNotificationTopicOutput {
    pub success: bool,
}

// update_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTemplateOutput {
    pub success: bool,
}

// verify_domain_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyDomainIdentityOutput {
    pub success: bool,
    pub verification_token: String,
}

// verify_domain_dkim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyDomainDkimOutput {
    pub success: bool,
    pub dkim_tokens: Vec<String>,
}

// verify_email_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyEmailIdentityOutput {
    pub success: bool,
}

// Helper structs for input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEmailDestination {
    pub destination: EmailDestination,
    pub replacement_template_data: Option<String>,
    pub replacement_tags: Option<Vec<MessageTag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailDestination {
    pub to_addresses: Vec<String>,
    pub cc_addresses: Option<Vec<String>>,
    pub bcc_addresses: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageTag {
    pub name: String,
    pub value: String,
}
