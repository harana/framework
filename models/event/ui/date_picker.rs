// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DateSelected {
    pub picker_id: String,
    pub picker_name: Option<String>,
    pub selected_date: chrono::DateTime<chrono::Utc>,
    pub previous_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub selected_at: chrono::DateTime<chrono::Utc>,
}

impl DateSelected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DateRangeSelected {
    pub picker_id: String,
    pub picker_name: Option<String>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub previous_start: Option<chrono::DateTime<chrono::Utc>>,
    pub previous_end: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub selected_at: chrono::DateTime<chrono::Utc>,
}

impl DateRangeSelected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimeSelected {
    pub picker_id: String,
    pub picker_name: Option<String>,
    pub selected_time: String,
    pub previous_time: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub selected_at: chrono::DateTime<chrono::Utc>,
}

impl TimeSelected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarMonthChanged {
    pub calendar_id: String,
    pub from_month: Option<i64>,
    pub to_month: i64,
    pub year: i64,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl CalendarMonthChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CalendarYearChanged {
    pub calendar_id: String,
    pub from_year: Option<i64>,
    pub to_year: i64,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl CalendarYearChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

