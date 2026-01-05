// Harana Actions - Form Module Output Types
// Auto-generated output structs for Form action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// validate_form
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateFormOutput {
    pub errors: Vec<HashMap<String, Value>>,
    pub valid: bool,
}

// submit_form
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitFormOutput {
    pub errors: Vec<HashMap<String, Value>>,
    pub submission_id: String,
    pub success: bool,
}

// get_form
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFormOutput {
    pub fields: Vec<HashMap<String, Value>>,
    pub metadata: HashMap<String, Value>,
    pub validation_rules: HashMap<String, Value>,
}

// get_submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubmissionOutput {
    pub data: HashMap<String, Value>,
    pub form_id: String,
    pub status: String,
    pub submitted_at: String, // datetime
}

// list_submissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSubmissionsOutput {
    pub submissions: Vec<HashMap<String, Value>>,
    pub total: i32,
}

// update_submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubmissionOutput {
    pub errors: Vec<HashMap<String, Value>>,
    pub success: bool,
}

// delete_submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubmissionOutput {
    pub success: bool,
}

// export_submissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSubmissionsOutput {
    pub content: Vec<u8>,
    pub count: i32,
    pub filename: String,
}
