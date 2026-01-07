// Harana Actions - Aws Ses Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// create_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplateOutput {
    pub success: bool
}

// delete_configuration_set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteConfigurationSetOutput {
    pub success: bool
}

// delete_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteIdentityOutput {
    pub success: bool
}

// delete_identity_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteIdentityPolicyOutput {
    pub success: bool
}

// delete_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTemplateOutput {
    pub success: bool
}

// get_identity_dkim_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIdentityDkimAttributesOutput {
    pub dkim_attributes: HashMap<String, Value>
}

// get_identity_policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIdentityPoliciesOutput {
    pub policies: HashMap<String, Value>
}

// get_identity_verification_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIdentityVerificationAttributesOutput {
    pub verification_attributes: HashMap<String, Value>
}

// get_send_quota
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSendQuotaOutput {
    pub max_send_rate: f64,
    pub sent_last_24_hours: f64,
    pub max_24_hour_send: f64
}

// get_send_statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSendStatisticsOutput {
    pub send_data_points: Vec<HashMap<String, Value>>
}

// get_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTemplateOutput {
    pub html_part: String,
    pub text_part: String,
    pub template_name: String,
    pub subject_part: String
}

// list_configuration_sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListConfigurationSetsOutput {
    pub configuration_sets: Vec<HashMap<String, Value>>,
    pub next_token: String
}

// list_identities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListIdentitiesOutput {
    pub identities: Vec<String>,
    pub next_token: String
}

// list_templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTemplatesOutput {
    pub templates: Vec<HashMap<String, Value>>,
    pub next_token: String
}

// put_configuration_set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutConfigurationSetOutput {
    pub success: bool
}

// put_identity_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutIdentityPolicyOutput {
    pub success: bool
}

// send_bulk_templated_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBulkTemplatedEmailOutput {
    pub status: Vec<HashMap<String, Value>>,
    pub success: bool,
    pub message_ids: Vec<String>
}

// send_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailOutput {
    pub success: bool,
    pub message_id: String
}

// send_raw_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendRawEmailOutput {
    pub success: bool,
    pub message_id: String
}

// send_templated_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTemplatedEmailOutput {
    pub success: bool,
    pub message_id: String
}

// set_identity_dkim_enabled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetIdentityDkimEnabledOutput {
    pub success: bool
}

// set_identity_feedback_forwarding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetIdentityFeedbackForwardingOutput {
    pub success: bool
}

// set_identity_notification_topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetIdentityNotificationTopicOutput {
    pub success: bool
}

// update_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTemplateOutput {
    pub success: bool
}

// verify_domain_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyDomainIdentityOutput {
    pub success: bool,
    pub verification_token: String
}

// verify_email_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyEmailIdentityOutput {
    pub success: bool
}
