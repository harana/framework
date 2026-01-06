// Harana Actions - Email Module Output Types
// Auto-generated output structs for Email action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// send_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailOutput {
    pub message_id: String,
    pub success: bool,
}

// send_template_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTemplateEmailOutput {
    pub message_id: String,
    pub success: bool,
}

// validate_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateEmailOutput {
    pub reason: String,
    pub valid: bool,
}

// get_email_status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEmailStatusOutput {
    pub clicked_at: Option<String>,   // datetime
    pub delivered_at: Option<String>, // datetime
    pub error: Option<String>,
    pub opened_at: Option<String>, // datetime
    pub status: String,            // queued | sent | delivered | bounced | failed
}

// send_bulk_email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBulkEmailOutput {
    pub failed: i32,
    pub message_ids: Vec<String>,
    pub successful: i32,
    pub total: i32,
}

// create_email_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmailTemplateOutput {
    pub success: bool,
    pub template_id: String,
}

// delete_email_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEmailTemplateOutput {
    pub success: bool,
}

// list_email_templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEmailTemplatesOutput {
    pub templates: Vec<HashMap<String, Value>>,
    pub total: i32,
}

// send
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendOutput {
    pub message_id: String,
    pub success: bool
}

// send_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTemplateOutput {
    pub message_id: String,
    pub success: bool
}

// status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusOutput {
    pub error: String,
    pub opened_at: String,
    pub clicked_at: String,
    pub status: String,
    pub delivered_at: String
}

// send_bulk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBulkOutput {
    pub total: i32,
    pub failed: i32,
    pub message_ids: Vec<String>,
    pub successful: i32
}

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