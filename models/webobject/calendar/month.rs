// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonthCalendarEvent {
    PreviousMonth,
    NextMonth,
    GoToToday,
    SelectView,
    SelectDate,
    AddEvent,
    ViewEvent,
    EditEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MonthCalendar {
    pub add_event_label: String,
    pub current_month_datetime: String,
    pub current_month_display: String,
    pub days: Option<String>,
    pub selected_view: String,
    pub title: String,
    pub today_label: String,
    pub view_options: Option<String>,
    pub weekday_labels: Option<String>,
}

