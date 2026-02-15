// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListItemClicked {
    pub list_id: String,
    pub item_id: String,
    pub item_index: Option<i64>,
    pub item_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl ListItemClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListItemReordered {
    pub list_id: String,
    pub item_id: String,
    pub from_index: i64,
    pub to_index: i64,
    #[serde(default = "chrono::Utc::now")]
    pub reordered_at: chrono::DateTime<chrono::Utc>,
}

impl ListItemReordered {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListItemDeleted {
    pub list_id: String,
    pub item_id: String,
    pub item_index: Option<i64>,
    pub item_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

impl ListItemDeleted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListItemArchived {
    pub list_id: String,
    pub item_id: String,
    pub item_index: Option<i64>,
    pub item_label: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub archived_at: chrono::DateTime<chrono::Utc>,
}

impl ListItemArchived {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListFiltered {
    pub list_id: String,
    pub filter_type: Option<String>,
    pub filter_value: Option<String>,
    pub results_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub filtered_at: chrono::DateTime<chrono::Utc>,
}

impl ListFiltered {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListGrouped {
    pub list_id: String,
    pub group_by: String,
    pub groups_count: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub grouped_at: chrono::DateTime<chrono::Utc>,
}

impl ListGrouped {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

