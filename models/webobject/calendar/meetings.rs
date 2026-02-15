// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeetingsEvent {
    PreviousMonth,
    NextMonth,
    SelectDate,
    AddEvent,
    EditMeeting,
    CancelMeeting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Meetings {
    pub current_month: String,
    pub days: Option<String>,
    pub meetings: Option<String>,
    pub selected_date: Option<String>,
    pub title: String,
}

