// Harana Actions - Aws Ses Module
// This module provides aws ses actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Create SES Template
pub async fn create_template(
    template_name: &str,
    subject_part: &str,
    text_part: Option<&str>,
    region: Option<&str>,
    html_part: Option<&str>,
) -> Result<CreateTemplateOutput, String> {
    unimplemented!("create_template")
}

/// Delete SES Configuration Set
pub async fn delete_configuration_set(
    configuration_set_name: &str,
    region: Option<&str>,
) -> Result<DeleteConfigurationSetOutput, String> {
    unimplemented!("delete_configuration_set")
}

/// Delete SES Identity
pub async fn delete_identity(
    identity: &str,
    region: Option<&str>,
) -> Result<DeleteIdentityOutput, String> {
    unimplemented!("delete_identity")
}

/// Delete SES Identity Policy
pub async fn delete_identity_policy(
    identity: &str,
    policy_name: &str,
    region: Option<&str>,
) -> Result<DeleteIdentityPolicyOutput, String> {
    unimplemented!("delete_identity_policy")
}

/// Delete SES Template
pub async fn delete_template(
    template_name: &str,
    region: Option<&str>,
) -> Result<DeleteTemplateOutput, String> {
    unimplemented!("delete_template")
}

/// Get SES Identity DKIM Attributes
pub async fn get_identity_dkim_attributes(
    identities: Vec<String>,
    region: Option<&str>,
) -> Result<GetIdentityDkimAttributesOutput, String> {
    unimplemented!("get_identity_dkim_attributes")
}

/// Get SES Identity Policies
pub async fn get_identity_policies(
    identity: &str,
    policy_names: Vec<String>,
    region: Option<&str>,
) -> Result<GetIdentityPoliciesOutput, String> {
    unimplemented!("get_identity_policies")
}

/// Get SES Identity Verification Attributes
pub async fn get_identity_verification_attributes(
    identities: Vec<String>,
    region: Option<&str>,
) -> Result<GetIdentityVerificationAttributesOutput, String> {
    unimplemented!("get_identity_verification_attributes")
}

/// Get SES Send Quota
pub async fn get_send_quota(
    region: Option<&str>,
) -> Result<GetSendQuotaOutput, String> {
    unimplemented!("get_send_quota")
}

/// Get SES Send Statistics
pub async fn get_send_statistics(
    region: Option<&str>,
) -> Result<GetSendStatisticsOutput, String> {
    unimplemented!("get_send_statistics")
}

/// Get SES Template
pub async fn get_template(
    template_name: &str,
    region: Option<&str>,
) -> Result<GetTemplateOutput, String> {
    unimplemented!("get_template")
}

/// List SES Configuration Sets
pub async fn list_configuration_sets(
    max_items: Option<i32>,
    region: Option<&str>,
) -> Result<ListConfigurationSetsOutput, String> {
    unimplemented!("list_configuration_sets")
}

/// List SES Identities
pub async fn list_identities(
    identity_type: Option<&str>,
    max_items: Option<i32>,
    region: Option<&str>,
) -> Result<ListIdentitiesOutput, String> {
    unimplemented!("list_identities")
}

/// List SES Templates
pub async fn list_templates(
    region: Option<&str>,
    max_items: Option<i32>,
) -> Result<ListTemplatesOutput, String> {
    unimplemented!("list_templates")
}

/// Put SES Configuration Set
pub async fn put_configuration_set(
    configuration_set_name: &str,
    region: Option<&str>,
) -> Result<PutConfigurationSetOutput, String> {
    unimplemented!("put_configuration_set")
}

/// Put SES Identity Policy
pub async fn put_identity_policy(
    policy_name: &str,
    identity: &str,
    policy: &str,
    region: Option<&str>,
) -> Result<PutIdentityPolicyOutput, String> {
    unimplemented!("put_identity_policy")
}

/// Send SES Bulk Templated Email
pub async fn send_bulk_templated_email(
    from_address: &str,
    destinations: Vec<HashMap<String, Value>>,
    template_name: &str,
    region: Option<&str>,
    default_template_data: Option<&str>,
    return_path: Option<&str>,
    configuration_set_name: Option<&str>,
    default_tags: Option<HashMap<String, Value>>,
    reply_to_addresses: Option<Vec<String>>,
) -> Result<SendBulkTemplatedEmailOutput, String> {
    unimplemented!("send_bulk_templated_email")
}

/// Send SES Email
pub async fn send_email(
    from_address: &str,
    to_addresses: Vec<String>,
    subject: &str,
    bcc_addresses: Option<Vec<String>>,
    tags: Option<HashMap<String, Value>>,
    return_path: Option<&str>,
    cc_addresses: Option<Vec<String>>,
    html_body: Option<&str>,
    region: Option<&str>,
    reply_to_addresses: Option<Vec<String>>,
    text_body: Option<&str>,
    configuration_set_name: Option<&str>,
) -> Result<SendEmailOutput, String> {
    unimplemented!("send_email")
}

/// Send SES Raw Email
pub async fn send_raw_email(
    raw_message: &[u8],
    destinations: Option<Vec<String>>,
    source: Option<&str>,
    return_path_arn: Option<&str>,
    from_arn: Option<&str>,
    tags: Option<HashMap<String, Value>>,
    configuration_set_name: Option<&str>,
    source_arn: Option<&str>,
    region: Option<&str>,
) -> Result<SendRawEmailOutput, String> {
    unimplemented!("send_raw_email")
}

/// Send SES Templated Email
pub async fn send_templated_email(
    from_address: &str,
    template_name: &str,
    to_addresses: Vec<String>,
    template_data: &str,
    reply_to_addresses: Option<Vec<String>>,
    bcc_addresses: Option<Vec<String>>,
    tags: Option<HashMap<String, Value>>,
    cc_addresses: Option<Vec<String>>,
    return_path: Option<&str>,
    region: Option<&str>,
    configuration_set_name: Option<&str>,
) -> Result<SendTemplatedEmailOutput, String> {
    unimplemented!("send_templated_email")
}

/// Set SES Identity DKIM Enabled
pub async fn set_identity_dkim_enabled(
    dkim_enabled: bool,
    identity: &str,
    region: Option<&str>,
) -> Result<SetIdentityDkimEnabledOutput, String> {
    unimplemented!("set_identity_dkim_enabled")
}

/// Set SES Identity Feedback Forwarding
pub async fn set_identity_feedback_forwarding(
    forwarding_enabled: bool,
    identity: &str,
    region: Option<&str>,
) -> Result<SetIdentityFeedbackForwardingOutput, String> {
    unimplemented!("set_identity_feedback_forwarding")
}

/// Set SES Identity Notification Topic
pub async fn set_identity_notification_topic(
    identity: &str,
    notification_type: &str,
    region: Option<&str>,
    sns_topic: Option<&str>,
) -> Result<SetIdentityNotificationTopicOutput, String> {
    unimplemented!("set_identity_notification_topic")
}

/// Update SES Template
pub async fn update_template(
    template_name: &str,
    subject_part: Option<&str>,
    html_part: Option<&str>,
    text_part: Option<&str>,
    region: Option<&str>,
) -> Result<UpdateTemplateOutput, String> {
    unimplemented!("update_template")
}

/// Verify SES Domain Identity
pub async fn verify_domain_identity(
    domain: &str,
    region: Option<&str>,
) -> Result<VerifyDomainIdentityOutput, String> {
    unimplemented!("verify_domain_identity")
}

/// Verify SES Email Identity
pub async fn verify_email_identity(
    email_address: &str,
    region: Option<&str>,
) -> Result<VerifyEmailIdentityOutput, String> {
    unimplemented!("verify_email_identity")
}
