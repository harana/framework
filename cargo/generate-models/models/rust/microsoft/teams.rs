// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// teams_team
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsTeam {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub display_name: String,
    #[serde(default)]
    pub is_archived: bool,
    pub team_id: String,
    pub visibility: String,
    pub web_url: Option<String>,
}

impl TeamsTeam {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// teams_channel
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsChannel {
    pub channel_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub display_name: String,
    pub membership_type: String,
    /// Reference: teams_team.id
    pub team_id: String,
    pub web_url: Option<String>,
}

impl TeamsChannel {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// teams_member
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsMember {
    pub display_name: Option<String>,
    pub email: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub joined_at: chrono::DateTime<chrono::Utc>,
    pub membership_id: Option<String>,
    pub roles: Option<String>,
    /// Reference: teams_team.id
    pub team_id: String,
    pub user_id: String,
}

impl TeamsMember {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// teams_message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsMessage {
    pub channel_id: String,
    pub content: Option<String>,
    pub content_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub importance: String,
    pub message_id: String,
    pub sender_id: Option<String>,
    pub thread_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl TeamsMessage {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// teams_meeting
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsMeeting {
    pub body: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub is_online: bool,
    pub join_url: Option<String>,
    pub location: Option<String>,
    pub meeting_id: String,
    pub organizer_id: Option<String>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub subject: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl TeamsMeeting {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// teams_adaptive_card
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsAdaptiveCard {
    pub body: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub schema_version: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl TeamsAdaptiveCard {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// teams_user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsUser {
    pub user_id: String,
    pub display_name: String,
    pub email: String,
}

impl TeamsUser {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// teams_attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsAttachment {
    pub id: String,
    pub content_type: String,
    pub content_url: String,
    pub name: String,
}

impl TeamsAttachment {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// teams_mention
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsMention {
    pub id: String,
    pub mentioned_id: String,
    pub mentioned_text: String,
}

impl TeamsMention {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// teams_attendee
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TeamsAttendee {
    pub email: String,
    pub display_name: String,
    pub response_status: String,
}

impl TeamsAttendee {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

