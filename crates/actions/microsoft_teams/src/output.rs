// Harana Actions - Microsoft Teams Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// send_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageOutput {
    pub created_at: DateTime<Utc>,
    pub message_id: String,
    pub success: bool,
}

// reply_to_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyToMessageOutput {
    pub reply_id: String,
    pub success: bool,
}

// update_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageOutput {
    pub success: bool,
}

// delete_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessageOutput {
    pub success: bool,
}

// send_chat_message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendChatMessageOutput {
    pub message_id: String,
    pub success: bool,
}

// create_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelOutput {
    pub channel_id: String,
    pub success: bool,
    pub web_url: Option<String>,
}

// delete_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteChannelOutput {
    pub success: bool,
}

// list_channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListChannelsOutput {
    pub channels: Vec<TeamsChannel>,
    pub count: i32,
}

// get_channel_messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChannelMessagesOutput {
    pub messages: Vec<TeamsMessage>,
    pub next_link: Option<String>,
}

// create_meeting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMeetingOutput {
    pub join_url: String,
    pub meeting_id: String,
    pub success: bool,
}

// get_team
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTeamOutput {
    pub description: Option<String>,
    pub display_name: String,
    pub id: String,
    pub visibility: String,
}

// list_teams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTeamsOutput {
    pub next_link: Option<String>,
    pub teams: Vec<TeamsTeam>,
}

// list_team_members
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTeamMembersOutput {
    pub members: Vec<TeamsMember>,
    pub next_link: Option<String>,
}

// add_team_member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTeamMemberOutput {
    pub member_id: String,
    pub success: bool,
}

// remove_team_member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveTeamMemberOutput {
    pub success: bool,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsChannel {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub membership_type: String,
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsMessage {
    pub id: String,
    pub body: TeamsMessageBody,
    pub from: Option<TeamsIdentity>,
    pub created_date_time: DateTime<Utc>,
    pub importance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsMessageBody {
    pub content: String,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsIdentity {
    pub user: Option<TeamsUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsUser {
    pub id: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsTeam {
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub visibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsMember {
    pub id: String,
    pub display_name: String,
    pub email: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsAttachment {
    pub content_type: String,
    pub content_url: Option<String>,
    pub content: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsMention {
    pub id: i32,
    pub mentioned: TeamsMentioned,
    pub mention_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsMentioned {
    pub user: TeamsUser,
}
