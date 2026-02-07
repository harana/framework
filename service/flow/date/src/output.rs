// Harana Actions - Date Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};

// add
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddOutput {
    pub result: String,
}

// convert_timezone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertTimezoneOutput {
    pub result: String,
}

// diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffOutput {
    pub difference: f64,
}

// end_of
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndOfOutput {
    pub result: String,
}

// format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatOutput {
    pub formatted: String,
}

// now
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowOutput {
    pub timestamp: String,
    pub unix: i64,
    pub unix_millis: i64,
}

// parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub timestamp: String,
    pub unix: i64,
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}

// start_of
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartOfOutput {
    pub result: String,
}

// subtract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtractOutput {
    pub result: String,
}
