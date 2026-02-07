// Harana Actions - Archive Module
// This module provides archive actions and functionality.

pub mod output;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::Path;
use serde_json::json;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use walkdir::WalkDir;
use output::*;


/// Add To Archive - adds files to an existing zip archive
pub async fn add_to_archive(
    archive_path: &str,
    source_paths: Vec<String>,
    _archive_type: &str,
) -> Result<AddToArchiveOutput, String> {
    let file = File::options()
        .read(true)
        .write(true)
        .open(archive_path)
        .map_err(|e| format!("Failed to open archive: {}", e))?;
    
    let mut archive = zip::ZipWriter::new_append(file)
        .map_err(|e| format!("Failed to open zip for appending: {}", e))?;
    
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    
    let mut added_count = 0;
    
    for source in &source_paths {
        let path = Path::new(source);
        if path.is_file() {
            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file");
            
            archive.start_file(file_name, options)
                .map_err(|e| format!("Failed to start file: {}", e))?;
            
            let mut file = File::open(path)
                .map_err(|e| format!("Failed to open source file: {}", e))?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .map_err(|e| format!("Failed to read source file: {}", e))?;
            
            archive.write_all(&buffer)
                .map_err(|e| format!("Failed to write to archive: {}", e))?;
            
            added_count += 1;
        }
    }
    
    archive.finish().map_err(|e| format!("Failed to finish archive: {}", e))?;
    
    Ok(AddToArchiveOutput {
        archive_path: archive_path.to_string(),
        added_count,
        success: true,
    })
}

/// Decompress Bzip2 - not implemented (requires bzip2 crate)
pub async fn bunzip2(
    _archive_path: &str,
    _keep_archive: Option<bool>,
    _output_path: Option<&str>,
) -> Result<Bunzip2Output, String> {
    Err("bzip2 decompression not implemented - requires bzip2 crate".to_string())
}

/// Compress With Bzip2 - not implemented (requires bzip2 crate)
pub async fn bzip2(
    _source_path: &str,
    _keep_source: Option<bool>,
    _output_path: Option<&str>,
    _compression_level: Option<&str>,
) -> Result<Bzip2Output, String> {
    Err("bzip2 compression not implemented - requires bzip2 crate".to_string())
}

/// Get Archive Info
pub async fn get_archive_info(
    archive_path: &str,
) -> Result<GetArchiveInfoOutput, String> {
    let file = File::open(archive_path)
        .map_err(|e| format!("Failed to open archive: {}", e))?;
    
    let metadata = file.metadata()
        .map_err(|e| format!("Failed to get metadata: {}", e))?;
    
    let compressed_size = metadata.len() as i32;
    
    // Try to detect archive type and get info
    let mut archive = zip::ZipArchive::new(BufReader::new(file))
        .map_err(|e| format!("Failed to read archive: {}", e))?;
    
    let file_count = archive.len() as i32;
    let mut total_size: u64 = 0;
    
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            total_size += file.size();
        }
    }
    
    let compression_ratio = if total_size > 0 {
        1.0 - (compressed_size as f64 / total_size as f64)
    } else {
        0.0
    };
    
    Ok(GetArchiveInfoOutput {
        compressed_size,
        archive_type: "zip".to_string(),
        is_encrypted: false,
        created_at: 0,
        file_count,
        size: total_size as i32,
        compression_ratio,
    })
}

/// Decompress Gzip
pub async fn gunzip(
    archive_path: &str,
    output_path: Option<&str>,
    keep_archive: Option<bool>,
) -> Result<GunzipOutput, String> {
    let input = File::open(archive_path)
        .map_err(|e| format!("Failed to open gzip file: {}", e))?;
    
    let mut decoder = GzDecoder::new(BufReader::new(input));
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)
        .map_err(|e| format!("Failed to decompress: {}", e))?;
    
    let out_path = output_path.map(|s| s.to_string()).unwrap_or_else(|| {
        archive_path.trim_end_matches(".gz").to_string()
    });
    
    fs::write(&out_path, &decompressed)
        .map_err(|e| format!("Failed to write output: {}", e))?;
    
    if !keep_archive.unwrap_or(true) {
        let _ = fs::remove_file(archive_path);
    }
    
    Ok(GunzipOutput {
        decompressed_path: out_path,
        success: true,
        size: decompressed.len() as i32,
    })
}

/// Compress With Gzip
pub async fn gzip(
    source_path: &str,
    compression_level: Option<&str>,
    keep_source: Option<bool>,
    output_path: Option<&str>,
) -> Result<GzipOutput, String> {
    let level = match compression_level.unwrap_or("6") {
        "1" => Compression::fast(),
        "9" => Compression::best(),
        _ => Compression::default(),
    };
    
    let input = fs::read(source_path)
        .map_err(|e| format!("Failed to read source: {}", e))?;
    
    let original_size = input.len() as i32;
    
    let out_path = output_path.map(|s| s.to_string()).unwrap_or_else(|| {
        format!("{}.gz", source_path)
    });
    
    let output = File::create(&out_path)
        .map_err(|e| format!("Failed to create output: {}", e))?;
    
    let mut encoder = GzEncoder::new(BufWriter::new(output), level);
    encoder.write_all(&input)
        .map_err(|e| format!("Failed to compress: {}", e))?;
    encoder.finish()
        .map_err(|e| format!("Failed to finish compression: {}", e))?;
    
    let compressed_size = fs::metadata(&out_path)
        .map(|m| m.len() as i32)
        .unwrap_or(0);
    
    if !keep_source.unwrap_or(true) {
        let _ = fs::remove_file(source_path);
    }
    
    let compression_ratio = if original_size > 0 {
        1.0 - (compressed_size as f64 / original_size as f64)
    } else {
        0.0
    };
    
    Ok(GzipOutput {
        original_size,
        success: true,
        compression_ratio,
        compressed_path: out_path,
        compressed_size,
    })
}

/// List Archive Contents
pub async fn list(
    archive_path: &str,
    _archive_type: &str,
    _password: Option<&str>,
) -> Result<ListOutput, String> {
    let file = File::open(archive_path)
        .map_err(|e| format!("Failed to open archive: {}", e))?;
    
    let mut archive = zip::ZipArchive::new(BufReader::new(file))
        .map_err(|e| format!("Failed to read archive: {}", e))?;
    
    let mut files = Vec::new();
    let mut total_size: i64 = 0;
    
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            let mut entry = HashMap::new();
            entry.insert("name".to_string(), json!(file.name()));
            entry.insert("size".to_string(), json!(file.size()));
            entry.insert("compressed_size".to_string(), json!(file.compressed_size()));
            entry.insert("is_dir".to_string(), json!(file.is_dir()));
            files.push(entry);
            total_size += file.size() as i64;
        }
    }
    
    let total_files = files.len() as i32;
    
    Ok(ListOutput {
        files,
        total_size: total_size as i32,
        total_files,
    })
}

/// Remove From Archive - removes files from zip archive
pub async fn remove_from_archive(
    file_paths: Vec<String>,
    archive_path: &str,
    _archive_type: &str,
) -> Result<RemoveFromArchiveOutput, String> {
    // Read existing archive
    let file = File::open(archive_path)
        .map_err(|e| format!("Failed to open archive: {}", e))?;
    
    let mut archive = zip::ZipArchive::new(BufReader::new(file))
        .map_err(|e| format!("Failed to read archive: {}", e))?;
    
    // Create a temporary file for the new archive
    let temp_path = format!("{}.tmp", archive_path);
    let temp_file = File::create(&temp_path)
        .map_err(|e| format!("Failed to create temp file: {}", e))?;
    
    let mut new_archive = zip::ZipWriter::new(BufWriter::new(temp_file));
    let options = zip::write::SimpleFileOptions::default();
    
    let file_set: std::collections::HashSet<_> = file_paths.iter().collect();
    let mut removed_count = 0;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let name = file.name().to_string();
        
        if file_set.contains(&name) {
            removed_count += 1;
            continue;
        }
        
        new_archive.start_file(&name, options)
            .map_err(|e| format!("Failed to start file: {}", e))?;
        
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        new_archive.write_all(&buffer)
            .map_err(|e| format!("Failed to write file: {}", e))?;
    }
    
    new_archive.finish()
        .map_err(|e| format!("Failed to finish archive: {}", e))?;
    
    // Replace original with new archive
    fs::rename(&temp_path, archive_path)
        .map_err(|e| format!("Failed to replace archive: {}", e))?;
    
    Ok(RemoveFromArchiveOutput {
        removed_count,
        archive_path: archive_path.to_string(),
        success: true,
    })
}

/// Create Tar Archive
pub async fn tar(
    source_paths: Vec<String>,
    output_path: &str,
    _preserve_permissions: Option<bool>,
    include_hidden: Option<bool>,
    _format: Option<&str>,
) -> Result<TarOutput, String> {
    let file = File::create(output_path)
        .map_err(|e| format!("Failed to create tar file: {}", e))?;
    
    let mut builder = tar::Builder::new(BufWriter::new(file));
    let include_hidden = include_hidden.unwrap_or(false);
    let mut file_count = 0;
    
    for source in &source_paths {
        let path = Path::new(source);
        
        if path.is_file() {
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file");
            
            builder.append_path_with_name(path, name)
                .map_err(|e| format!("Failed to add file: {}", e))?;
            file_count += 1;
        } else if path.is_dir() {
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                let entry_path = entry.path();
                let file_name = entry_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                
                // Skip hidden files if not included
                if !include_hidden && file_name.starts_with('.') {
                    continue;
                }
                
                if entry_path.is_file() {
                    let relative = entry_path.strip_prefix(path)
                        .unwrap_or(entry_path);
                    
                    builder.append_path_with_name(entry_path, relative)
                        .map_err(|e| format!("Failed to add file: {}", e))?;
                    file_count += 1;
                }
            }
        }
    }
    
    builder.finish()
        .map_err(|e| format!("Failed to finish tar: {}", e))?;
    
    let size = fs::metadata(output_path)
        .map(|m| m.len() as i32)
        .unwrap_or(0);
    
    Ok(TarOutput {
        success: true,
        size,
        archive_path: output_path.to_string(),
        file_count,
    })
}

/// Extract Tar Archive
pub async fn untar(
    archive_path: &str,
    destination_path: &str,
    overwrite: Option<bool>,
    _format: Option<&str>,
    extract_files: Option<Vec<String>>,
) -> Result<UntarOutput, String> {
    let file = File::open(archive_path)
        .map_err(|e| format!("Failed to open tar file: {}", e))?;
    
    let mut archive = tar::Archive::new(BufReader::new(file));
    let dest = Path::new(destination_path);
    
    fs::create_dir_all(dest)
        .map_err(|e| format!("Failed to create destination: {}", e))?;
    
    let extract_set: Option<std::collections::HashSet<_>> = 
        extract_files.map(|f| f.into_iter().collect());
    
    let mut file_count = 0;
    
    for entry in archive.entries().map_err(|e| format!("Failed to read entries: {}", e))? {
        let mut entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let entry_path = entry.path()
            .map_err(|e| format!("Failed to get path: {}", e))?;
        
        let entry_name = entry_path.to_string_lossy().to_string();
        
        // Filter by extract_files if specified
        if let Some(ref set) = extract_set {
            if !set.contains(&entry_name) {
                continue;
            }
        }
        
        let dest_path = dest.join(&entry_path);
        
        // Check if we should overwrite
        if dest_path.exists() && !overwrite.unwrap_or(true) {
            continue;
        }
        
        entry.unpack(&dest_path)
            .map_err(|e| format!("Failed to extract: {}", e))?;
        file_count += 1;
    }
    
    Ok(UntarOutput {
        success: true,
        extracted_path: destination_path.to_string(),
        file_count,
    })
}

/// Decompress XZ - not implemented (requires xz2 crate)
pub async fn unxz(
    _archive_path: &str,
    _keep_archive: Option<bool>,
    _output_path: Option<&str>,
) -> Result<UnxzOutput, String> {
    Err("xz decompression not implemented - requires xz2 crate".to_string())
}

/// Extract Zip Archive
pub async fn unzip(
    destination_path: &str,
    archive_path: &str,
    _password: Option<&str>,
    overwrite: Option<bool>,
    extract_files: Option<Vec<String>>,
) -> Result<UnzipOutput, String> {
    let file = File::open(archive_path)
        .map_err(|e| format!("Failed to open zip file: {}", e))?;
    
    let mut archive = zip::ZipArchive::new(BufReader::new(file))
        .map_err(|e| format!("Failed to read zip: {}", e))?;
    
    let dest = Path::new(destination_path);
    fs::create_dir_all(dest)
        .map_err(|e| format!("Failed to create destination: {}", e))?;
    
    let extract_set: Option<std::collections::HashSet<_>> = 
        extract_files.map(|f| f.into_iter().collect());
    
    let mut file_count = 0;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let name = file.name().to_string();
        
        // Filter by extract_files if specified
        if let Some(ref set) = extract_set {
            if !set.contains(&name) {
                continue;
            }
        }
        
        let dest_path = dest.join(&name);
        
        // Check if we should overwrite
        if dest_path.exists() && !overwrite.unwrap_or(true) {
            continue;
        }
        
        if file.is_dir() {
            fs::create_dir_all(&dest_path)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        } else {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directory: {}", e))?;
            }
            
            let mut outfile = File::create(&dest_path)
                .map_err(|e| format!("Failed to create file: {}", e))?;
            
            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to extract file: {}", e))?;
            
            file_count += 1;
        }
    }
    
    Ok(UnzipOutput {
        file_count,
        success: true,
        extracted_path: destination_path.to_string(),
    })
}

/// Verify Archive Integrity
pub async fn verify_archive(
    archive_path: &str,
    _archive_type: &str,
    _password: Option<&str>,
) -> Result<VerifyArchiveOutput, String> {
    let file = File::open(archive_path)
        .map_err(|e| format!("Failed to open archive: {}", e))?;
    
    match zip::ZipArchive::new(BufReader::new(file)) {
        Ok(mut archive) => {
            let file_count = archive.len() as i32;
            
            // Try to read each file to verify integrity
            for i in 0..archive.len() {
                if let Err(e) = archive.by_index(i) {
                    return Ok(VerifyArchiveOutput {
                        valid: false,
                        file_count,
                        error: format!("Corrupted entry at index {}: {}", i, e),
                    });
                }
            }
            
            Ok(VerifyArchiveOutput {
                valid: true,
                file_count,
                error: String::new(),
            })
        }
        Err(e) => Ok(VerifyArchiveOutput {
            valid: false,
            file_count: 0,
            error: format!("Invalid archive: {}", e),
        }),
    }
}

/// Compress With XZ - not implemented (requires xz2 crate)
pub async fn xz(
    _source_path: &str,
    _compression_level: Option<&str>,
    _output_path: Option<&str>,
    _keep_source: Option<bool>,
) -> Result<XzOutput, String> {
    Err("xz compression not implemented - requires xz2 crate".to_string())
}

/// Create Zip Archive
pub async fn zip(
    output_path: &str,
    source_paths: Vec<String>,
    _password: Option<&str>,
    include_hidden: Option<bool>,
    compression_level: Option<&str>,
) -> Result<ZipOutput, String> {
    let file = File::create(output_path)
        .map_err(|e| format!("Failed to create zip file: {}", e))?;
    
    let mut archive = zip::ZipWriter::new(BufWriter::new(file));
    
    let method = match compression_level.unwrap_or("6") {
        "0" => zip::CompressionMethod::Stored,
        _ => zip::CompressionMethod::Deflated,
    };
    
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(method);
    
    let include_hidden = include_hidden.unwrap_or(false);
    let mut file_count = 0;
    
    for source in &source_paths {
        let path = Path::new(source);
        
        if path.is_file() {
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file");
            
            archive.start_file(name, options)
                .map_err(|e| format!("Failed to start file: {}", e))?;
            
            let mut file = File::open(path)
                .map_err(|e| format!("Failed to open source: {}", e))?;
            
            std::io::copy(&mut file, &mut archive)
                .map_err(|e| format!("Failed to write file: {}", e))?;
            
            file_count += 1;
        } else if path.is_dir() {
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                let entry_path = entry.path();
                let file_name = entry_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                
                // Skip hidden files if not included
                if !include_hidden && file_name.starts_with('.') {
                    continue;
                }
                
                if entry_path.is_file() {
                    let relative = entry_path.strip_prefix(path)
                        .unwrap_or(entry_path)
                        .to_string_lossy();
                    
                    archive.start_file(relative.to_string(), options)
                        .map_err(|e| format!("Failed to start file: {}", e))?;
                    
                    let mut file = File::open(entry_path)
                        .map_err(|e| format!("Failed to open source: {}", e))?;
                    
                    std::io::copy(&mut file, &mut archive)
                        .map_err(|e| format!("Failed to write file: {}", e))?;
                    
                    file_count += 1;
                }
            }
        }
    }
    
    archive.finish()
        .map_err(|e| format!("Failed to finish zip: {}", e))?;
    
    let size = fs::metadata(output_path)
        .map(|m| m.len() as i32)
        .unwrap_or(0);
    
    Ok(ZipOutput {
        success: true,
        size,
        archive_path: output_path.to_string(),
        file_count,
    })
}

#[cfg(test)]
mod tests;
