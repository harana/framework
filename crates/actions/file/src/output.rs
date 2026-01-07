// Harana Actions - File Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// copy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyOutput {
    pub success: bool
}

// create_directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDirectoryOutput {
    pub success: bool
}

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub success: bool
}

// delete_directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDirectoryOutput {
    pub success: bool
}

// exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistsOutput {
    pub exists: bool
}

// info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoOutput {
    pub is_directory: bool,
    pub permissions: String,
    pub modified: String,
    pub created: String,
    pub size: i32
}

// list_directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDirectoryOutput {
    pub files: Vec<String>
}

// move
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveOutput {
    pub success: bool
}

// read
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadOutput {
    pub content: String
}

// write
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteOutput {
    pub success: bool
}
