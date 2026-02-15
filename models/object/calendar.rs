// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarEvent {
    pub id: String,
    pub name: String,
    pub datetime: String,
    pub time_display: String,
    pub href: Option<String>,
    pub location: Option<String>,
    pub color: String,
    pub grid_row_start: Option<i64>,
    pub grid_row_span: Option<i64>,
    pub day_column: Option<i64>,
}

impl CalendarEvent {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarMeeting {
    pub id: String,
    pub name: String,
    pub image_url: Option<String>,
    pub datetime: String,
    pub datetime_display: String,
    pub location: Option<String>,
}

impl CalendarMeeting {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarDay {
    pub date: String,
    pub day_number: i64,
    pub weekday_short: Option<String>,
    pub weekday_long: Option<String>,
    #[serde(default)]
    pub is_current_month: bool,
    #[serde(default)]
    pub is_today: bool,
    #[serde(default)]
    pub is_selected: bool,
    pub events: Option<String>,
}

impl CalendarDay {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarViewOption {
    pub id: String,
    pub label: String,
    pub href: Option<String>,
    #[serde(default)]
    pub is_active: bool,
}

impl CalendarViewOption {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarMonth {
    pub name: String,
    pub days: Option<String>,
}

impl CalendarMonth {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimeSlot {
    pub label: String,
    pub hour: i64,
}

impl TimeSlot {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl Calendar {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarAttendee {
    pub email: String,
    pub name: String,
    pub response_status: String,
    pub optional: bool,
}

impl CalendarAttendee {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarReminder {
    pub method: String,
    pub minutes: i64,
}

impl CalendarReminder {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarTimePeriod {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub calendar_id: String,
}

impl CalendarTimePeriod {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarTimeSlot {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub duration_minutes: i64,
}

impl CalendarTimeSlot {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

