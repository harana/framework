// Harana Actions - File Module
// This module provides file actions and functionality.

pub mod output;

#[cfg(test)]
mod tests;

use output::*;
use std::path::Path;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use walkdir::WalkDir;
use chrono::{DateTime, Utc};

/// Copy File To Destination
pub async fn copy(
    from: &str,
    to: &str,
    overwrite: Option<bool>,
) -> Result<CopyOutput, String> {
    let overwrite = overwrite.unwrap_or(false);
    let from_path = Path::new(from);
    let to_path = Path::new(to);
    
    // Check if source exists
    if !from_path.exists() {
        return Err(format!("Source file does not exist: {}", from));
    }
    
    // Check if destination exists and overwrite is false
    if to_path.exists() && !overwrite {
        return Err(format!("Destination file already exists: {}", to));
    }
    
    // Create parent directories if needed
    if let Some(parent) = to_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).await
                .map_err(|e| format!("Failed to create parent directories: {}", e))?;
        }
    }
    
    fs::copy(from, to).await
        .map_err(|e| format!("Failed to copy file: {}", e))?;
    
    Ok(CopyOutput { success: true })
}

/// Create New Directory Path
pub async fn create_directory(
    path: &str,
    recursive: Option<bool>,
) -> Result<CreateDirectoryOutput, String> {
    let recursive = recursive.unwrap_or(true);
    
    if recursive {
        fs::create_dir_all(path).await
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    } else {
        fs::create_dir(path).await
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    
    Ok(CreateDirectoryOutput { success: true })
}

/// Delete File At Path
pub async fn delete(
    path: &str,
) -> Result<DeleteOutput, String> {
    let file_path = Path::new(path);
    
    if !file_path.exists() {
        return Err(format!("File does not exist: {}", path));
    }
    
    if file_path.is_dir() {
        return Err(format!("Path is a directory, use delete_directory: {}", path));
    }
    
    fs::remove_file(path).await
        .map_err(|e| format!("Failed to delete file: {}", e))?;
    
    Ok(DeleteOutput { success: true })
}

/// Delete Directory At Path
pub async fn delete_directory(
    path: &str,
    recursive: Option<bool>,
) -> Result<DeleteDirectoryOutput, String> {
    let dir_path = Path::new(path);
    let recursive = recursive.unwrap_or(false);
    
    if !dir_path.exists() {
        return Err(format!("Directory does not exist: {}", path));
    }
    
    if !dir_path.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }
    
    if recursive {
        fs::remove_dir_all(path).await
            .map_err(|e| format!("Failed to delete directory: {}", e))?;
    } else {
        fs::remove_dir(path).await
            .map_err(|e| format!("Failed to delete directory (not empty?): {}", e))?;
    }
    
    Ok(DeleteDirectoryOutput { success: true })
}

/// Check If File Exists
pub async fn exists(
    path: &str,
) -> Result<ExistsOutput, String> {
    let file_path = Path::new(path);
    Ok(ExistsOutput { exists: file_path.exists() })
}

/// Get File Metadata Info
pub async fn info(
    path: &str,
) -> Result<InfoOutput, String> {
    let metadata = fs::metadata(path).await
        .map_err(|e| format!("Failed to get file info: {}", e))?;
    
    let is_directory = metadata.is_dir();
    let size = metadata.len() as i32;
    
    // Get permissions as octal string
    #[cfg(unix)]
    let permissions = {
        use std::os::unix::fs::PermissionsExt;
        format!("{:o}", metadata.permissions().mode() & 0o777)
    };
    #[cfg(not(unix))]
    let permissions = if metadata.permissions().readonly() { "r--" } else { "rw-" }.to_string();
    
    // Get modified time
    let modified = metadata.modified()
        .map(|t| {
            let datetime: DateTime<Utc> = t.into();
            datetime.to_rfc3339()
        })
        .unwrap_or_else(|_| "unknown".to_string());
    
    // Get created time (may not be available on all platforms)
    let created = metadata.created()
        .map(|t| {
            let datetime: DateTime<Utc> = t.into();
            datetime.to_rfc3339()
        })
        .unwrap_or_else(|_| "unknown".to_string());
    
    Ok(InfoOutput {
        is_directory,
        permissions,
        modified,
        created,
        size,
    })
}

/// List Files In Directory
pub async fn list_directory(
    path: &str,
    recursive: Option<bool>,
    pattern: Option<&str>,
) -> Result<ListDirectoryOutput, String> {
    let dir_path = Path::new(path);
    let recursive = recursive.unwrap_or(false);
    
    if !dir_path.exists() {
        return Err(format!("Directory does not exist: {}", path));
    }
    
    if !dir_path.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }
    
    let mut files = Vec::new();
    
    // Compile glob pattern if provided
    let glob_pattern = pattern.map(|p| glob::Pattern::new(p))
        .transpose()
        .map_err(|e| format!("Invalid glob pattern: {}", e))?;
    
    if recursive {
        for entry in WalkDir::new(path).min_depth(1) {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let entry_path = entry.path().to_string_lossy().to_string();
            
            // Apply pattern filter if provided
            if let Some(ref pat) = glob_pattern {
                let file_name = entry.file_name().to_string_lossy();
                if !pat.matches(&file_name) {
                    continue;
                }
            }
            
            files.push(entry_path);
        }
    } else {
        let mut read_dir = fs::read_dir(path).await
            .map_err(|e| format!("Failed to read directory: {}", e))?;
        
        while let Some(entry) = read_dir.next_entry().await
            .map_err(|e| format!("Failed to read directory entry: {}", e))? {
            let entry_path = entry.path().to_string_lossy().to_string();
            
            // Apply pattern filter if provided
            if let Some(ref pat) = glob_pattern {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if !pat.matches(&file_name) {
                    continue;
                }
            }
            
            files.push(entry_path);
        }
    }
    
    files.sort();
    Ok(ListDirectoryOutput { files })
}

/// Move File To Destination
pub async fn r#move(
    from: &str,
    to: &str,
    overwrite: Option<bool>,
) -> Result<MoveOutput, String> {
    let overwrite = overwrite.unwrap_or(false);
    let from_path = Path::new(from);
    let to_path = Path::new(to);
    
    // Check if source exists
    if !from_path.exists() {
        return Err(format!("Source file does not exist: {}", from));
    }
    
    // Check if destination exists and overwrite is false
    if to_path.exists() && !overwrite {
        return Err(format!("Destination file already exists: {}", to));
    }
    
    // Create parent directories if needed
    if let Some(parent) = to_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).await
                .map_err(|e| format!("Failed to create parent directories: {}", e))?;
        }
    }
    
    // Try rename first (fastest if same filesystem)
    match fs::rename(from, to).await {
        Ok(_) => Ok(MoveOutput { success: true }),
        Err(_) => {
            // Fall back to copy + delete for cross-filesystem moves
            fs::copy(from, to).await
                .map_err(|e| format!("Failed to copy file during move: {}", e))?;
            fs::remove_file(from).await
                .map_err(|e| format!("Failed to remove source file after copy: {}", e))?;
            Ok(MoveOutput { success: true })
        }
    }
}

/// Read Content From File
pub async fn read(
    path: &str,
    mode: Option<&str>,
) -> Result<ReadOutput, String> {
    let mode = mode.unwrap_or("text");
    
    let file_path = Path::new(path);
    if !file_path.exists() {
        return Err(format!("File does not exist: {}", path));
    }
    
    let mut file = fs::File::open(path).await
        .map_err(|e| format!("Failed to open file: {}", e))?;
    
    let content = match mode {
        "binary" | "base64" => {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).await
                .map_err(|e| format!("Failed to read file: {}", e))?;
            // Encode as hex for binary mode
            buffer.iter().map(|b| format!("{:02x}", b)).collect::<String>()
        }
        _ => {
            // Default to text mode
            let mut content = String::new();
            file.read_to_string(&mut content).await
                .map_err(|e| format!("Failed to read file as text: {}", e))?;
            content
        }
    };
    
    Ok(ReadOutput { content })
}

/// Write Content To File
pub async fn write(
    path: &str,
    content: &str,
    overwrite: Option<bool>,
    mode: Option<&str>,
) -> Result<WriteOutput, String> {
    let overwrite = overwrite.unwrap_or(true);
    let mode = mode.unwrap_or("text");
    
    let file_path = Path::new(path);
    
    // Check if file exists and overwrite is false
    if file_path.exists() && !overwrite {
        return Err(format!("File already exists: {}", path));
    }
    
    // Create parent directories if needed
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).await
                .map_err(|e| format!("Failed to create parent directories: {}", e))?;
        }
    }
    
    let mut file = fs::File::create(path).await
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    match mode {
        "binary" | "hex" => {
            // Decode hex string to bytes
            let bytes: Result<Vec<u8>, _> = (0..content.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&content[i..i + 2], 16))
                .collect();
            let bytes = bytes.map_err(|e| format!("Invalid hex content: {}", e))?;
            file.write_all(&bytes).await
                .map_err(|e| format!("Failed to write file: {}", e))?;
        }
        _ => {
            // Default to text mode
            file.write_all(content.as_bytes()).await
                .map_err(|e| format!("Failed to write file: {}", e))?;
        }
    }
    
    // Flush to ensure content is written to disk
    file.flush().await
        .map_err(|e| format!("Failed to flush file: {}", e))?;
    
    Ok(WriteOutput { success: true })
}
