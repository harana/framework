// Harana Actions - Response Module Output Types
// Auto-generated output structs for Response action methods.

use serde::{Deserialize, Serialize};

// template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateOutput {
    pub rendered: String,
    pub success: bool,
}

// template_file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateFileOutput {
    pub rendered: String,
    pub success: bool,
}

// json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonOutput {
    pub success: bool,
}

// text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextOutput {
    pub success: bool,
}

// html
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlOutput {
    pub success: bool,
}

// xml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlOutput {
    pub success: bool,
}

// binary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryOutput {
    pub success: bool,
}

// sse_event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseEventOutput {
    pub success: bool,
}

// sse_stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseStreamOutput {
    pub success: bool,
}

// not_found
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundOutput {
    pub success: bool,
}

// bad_request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadRequestOutput {
    pub success: bool,
}
