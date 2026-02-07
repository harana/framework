// Harana Actions - Calendar Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// CalendarEvent - full event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub event_id: String,
    pub calendar_id: String,
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub timezone: String,
    pub location: Option<String>,
    pub attendees: Vec<CalendarAttendee>,
    pub recurrence: Option<String>,
    pub status: String,
    pub all_day: bool,
    pub html_link: Option<String>,
    pub reminders: Vec<CalendarReminder>,
}

// Calendar - calendar info structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar {
    pub calendar_id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub timezone: String,
    pub primary: bool,
    pub hidden: bool,
}

// CalendarAttendee
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarAttendee {
    pub email: String,
    pub name: Option<String>,
    pub response_status: String,
    pub optional: bool,
}

// CalendarReminder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarReminder {
    pub method: String,
    pub minutes: i32,
}

// CalendarTimePeriod
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarTimePeriod {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub calendar_id: String,
}

// CalendarTimeSlot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarTimeSlot {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration_minutes: i32,
}

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
    pub event_id: String,
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub location: Option<String>,
    pub attendees: Vec<CalendarAttendee>,
    pub status: String,
    pub html_link: Option<String>,
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
    pub calendars: Vec<Calendar>,
    pub total: i32,
}

// check_availability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckAvailabilityOutput {
    pub busy_periods: Vec<CalendarTimePeriod>,
    pub free_periods: Vec<CalendarTimePeriod>,
}

// add_attendee
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAttendeeOutput {
    pub success: bool,
}

// remove_attendee
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveAttendeeOutput {
    pub success: bool,
}

// respond_to_invitation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespondToInvitationOutput {
    pub success: bool,
}

// find_available_slots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindAvailableSlotsOutput {
    pub available_slots: Vec<CalendarTimeSlot>,
    pub total: i32,
}
