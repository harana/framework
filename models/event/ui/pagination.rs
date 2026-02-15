// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PageChanged {
    pub pagination_id: String,
    pub from_page: Option<i64>,
    pub to_page: i64,
    pub total_pages: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl PageChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PreviousPageClicked {
    pub pagination_id: String,
    pub current_page: i64,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl PreviousPageClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NextPageClicked {
    pub pagination_id: String,
    pub current_page: i64,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl NextPageClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PageNumberClicked {
    pub pagination_id: String,
    pub page_number: i64,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl PageNumberClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FirstPageReached {
    pub pagination_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub reached_at: chrono::DateTime<chrono::Utc>,
}

impl FirstPageReached {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LastPageReached {
    pub pagination_id: String,
    pub total_pages: i64,
    #[serde(default = "chrono::Utc::now")]
    pub reached_at: chrono::DateTime<chrono::Utc>,
}

impl LastPageReached {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

