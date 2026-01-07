// Harana Actions - Slack Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// add_reaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddReactionOutput {
    pub success: bool
}

// archive_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveChannelOutput {
    pub success: bool
}

// create_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelOutput {
    pub name: String,
    pub success: bool,
    pub channel_id: String
}

// delete_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessageOutput {
    pub success: bool
}

// get_channel_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChannelInfoOutput {
    pub is_archived: bool,
    pub name: String,
    pub topic: String,
    pub num_members: i32,
    pub purpose: String,
    pub is_private: bool,
    pub channel_id: String
}

// get_user_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserInfoOutput {
    pub real_name: String,
    pub is_admin: bool,
    pub timezone: String,
    pub is_bot: bool,
    pub user_id: String,
    pub name: String,
    pub email: String
}

// invite_users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteUsersOutput {
    pub success: bool
}

// kick_user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KickUserOutput {
    pub success: bool
}

// list_channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListChannelsOutput {
    pub channels: Vec<HashMap<String, Value>>
}

// list_users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersOutput {
    pub users: Vec<HashMap<String, Value>>
}

// pin_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinMessageOutput {
    pub success: bool
}

// remove_reaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveReactionOutput {
    pub success: bool
}

// send_dm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendDmOutput {
    pub success: bool,
    pub channel_id: String,
    pub message_ts: String
}

// send_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageOutput {
    pub message_ts: String,
    pub channel_id: String,
    pub success: bool
}

// set_channel_purpose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChannelPurposeOutput {
    pub success: bool
}

// set_channel_topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetChannelTopicOutput {
    pub success: bool
}

// unarchive_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnarchiveChannelOutput {
    pub success: bool
}

// unpin_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnpinMessageOutput {
    pub success: bool
}

// update_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageOutput {
    pub success: bool,
    pub message_ts: String
}

// upload_file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileOutput {
    pub file_id: String,
    pub success: bool
}
