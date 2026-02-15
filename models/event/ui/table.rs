// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RowSelected {
    pub table_id: String,
    pub row_id: String,
    pub row_index: Option<i64>,
    pub row_data_key: Option<String>,
    pub selection_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub selected_at: chrono::DateTime<chrono::Utc>,
}

impl RowSelected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RowDeselected {
    pub table_id: String,
    pub row_id: String,
    pub row_index: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub deselected_at: chrono::DateTime<chrono::Utc>,
}

impl RowDeselected {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RowExpanded {
    pub table_id: String,
    pub row_id: String,
    pub row_index: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub expanded_at: chrono::DateTime<chrono::Utc>,
}

impl RowExpanded {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RowCollapsed {
    pub table_id: String,
    pub row_id: String,
    pub row_index: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub collapsed_at: chrono::DateTime<chrono::Utc>,
}

impl RowCollapsed {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ColumnSorted {
    pub table_id: String,
    pub column_id: String,
    pub column_name: Option<String>,
    pub sort_direction: String,
    #[serde(default = "chrono::Utc::now")]
    pub sorted_at: chrono::DateTime<chrono::Utc>,
}

impl ColumnSorted {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ColumnResized {
    pub table_id: String,
    pub column_id: String,
    pub column_name: Option<String>,
    pub old_width: Option<i64>,
    pub new_width: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub resized_at: chrono::DateTime<chrono::Utc>,
}

impl ColumnResized {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BulkSelectionToggled {
    pub table_id: String,
    pub new_state: String,
    pub rows_affected: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub toggled_at: chrono::DateTime<chrono::Utc>,
}

impl BulkSelectionToggled {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SelectAllClicked {
    pub table_id: String,
    pub total_rows: Option<i64>,
    pub selected_rows: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl SelectAllClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaginationChanged {
    pub table_id: String,
    pub from_page: Option<i64>,
    pub to_page: i64,
    pub total_pages: Option<i64>,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl PaginationChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PageSizeChanged {
    pub table_id: String,
    pub old_page_size: Option<i64>,
    pub new_page_size: i64,
    #[serde(default = "chrono::Utc::now")]
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

impl PageSizeChanged {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RowClicked {
    pub table_id: String,
    pub row_id: String,
    pub row_index: Option<i64>,
    pub row_data_key: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub clicked_at: chrono::DateTime<chrono::Utc>,
}

impl RowClicked {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CellEdited {
    pub table_id: String,
    pub row_id: String,
    pub column_id: String,
    pub column_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub edited_at: chrono::DateTime<chrono::Utc>,
}

impl CellEdited {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

