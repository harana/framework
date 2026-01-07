// Harana Actions - Log Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// debug
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct DebugOutput {
    pub success: bool
}

pub struct ErrorOutput {
    pub success: bool
}

pub struct InfoOutput {
    pub success: bool
}

pub struct StructuredOutput {
    pub success: bool
}

pub struct WarnOutput {
    pub success: bool
}
