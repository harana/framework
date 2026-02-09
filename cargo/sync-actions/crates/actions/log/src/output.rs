use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
