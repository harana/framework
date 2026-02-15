// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum YearCalendarEvent {
    PreviousYear,
    NextYear,
    GoToToday,
    SelectView,
    SelectDate,
    AddEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct YearCalendar {
    pub add_event_label: String,
    pub current_year_datetime: String,
    pub current_year: String,
    pub months: Option<String>,
    pub selected_view: String,
    pub today_label: String,
    pub view_options: Option<String>,
    pub weekday_labels: Option<String>,
}

