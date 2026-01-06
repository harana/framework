// Harana Actions - File Module Output Types
// Auto-generated output structs for File action methods.

use serde::{Deserialize, Serialize};

// read_file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadFileOutput {
    pub content: Vec<u8>, // Can be string or bytes depending on mode
}

// write_file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteFileOutput {
    pub success: bool,
}

// delete_file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileOutput {
    pub success: bool,
}

// copy_file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileOutput {
    pub success: bool,
}

// move_file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFileOutput {
    pub success: bool,
}

// file_exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileExistsOutput {
    pub exists: bool,
}

// file_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfoOutput {
    pub created: String, // datetime
    pub is_directory: bool,
    pub modified: String, // datetime
    pub permissions: String,
    pub size: i32,
}

// list_directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDirectoryOutput {
    pub files: Vec<String>,
}

// create_directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDirectoryOutput {
    pub success: bool,
}

// delete_directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDirectoryOutput {
    pub success: bool,
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

// delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOutput {
    pub success: bool
}

// copy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyOutput {
    pub success: bool
}

// move
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveOutput {
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
    pub modified: String,
    pub created: String,
    pub permissions: String,
    pub size: i32
}