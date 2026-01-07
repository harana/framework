// Harana Actions - Excel Module
// This module provides excel actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// List Sheets In Workbook
pub async fn get_sheets(
    file: &str,
) -> Result<GetSheetsOutput, String> {
    unimplemented!("get_sheets")
}

/// Read Excel File To Data
pub async fn read(
    file: &str,
    has_headers: Option<bool>,
    sheet: Option<&str>,
    range: Option<&str>,
) -> Result<ReadOutput, String> {
    unimplemented!("read")
}

/// Read Specific Sheet
pub async fn read_sheet(
    sheet: &str,
    file: &str,
    range: Option<&str>,
    has_headers: Option<bool>,
) -> Result<ReadSheetOutput, String> {
    unimplemented!("read_sheet")
}

/// Write Data To Excel File
pub async fn write(
    file: &str,
    data: Vec<HashMap<String, Value>>,
    headers: Option<Vec<String>>,
    include_headers: Option<bool>,
    sheet: Option<&str>,
) -> Result<WriteOutput, String> {
    unimplemented!("write")
}
