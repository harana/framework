// Harana Actions - Slack Module Output Types
// Output structs for Slack action methods.

use serde::{Deserialize, Serialize};

// send_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageOutput {
    pub channel_id: String,
    pub message_ts: String,
    pub success: bool,
}

// update_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageOutput {
    pub message_ts: String,
    pub success: bool,
}

// delete_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessageOutput {
    pub success: bool,
}

// send_dm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendDmOutput {
    pub channel_id: String,
    pub message_ts: String,
    pub success: bool,
}

// create_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelOutput {
    pub channel_id: String,
    pub name: String,
    pub success: bool,
}

// archive_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveChannelOutput {
    pub success: bool,
}

// unarchive_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnarchiveChannelOutput {
    pub success: bool,
}

// invite_users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteUsersOutput {
    pub success: bool,
}

// kick_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KickUserOutput {
    pub success: bool,
}

// get_channel_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChannelInfoOutput {
    pub channel_id: String,
    pub name: String,
    pub is_private: bool,
    pub is_archived: bool,
    pub num_members: i32,
    pub topic: String,
    pub purpose: String,
}

// list_channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListChannelsOutput {
    pub channels: Vec<SlackChannel>,
}

// get_user_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserInfoOutput {
    pub user_id: String,
    pub name: String,
    pub real_name: String,
    pub email: String,
    pub is_admin: bool,
    pub is_bot: bool,
    pub timezone: String,
}

// list_users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersOutput {
    pub users: Vec<SlackUser>,
}

// add_reaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddReactionOutput {
    pub success: bool,
}

// remove_reaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveReactionOutput {
    pub success: bool,
}

// upload_file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileOutput {
    pub file_id: String,
    pub success: bool,
}

// pin_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinMessageOutput {
    pub success: bool,
}

// unpin_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnpinMessageOutput {
    pub success: bool,
}

// set_channel_topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChannelTopicOutput {
    pub success: bool,
}

// set_channel_purpose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChannelPurposeOutput {
    pub success: bool,
}

// ---- Domain types ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackMessage {
    pub message_ts: String,
    pub channel_id: String,
    pub text: String,
    pub user_id: Option<String>,
    pub thread_ts: Option<String>,
    pub blocks: Option<Vec<SlackBlock>>,
    pub attachments: Option<Vec<SlackAttachment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    pub block_id: Option<String>,
    pub text: Option<SlackTextObject>,
    pub elements: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackTextObject {
    #[serde(rename = "type")]
    pub text_type: String,
    pub text: String,
    pub emoji: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackAttachment {
    pub fallback: Option<String>,
    pub color: Option<String>,
    pub pretext: Option<String>,
    pub title: Option<String>,
    pub title_link: Option<String>,
    pub text: Option<String>,
    pub fields: Option<Vec<SlackAttachmentField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackAttachmentField {
    pub title: String,
    pub value: String,
    pub short: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackChannel {
    pub channel_id: String,
    pub name: String,
    pub is_private: bool,
    pub is_archived: bool,
    pub num_members: i32,
    pub topic: Option<String>,
    pub purpose: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackUser {
    pub user_id: String,
    pub name: String,
    pub real_name: String,
    pub email: Option<String>,
    pub is_admin: bool,
    pub is_bot: bool,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChannelType {
    Public,
    Private,
    Mpim,
    Im,
}