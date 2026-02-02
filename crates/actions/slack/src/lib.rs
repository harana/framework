// Harana Actions - Slack Module
// This module provides Slack messaging and channel management actions using slack-morphism.

pub mod output;

use output::*;
use slack_morphism::prelude::*;
use std::sync::Arc;

type SlackHyperClient = SlackClient<SlackClientHyperConnector<hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>>>;

/// Create a Slack client session from a token
fn create_session(token: &str) -> Result<(Arc<SlackHyperClient>, SlackApiToken), String> {
    let client = SlackClient::new(
        SlackClientHyperConnector::new()
            .map_err(|e| format!("Failed to create Slack connector: {}", e))?
    );
    let token_value: SlackApiTokenValue = token.into();
    let api_token = SlackApiToken::new(token_value);
    Ok((Arc::new(client), api_token))
}

/// Convert our attachments to slack-morphism format
fn convert_attachments_to_slack(
    attachments: Option<Vec<crate::output::SlackAttachment>>,
) -> Option<Vec<SlackMessageAttachment>> {
    attachments.map(|atts| {
        atts.into_iter()
            .map(|att| {
                let mut slack_att = SlackMessageAttachment::new();
                if let Some(fallback) = att.fallback {
                    slack_att = slack_att.with_fallback(fallback);
                }
                if let Some(color) = att.color {
                    slack_att = slack_att.with_color(color);
                }
                if let Some(title) = att.title {
                    slack_att = slack_att.with_title(title);
                }
                if let Some(text) = att.text {
                    slack_att = slack_att.with_text(text);
                }
                if let Some(fields) = att.fields {
                    let slack_fields: Vec<SlackMessageAttachmentFieldObject> = fields
                        .into_iter()
                        .map(|f| {
                            let mut field = SlackMessageAttachmentFieldObject::new()
                                .with_title(f.title)
                                .with_value(f.value);
                            if let Some(short) = f.short {
                                field = field.with_short(short);
                            }
                            field
                        })
                        .collect();
                    slack_att = slack_att.with_fields(slack_fields);
                }
                slack_att
            })
            .collect()
    })
}

/// Send a message to a Slack channel
pub async fn send_message(
    token: &str,
    channel_id: &str,
    text: &str,
    attachments: Option<Vec<crate::output::SlackAttachment>>,
    _blocks: Option<Vec<crate::output::SlackBlock>>,
    icon_emoji: Option<&str>,
    icon_url: Option<&str>,
    thread_ts: Option<&str>,
    username: Option<&str>,
) -> Result<SendMessageOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let mut content = SlackMessageContent::new().with_text(text.to_string());

    if let Some(slack_attachments) = convert_attachments_to_slack(attachments) {
        content = content.with_attachments(slack_attachments);
    }

    let mut request = SlackApiChatPostMessageRequest::new(
        SlackChannelId::new(channel_id.to_string()),
        content,
    );

    if let Some(emoji) = icon_emoji {
        request = request.with_icon_emoji(emoji.to_string());
    }
    if let Some(url) = icon_url {
        request = request.with_icon_url(url.to_string());
    }
    if let Some(ts) = thread_ts {
        request = request.with_thread_ts(SlackTs::new(ts.to_string()));
    }
    if let Some(name) = username {
        request = request.with_username(name.to_string());
    }

    let response: SlackApiChatPostMessageResponse = session
        .chat_post_message(&request)
        .await
        .map_err(|e| format!("Failed to send message: {}", e))?;

    Ok(SendMessageOutput {
        channel_id: response.channel.to_string(),
        message_ts: response.ts.to_string(),
        success: true,
    })
}

/// Update an existing message in a Slack channel
pub async fn update_message(
    token: &str,
    channel_id: &str,
    text: &str,
    timestamp: &str,
    attachments: Option<Vec<crate::output::SlackAttachment>>,
    _blocks: Option<Vec<crate::output::SlackBlock>>,
) -> Result<UpdateMessageOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let mut content = SlackMessageContent::new().with_text(text.to_string());

    if let Some(slack_attachments) = convert_attachments_to_slack(attachments) {
        content = content.with_attachments(slack_attachments);
    }

    let request = SlackApiChatUpdateRequest::new(
        SlackChannelId::new(channel_id.to_string()),
        content,
        SlackTs::new(timestamp.to_string()),
    );

    let response: SlackApiChatUpdateResponse = session
        .chat_update(&request)
        .await
        .map_err(|e| format!("Failed to update message: {}", e))?;

    Ok(UpdateMessageOutput {
        message_ts: response.ts.to_string(),
        success: true,
    })
}

/// Delete a message from a Slack channel
pub async fn delete_message(
    token: &str,
    channel_id: &str,
    timestamp: &str,
) -> Result<DeleteMessageOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let request = SlackApiChatDeleteRequest::new(
        SlackChannelId::new(channel_id.to_string()),
        SlackTs::new(timestamp.to_string()),
    );

    session
        .chat_delete(&request)
        .await
        .map_err(|e| format!("Failed to delete message: {}", e))?;

    Ok(DeleteMessageOutput { success: true })
}

/// Send a direct message to a user
pub async fn send_dm(
    token: &str,
    user_id: &str,
    text: &str,
    attachments: Option<Vec<crate::output::SlackAttachment>>,
    _blocks: Option<Vec<crate::output::SlackBlock>>,
) -> Result<SendDmOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let open_request = SlackApiConversationsOpenRequest::new()
        .with_users(vec![SlackUserId::new(user_id.to_string())]);

    let open_response = session
        .conversations_open(&open_request)
        .await
        .map_err(|e| format!("Failed to open DM channel: {}", e))?;

    let dm_channel_id = open_response.channel.id.to_string();

    let mut content = SlackMessageContent::new().with_text(text.to_string());

    if let Some(slack_attachments) = convert_attachments_to_slack(attachments) {
        content = content.with_attachments(slack_attachments);
    }

    let request = SlackApiChatPostMessageRequest::new(
        SlackChannelId::new(dm_channel_id.clone()),
        content,
    );

    let response: SlackApiChatPostMessageResponse = session
        .chat_post_message(&request)
        .await
        .map_err(|e| format!("Failed to send DM: {}", e))?;

    Ok(SendDmOutput {
        channel_id: dm_channel_id,
        message_ts: response.ts.to_string(),
        success: true,
    })
}

/// Create a new Slack channel
pub async fn create_channel(
    token: &str,
    name: &str,
    description: Option<&str>,
    is_private: Option<bool>,
) -> Result<CreateChannelOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let mut request = SlackApiConversationsCreateRequest::new(name.to_string());

    if let Some(private) = is_private {
        request = request.with_is_private(private);
    }

    let response: SlackApiConversationsCreateResponse = session
        .conversations_create(&request)
        .await
        .map_err(|e| format!("Failed to create channel: {}", e))?;

    let channel_id = response.channel.id.to_string();
    let channel_name = response.channel.name.unwrap_or_default();

    if let Some(desc) = description {
        let purpose_request = SlackApiConversationsSetPurposeRequest::new(
            SlackChannelId::new(channel_id.clone()),
            desc.to_string(),
        );
        let _ = session.conversations_set_purpose(&purpose_request).await;
    }

    Ok(CreateChannelOutput {
        channel_id,
        name: channel_name,
        success: true,
    })
}

/// Archive a Slack channel
pub async fn archive_channel(
    token: &str,
    channel_id: &str,
) -> Result<ArchiveChannelOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let request = SlackApiConversationsArchiveRequest::new(
        SlackChannelId::new(channel_id.to_string()),
    );

    session
        .conversations_archive(&request)
        .await
        .map_err(|e| format!("Failed to archive channel: {}", e))?;

    Ok(ArchiveChannelOutput { success: true })
}

/// Unarchive a Slack channel
pub async fn unarchive_channel(
    token: &str,
    channel_id: &str,
) -> Result<UnarchiveChannelOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let request = SlackApiConversationsUnarchiveRequest::new(
        SlackChannelId::new(channel_id.to_string()),
    );

    session
        .conversations_unarchive(&request)
        .await
        .map_err(|e| format!("Failed to unarchive channel: {}", e))?;

    Ok(UnarchiveChannelOutput { success: true })
}

/// Invite users to a Slack channel
pub async fn invite_users(
    token: &str,
    channel_id: &str,
    user_ids: Vec<String>,
) -> Result<InviteUsersOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let slack_user_ids: Vec<SlackUserId> = user_ids
        .into_iter()
        .map(SlackUserId::new)
        .collect();

    let request = SlackApiConversationsInviteRequest::new(
        SlackChannelId::new(channel_id.to_string()),
        slack_user_ids,
    );

    session
        .conversations_invite(&request)
        .await
        .map_err(|e| format!("Failed to invite users: {}", e))?;

    Ok(InviteUsersOutput { success: true })
}

/// Kick (remove) a user from a Slack channel
pub async fn kick_user(
    token: &str,
    channel_id: &str,
    user_id: &str,
) -> Result<KickUserOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let request = SlackApiConversationsKickRequest::new(
        SlackChannelId::new(channel_id.to_string()),
        SlackUserId::new(user_id.to_string()),
    );

    session
        .conversations_kick(&request)
        .await
        .map_err(|e| format!("Failed to kick user: {}", e))?;

    Ok(KickUserOutput { success: true })
}

/// Get information about a Slack channel
pub async fn get_channel_info(
    token: &str,
    channel_id: &str,
) -> Result<GetChannelInfoOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let request = SlackApiConversationsInfoRequest::new(
        SlackChannelId::new(channel_id.to_string()),
    );

    let response: SlackApiConversationsInfoResponse = session
        .conversations_info(&request)
        .await
        .map_err(|e| format!("Failed to get channel info: {}", e))?;

    let channel = response.channel;
    let flags = &channel.flags;

    Ok(GetChannelInfoOutput {
        channel_id: channel.id.to_string(),
        name: channel.name.unwrap_or_default(),
        is_private: flags.is_private.unwrap_or(false),
        is_archived: flags.is_archived.unwrap_or(false),
        num_members: channel.num_members.unwrap_or(0) as i32,
        topic: channel.topic.map(|t| t.value).unwrap_or_default(),
        purpose: channel.purpose.map(|p| p.value).unwrap_or_default(),
    })
}

/// List Slack channels
pub async fn list_channels(
    token: &str,
    exclude_archived: Option<bool>,
    limit: Option<i32>,
    types: Option<ChannelType>,
) -> Result<ListChannelsOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let mut request = SlackApiConversationsListRequest::new();

    if let Some(exclude) = exclude_archived {
        request = request.with_exclude_archived(exclude);
    }

    if let Some(lim) = limit {
        request = request.with_limit(lim as u16);
    }

    if let Some(channel_type) = types {
        let conv_type = match channel_type {
            ChannelType::Public => SlackConversationType::Public,
            ChannelType::Private => SlackConversationType::Private,
            ChannelType::Mpim => SlackConversationType::Mpim,
            ChannelType::Im => SlackConversationType::Im,
        };
        request = request.with_types(vec![conv_type]);
    }

    let response: SlackApiConversationsListResponse = session
        .conversations_list(&request)
        .await
        .map_err(|e| format!("Failed to list channels: {}", e))?;

    let channels: Vec<crate::output::SlackChannel> = response
        .channels
        .into_iter()
        .map(|ch| {
            let flags = &ch.flags;
            crate::output::SlackChannel {
                channel_id: ch.id.to_string(),
                name: ch.name.unwrap_or_default(),
                is_private: flags.is_private.unwrap_or(false),
                is_archived: flags.is_archived.unwrap_or(false),
                num_members: ch.num_members.unwrap_or(0) as i32,
                topic: ch.topic.map(|t| t.value),
                purpose: ch.purpose.map(|p| p.value),
            }
        })
        .collect();

    Ok(ListChannelsOutput { channels })
}

/// Get information about a Slack user
pub async fn get_user_info(
    token: &str,
    user_id: &str,
) -> Result<GetUserInfoOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let request = SlackApiUsersInfoRequest::new(SlackUserId::new(user_id.to_string()));

    let response: SlackApiUsersInfoResponse = session
        .users_info(&request)
        .await
        .map_err(|e| format!("Failed to get user info: {}", e))?;

    let user = response.user;
    let flags = &user.flags;

    Ok(GetUserInfoOutput {
        user_id: user.id.to_string(),
        name: user.name.unwrap_or_default(),
        real_name: user.profile.as_ref().and_then(|p| p.real_name.clone()).unwrap_or_default(),
        email: user.profile.as_ref().and_then(|p| p.email.as_ref().map(|e| e.to_string())).unwrap_or_default(),
        is_admin: flags.is_admin.unwrap_or(false),
        is_bot: flags.is_bot.unwrap_or(false),
        timezone: user.tz.unwrap_or_default(),
    })
}

/// List Slack users
pub async fn list_users(
    token: &str,
    limit: Option<i32>,
) -> Result<ListUsersOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let mut request = SlackApiUsersListRequest::new();

    if let Some(lim) = limit {
        request = request.with_limit(lim as u16);
    }

    let response: SlackApiUsersListResponse = session
        .users_list(&request)
        .await
        .map_err(|e| format!("Failed to list users: {}", e))?;

    let users: Vec<crate::output::SlackUser> = response
        .members
        .into_iter()
        .map(|u| {
            let flags = &u.flags;
            crate::output::SlackUser {
                user_id: u.id.to_string(),
                name: u.name.unwrap_or_default(),
                real_name: u.profile.as_ref().and_then(|p| p.real_name.clone()).unwrap_or_default(),
                email: u.profile.as_ref().and_then(|p| p.email.as_ref().map(|e| e.to_string())),
                is_admin: flags.is_admin.unwrap_or(false),
                is_bot: flags.is_bot.unwrap_or(false),
                timezone: u.tz,
            }
        })
        .collect();

    Ok(ListUsersOutput { users })
}

/// Add a reaction (emoji) to a message
pub async fn add_reaction(
    token: &str,
    channel_id: &str,
    emoji: &str,
    timestamp: &str,
) -> Result<AddReactionOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let request = SlackApiReactionsAddRequest::new(
        SlackChannelId::new(channel_id.to_string()),
        SlackReactionName::new(emoji.to_string()),
        SlackTs::new(timestamp.to_string()),
    );

    session
        .reactions_add(&request)
        .await
        .map_err(|e| format!("Failed to add reaction: {}", e))?;

    Ok(AddReactionOutput { success: true })
}

/// Remove a reaction (emoji) from a message
pub async fn remove_reaction(
    token: &str,
    channel_id: &str,
    emoji: &str,
    timestamp: &str,
) -> Result<RemoveReactionOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let request = SlackApiReactionsRemoveRequest::new(
        SlackReactionName::new(emoji.to_string()),
    )
    .with_channel(SlackChannelId::new(channel_id.to_string()))
    .with_timestamp(SlackTs::new(timestamp.to_string()));

    session
        .reactions_remove(&request)
        .await
        .map_err(|e| format!("Failed to remove reaction: {}", e))?;

    Ok(RemoveReactionOutput { success: true })
}

/// Upload a file to Slack
pub async fn upload_file(
    token: &str,
    channels: Option<Vec<String>>,
    content: Option<&str>,
    file: Option<&str>,
    filename: Option<&str>,
    initial_comment: Option<&str>,
    title: Option<&str>,
) -> Result<UploadFileOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let file_content = if let Some(c) = content {
        c.as_bytes().to_vec()
    } else if let Some(file_path) = file {
        tokio::fs::read(file_path)
            .await
            .map_err(|e| format!("Failed to read file: {}", e))?
    } else {
        return Err("Either content or file must be provided".to_string());
    };

    let fname = filename
        .or(file)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "file".to_string());

    let get_url_request = SlackApiFilesGetUploadUrlExternalRequest::new(
        fname.clone(),
        file_content.len(),
    );

    let url_response = session
        .get_upload_url_external(&get_url_request)
        .await
        .map_err(|e| format!("Failed to get upload URL: {}", e))?;

    let http_client = reqwest::Client::new();
    http_client
        .post(url_response.upload_url.0.to_string())
        .body(file_content)
        .send()
        .await
        .map_err(|e| format!("Failed to upload file content: {}", e))?;

    let file_info = SlackApiFilesComplete {
        id: url_response.file_id.clone(),
        title: title.map(|t| t.to_string()),
    };

    let mut complete_request = SlackApiFilesCompleteUploadExternalRequest::new(vec![file_info]);

    if let Some(ch) = channels {
        if let Some(first_channel) = ch.first() {
            complete_request = complete_request.with_channel_id(SlackChannelId::new(first_channel.clone()));
        }
    }

    if let Some(comment) = initial_comment {
        complete_request = complete_request.with_initial_comment(comment.to_string());
    }

    let complete_response = session
        .files_complete_upload_external(&complete_request)
        .await
        .map_err(|e| format!("Failed to complete upload: {}", e))?;

    let file_id = complete_response
        .files
        .first()
        .map(|f| f.id.to_string())
        .unwrap_or_else(|| url_response.file_id.to_string());

    Ok(UploadFileOutput {
        file_id,
        success: true,
    })
}

/// Pin a message to a channel
pub async fn pin_message(
    token: &str,
    channel_id: &str,
    timestamp: &str,
) -> Result<PinMessageOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let request = SlackApiPinsAddRequest::new(
        SlackChannelId::new(channel_id.to_string()),
        SlackTs::new(timestamp.to_string()),
    );

    session
        .pins_add(&request)
        .await
        .map_err(|e| format!("Failed to pin message: {}", e))?;

    Ok(PinMessageOutput { success: true })
}

/// Unpin a message from a channel
pub async fn unpin_message(
    token: &str,
    channel_id: &str,
    timestamp: &str,
) -> Result<UnpinMessageOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let request = SlackApiPinsRemoveRequest::new(
        SlackChannelId::new(channel_id.to_string()),
        SlackTs::new(timestamp.to_string()),
    );

    session
        .pins_remove(&request)
        .await
        .map_err(|e| format!("Failed to unpin message: {}", e))?;

    Ok(UnpinMessageOutput { success: true })
}

/// Set the topic of a Slack channel
pub async fn set_channel_topic(
    token: &str,
    channel_id: &str,
    topic: &str,
) -> Result<SetChannelTopicOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let request = SlackApiConversationsSetTopicRequest::new(
        SlackChannelId::new(channel_id.to_string()),
        topic.to_string(),
    );

    session
        .conversations_set_topic(&request)
        .await
        .map_err(|e| format!("Failed to set channel topic: {}", e))?;

    Ok(SetChannelTopicOutput { success: true })
}

/// Set the purpose of a Slack channel
pub async fn set_channel_purpose(
    token: &str,
    channel_id: &str,
    purpose: &str,
) -> Result<SetChannelPurposeOutput, String> {
    let (client, api_token) = create_session(token)?;
    let session = client.open_session(&api_token);

    let request = SlackApiConversationsSetPurposeRequest::new(
        SlackChannelId::new(channel_id.to_string()),
        purpose.to_string(),
    );

    session
        .conversations_set_purpose(&request)
        .await
        .map_err(|e| format!("Failed to set channel purpose: {}", e))?;

    Ok(SetChannelPurposeOutput { success: true })
}
