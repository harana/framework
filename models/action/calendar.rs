// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEventInput {
    #[serde(default)]
    pub all_day: bool,
    pub attendees: Vec<String>,
    pub calendar_id: String,
    pub description: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub location: String,
    pub recurrence: String,
    pub reminders: Vec<String>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub timezone: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEventOutput {
    pub event_id: String,
    pub html_link: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateEventInput {
    pub attendees: Vec<String>,
    pub calendar_id: String,
    pub description: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub event_id: String,
    pub location: String,
    pub recurrence: String,
    pub reminders: Vec<String>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub timezone: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateEventOutput {
    pub event_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteEventInput {
    pub calendar_id: String,
    pub event_id: String,
    pub send_updates: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteEventOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetEventInput {
    pub calendar_id: String,
    pub event_id: String,
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
pub struct ListEventsInput {
    pub calendar_id: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub max_results: i64,
    pub order_by: String,
    pub query: String,
    #[serde(default)]
    pub single_events: bool,
    pub start_time: chrono::DateTime<chrono::Utc>,
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
pub struct CreateCalendarInput {
    pub color: String,
    pub description: String,
    pub name: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateCalendarOutput {
    pub calendar_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteCalendarInput {
    pub calendar_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteCalendarOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCalendarsInput {
    pub max_results: i64,
    #[serde(default)]
    pub show_hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCalendarsOutput {
    pub calendars: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckAvailabilityInput {
    pub calendar_ids: Vec<String>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CheckAvailabilityOutput {
    pub busy_periods: Vec<String>,
    pub free_periods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddAttendeeInput {
    pub calendar_id: String,
    pub email: String,
    pub event_id: String,
    #[serde(default)]
    pub optional: bool,
    pub send_updates: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddAttendeeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveAttendeeInput {
    pub calendar_id: String,
    pub email: String,
    pub event_id: String,
    pub send_updates: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveAttendeeOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RespondToInvitationInput {
    pub calendar_id: String,
    pub comment: String,
    pub event_id: String,
    pub response: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RespondToInvitationOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FindAvailableSlotsInput {
    pub attendees: Vec<String>,
    pub duration_minutes: i64,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub timezone: String,
    #[serde(default)]
    pub working_hours_only: bool,
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

#[async_trait]
pub trait CalendarAction {
    async fn create_event(&self, input: CreateEventInput) -> Result<CreateEventOutput, Box<dyn std::error::Error>>;
    async fn update_event(&self, input: UpdateEventInput) -> Result<UpdateEventOutput, Box<dyn std::error::Error>>;
    async fn delete_event(&self, input: DeleteEventInput) -> Result<DeleteEventOutput, Box<dyn std::error::Error>>;
    async fn get_event(&self, input: GetEventInput) -> Result<GetEventOutput, Box<dyn std::error::Error>>;
    async fn list_events(&self, input: ListEventsInput) -> Result<ListEventsOutput, Box<dyn std::error::Error>>;
    async fn create_calendar(&self, input: CreateCalendarInput) -> Result<CreateCalendarOutput, Box<dyn std::error::Error>>;
    async fn delete_calendar(&self, input: DeleteCalendarInput) -> Result<DeleteCalendarOutput, Box<dyn std::error::Error>>;
    async fn list_calendars(&self, input: ListCalendarsInput) -> Result<ListCalendarsOutput, Box<dyn std::error::Error>>;
    async fn check_availability(&self, input: CheckAvailabilityInput) -> Result<CheckAvailabilityOutput, Box<dyn std::error::Error>>;
    async fn add_attendee(&self, input: AddAttendeeInput) -> Result<AddAttendeeOutput, Box<dyn std::error::Error>>;
    async fn remove_attendee(&self, input: RemoveAttendeeInput) -> Result<RemoveAttendeeOutput, Box<dyn std::error::Error>>;
    async fn respond_to_invitation(&self, input: RespondToInvitationInput) -> Result<RespondToInvitationOutput, Box<dyn std::error::Error>>;
    async fn find_available_slots(&self, input: FindAvailableSlotsInput) -> Result<FindAvailableSlotsOutput, Box<dyn std::error::Error>>;
}
