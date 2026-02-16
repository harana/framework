// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEventOutput {
    pub event_id: String,
    pub html_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEventOutput {
    pub attendees: Vec<String>,
    pub description: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub event_id: String,
    pub html_link: String,
    pub location: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListEventsOutput {
    pub events: Vec<String>,
    pub next_page_token: String,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCalendarsOutput {
    pub calendars: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckAvailabilityOutput {
    pub busy_periods: Vec<String>,
    pub free_periods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindAvailableSlotsOutput {
    pub available_slots: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarEvent {
    pub event_id: String,
    pub calendar_id: String,
    pub title: String,
    pub description: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub timezone: String,
    pub location: String,
    pub attendees: Vec<String>,
    pub recurrence: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Calendar {
    pub calendar_id: String,
    pub name: String,
    pub description: String,
    pub color: String,
    pub timezone: String,
    pub primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarAttendee {
    pub email: String,
    pub name: String,
    pub response_status: String,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarReminder {
    pub method: String,
    pub minutes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarTimePeriod {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub calendar_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarTimeSlot {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub duration_minutes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarMeeting {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub datetime: String,
    pub datetime_display: String,
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarDay {
    pub date: String,
    pub day_number: i64,
    pub weekday_short: String,
    pub weekday_long: String,
    #[serde(default)]
    pub is_current_month: bool,
    #[serde(default)]
    pub is_today: bool,
    #[serde(default)]
    pub is_selected: bool,
    pub events: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarViewOption {
    pub id: String,
    pub label: String,
    pub href: String,
    #[serde(default)]
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarMonth {
    pub name: String,
    pub days: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimeSlot {
    pub label: String,
    pub hour: i64,
}

#[async_trait]
pub trait CalendarAction {
    async fn create_event(&self, all_day: bool, attendees: Vec<String>, calendar_id: String, description: String, end_time: chrono::DateTime<chrono::Utc>, location: String, recurrence: String, reminders: Vec<String>, start_time: chrono::DateTime<chrono::Utc>, timezone: String, title: String) -> Result<CreateEventOutput, Box<dyn std::error::Error>>;
    async fn update_event(&self, attendees: Vec<String>, calendar_id: String, description: String, end_time: chrono::DateTime<chrono::Utc>, event_id: String, location: String, recurrence: String, reminders: Vec<String>, start_time: chrono::DateTime<chrono::Utc>, timezone: String, title: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_event(&self, calendar_id: String, event_id: String, send_updates: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_event(&self, calendar_id: String, event_id: String) -> Result<GetEventOutput, Box<dyn std::error::Error>>;
    async fn list_events(&self, calendar_id: String, end_time: chrono::DateTime<chrono::Utc>, max_results: i64, order_by: String, query: String, single_events: bool, start_time: chrono::DateTime<chrono::Utc>) -> Result<ListEventsOutput, Box<dyn std::error::Error>>;
    async fn create_calendar(&self, color: String, description: String, name: String, timezone: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_calendar(&self, calendar_id: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_calendars(&self, max_results: i64, show_hidden: bool) -> Result<ListCalendarsOutput, Box<dyn std::error::Error>>;
    async fn check_availability(&self, calendar_ids: Vec<String>, end_time: chrono::DateTime<chrono::Utc>, start_time: chrono::DateTime<chrono::Utc>, timezone: String) -> Result<CheckAvailabilityOutput, Box<dyn std::error::Error>>;
    async fn add_attendee(&self, calendar_id: String, email: String, event_id: String, optional: bool, send_updates: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn remove_attendee(&self, calendar_id: String, email: String, event_id: String, send_updates: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn respond_to_invitation(&self, calendar_id: String, comment: String, event_id: String, response: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn find_available_slots(&self, attendees: Vec<String>, duration_minutes: i64, end_time: chrono::DateTime<chrono::Utc>, start_time: chrono::DateTime<chrono::Utc>, timezone: String, working_hours_only: bool) -> Result<FindAvailableSlotsOutput, Box<dyn std::error::Error>>;
}
