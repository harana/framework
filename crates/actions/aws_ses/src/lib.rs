//! Harana Actions - AWS SES Module
//!
//! This module provides AWS Simple Email Service (SES) actions for sending emails,
//! managing templates, identities, and configuration sets.

/// Output types for AWS SES actions
pub mod output;

use aws_config::BehaviorVersion;
use aws_sdk_ses::{
    config::Region,
    types::{
        Body, Content, Destination, Message, Template, RawMessage,
        BulkEmailDestination as AwsBulkEmailDestination, MessageTag as AwsMessageTag,
        NotificationType,
    },
    Client,
};
use chrono::{DateTime, Utc};
use output::*;
use std::collections::HashMap;

/// Creates an SES client with the specified region
async fn create_client(region: Option<&str>) -> Result<Client, String> {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let ses_config = if let Some(region_str) = region {
        aws_sdk_ses::config::Builder::from(&config)
            .region(Region::new(region_str.to_string()))
            .build()
    } else {
        aws_sdk_ses::config::Builder::from(&config).build()
    };

    Ok(Client::from_conf(ses_config))
}

/// Create SES Template
///
/// Creates an email template that can be used with templated email operations.
///
pub async fn create_template(
    template_name: &str,
    subject_part: &str,
    text_part: Option<&str>,
    region: Option<&str>,
    html_part: Option<&str>,
) -> Result<CreateTemplateOutput, String> {
    let client = create_client(region).await?;

    let mut template_builder = Template::builder()
        .template_name(template_name)
        .subject_part(subject_part);

    if let Some(text) = text_part {
        template_builder = template_builder.text_part(text);
    }

    if let Some(html) = html_part {
        template_builder = template_builder.html_part(html);
    }

    let template = template_builder.build().map_err(|e| format!("Failed to build template: {}", e))?;

    client
        .create_template()
        .template(template)
        .send()
        .await
        .map_err(|e| format!("Failed to create template: {}", e))?;

    Ok(CreateTemplateOutput { success: true })
}

/// Delete SES Configuration Set
///
/// Deletes a configuration set.
///
pub async fn delete_configuration_set(
    configuration_set_name: &str,
    region: Option<&str>,
) -> Result<DeleteConfigurationSetOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_configuration_set()
        .configuration_set_name(configuration_set_name)
        .send()
        .await
        .map_err(|e| format!("Failed to delete configuration set: {}", e))?;

    Ok(DeleteConfigurationSetOutput { success: true })
}

/// Delete SES Identity
///
/// Deletes the specified identity (an email address or a domain) from the list of verified identities.
///
pub async fn delete_identity(
    identity: &str,
    region: Option<&str>,
) -> Result<DeleteIdentityOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_identity()
        .identity(identity)
        .send()
        .await
        .map_err(|e| format!("Failed to delete identity: {}", e))?;

    Ok(DeleteIdentityOutput { success: true })
}

/// Delete SES Identity Policy
///
/// Deletes the specified sending authorization policy for the given identity.
///
pub async fn delete_identity_policy(
    identity: &str,
    policy_name: &str,
    region: Option<&str>,
) -> Result<DeleteIdentityPolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_identity_policy()
        .identity(identity)
        .policy_name(policy_name)
        .send()
        .await
        .map_err(|e| format!("Failed to delete identity policy: {}", e))?;

    Ok(DeleteIdentityPolicyOutput { success: true })
}

/// Delete SES Template
///
/// Deletes an email template.
///
pub async fn delete_template(
    template_name: &str,
    region: Option<&str>,
) -> Result<DeleteTemplateOutput, String> {
    let client = create_client(region).await?;

    client
        .delete_template()
        .template_name(template_name)
        .send()
        .await
        .map_err(|e| format!("Failed to delete template: {}", e))?;

    Ok(DeleteTemplateOutput { success: true })
}

/// Get SES Identity DKIM Attributes
///
/// Returns the DKIM attributes for one or more identities.
///
pub async fn get_identity_dkim_attributes(
    identities: Vec<String>,
    region: Option<&str>,
) -> Result<GetIdentityDkimAttributesOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_identity_dkim_attributes()
        .set_identities(Some(identities))
        .send()
        .await
        .map_err(|e| format!("Failed to get identity DKIM attributes: {}", e))?;

    let dkim_attributes: HashMap<String, DkimAttributes> = response
        .dkim_attributes()
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                DkimAttributes {
                    dkim_enabled: v.dkim_enabled(),
                    dkim_verification_status: v
                        .dkim_verification_status()
                        .as_str()
                        .to_string(),
                    dkim_tokens: if v.dkim_tokens().is_empty() {
                        None
                    } else {
                        Some(v.dkim_tokens().iter().map(|s| s.to_string()).collect())
                    },
                },
            )
        })
        .collect();

    Ok(GetIdentityDkimAttributesOutput { dkim_attributes })
}

/// Get SES Identity Policies
///
/// Returns the sending authorization policies for the specified identity.
///
pub async fn get_identity_policies(
    identity: &str,
    policy_names: Vec<String>,
    region: Option<&str>,
) -> Result<GetIdentityPoliciesOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_identity_policies()
        .identity(identity)
        .set_policy_names(Some(policy_names))
        .send()
        .await
        .map_err(|e| format!("Failed to get identity policies: {}", e))?;

    let policies: HashMap<String, String> = response
        .policies()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    Ok(GetIdentityPoliciesOutput { policies })
}

/// Get SES Identity Verification Attributes
///
/// Returns the verification status for one or more identities.
///
pub async fn get_identity_verification_attributes(
    identities: Vec<String>,
    region: Option<&str>,
) -> Result<GetIdentityVerificationAttributesOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_identity_verification_attributes()
        .set_identities(Some(identities))
        .send()
        .await
        .map_err(|e| format!("Failed to get identity verification attributes: {}", e))?;

    let verification_attributes: HashMap<String, VerificationAttributes> = response
        .verification_attributes()
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                VerificationAttributes {
                    verification_status: v
                        .verification_status()
                        .as_str()
                        .to_string(),
                    verification_token: v.verification_token().map(|s| s.to_string()),
                },
            )
        })
        .collect();

    Ok(GetIdentityVerificationAttributesOutput { verification_attributes })
}

/// Get SES Send Quota
///
/// Returns the user's current sending limits.
///
pub async fn get_send_quota(
    region: Option<&str>,
) -> Result<GetSendQuotaOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_send_quota()
        .send()
        .await
        .map_err(|e| format!("Failed to get send quota: {}", e))?;

    Ok(GetSendQuotaOutput {
        max_send_rate: response.max_send_rate(),
        sent_last_24_hours: response.sent_last24_hours(),
        max_24_hour_send: response.max24_hour_send(),
    })
}

/// Get SES Send Statistics
///
/// Returns the user's sending statistics.
///
pub async fn get_send_statistics(
    region: Option<&str>,
) -> Result<GetSendStatisticsOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_send_statistics()
        .send()
        .await
        .map_err(|e| format!("Failed to get send statistics: {}", e))?;

    let send_data_points: Vec<SendDataPoint> = response
        .send_data_points()
        .iter()
        .map(|dp| SendDataPoint {
            timestamp: dp.timestamp().map(|t| {
                DateTime::<Utc>::from_timestamp(t.secs(), t.subsec_nanos())
                    .unwrap_or_default()
            }),
            delivery_attempts: dp.delivery_attempts(),
            bounces: dp.bounces(),
            complaints: dp.complaints(),
            rejects: dp.rejects(),
        })
        .collect();

    Ok(GetSendStatisticsOutput { send_data_points })
}

/// Get SES Template
///
/// Retrieves the content of an email template.
///
pub async fn get_template(
    template_name: &str,
    region: Option<&str>,
) -> Result<GetTemplateOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .get_template()
        .template_name(template_name)
        .send()
        .await
        .map_err(|e| format!("Failed to get template: {}", e))?;

    let template = response.template().ok_or("No template in response")?;

    Ok(GetTemplateOutput {
        template_name: template.template_name().to_string(),
        subject_part: template.subject_part().unwrap_or_default().to_string(),
        html_part: template.html_part().map(|s| s.to_string()),
        text_part: template.text_part().map(|s| s.to_string()),
    })
}

/// List SES Configuration Sets
///
/// Lists the configuration sets associated with your AWS account.
///
pub async fn list_configuration_sets(
    max_items: Option<i32>,
    region: Option<&str>,
) -> Result<ListConfigurationSetsOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_configuration_sets();

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list configuration sets: {}", e))?;

    let configuration_sets: Vec<ConfigurationSet> = response
        .configuration_sets()
        .iter()
        .map(|cs| ConfigurationSet {
            name: cs.name().to_string(),
        })
        .collect();

    Ok(ListConfigurationSetsOutput {
        configuration_sets,
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

/// List SES Identities
///
/// Returns a list containing all of the identities (email addresses and domains).
///
pub async fn list_identities(
    identity_type: Option<&str>,
    max_items: Option<i32>,
    region: Option<&str>,
) -> Result<ListIdentitiesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_identities();

    if let Some(id_type) = identity_type {
        let identity_type_enum = match id_type.to_lowercase().as_str() {
            "emailaddress" | "email" => aws_sdk_ses::types::IdentityType::EmailAddress,
            "domain" => aws_sdk_ses::types::IdentityType::Domain,
            _ => return Err(format!("Invalid identity type: {}", id_type)),
        };
        request = request.identity_type(identity_type_enum);
    }

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list identities: {}", e))?;

    Ok(ListIdentitiesOutput {
        identities: response.identities().iter().map(|s| s.to_string()).collect(),
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

/// List SES Templates
///
/// Lists the email templates in your Amazon SES account.
///
pub async fn list_templates(
    region: Option<&str>,
    max_items: Option<i32>,
) -> Result<ListTemplatesOutput, String> {
    let client = create_client(region).await?;

    let mut request = client.list_templates();

    if let Some(max) = max_items {
        request = request.max_items(max);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to list templates: {}", e))?;

    let templates: Vec<TemplateMetadata> = response
        .templates_metadata()
        .iter()
        .map(|t| TemplateMetadata {
            name: t.name().unwrap_or_default().to_string(),
            created_timestamp: t.created_timestamp().map(|ts| {
                DateTime::<Utc>::from_timestamp(ts.secs(), ts.subsec_nanos())
                    .unwrap_or_default()
            }),
        })
        .collect();

    Ok(ListTemplatesOutput {
        templates,
        next_token: response.next_token().map(|s| s.to_string()),
    })
}

/// Create SES Configuration Set
///
/// Creates a configuration set for tracking email sending events.
///
pub async fn put_configuration_set(
    configuration_set_name: &str,
    region: Option<&str>,
) -> Result<PutConfigurationSetOutput, String> {
    let client = create_client(region).await?;

    let config_set = aws_sdk_ses::types::ConfigurationSet::builder()
        .name(configuration_set_name)
        .build()
        .map_err(|e| format!("Failed to build configuration set: {}", e))?;

    client
        .create_configuration_set()
        .configuration_set(config_set)
        .send()
        .await
        .map_err(|e| format!("Failed to create configuration set: {}", e))?;

    Ok(PutConfigurationSetOutput { success: true })
}

/// Put SES Identity Policy
///
/// Adds or updates a sending authorization policy for the specified identity.
///
pub async fn put_identity_policy(
    policy_name: &str,
    identity: &str,
    policy: &str,
    region: Option<&str>,
) -> Result<PutIdentityPolicyOutput, String> {
    let client = create_client(region).await?;

    client
        .put_identity_policy()
        .identity(identity)
        .policy_name(policy_name)
        .policy(policy)
        .send()
        .await
        .map_err(|e| format!("Failed to put identity policy: {}", e))?;

    Ok(PutIdentityPolicyOutput { success: true })
}

/// Send SES Bulk Templated Email
///
/// Composes an email message using a template and sends it to multiple destinations.
///
pub async fn send_bulk_templated_email(
    from_address: &str,
    destinations: Vec<BulkEmailDestination>,
    template_name: &str,
    region: Option<&str>,
    default_template_data: Option<&str>,
    return_path: Option<&str>,
    configuration_set_name: Option<&str>,
    default_tags: Option<Vec<MessageTag>>,
    reply_to_addresses: Option<Vec<String>>,
) -> Result<SendBulkTemplatedEmailOutput, String> {
    let client = create_client(region).await?;

    let aws_destinations: Vec<AwsBulkEmailDestination> = destinations
        .iter()
        .map(|d| {
            let mut dest_builder = Destination::builder()
                .set_to_addresses(Some(d.destination.to_addresses.clone()));

            if let Some(cc) = &d.destination.cc_addresses {
                dest_builder = dest_builder.set_cc_addresses(Some(cc.clone()));
            }

            if let Some(bcc) = &d.destination.bcc_addresses {
                dest_builder = dest_builder.set_bcc_addresses(Some(bcc.clone()));
            }

            let destination = dest_builder.build();

            let mut bulk_dest_builder = AwsBulkEmailDestination::builder()
                .destination(destination);

            if let Some(data) = &d.replacement_template_data {
                bulk_dest_builder = bulk_dest_builder.replacement_template_data(data);
            }

            if let Some(tags) = &d.replacement_tags {
                for tag in tags {
                    let aws_tag = AwsMessageTag::builder()
                        .name(&tag.name)
                        .value(&tag.value)
                        .build()
                        .map_err(|e| format!("Failed to build tag: {}", e)).unwrap();
                    bulk_dest_builder = bulk_dest_builder.replacement_tags(aws_tag);
                }
            }

            bulk_dest_builder.build()
        })
        .collect();

    let mut request = client
        .send_bulk_templated_email()
        .source(from_address)
        .template(template_name)
        .set_destinations(Some(aws_destinations));

    if let Some(data) = default_template_data {
        request = request.default_template_data(data);
    }

    if let Some(path) = return_path {
        request = request.return_path(path);
    }

    if let Some(config_set) = configuration_set_name {
        request = request.configuration_set_name(config_set);
    }

    if let Some(tags) = default_tags {
        for tag in tags {
            let aws_tag = AwsMessageTag::builder()
                .name(&tag.name)
                .value(&tag.value)
                .build()
                .map_err(|e| format!("Failed to build message tag: {}", e))?;
            request = request.default_tags(aws_tag);
        }
    }

    if let Some(reply_to) = reply_to_addresses {
        request = request.set_reply_to_addresses(Some(reply_to));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to send bulk templated email: {}", e))?;

    let status: Vec<BulkEmailDestinationStatus> = response
        .status()
        .iter()
        .map(|s| BulkEmailDestinationStatus {
            status: s.status().map(|st| st.as_str().to_string()).unwrap_or_default(),
            error: s.error().map(|e| e.to_string()),
            message_id: s.message_id().map(|m| m.to_string()),
        })
        .collect();

    Ok(SendBulkTemplatedEmailOutput {
        status,
        success: true,
    })
}

/// Send SES Email
///
/// Composes and sends an email message.
///
pub async fn send_email(
    from_address: &str,
    to_addresses: Vec<String>,
    subject: &str,
    bcc_addresses: Option<Vec<String>>,
    tags: Option<Vec<MessageTag>>,
    return_path: Option<&str>,
    cc_addresses: Option<Vec<String>>,
    html_body: Option<&str>,
    region: Option<&str>,
    reply_to_addresses: Option<Vec<String>>,
    text_body: Option<&str>,
    configuration_set_name: Option<&str>,
) -> Result<SendEmailOutput, String> {
    let client = create_client(region).await?;

    // Build destination
    let mut destination_builder = Destination::builder()
        .set_to_addresses(Some(to_addresses));

    if let Some(cc) = cc_addresses {
        destination_builder = destination_builder.set_cc_addresses(Some(cc));
    }

    if let Some(bcc) = bcc_addresses {
        destination_builder = destination_builder.set_bcc_addresses(Some(bcc));
    }

    let destination = destination_builder.build();

    // Build message body
    let mut body_builder = Body::builder();

    if let Some(text) = text_body {
        let text_content = Content::builder()
            .data(text)
            .charset("UTF-8")
            .build()
            .map_err(|e| format!("Failed to build text content: {}", e))?;
        body_builder = body_builder.text(text_content);
    }

    if let Some(html) = html_body {
        let html_content = Content::builder()
            .data(html)
            .charset("UTF-8")
            .build()
            .map_err(|e| format!("Failed to build HTML content: {}", e))?;
        body_builder = body_builder.html(html_content);
    }

    let body = body_builder.build();

    // Build subject
    let subject_content = Content::builder()
        .data(subject)
        .charset("UTF-8")
        .build()
        .map_err(|e| format!("Failed to build subject: {}", e))?;

    // Build message
    let message = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    // Build request
    let mut request = client
        .send_email()
        .source(from_address)
        .destination(destination)
        .message(message);

    if let Some(path) = return_path {
        request = request.return_path(path);
    }

    if let Some(reply_to) = reply_to_addresses {
        request = request.set_reply_to_addresses(Some(reply_to));
    }

    if let Some(config_set) = configuration_set_name {
        request = request.configuration_set_name(config_set);
    }

    if let Some(msg_tags) = tags {
        for tag in msg_tags {
            let aws_tag = AwsMessageTag::builder()
                .name(&tag.name)
                .value(&tag.value)
                .build()
                .map_err(|e| format!("Failed to build message tag: {}", e))?;
            request = request.tags(aws_tag);
        }
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to send email: {}", e))?;

    Ok(SendEmailOutput {
        success: true,
        message_id: response.message_id().to_string(),
    })
}

/// Send SES Raw Email
///
/// Sends an email message, with header and content specified by the client.
///
pub async fn send_raw_email(
    raw_message: &[u8],
    destinations: Option<Vec<String>>,
    source: Option<&str>,
    return_path_arn: Option<&str>,
    from_arn: Option<&str>,
    tags: Option<Vec<MessageTag>>,
    configuration_set_name: Option<&str>,
    source_arn: Option<&str>,
    region: Option<&str>,
) -> Result<SendRawEmailOutput, String> {
    let client = create_client(region).await?;

    let raw_msg = RawMessage::builder()
        .data(aws_sdk_ses::primitives::Blob::new(raw_message))
        .build()
        .map_err(|e| format!("Failed to build raw message: {}", e))?;

    let mut request = client.send_raw_email().raw_message(raw_msg);

    if let Some(dests) = destinations {
        request = request.set_destinations(Some(dests));
    }

    if let Some(src) = source {
        request = request.source(src);
    }

    if let Some(return_path) = return_path_arn {
        request = request.return_path_arn(return_path);
    }

    if let Some(from) = from_arn {
        request = request.from_arn(from);
    }

    if let Some(src_arn) = source_arn {
        request = request.source_arn(src_arn);
    }

    if let Some(config_set) = configuration_set_name {
        request = request.configuration_set_name(config_set);
    }

    if let Some(msg_tags) = tags {
        for tag in msg_tags {
            let aws_tag = AwsMessageTag::builder()
                .name(&tag.name)
                .value(&tag.value)
                .build()
                .map_err(|e| format!("Failed to build message tag: {}", e))?;
            request = request.tags(aws_tag);
        }
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to send raw email: {}", e))?;

    Ok(SendRawEmailOutput {
        success: true,
        message_id: response.message_id().to_string(),
    })
}

/// Send SES Templated Email
///
/// Sends an email message using a template.
///
pub async fn send_templated_email(
    from_address: &str,
    template_name: &str,
    to_addresses: Vec<String>,
    template_data: &str,
    reply_to_addresses: Option<Vec<String>>,
    bcc_addresses: Option<Vec<String>>,
    tags: Option<Vec<MessageTag>>,
    cc_addresses: Option<Vec<String>>,
    return_path: Option<&str>,
    region: Option<&str>,
    configuration_set_name: Option<&str>,
) -> Result<SendTemplatedEmailOutput, String> {
    let client = create_client(region).await?;

    // Build destination
    let mut destination_builder = Destination::builder()
        .set_to_addresses(Some(to_addresses));

    if let Some(cc) = cc_addresses {
        destination_builder = destination_builder.set_cc_addresses(Some(cc));
    }

    if let Some(bcc) = bcc_addresses {
        destination_builder = destination_builder.set_bcc_addresses(Some(bcc));
    }

    let destination = destination_builder.build();

    let mut request = client
        .send_templated_email()
        .source(from_address)
        .destination(destination)
        .template(template_name)
        .template_data(template_data);

    if let Some(path) = return_path {
        request = request.return_path(path);
    }

    if let Some(reply_to) = reply_to_addresses {
        request = request.set_reply_to_addresses(Some(reply_to));
    }

    if let Some(config_set) = configuration_set_name {
        request = request.configuration_set_name(config_set);
    }

    if let Some(msg_tags) = tags {
        for tag in msg_tags {
            let aws_tag = AwsMessageTag::builder()
                .name(&tag.name)
                .value(&tag.value)
                .build()
                .map_err(|e| format!("Failed to build message tag: {}", e))?;
            request = request.tags(aws_tag);
        }
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to send templated email: {}", e))?;

    Ok(SendTemplatedEmailOutput {
        success: true,
        message_id: response.message_id().to_string(),
    })
}

/// Set SES Identity DKIM Enabled
///
/// Enables or disables Easy DKIM signing of email for an identity.
///
pub async fn set_identity_dkim_enabled(
    dkim_enabled: bool,
    identity: &str,
    region: Option<&str>,
) -> Result<SetIdentityDkimEnabledOutput, String> {
    let client = create_client(region).await?;

    client
        .set_identity_dkim_enabled()
        .identity(identity)
        .dkim_enabled(dkim_enabled)
        .send()
        .await
        .map_err(|e| format!("Failed to set identity DKIM enabled: {}", e))?;

    Ok(SetIdentityDkimEnabledOutput { success: true })
}

/// Set SES Identity Feedback Forwarding
///
/// Enables or disables feedback forwarding for an identity.
///
pub async fn set_identity_feedback_forwarding(
    forwarding_enabled: bool,
    identity: &str,
    region: Option<&str>,
) -> Result<SetIdentityFeedbackForwardingOutput, String> {
    let client = create_client(region).await?;

    client
        .set_identity_feedback_forwarding_enabled()
        .identity(identity)
        .forwarding_enabled(forwarding_enabled)
        .send()
        .await
        .map_err(|e| format!("Failed to set identity feedback forwarding: {}", e))?;

    Ok(SetIdentityFeedbackForwardingOutput { success: true })
}

/// Set SES Identity Notification Topic
///
/// Sets the Amazon SNS topic to which Amazon SES publishes bounce, complaint, or delivery notifications.
///
pub async fn set_identity_notification_topic(
    identity: &str,
    notification_type: &str,
    region: Option<&str>,
    sns_topic: Option<&str>,
) -> Result<SetIdentityNotificationTopicOutput, String> {
    let client = create_client(region).await?;

    let notification_type_enum = match notification_type.to_lowercase().as_str() {
        "bounce" => NotificationType::Bounce,
        "complaint" => NotificationType::Complaint,
        "delivery" => NotificationType::Delivery,
        _ => return Err(format!("Invalid notification type: {}", notification_type)),
    };

    let mut request = client
        .set_identity_notification_topic()
        .identity(identity)
        .notification_type(notification_type_enum);

    if let Some(topic) = sns_topic {
        request = request.sns_topic(topic);
    }

    request
        .send()
        .await
        .map_err(|e| format!("Failed to set identity notification topic: {}", e))?;

    Ok(SetIdentityNotificationTopicOutput { success: true })
}

/// Update SES Template
///
/// Updates an email template.
///
pub async fn update_template(
    template_name: &str,
    subject_part: Option<&str>,
    html_part: Option<&str>,
    text_part: Option<&str>,
    region: Option<&str>,
) -> Result<UpdateTemplateOutput, String> {
    let client = create_client(region).await?;

    // First, get the existing template
    let existing = client
        .get_template()
        .template_name(template_name)
        .send()
        .await
        .map_err(|e| format!("Failed to get existing template: {}", e))?;

    let existing_template = existing.template().ok_or("No template found")?;

    // Build updated template
    let mut template_builder = Template::builder().template_name(template_name);

    // Use new values if provided, otherwise use existing
    if let Some(subject) = subject_part {
        template_builder = template_builder.subject_part(subject);
    } else if let Some(existing_subject) = existing_template.subject_part() {
        template_builder = template_builder.subject_part(existing_subject);
    }

    if let Some(html) = html_part {
        template_builder = template_builder.html_part(html);
    } else if let Some(existing_html) = existing_template.html_part() {
        template_builder = template_builder.html_part(existing_html);
    }

    if let Some(text) = text_part {
        template_builder = template_builder.text_part(text);
    } else if let Some(existing_text) = existing_template.text_part() {
        template_builder = template_builder.text_part(existing_text);
    }

    let template = template_builder.build().map_err(|e| format!("Failed to build template: {}", e))?;

    client
        .update_template()
        .template(template)
        .send()
        .await
        .map_err(|e| format!("Failed to update template: {}", e))?;

    Ok(UpdateTemplateOutput { success: true })
}

/// Verify SES Domain Identity
///
/// Adds a domain to the list of identities and attempts to verify it.
///
pub async fn verify_domain_identity(
    domain: &str,
    region: Option<&str>,
) -> Result<VerifyDomainIdentityOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .verify_domain_identity()
        .domain(domain)
        .send()
        .await
        .map_err(|e| format!("Failed to verify domain identity: {}", e))?;

    Ok(VerifyDomainIdentityOutput {
        success: true,
        verification_token: response.verification_token().to_string(),
    })
}

/// Verify SES Domain DKIM
///
/// Returns a set of DKIM tokens for a domain.
///
pub async fn verify_domain_dkim(
    domain: &str,
    region: Option<&str>,
) -> Result<VerifyDomainDkimOutput, String> {
    let client = create_client(region).await?;

    let response = client
        .verify_domain_dkim()
        .domain(domain)
        .send()
        .await
        .map_err(|e| format!("Failed to verify domain DKIM: {}", e))?;

    Ok(VerifyDomainDkimOutput {
        success: true,
        dkim_tokens: response.dkim_tokens().iter().map(|s| s.to_string()).collect(),
    })
}

/// Verify SES Email Identity
///
/// Adds an email address to the list of identities and attempts to verify it.
///
pub async fn verify_email_identity(
    email_address: &str,
    region: Option<&str>,
) -> Result<VerifyEmailIdentityOutput, String> {
    let client = create_client(region).await?;

    client
        .verify_email_identity()
        .email_address(email_address)
        .send()
        .await
        .map_err(|e| format!("Failed to verify email identity: {}", e))?;

    Ok(VerifyEmailIdentityOutput { success: true })
}
