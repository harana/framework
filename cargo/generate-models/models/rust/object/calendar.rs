// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// calendar_event
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// calendar_meeting
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// calendar_day
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// calendar_view_option
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
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// calendar_month
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarMonth {
    pub name: String,
    pub days: Option<String>,
}

impl CalendarMonth {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// time_slot
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimeSlot {
    pub label: String,
    pub hour: i64,
}

impl TimeSlot {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

