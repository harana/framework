// Harana Actions - AWS SES Module Output Types
// Auto-generated output structs for AWS SES action methods.

use serde::{Deserialize, Serialize};

// send_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailOutput {
    pub message_id: String,
    pub success: bool,
}


// send_raw_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendRawEmailOutput {
    pub message_id: String,
    pub success: bool
}

// send_templated_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTemplatedEmailOutput {
    pub success: bool,
    pub message_id: String
}

// send_bulk_templated_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBulkTemplatedEmailOutput {
    pub success: bool,
    pub message_ids: Vec<String>,
    pub status: Vec<HashMap<String, Value>>
}

// verify_email_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyEmailIdentityOutput {
    pub success: bool
}

// verify_domain_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyDomainIdentityOutput {
    pub verification_token: String,
    pub success: bool
}

// delete_identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteIdentityOutput {
    pub success: bool
}

// list_identities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListIdentitiesOutput {
    pub next_token: String,
    pub identities: Vec<String>
}

// get_identity_verification_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIdentityVerificationAttributesOutput {
    pub verification_attributes: HashMap<String, Value>
}

// set_identity_notification_topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetIdentityNotificationTopicOutput {
    pub success: bool
}

// create_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplateOutput {
    pub success: bool
}

// update_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTemplateOutput {
    pub success: bool
}

// delete_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTemplateOutput {
    pub success: bool
}

// get_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTemplateOutput {
    pub subject_part: String,
    pub html_part: String,
    pub template_name: String,
    pub text_part: String
}

// list_templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTemplatesOutput {
    pub next_token: String,
    pub templates: Vec<HashMap<String, Value>>
}

// get_send_quota
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSendQuotaOutput {
    pub max_24_hour_send: f64,
    pub max_send_rate: f64,
    pub sent_last_24_hours: f64
}

// get_send_statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSendStatisticsOutput {
    pub send_data_points: Vec<HashMap<String, Value>>
}

// put_configuration_set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutConfigurationSetOutput {
    pub success: bool
}

// delete_configuration_set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteConfigurationSetOutput {
    pub success: bool
}

// list_configuration_sets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListConfigurationSetsOutput {
    pub configuration_sets: Vec<HashMap<String, Value>>,
    pub next_token: String
}

// put_identity_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutIdentityPolicyOutput {
    pub success: bool
}

// get_identity_policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIdentityPoliciesOutput {
    pub policies: HashMap<String, Value>
}

// delete_identity_policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteIdentityPolicyOutput {
    pub success: bool
}

// set_identity_feedback_forwarding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetIdentityFeedbackForwardingOutput {
    pub success: bool
}

// set_identity_dkim_enabled
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetIdentityDkimEnabledOutput {
    pub success: bool
}

// get_identity_dkim_attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIdentityDkimAttributesOutput {
    pub dkim_attributes: HashMap<String, Value>
}
// TODO: Add remaining output types - see core/schema/actions/aws_ses.yml
