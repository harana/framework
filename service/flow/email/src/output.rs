// Harana Actions - Email Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// create_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplateOutput {
    pub template_id: String,
    pub success: bool
}

// delete_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTemplateOutput {
    pub success: bool
}

// list_templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTemplatesOutput {
    pub templates: Vec<HashMap<String, Value>>,
    pub total: i32
}

// send
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendOutput {
    pub message_id: String,
    pub success: bool
}

// send_bulk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBulkOutput {
    pub message_ids: Vec<String>,
    pub failed: i32,
    pub successful: i32,
    pub total: i32
}

// send_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTemplateOutput {
    pub success: bool,
    pub message_id: String
}

// status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusOutput {
    pub error: String,
    pub opened_at: String,
    pub status: String,
    pub delivered_at: String,
    pub clicked_at: String
}

// validate_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateEmailOutput {
    pub reason: String,
    pub valid: bool
}
