// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideCalendarEvent {
    PreviousMonth,
    NextMonth,
    SelectDate,
    EditMeeting,
    CancelMeeting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SideCalendar {
    pub current_month_display: String,
    pub days: Option<String>,
    pub meetings: Option<String>,
    pub schedule_title: String,
    pub selected_date_datetime: Option<String>,
    pub selected_date_display: Option<String>,
    pub selected_date: Option<String>,
    pub weekday_labels: Option<String>,
}

