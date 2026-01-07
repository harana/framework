// Harana Actions - Date Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// add
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddOutput {
    pub result: String
}

// convert_timezone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertTimezoneOutput {
    pub result: String
}

// diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffOutput {
    pub difference: f64
}

// end_of
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndOfOutput {
    pub result: String
}

// format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatOutput {
    pub formatted: String
}

// now
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowOutput {
    pub timestamp: String
}

// parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub timestamp: String
}

// start_of
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartOfOutput {
    pub result: String
}

// subtract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtractOutput {
    pub result: String
}
