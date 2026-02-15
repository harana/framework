// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// excel_workbook
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExcelWorkbook {
    /// Reference: blob_object.id
    pub blob_id: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Reference: user.id
    pub created_by: Option<String>,
    pub file_path: Option<String>,
    pub sheet_count: i64,
    pub size: i64,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ExcelWorkbook {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// excel_sheet
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ExcelSheet {
    pub column_count: i64,
    pub name: String,
    pub row_count: i64,
    /// Reference: excel_workbook.id
    pub workbook_id: String,
}

impl ExcelSheet {
    /// Validate the model according to schema constraints
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

