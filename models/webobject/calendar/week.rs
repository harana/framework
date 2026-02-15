// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeekCalendarEvent {
    PreviousWeek,
    NextWeek,
    GoToToday,
    SelectView,
    SelectDay,
    AddEvent,
    ViewEvent,
    EditEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WeekCalendar {
    pub add_event_label: String,
    pub current_week_datetime: String,
    pub current_week_display: String,
    pub days: Option<String>,
    pub events: Option<String>,
    pub selected_view: String,
    pub time_slots: Option<String>,
    pub title: String,
    pub today_label: String,
    pub view_options: Option<String>,
}

