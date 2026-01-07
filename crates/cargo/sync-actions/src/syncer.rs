use crate::analyzer::RustModule;
use crate::generator::RustGenerator;
use crate::parser::{Action, ActionSchema};
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn sync_schema(
    schema: &ActionSchema,
    actions_dir: &Path,
    dry_run: bool,
    force: bool,
    replace: bool,
) -> Result<(usize, usize)> {
    println!("\n📦 Processing module: {}", schema.module_name);
    println!("   Actions defined: {}", schema.actions.len());

    let crate_dir = actions_dir.join(&schema.module_name);

    // Check if the crate directory exists, create it if not
    if !crate_dir.exists() {
        println!("   📁 Creating new crate: {}", crate_dir.display());
        initialize_crate(&crate_dir, &schema.module_name, schema)?;
    } else {
        // Ensure required files exist
        ensure_crate_files(&crate_dir, &schema.module_name)?;
    }

    // If replace mode, completely regenerate the files
    if replace {
        if dry_run {
            println!("   🔄 Would regenerate lib.rs and output.rs from scratch");
            return Ok((schema.actions.len(), 0));
        }
        return regenerate_files(&crate_dir, &schema.module_name, schema);
    }

    // Analyze existing Rust module
    let rust_module =
        RustModule::analyze(&crate_dir).with_context(|| format!("Failed to analyze module {}", schema.module_name))?;

    println!("   Existing functions: {}", rust_module.existing_functions.len());

    // Build a set of expected action names from the schema
    let expected_actions: HashSet<String> = schema.actions.iter().map(|a| a.action.clone()).collect();
    let expected_outputs: HashSet<String> = schema
        .actions
        .iter()
        .map(|a| RustGenerator::to_pascal_case(&a.action) + "Output")
        .collect();

    // Find missing actions
    let mut missing_actions = Vec::new();
    let mut missing_outputs = Vec::new();

    for action in &schema.actions {
        if !rust_module.has_function(&action.action) {
            missing_actions.push(action);
        }

        let output_struct = RustGenerator::to_pascal_case(&action.action) + "Output";
        if !rust_module.has_output_struct(&output_struct) {
            missing_outputs.push(action);
        }
    }

    // Find extra actions (if force mode is enabled)
    let mut extra_functions = Vec::new();
    let mut extra_structs = Vec::new();

    if force {
        for func_name in &rust_module.existing_functions {
            if !expected_actions.contains(func_name) {
                extra_functions.push(func_name.clone());
            }
        }

        for struct_info in &rust_module.output_structs {
            if !expected_outputs.contains(&struct_info.name) {
                extra_structs.push(struct_info.name.clone());
            }
        }

        if !extra_functions.is_empty() || !extra_structs.is_empty() {
            println!("   🗑️  Extra functions to remove: {}", extra_functions.len());
            println!("   🗑️  Extra output structs to remove: {}", extra_structs.len());
        }
    }

    if missing_actions.is_empty()
        && missing_outputs.is_empty()
        && extra_functions.is_empty()
        && extra_structs.is_empty()
    {
        // Even if nothing to add/remove, sort the existing functions and structs
        sort_existing_files(&crate_dir, &rust_module)?;
        println!("   ✅ All actions are in sync (sorted alphabetically)");
        return Ok((0, 0));
    }

    println!("   🔧 Missing functions: {}", missing_actions.len());
    println!("   🔧 Missing output structs: {}", missing_outputs.len());

    if dry_run {
        for action in &missing_actions {
            println!("      + {}", action.action);
        }
        for func_name in &extra_functions {
            println!("      - {}", func_name);
        }
        return Ok((missing_actions.len(), extra_functions.len() + extra_structs.len()));
    }

    // Remove extra functions and structs (if force mode)
    let removed_count = if force && (!extra_functions.is_empty() || !extra_structs.is_empty()) {
        let mut total_removed = 0;

        if !extra_functions.is_empty() {
            total_removed += remove_functions(&crate_dir, &rust_module, &extra_functions)?;
        }

        if !extra_structs.is_empty() {
            total_removed += remove_output_structs(&crate_dir, &rust_module, &extra_structs)?;
        }

        total_removed
    } else {
        0
    };

    // Add missing functions to lib.rs
    if !missing_actions.is_empty() {
        add_missing_functions(&crate_dir, &rust_module, &missing_actions)?;
    }

    // Add missing output structs to output.rs
    if !missing_outputs.is_empty() {
        add_missing_output_structs(&crate_dir, &rust_module, &missing_outputs)?;
    }

    Ok((missing_actions.len(), removed_count))
}

fn add_missing_functions(crate_dir: &Path, rust_module: &RustModule, missing_actions: &[&Action]) -> Result<()> {
    let lib_path = crate_dir.join("src/lib.rs");
    let mut content = rust_module.lib_content.clone();

    // Find the position to insert new functions (before the #[cfg(test)] section or at the end)
    let insert_pos = if let Some(pos) = content.find("\n#[cfg(test)]") {
        pos
    } else {
        // Remove trailing newlines and add a couple of newlines before insertion
        content = content.trim_end().to_string();
        content.len()
    };

    // Generate new functions
    let mut new_functions = String::new();
    for action in missing_actions {
        new_functions.push_str("\n\n");
        new_functions.push_str(&RustGenerator::generate_function(action));
    }

    // Insert new functions
    content.insert_str(insert_pos, &new_functions);

    // Sort all functions alphabetically
    content = sort_functions_in_content(&content)?;

    // Write back to file
    fs::write(&lib_path, content).with_context(|| format!("Failed to write {}", lib_path.display()))?;

    println!(
        "   ✅ Added {} functions to lib.rs (sorted alphabetically)",
        missing_actions.len()
    );
    Ok(())
}

fn add_missing_output_structs(crate_dir: &Path, rust_module: &RustModule, missing_outputs: &[&Action]) -> Result<()> {
    let output_path = crate_dir.join("src/output.rs");

    let mut content = if let Some(ref existing) = rust_module.output_content {
        existing.clone()
    } else {
        // Create new output.rs with header
        format!(
            r#"// Harana Actions - {} Module Output Types
// Auto-generated output structs for action methods.

use serde::{{Deserialize, Serialize}};
use serde_json::Value;
use std::collections::HashMap;
"#,
            rust_module
                .lib_content
                .lines()
                .find(|line| line.contains("Harana Actions -"))
                .and_then(|line| line.split('-').nth(1))
                .map(|s| s.trim())
                .unwrap_or(&crate_dir.file_name().unwrap().to_string_lossy())
        )
    };

    // Find insert position (before the last comment about TODO if it exists, otherwise at end)
    let insert_pos = if let Some(pos) = content.find("\n// TODO:") {
        pos
    } else {
        content = content.trim_end().to_string();
        content.len()
    };

    // Generate new output structs
    let mut new_structs = String::new();
    for action in missing_outputs {
        new_structs.push_str("\n\n");
        new_structs.push_str(&RustGenerator::generate_output_struct(action));
    }

    // Insert new structs
    content.insert_str(insert_pos, &new_structs);

    // Sort all output structs alphabetically
    content = sort_structs_in_content(&content)?;

    // Write back to file
    fs::write(&output_path, content).with_context(|| format!("Failed to write {}", output_path.display()))?;

    println!(
        "   ✅ Added {} output structs to output.rs (sorted alphabetically)",
        missing_outputs.len()
    );
    Ok(())
}

fn initialize_crate(crate_dir: &Path, module_name: &str, _schema: &ActionSchema) -> Result<()> {
    // Create directories
    fs::create_dir_all(&crate_dir).with_context(|| format!("Failed to create directory {}", crate_dir.display()))?;

    let src_dir = crate_dir.join("src");
    fs::create_dir_all(&src_dir).with_context(|| format!("Failed to create src directory {}", src_dir.display()))?;

    // Create Cargo.toml
    let cargo_toml_path = crate_dir.join("Cargo.toml");
    let cargo_toml_content = format!(
        r#"[package]
name = "harana-actions-{}"
version = "0.1.0"
edition = "2021"

[dependencies]

"#,
        module_name
    );
    fs::write(&cargo_toml_path, cargo_toml_content)
        .with_context(|| format!("Failed to write {}", cargo_toml_path.display()))?;
    println!("   ✅ Created Cargo.toml");

    // Create lib.rs
    let lib_path = src_dir.join("lib.rs");
    let module_display_name = module_name
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let lib_content = format!(
        r#"// Harana Actions - {} Module
// This module provides {} actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;
"#,
        module_display_name,
        module_name.replace('_', " ")
    );
    fs::write(&lib_path, lib_content).with_context(|| format!("Failed to write {}", lib_path.display()))?;
    println!("   ✅ Created lib.rs");

    // Create output.rs
    let output_path = src_dir.join("output.rs");
    let output_content = format!(
        r#"// Harana Actions - {} Module Output Types
// Auto-generated output structs for action methods.

use serde::{{Deserialize, Serialize}};
use serde_json::Value;
use std::collections::HashMap;
"#,
        module_display_name
    );
    fs::write(&output_path, output_content).with_context(|| format!("Failed to write {}", output_path.display()))?;
    println!("   ✅ Created output.rs");

    Ok(())
}

fn ensure_crate_files(crate_dir: &Path, module_name: &str) -> Result<()> {
    let cargo_toml_path = crate_dir.join("Cargo.toml");
    let src_dir = crate_dir.join("src");
    let lib_path = src_dir.join("lib.rs");

    // Ensure src directory exists
    if !src_dir.exists() {
        fs::create_dir_all(&src_dir)
            .with_context(|| format!("Failed to create src directory {}", src_dir.display()))?;
    }

    // Ensure Cargo.toml exists
    if !cargo_toml_path.exists() {
        let cargo_toml_content = format!(
            r#"[package]
name = "harana-actions-{}"
version = "0.1.0"
edition = "2021"

[dependencies]

"#,
            module_name
        );
        fs::write(&cargo_toml_path, cargo_toml_content)
            .with_context(|| format!("Failed to write {}", cargo_toml_path.display()))?;
        println!("   ✅ Created missing Cargo.toml");
    }

    // Ensure lib.rs exists
    if !lib_path.exists() {
        let module_display_name = module_name
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        let lib_content = format!(
            r#"// Harana Actions - {} Module
// This module provides {} actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;
"#,
            module_display_name,
            module_name.replace('_', " ")
        );
        fs::write(&lib_path, lib_content).with_context(|| format!("Failed to write {}", lib_path.display()))?;
        println!("   ✅ Created missing lib.rs");
    }

    // Ensure output.rs exists
    let output_path = src_dir.join("output.rs");
    if !output_path.exists() {
        let module_display_name = module_name
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        let output_content = format!(
            r#"// Harana Actions - {} Module Output Types
// Auto-generated output structs for action methods.

use serde::{{Deserialize, Serialize}};
use serde_json::Value;
use std::collections::HashMap;
"#,
            module_display_name
        );
        fs::write(&output_path, output_content)
            .with_context(|| format!("Failed to write {}", output_path.display()))?;
        println!("   ✅ Created missing output.rs");
    }

    Ok(())
}

fn remove_functions(crate_dir: &Path, rust_module: &RustModule, functions_to_remove: &[String]) -> Result<usize> {
    let lib_path = crate_dir.join("src/lib.rs");
    let mut content = rust_module.lib_content.clone();
    let mut removed_count = 0;

    for func_name in functions_to_remove {
        // Find the function in the content
        // Look for patterns like:
        // /// Documentation
        // pub async fn function_name(
        // or
        // pub async fn function_name() -> ...

        if let Some(func_start) = find_function_start(&content, func_name) {
            if let Some(func_end) = find_function_end(&content, func_start) {
                // Remove the function including any preceding doc comments and blank lines
                let actual_start = find_doc_comment_start(&content, func_start);

                // Also remove trailing blank lines
                let mut actual_end = func_end;
                while actual_end < content.len() && content[actual_end..].starts_with('\n') {
                    actual_end += 1;
                }

                content.replace_range(actual_start..actual_end, "");
                removed_count += 1;
                println!("      🗑️  Removed function: {}", func_name);
            }
        }
    }

    if removed_count > 0 {
        fs::write(&lib_path, content).with_context(|| format!("Failed to write {}", lib_path.display()))?;
        println!("   ✅ Removed {} functions from lib.rs", removed_count);
    }

    Ok(removed_count)
}

fn remove_output_structs(crate_dir: &Path, rust_module: &RustModule, structs_to_remove: &[String]) -> Result<usize> {
    let output_path = crate_dir.join("src/output.rs");

    if let Some(ref output_content) = rust_module.output_content {
        let mut content = output_content.clone();
        let mut removed_count = 0;

        for struct_name in structs_to_remove {
            // Find the struct in the content
            // Look for patterns like:
            // // comment
            // #[derive(Debug, Clone, Serialize, Deserialize)]
            // pub struct StructName {

            if let Some(struct_start) = find_struct_start(&content, struct_name) {
                if let Some(struct_end) = find_struct_end(&content, struct_start) {
                    // Remove the struct including any preceding comments and blank lines
                    let actual_start = find_comment_start(&content, struct_start);

                    // Also remove trailing blank lines
                    let mut actual_end = struct_end;
                    while actual_end < content.len() && content[actual_end..].starts_with('\n') {
                        actual_end += 1;
                    }

                    content.replace_range(actual_start..actual_end, "");
                    removed_count += 1;
                    println!("      🗑️  Removed struct: {}", struct_name);
                }
            }
        }

        if removed_count > 0 {
            fs::write(&output_path, content).with_context(|| format!("Failed to write {}", output_path.display()))?;
            println!("   ✅ Removed {} structs from output.rs", removed_count);
        }

        Ok(removed_count)
    } else {
        Ok(0)
    }
}

fn find_function_start(content: &str, func_name: &str) -> Option<usize> {
    // Look for "pub async fn function_name" or "pub fn function_name"
    // Also handle raw identifiers like "pub async fn r#move"
    let pattern1 = format!("pub async fn {}(", func_name);
    let pattern2 = format!("pub fn {}(", func_name);
    let pattern3 = format!("pub async fn r#{}(", func_name);
    let pattern4 = format!("pub fn r#{}(", func_name);

    content
        .find(&pattern1)
        .or_else(|| content.find(&pattern2))
        .or_else(|| content.find(&pattern3))
        .or_else(|| content.find(&pattern4))
}

fn find_function_end(content: &str, start: usize) -> Option<usize> {
    // Find the closing brace of the function
    let mut brace_count = 0;
    let mut in_function = false;
    let mut chars = content[start..].char_indices();

    while let Some((idx, ch)) = chars.next() {
        match ch {
            '{' => {
                in_function = true;
                brace_count += 1;
            }
            '}' => {
                brace_count -= 1;
                if in_function && brace_count == 0 {
                    return Some(start + idx + 1);
                }
            }
            _ => {}
        }
    }

    None
}

fn find_doc_comment_start(content: &str, func_start: usize) -> usize {
    let before = &content[..func_start];

    // Find the start of the line where the function begins
    let func_line_start = before.rfind('\n').map(|pos| pos + 1).unwrap_or(0);

    // Collect lines backwards, stopping at non-doc/non-blank content
    let mut lines_to_include = Vec::new();
    let mut search_pos = func_line_start;
    let mut found_doc_comment = false;

    while search_pos > 0 {
        // Find the previous line
        let prev_newline = before[..search_pos.saturating_sub(1)].rfind('\n');
        let line_start = prev_newline.map(|pos| pos + 1).unwrap_or(0);
        let line_end = search_pos.saturating_sub(1);

        if line_start >= line_end {
            break;
        }

        let line = &before[line_start..line_end];
        let trimmed = line.trim();

        // Check what kind of line this is
        if trimmed.starts_with("///") {
            // Doc comment - include it
            lines_to_include.push(line_start);
            found_doc_comment = true;
            search_pos = line_start;
        } else if trimmed.is_empty() {
            // Blank line
            if found_doc_comment {
                // We already found doc comments, so include one blank line but stop after
                lines_to_include.push(line_start);
                search_pos = line_start;
                // Don't include multiple blank lines - stop here
                break;
            } else {
                // Blank line before any doc comments - skip it and keep searching
                search_pos = line_start;
            }
        } else {
            // Hit a non-doc, non-blank line - stop completely
            break;
        }

        if line_start == 0 {
            break;
        }
    }

    // Return the earliest line we want to include
    lines_to_include.last().copied().unwrap_or(func_line_start)
}

fn find_struct_start(content: &str, struct_name: &str) -> Option<usize> {
    // Look for "pub struct StructName"
    let pattern = format!("pub struct {} ", struct_name);
    content.find(&pattern)
}

fn find_struct_end(content: &str, start: usize) -> Option<usize> {
    // Find the closing brace of the struct
    let mut brace_count = 0;
    let mut in_struct = false;
    let mut chars = content[start..].char_indices();

    while let Some((idx, ch)) = chars.next() {
        match ch {
            '{' => {
                in_struct = true;
                brace_count += 1;
            }
            '}' => {
                brace_count -= 1;
                if in_struct && brace_count == 0 {
                    return Some(start + idx + 1);
                }
            }
            _ => {}
        }
    }

    None
}

fn find_comment_start(content: &str, struct_start: usize) -> usize {
    let before = &content[..struct_start];
    let mut lines: Vec<&str> = before.lines().collect();
    let mut actual_start = struct_start;

    // Walk backwards to find comments, derive attributes, and blank lines
    while let Some(line) = lines.pop() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("#[") || trimmed.is_empty() {
            // Calculate the position of this line
            let line_start = before.rfind(line).unwrap_or(struct_start);
            actual_start = line_start;
        } else {
            break;
        }
    }

    actual_start
}

/// Sort existing files even if no changes are needed
fn sort_existing_files(crate_dir: &Path, rust_module: &RustModule) -> Result<()> {
    // Sort lib.rs functions
    let lib_path = crate_dir.join("src/lib.rs");
    let sorted_lib = sort_functions_in_content(&rust_module.lib_content)?;

    // Only write if content changed
    if sorted_lib != rust_module.lib_content {
        fs::write(&lib_path, sorted_lib).with_context(|| format!("Failed to write {}", lib_path.display()))?;
    }

    // Sort output.rs structs
    if let Some(ref output_content) = rust_module.output_content {
        let output_path = crate_dir.join("src/output.rs");
        let sorted_output = sort_structs_in_content(output_content)?;

        // Only write if content changed
        if sorted_output != *output_content {
            fs::write(&output_path, sorted_output)
                .with_context(|| format!("Failed to write {}", output_path.display()))?;
        }
    }

    Ok(())
}

/// Extract individual functions from file content
fn extract_functions(content: &str) -> Result<Vec<(String, String)>> {
    let mut functions = Vec::new();
    let mut current_pos = 0;

    while current_pos < content.len() {
        if let Some(func_start) = find_next_function_start(&content[current_pos..]) {
            let absolute_start = current_pos + func_start;

            if let Some(func_end) = find_function_end(content, absolute_start) {
                // Find doc comments - must do this BEFORE extracting the function text
                let actual_start = find_doc_comment_start(content, absolute_start);

                // Extract function name from the function signature (not including doc comments)
                if let Some(name) = extract_function_name(&content[absolute_start..func_end]) {
                    // Extract full function text including doc comments
                    let func_text = content[actual_start..func_end].to_string();
                    functions.push((name, func_text));
                }

                // Move past this function to avoid re-processing
                current_pos = func_end;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(functions)
}

/// Find the next function start position
fn find_next_function_start(content: &str) -> Option<usize> {
    // Look for "pub async fn" or "pub fn"
    let async_pos = content.find("pub async fn ");
    let sync_pos = content.find("pub fn ");

    match (async_pos, sync_pos) {
        (Some(a), Some(s)) => Some(a.min(s)),
        (Some(a), None) => Some(a),
        (None, Some(s)) => Some(s),
        (None, None) => None,
    }
}

/// Extract function name from function text
fn extract_function_name(func_text: &str) -> Option<String> {
    // Look for pattern: "pub async fn name(" or "pub fn name("
    let text = func_text.trim();

    if let Some(fn_pos) = text.find("fn ") {
        let after_fn = &text[fn_pos + 3..];
        if let Some(paren_pos) = after_fn.find('(') {
            let name = after_fn[..paren_pos].trim();
            // Remove r# prefix if present
            let clean_name = if name.starts_with("r#") { &name[2..] } else { name };
            return Some(clean_name.to_string());
        }
    }

    None
}

/// Sort functions alphabetically in file content
fn sort_functions_in_content(content: &str) -> Result<String> {
    // Split content into header (before first function) and functions
    let first_func_pos = find_next_function_start(content).unwrap_or(content.len());
    let mut header = content[..first_func_pos].to_string();

    // Remove any trailing doc comments from the header (orphaned comments)
    // These might be left over from previous sorts
    let header_lines: Vec<&str> = header.lines().collect();
    let mut keep_until = header_lines.len();

    // Walk backwards from the end of header to find where non-doc content ends
    for (i, line) in header_lines.iter().enumerate().rev() {
        let trimmed = line.trim();
        if trimmed.starts_with("///") || trimmed.is_empty() {
            // This is a doc comment or blank line at the end - don't include it
            keep_until = i;
        } else {
            // Found real content - keep everything up to and including this line
            break;
        }
    }

    // Rebuild header without trailing doc comments
    header = header_lines[..keep_until].join("\n");
    if !header.is_empty() && keep_until > 0 {
        header.push('\n');
    }

    // Extract all functions
    let mut functions = extract_functions(content)?;

    // Sort functions alphabetically by name
    functions.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    // Rebuild content
    let mut result = header.trim_end().to_string();

    for (_, func_text) in functions {
        result.push_str("\n\n");
        result.push_str(&func_text);
    }

    // Add trailing newline
    result.push('\n');

    Ok(result)
}

/// Extract individual structs from file content
fn extract_structs(content: &str) -> Result<Vec<(String, String)>> {
    let mut structs = Vec::new();
    let mut current_pos = 0;

    while let Some(struct_start) = find_next_struct_start(&content[current_pos..]) {
        let absolute_start = current_pos + struct_start;

        if let Some(struct_end) = find_struct_end(content, absolute_start) {
            // Find comments
            let actual_start = find_comment_start(content, absolute_start);

            // Extract struct name
            if let Some(name) = extract_struct_name(&content[absolute_start..struct_end]) {
                let struct_text = content[actual_start..struct_end].to_string();
                structs.push((name, struct_text));
            }

            current_pos = struct_end;
        } else {
            break;
        }
    }

    Ok(structs)
}

/// Find the next struct start position
fn find_next_struct_start(content: &str) -> Option<usize> {
    content.find("pub struct ")
}

/// Extract struct name from struct text
fn extract_struct_name(struct_text: &str) -> Option<String> {
    // Look for pattern: "pub struct Name"
    if let Some(struct_pos) = struct_text.find("pub struct ") {
        let after_struct = &struct_text[struct_pos + 11..];
        // Find the first whitespace or {
        let end_pos = after_struct
            .find(|c: char| c.is_whitespace() || c == '{')
            .unwrap_or(after_struct.len());
        let name = after_struct[..end_pos].trim();
        return Some(name.to_string());
    }

    None
}

/// Sort structs alphabetically in file content
fn sort_structs_in_content(content: &str) -> Result<String> {
    // Split content into header (before first struct) and structs
    let first_struct_pos = find_next_struct_start(content).unwrap_or(content.len());
    let header = &content[..first_struct_pos];

    // Extract all structs
    let mut structs = extract_structs(content)?;

    // Sort structs alphabetically by name
    structs.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    // Rebuild content
    let mut result = header.trim_end().to_string();

    for (_, struct_text) in structs {
        result.push_str("\n\n");
        result.push_str(&struct_text);
    }

    // Add trailing newline
    result.push('\n');

    Ok(result)
}

/// Completely regenerate lib.rs and output.rs from the YAML schema
fn regenerate_files(crate_dir: &Path, module_name: &str, schema: &ActionSchema) -> Result<(usize, usize)> {
    println!("   🔄 Regenerating files from scratch...");

    // Generate lib.rs content
    let lib_content = generate_lib_rs(module_name, &schema.actions)?;
    let lib_path = crate_dir.join("src/lib.rs");
    fs::write(&lib_path, lib_content).with_context(|| format!("Failed to write {}", lib_path.display()))?;

    // Generate output.rs content
    let output_content = generate_output_rs(module_name, &schema.actions)?;
    let output_path = crate_dir.join("src/output.rs");
    fs::write(&output_path, output_content).with_context(|| format!("Failed to write {}", output_path.display()))?;

    println!("   ✅ Regenerated lib.rs with {} functions", schema.actions.len());
    println!("   ✅ Regenerated output.rs with {} structs", schema.actions.len());

    Ok((schema.actions.len(), 0))
}

/// Generate complete lib.rs file from actions
fn generate_lib_rs(module_name: &str, actions: &[Action]) -> Result<String> {
    let module_display_name = module_name
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let mut content = format!(
        r#"// Harana Actions - {} Module
// This module provides {} actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;
"#,
        module_display_name,
        module_name.replace('_', " ")
    );

    // Sort actions alphabetically by action name
    let mut sorted_actions = actions.to_vec();
    sorted_actions.sort_by(|a, b| a.action.to_lowercase().cmp(&b.action.to_lowercase()));

    // Generate each function
    for action in sorted_actions {
        content.push_str("\n");
        content.push_str(&RustGenerator::generate_function(&action));
        content.push_str("\n");
    }

    Ok(content)
}

/// Generate complete output.rs file from actions
fn generate_output_rs(module_name: &str, actions: &[Action]) -> Result<String> {
    let module_display_name = module_name
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let mut content = format!(
        r#"// Harana Actions - {} Module Output Types
// Auto-generated output structs for action methods.

use serde::{{Deserialize, Serialize}};
use serde_json::Value;
use std::collections::HashMap;
"#,
        module_display_name
    );

    // Sort actions alphabetically by action name
    let mut sorted_actions = actions.to_vec();
    sorted_actions.sort_by(|a, b| a.action.to_lowercase().cmp(&b.action.to_lowercase()));

    // Generate each output struct
    for action in sorted_actions {
        content.push_str("\n");
        content.push_str(&RustGenerator::generate_output_struct(&action));
        content.push_str("\n");
    }

    Ok(content)
}
