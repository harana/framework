// Harana Actions - Form Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// delete_submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubmissionOutput {
    pub success: bool
}

// export_submissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSubmissionsOutput {
    pub count: i32,
    pub filename: String,
    pub content: Vec<u8>
}

// get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutput {
    pub validation_rules: HashMap<String, Value>,
    pub fields: Vec<HashMap<String, Value>>,
    pub metadata: HashMap<String, Value>
}

// get_submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubmissionOutput {
    pub data: HashMap<String, Value>,
    pub form_id: String,
    pub status: String,
    pub submitted_at: String
}

// list_submissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSubmissionsOutput {
    pub submissions: Vec<HashMap<String, Value>>,
    pub total: i32
}

// submit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitOutput {
    pub errors: Vec<HashMap<String, Value>>,
    pub success: bool,
    pub submission_id: String
}

// update_submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubmissionOutput {
    pub success: bool,
    pub errors: Vec<HashMap<String, Value>>
}

// validate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub errors: Vec<HashMap<String, Value>>,
    pub valid: bool
}
