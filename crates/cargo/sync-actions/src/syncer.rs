use crate::analyzer::RustModule;
use crate::generator::RustGenerator;
use crate::parser::{Action, ActionSchema};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn sync_schema(schema: &ActionSchema, actions_dir: &Path, dry_run: bool) -> Result<usize> {
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

    // Analyze existing Rust module
    let rust_module =
        RustModule::analyze(&crate_dir).with_context(|| format!("Failed to analyze module {}", schema.module_name))?;

    println!("   Existing functions: {}", rust_module.existing_functions.len());

    // Find missing actions
    let mut missing_actions = Vec::new();
    let mut missing_outputs = Vec::new();

    for action in &schema.actions {
        if !rust_module.has_function(&action.class) {
            missing_actions.push(action);
        }

        let output_struct = RustGenerator::to_pascal_case(&action.class) + "Output";
        if !rust_module.has_output_struct(&output_struct) {
            missing_outputs.push(action);
        }
    }

    if missing_actions.is_empty() && missing_outputs.is_empty() {
        println!("   ✅ All actions are already implemented");
        return Ok(0);
    }

    println!("   🔧 Missing functions: {}", missing_actions.len());
    println!("   🔧 Missing output structs: {}", missing_outputs.len());

    if dry_run {
        for action in &missing_actions {
            println!("      - {}", action.class);
        }
        return Ok(missing_actions.len());
    }

    // Add missing functions to lib.rs
    if !missing_actions.is_empty() {
        add_missing_functions(&crate_dir, &rust_module, &missing_actions)?;
    }

    // Add missing output structs to output.rs
    if !missing_outputs.is_empty() {
        add_missing_output_structs(&crate_dir, &rust_module, &missing_outputs)?;
    }

    Ok(missing_actions.len())
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

    // Write back to file
    fs::write(&lib_path, content).with_context(|| format!("Failed to write {}", lib_path.display()))?;

    println!("   ✅ Added {} functions to lib.rs", missing_actions.len());
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

    // Write back to file
    fs::write(&output_path, content).with_context(|| format!("Failed to write {}", output_path.display()))?;

    println!("   ✅ Added {} output structs to output.rs", missing_outputs.len());
    Ok(())
}

fn initialize_crate(crate_dir: &Path, module_name: &str, schema: &ActionSchema) -> Result<()> {
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
