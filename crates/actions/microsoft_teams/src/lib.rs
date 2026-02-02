// Harana Actions - Microsoft Teams Module
// This module provides Microsoft Teams integration actions via Microsoft Graph API.

pub mod output;

use chrono::{DateTime, Utc};
use output::*;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

const GRAPH_API_BASE: &str = "https://graph.microsoft.com/v1.0";

/// Creates a reqwest client with the specified timeout.
fn create_client(timeout_secs: u64) -> Result<Client, String> {
    Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))
}

/// Builds authorization header for Microsoft Graph API.
fn build_auth_header(access_token: &str) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    if let Ok(auth_value) = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", access_token)) {
        headers.insert(reqwest::header::AUTHORIZATION, auth_value);
    }
    if let Ok(content_type) = reqwest::header::HeaderValue::from_str("application/json") {
        headers.insert(reqwest::header::CONTENT_TYPE, content_type);
    }
    headers
}

/// Extracts error message from Microsoft Graph API response.
fn extract_error_message(response_body: &str) -> String {
    if let Ok(json) = serde_json::from_str::<Value>(response_body) {
        if let Some(error) = json.get("error") {
            if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                return message.to_string();
            }
        }
    }
    response_body.to_string()
}

/// Send Teams Message
/// 
/// Sends a message to a Microsoft Teams channel.
/// Requires a valid Microsoft Graph API access token with appropriate permissions.
pub async fn send_message(
    access_token: &str,
    channel_id: &str,
    content: &str,
    team_id: &str,
    attachments: Option<Vec<TeamsAttachment>>,
    content_type: Option<&str>,
    importance: Option<&str>,
    mentions: Option<Vec<TeamsMention>>,
) -> Result<SendMessageOutput, String> {
    let client = create_client(30)?;
    let url = format!(
        "{}/teams/{}/channels/{}/messages",
        GRAPH_API_BASE, team_id, channel_id
    );

    let content_type = content_type.unwrap_or("text");
    let importance = importance.unwrap_or("normal");

    let mut body = json!({
        "body": {
            "contentType": content_type,
            "content": content
        },
        "importance": importance
    });

    if let Some(attachments) = attachments {
        let attachments_json: Vec<Value> = attachments
            .iter()
            .map(|a| {
                json!({
                    "contentType": a.content_type,
                    "contentUrl": a.content_url,
                    "content": a.content,
                    "name": a.name
                })
            })
            .collect();
        body["attachments"] = Value::Array(attachments_json);
    }

    if let Some(mentions) = mentions {
        let mentions_json: Vec<Value> = mentions
            .iter()
            .map(|m| {
                json!({
                    "id": m.id,
                    "mentionText": m.mention_text,
                    "mentioned": {
                        "user": {
                            "id": m.mentioned.user.id,
                            "displayName": m.mentioned.user.display_name
                        }
                    }
                })
            })
            .collect();
        body["mentions"] = Value::Array(mentions_json);
    }

    let response = client
        .post(&url)
        .headers(build_auth_header(access_token))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to send message: {}", e))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "Failed to send message: {}",
            extract_error_message(&response_body)
        ));
    }

    let json: Value = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let message_id = json
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("Missing message ID in response")?
        .to_string();

    let created_at = json
        .get("createdDateTime")
        .and_then(|v| v.as_str())
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(Utc::now);

    Ok(SendMessageOutput {
        created_at,
        message_id,
        success: true,
    })
}

/// Reply To Teams Message
/// 
/// Replies to an existing message in a Microsoft Teams channel.
pub async fn reply_to_message(
    access_token: &str,
    channel_id: &str,
    content: &str,
    message_id: &str,
    team_id: &str,
    attachments: Option<Vec<TeamsAttachment>>,
    content_type: Option<&str>,
) -> Result<ReplyToMessageOutput, String> {
    let client = create_client(30)?;
    let url = format!(
        "{}/teams/{}/channels/{}/messages/{}/replies",
        GRAPH_API_BASE, team_id, channel_id, message_id
    );

    let content_type = content_type.unwrap_or("text");

    let mut body = json!({
        "body": {
            "contentType": content_type,
            "content": content
        }
    });

    if let Some(attachments) = attachments {
        let attachments_json: Vec<Value> = attachments
            .iter()
            .map(|a| {
                json!({
                    "contentType": a.content_type,
                    "contentUrl": a.content_url,
                    "content": a.content,
                    "name": a.name
                })
            })
            .collect();
        body["attachments"] = Value::Array(attachments_json);
    }

    let response = client
        .post(&url)
        .headers(build_auth_header(access_token))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to reply to message: {}", e))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "Failed to reply to message: {}",
            extract_error_message(&response_body)
        ));
    }

    let json: Value = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let reply_id = json
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("Missing reply ID in response")?
        .to_string();

    Ok(ReplyToMessageOutput {
        reply_id,
        success: true,
    })
}

/// Update Teams Message
/// 
/// Updates an existing message in a Microsoft Teams channel.
pub async fn update_message(
    access_token: &str,
    channel_id: &str,
    content: &str,
    message_id: &str,
    team_id: &str,
    content_type: Option<&str>,
) -> Result<UpdateMessageOutput, String> {
    let client = create_client(30)?;
    let url = format!(
        "{}/teams/{}/channels/{}/messages/{}",
        GRAPH_API_BASE, team_id, channel_id, message_id
    );

    let content_type = content_type.unwrap_or("text");

    let body = json!({
        "body": {
            "contentType": content_type,
            "content": content
        }
    });

    let response = client
        .patch(&url)
        .headers(build_auth_header(access_token))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to update message: {}", e))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "Failed to update message: {}",
            extract_error_message(&response_body)
        ));
    }

    Ok(UpdateMessageOutput { success: true })
}

/// Delete Teams Message
/// 
/// Soft-deletes a message in a Microsoft Teams channel.
/// Note: Microsoft Graph API performs a soft delete, setting the message content to indicate deletion.
pub async fn delete_message(
    access_token: &str,
    channel_id: &str,
    message_id: &str,
    team_id: &str,
) -> Result<DeleteMessageOutput, String> {
    let client = create_client(30)?;
    let url = format!(
        "{}/teams/{}/channels/{}/messages/{}/softDelete",
        GRAPH_API_BASE, team_id, channel_id, message_id
    );

    let response = client
        .post(&url)
        .headers(build_auth_header(access_token))
        .send()
        .await
        .map_err(|e| format!("Failed to delete message: {}", e))?;

    let status = response.status();

    if !status.is_success() {
        let response_body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        return Err(format!(
            "Failed to delete message: {}",
            extract_error_message(&response_body)
        ));
    }

    Ok(DeleteMessageOutput { success: true })
}

/// Send Teams Chat Message
/// 
/// Sends a message to a Microsoft Teams chat (1:1 or group chat).
pub async fn send_chat_message(
    access_token: &str,
    chat_id: &str,
    content: &str,
    attachments: Option<Vec<TeamsAttachment>>,
    content_type: Option<&str>,
) -> Result<SendChatMessageOutput, String> {
    let client = create_client(30)?;
    let url = format!("{}/chats/{}/messages", GRAPH_API_BASE, chat_id);

    let content_type = content_type.unwrap_or("text");

    let mut body = json!({
        "body": {
            "contentType": content_type,
            "content": content
        }
    });

    if let Some(attachments) = attachments {
        let attachments_json: Vec<Value> = attachments
            .iter()
            .map(|a| {
                json!({
                    "contentType": a.content_type,
                    "contentUrl": a.content_url,
                    "content": a.content,
                    "name": a.name
                })
            })
            .collect();
        body["attachments"] = Value::Array(attachments_json);
    }

    let response = client
        .post(&url)
        .headers(build_auth_header(access_token))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to send chat message: {}", e))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "Failed to send chat message: {}",
            extract_error_message(&response_body)
        ));
    }

    let json: Value = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let message_id = json
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("Missing message ID in response")?
        .to_string();

    Ok(SendChatMessageOutput {
        message_id,
        success: true,
    })
}

/// Create Teams Channel
/// 
/// Creates a new channel in a Microsoft Teams team.
pub async fn create_channel(
    access_token: &str,
    display_name: &str,
    team_id: &str,
    description: Option<&str>,
    membership_type: Option<&str>,
) -> Result<CreateChannelOutput, String> {
    let client = create_client(30)?;
    let url = format!("{}/teams/{}/channels", GRAPH_API_BASE, team_id);

    let membership_type = membership_type.unwrap_or("standard");

    let mut body = json!({
        "displayName": display_name,
        "membershipType": membership_type
    });

    if let Some(desc) = description {
        body["description"] = Value::String(desc.to_string());
    }

    let response = client
        .post(&url)
        .headers(build_auth_header(access_token))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to create channel: {}", e))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "Failed to create channel: {}",
            extract_error_message(&response_body)
        ));
    }

    let json: Value = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let channel_id = json
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("Missing channel ID in response")?
        .to_string();

    let web_url = json
        .get("webUrl")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(CreateChannelOutput {
        channel_id,
        success: true,
        web_url,
    })
}

/// Delete Teams Channel
/// 
/// Deletes a channel from a Microsoft Teams team.
pub async fn delete_channel(
    access_token: &str,
    channel_id: &str,
    team_id: &str,
) -> Result<DeleteChannelOutput, String> {
    let client = create_client(30)?;
    let url = format!(
        "{}/teams/{}/channels/{}",
        GRAPH_API_BASE, team_id, channel_id
    );

    let response = client
        .delete(&url)
        .headers(build_auth_header(access_token))
        .send()
        .await
        .map_err(|e| format!("Failed to delete channel: {}", e))?;

    let status = response.status();

    if !status.is_success() {
        let response_body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        return Err(format!(
            "Failed to delete channel: {}",
            extract_error_message(&response_body)
        ));
    }

    Ok(DeleteChannelOutput { success: true })
}

/// List Team Channels
/// 
/// Lists all channels in a Microsoft Teams team.
pub async fn list_channels(
    access_token: &str,
    team_id: &str,
    filter: Option<&str>,
) -> Result<ListChannelsOutput, String> {
    let client = create_client(30)?;
    let mut url = format!("{}/teams/{}/channels", GRAPH_API_BASE, team_id);

    if let Some(filter) = filter {
        url = format!("{}?$filter={}", url, urlencoding::encode(filter));
    }

    let response = client
        .get(&url)
        .headers(build_auth_header(access_token))
        .send()
        .await
        .map_err(|e| format!("Failed to list channels: {}", e))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "Failed to list channels: {}",
            extract_error_message(&response_body)
        ));
    }

    let json: Value = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let channels: Vec<TeamsChannel> = json
        .get("value")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|c| {
                    Some(TeamsChannel {
                        id: c.get("id")?.as_str()?.to_string(),
                        display_name: c.get("displayName")?.as_str()?.to_string(),
                        description: c.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        membership_type: c
                            .get("membershipType")
                            .and_then(|v| v.as_str())
                            .unwrap_or("standard")
                            .to_string(),
                        web_url: c.get("webUrl").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let count = channels.len() as i32;

    Ok(ListChannelsOutput { channels, count })
}

/// Get Channel Messages
/// 
/// Retrieves messages from a Microsoft Teams channel.
pub async fn get_channel_messages(
    access_token: &str,
    channel_id: &str,
    team_id: &str,
    order_by: Option<&str>,
    top: Option<i32>,
) -> Result<GetChannelMessagesOutput, String> {
    let client = create_client(30)?;
    let mut url = format!(
        "{}/teams/{}/channels/{}/messages",
        GRAPH_API_BASE, team_id, channel_id
    );

    let mut query_params = Vec::new();
    if let Some(order_by) = order_by {
        query_params.push(format!("$orderby={}", urlencoding::encode(order_by)));
    }
    if let Some(top) = top {
        query_params.push(format!("$top={}", top));
    }
    if !query_params.is_empty() {
        url = format!("{}?{}", url, query_params.join("&"));
    }

    let response = client
        .get(&url)
        .headers(build_auth_header(access_token))
        .send()
        .await
        .map_err(|e| format!("Failed to get channel messages: {}", e))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "Failed to get channel messages: {}",
            extract_error_message(&response_body)
        ));
    }

    let json: Value = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let messages: Vec<TeamsMessage> = json
        .get("value")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|m| {
                    let body = m.get("body")?;
                    let created_date_time = m
                        .get("createdDateTime")
                        .and_then(|v| v.as_str())
                        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(Utc::now);

                    Some(TeamsMessage {
                        id: m.get("id")?.as_str()?.to_string(),
                        body: TeamsMessageBody {
                            content: body.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                            content_type: body
                                .get("contentType")
                                .and_then(|v| v.as_str())
                                .unwrap_or("text")
                                .to_string(),
                        },
                        from: m.get("from").and_then(|f| {
                            Some(TeamsIdentity {
                                user: f.get("user").and_then(|u| {
                                    Some(TeamsUser {
                                        id: u.get("id")?.as_str()?.to_string(),
                                        display_name: u.get("displayName").and_then(|v| v.as_str()).map(|s| s.to_string()),
                                    })
                                }),
                            })
                        }),
                        created_date_time,
                        importance: m
                            .get("importance")
                            .and_then(|v| v.as_str())
                            .unwrap_or("normal")
                            .to_string(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let next_link = json
        .get("@odata.nextLink")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(GetChannelMessagesOutput {
        messages,
        next_link,
    })
}

/// Create Teams Meeting
/// 
/// Creates an online meeting using Microsoft Graph API.
/// Note: This creates an online meeting, which can be accessed via Teams.
pub async fn create_meeting(
    access_token: &str,
    subject: &str,
    start_date_time: DateTime<Utc>,
    end_date_time: DateTime<Utc>,
    attendees: Option<Vec<String>>,
    is_online_meeting: Option<bool>,
    location: Option<&str>,
) -> Result<CreateMeetingOutput, String> {
    let client = create_client(30)?;
    let url = format!("{}/me/onlineMeetings", GRAPH_API_BASE);

    let mut body = json!({
        "subject": subject,
        "startDateTime": start_date_time.to_rfc3339(),
        "endDateTime": end_date_time.to_rfc3339()
    });

    if let Some(attendees) = attendees {
        let participants: Vec<Value> = attendees
            .iter()
            .map(|email| {
                json!({
                    "upn": email
                })
            })
            .collect();
        body["participants"] = json!({
            "attendees": participants
        });
    }

    if let Some(is_online) = is_online_meeting {
        body["isOnlineMeeting"] = Value::Bool(is_online);
    }

    if let Some(loc) = location {
        body["lobbyBypassSettings"] = json!({
            "scope": "everyone"
        });
        // Store location info in the subject or use chatInfo
        body["subject"] = Value::String(format!("{} ({})", subject, loc));
    }

    let response = client
        .post(&url)
        .headers(build_auth_header(access_token))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to create meeting: {}", e))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "Failed to create meeting: {}",
            extract_error_message(&response_body)
        ));
    }

    let json: Value = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let meeting_id = json
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("Missing meeting ID in response")?
        .to_string();

    let join_url = json
        .get("joinWebUrl")
        .and_then(|v| v.as_str())
        .ok_or("Missing join URL in response")?
        .to_string();

    Ok(CreateMeetingOutput {
        join_url,
        meeting_id,
        success: true,
    })
}

/// Get Team
/// 
/// Retrieves information about a specific Microsoft Teams team.
pub async fn get_team(
    access_token: &str,
    team_id: &str,
) -> Result<GetTeamOutput, String> {
    let client = create_client(30)?;
    let url = format!("{}/teams/{}", GRAPH_API_BASE, team_id);

    let response = client
        .get(&url)
        .headers(build_auth_header(access_token))
        .send()
        .await
        .map_err(|e| format!("Failed to get team: {}", e))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "Failed to get team: {}",
            extract_error_message(&response_body)
        ));
    }

    let json: Value = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let id = json
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("Missing team ID in response")?
        .to_string();

    let display_name = json
        .get("displayName")
        .and_then(|v| v.as_str())
        .ok_or("Missing display name in response")?
        .to_string();

    let description = json
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let visibility = json
        .get("visibility")
        .and_then(|v| v.as_str())
        .unwrap_or("private")
        .to_string();

    Ok(GetTeamOutput {
        description,
        display_name,
        id,
        visibility,
    })
}

/// List Teams
/// 
/// Lists all teams the authenticated user is a member of.
pub async fn list_teams(
    access_token: &str,
    filter: Option<&str>,
    top: Option<i32>,
) -> Result<ListTeamsOutput, String> {
    let client = create_client(30)?;
    let mut url = format!("{}/me/joinedTeams", GRAPH_API_BASE);

    let mut query_params = Vec::new();
    if let Some(filter) = filter {
        query_params.push(format!("$filter={}", urlencoding::encode(filter)));
    }
    if let Some(top) = top {
        query_params.push(format!("$top={}", top));
    }
    if !query_params.is_empty() {
        url = format!("{}?{}", url, query_params.join("&"));
    }

    let response = client
        .get(&url)
        .headers(build_auth_header(access_token))
        .send()
        .await
        .map_err(|e| format!("Failed to list teams: {}", e))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "Failed to list teams: {}",
            extract_error_message(&response_body)
        ));
    }

    let json: Value = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let teams: Vec<TeamsTeam> = json
        .get("value")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|t| {
                    Some(TeamsTeam {
                        id: t.get("id")?.as_str()?.to_string(),
                        display_name: t.get("displayName")?.as_str()?.to_string(),
                        description: t.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        visibility: t
                            .get("visibility")
                            .and_then(|v| v.as_str())
                            .unwrap_or("private")
                            .to_string(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let next_link = json
        .get("@odata.nextLink")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(ListTeamsOutput { next_link, teams })
}

/// List Team Members
/// 
/// Lists all members of a Microsoft Teams team.
pub async fn list_team_members(
    access_token: &str,
    team_id: &str,
    filter: Option<&str>,
) -> Result<ListTeamMembersOutput, String> {
    let client = create_client(30)?;
    let mut url = format!("{}/teams/{}/members", GRAPH_API_BASE, team_id);

    if let Some(filter) = filter {
        url = format!("{}?$filter={}", url, urlencoding::encode(filter));
    }

    let response = client
        .get(&url)
        .headers(build_auth_header(access_token))
        .send()
        .await
        .map_err(|e| format!("Failed to list team members: {}", e))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "Failed to list team members: {}",
            extract_error_message(&response_body)
        ));
    }

    let json: Value = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let members: Vec<TeamsMember> = json
        .get("value")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|m| {
                    Some(TeamsMember {
                        id: m.get("id")?.as_str()?.to_string(),
                        display_name: m.get("displayName")?.as_str()?.to_string(),
                        email: m.get("email").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        roles: m
                            .get("roles")
                            .and_then(|v| v.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|r| r.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let next_link = json
        .get("@odata.nextLink")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(ListTeamMembersOutput { members, next_link })
}

/// Add Team Member
/// 
/// Adds a member to a Microsoft Teams team.
pub async fn add_team_member(
    access_token: &str,
    team_id: &str,
    user_id: &str,
    roles: Option<Vec<String>>,
) -> Result<AddTeamMemberOutput, String> {
    let client = create_client(30)?;
    let url = format!("{}/teams/{}/members", GRAPH_API_BASE, team_id);

    let roles = roles.unwrap_or_default();

    let body = json!({
        "@odata.type": "#microsoft.graph.aadUserConversationMember",
        "roles": roles,
        "user@odata.bind": format!("https://graph.microsoft.com/v1.0/users('{}')", user_id)
    });

    let response = client
        .post(&url)
        .headers(build_auth_header(access_token))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to add team member: {}", e))?;

    let status = response.status();
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!(
            "Failed to add team member: {}",
            extract_error_message(&response_body)
        ));
    }

    let json: Value = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let member_id = json
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("Missing member ID in response")?
        .to_string();

    Ok(AddTeamMemberOutput {
        member_id,
        success: true,
    })
}

/// Remove Team Member
/// 
/// Removes a member from a Microsoft Teams team.
pub async fn remove_team_member(
    access_token: &str,
    member_id: &str,
    team_id: &str,
) -> Result<RemoveTeamMemberOutput, String> {
    let client = create_client(30)?;
    let url = format!(
        "{}/teams/{}/members/{}",
        GRAPH_API_BASE, team_id, member_id
    );

    let response = client
        .delete(&url)
        .headers(build_auth_header(access_token))
        .send()
        .await
        .map_err(|e| format!("Failed to remove team member: {}", e))?;

    let status = response.status();

    if !status.is_success() {
        let response_body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        return Err(format!(
            "Failed to remove team member: {}",
            extract_error_message(&response_body)
        ));
    }

    Ok(RemoveTeamMemberOutput { success: true })
}
