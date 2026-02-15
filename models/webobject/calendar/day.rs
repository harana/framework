// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DayCalendarEvent {
    PreviousDay,
    NextDay,
    GoToToday,
    SelectView,
    SelectDay,
    SelectMiniCalendarDay,
    PreviousMiniCalendarMonth,
    NextMiniCalendarMonth,
    AddEvent,
    ViewEvent,
    EditEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DayCalendar {
    pub add_event_label: String,
    pub current_day_datetime: String,
    pub current_day_display_long: String,
    pub current_day_display_short: String,
    pub events: Option<String>,
    pub mini_calendar_days: Option<String>,
    pub mini_calendar_month_display: String,
    pub selected_view: String,
    pub time_slots: Option<String>,
    pub today_label: String,
    pub view_options: Option<String>,
    pub week_days: Option<String>,
    pub weekday_labels: Option<String>,
    pub weekday_name: String,
}

