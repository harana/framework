// Harana Actions - Excel Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// get_sheets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSheetsOutput {
    pub sheets: Vec<String>
}

// read
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadOutput {
    pub rows: Vec<HashMap<String, Value>>
}

// read_sheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadSheetOutput {
    pub rows: Vec<HashMap<String, Value>>
}

// write
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteOutput {
    pub path: String
}
