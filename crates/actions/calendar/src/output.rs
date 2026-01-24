// Harana Actions - Calendar Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// create_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventOutput {
    pub event_id: String,
    pub html_link: Option<String>,
    pub success: bool,
}

// update_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEventOutput {
    pub event_id: String,
    pub success: bool,
}

// delete_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEventOutput {
    pub success: bool,
}

// get_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEventOutput {
    pub attendees: Vec<CalendarAttendee>,
    pub description: Option<String>,
    pub end_time: DateTime<Utc>,
    pub event_id: String,
    pub html_link: Option<String>,
    pub location: Option<String>,
    pub start_time: DateTime<Utc>,
    pub status: String,
    pub title: String,
}

// list_events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEventsOutput {
    pub events: Vec<CalendarEvent>,
    pub next_page_token: Option<String>,
    pub total: i32,
}

// create_calendar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCalendarOutput {
    pub calendar_id: String,
    pub success: bool,
}

// delete_calendar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCalendarOutput {
    pub success: bool,
}

// list_calendars
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCalendarsOutput {
    pub calendars: Vec<CalendarInfo>,
    pub next_page_token: Option<String>,
}

// Helper structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub event_id: String,
    pub title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: String,
    pub all_day: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarAttendee {
    pub email: String,
    pub name: Option<String>,
    pub response_status: String,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarReminder {
    pub method: String,
    pub minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarInfo {
    pub calendar_id: String,
    pub name: String,
    pub description: Option<String>,
    pub timezone: String,
    pub color: Option<String>,
}
