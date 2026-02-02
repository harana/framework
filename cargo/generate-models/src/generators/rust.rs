use crate::models::*;
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn generate(schema_files: &[SchemaFile], output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir).context("Failed to create Rust output directory")?;

    // Generate a file for each schema file
    for schema_file in schema_files {
        let file_path = output_dir.join(format!("{}.rs", schema_file.file_name));
        let content = generate_file_content(schema_file)?;
        fs::write(&file_path, content)
            .with_context(|| format!("Failed to write Rust file: {}", file_path.display()))?;
    }

    // Generate mod.rs
    let mod_content = generate_mod_file(schema_files);
    fs::write(output_dir.join("mod.rs"), mod_content).context("Failed to write mod.rs")?;

    Ok(())
}

fn generate_file_content(schema_file: &SchemaFile) -> Result<String> {
    let mut output = String::new();

    // Header
    output.push_str("// This file is auto-generated. Do not edit manually.\n\n");
    output.push_str("use serde::{Deserialize, Serialize};\n");
    output.push_str("use chrono::{DateTime, Utc};\n\n");

    // Collect all enums
    let mut enums = HashSet::new();
    for model in &schema_file.models {
        for field in &model.schema {
            if let FieldType::Enum(variants) = &field.field_type {
                enums.insert((field.name.clone(), variants.clone()));
            }
        }
    }

    // Generate enums
    for (field_name, variants) in &enums {
        output.push_str(&generate_enum(field_name, variants));
        output.push_str("\n\n");
    }

    // Generate structs
    for model in &schema_file.models {
        output.push_str(&generate_struct(model));
        output.push_str("\n\n");
    }

    Ok(output)
}

fn generate_enum(field_name: &str, variants: &[String]) -> String {
    let enum_name = to_pascal_case(&format!("{}_type", field_name));
    let mut output = String::new();

    output.push_str(&format!(
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]\n"
    ));
    output.push_str(&format!("#[serde(rename_all = \"lowercase\")]\n"));
    output.push_str(&format!("pub enum {} {{\n", enum_name));

    for variant in variants {
        output.push_str(&format!("    {},\n", to_pascal_case(variant)));
    }

    output.push_str("}");
    output
}

fn generate_struct(model: &Model) -> String {
    let struct_name = to_pascal_case(&model.name);
    let mut output = String::new();

    // Struct documentation
    output.push_str(&format!("/// {}\n", model.name));
    output.push_str(&format!("/// Class: {}\n", model.class));
    output.push_str(&format!("#[derive(Debug, Clone, Serialize, Deserialize)]\n"));
    output.push_str(&format!("#[serde(rename_all = \"snake_case\")]\n"));
    output.push_str(&format!("pub struct {} {{\n", struct_name));

    // Fields
    for field in &model.schema {
        let field_name = &field.name;
        let field_type = field.field_type.to_rust_type(field.nullable);

        // Field documentation
        if !field.constraints.is_empty() {
            let constraints_str = field
                .constraints
                .iter()
                .map(|c| format!("{:?}", c.kind))
                .collect::<Vec<_>>()
                .join(", ");
            output.push_str(&format!("    /// Constraints: {}\n", constraints_str));
        }

        if let Some(ref_info) = &field.reference {
            output.push_str(&format!("    /// Reference: {}.{}\n", ref_info.model, ref_info.field));
        }

        // Handle defaults
        if let Some(default) = &field.default_value {
            if default == "now()" {
                output.push_str("    #[serde(default = \"chrono::Utc::now\")]\n");
            } else if default == "true" || default == "false" {
                output.push_str(&format!("    #[serde(default)]\n"));
            }
        }

        output.push_str(&format!("    pub {}: {},\n", to_snake_case(field_name), field_type));
    }

    output.push_str("}");

    // Add implementation with validation methods
    output.push_str(&format!("\n\nimpl {} {{\n", struct_name));
    output.push_str("    /// Validate the model according to schema constraints\n");
    output.push_str("    pub fn validate(&self) -> Result<(), String> {\n");

    for field in &model.schema {
        let field_name_snake = to_snake_case(&field.name);

        for constraint in &field.constraints {
            match constraint.kind {
                ConstraintKind::Required => {
                    if field.nullable {
                        output.push_str(&format!("        if self.{}.is_none() {{\n", field_name_snake));
                        output.push_str(&format!(
                            "            return Err(\"Field '{}' is required\".to_string());\n",
                            field.name
                        ));
                        output.push_str("        }\n");
                    }
                }
                ConstraintKind::Min => {
                    if let Some(value) = &constraint.value {
                        match field.field_type {
                            FieldType::Int => {
                                output.push_str(&format!("        if self.{} < {} {{\n", field_name_snake, value));
                            }
                            _ => {}
                        }
                    }
                }
                ConstraintKind::Max => {
                    if let Some(value) = &constraint.value {
                        match field.field_type {
                            FieldType::String => {
                                output.push_str(&format!(
                                    "        if let Some(ref val) = self.{} {{\n",
                                    field_name_snake
                                ));
                                output.push_str(&format!("            if val.len() > {} {{\n", value));
                                output.push_str(&format!(
                                    "                return Err(\"Field '{}' exceeds max length of {}\".to_string());\n",
                                    field.name, value
                                ));
                                output.push_str("            }\n");
                                output.push_str("        }\n");
                            }
                            FieldType::Int => {
                                output.push_str(&format!("        if self.{} > {} {{\n", field_name_snake, value));
                                output.push_str(&format!(
                                    "            return Err(\"Field '{}' exceeds max value of {}\".to_string());\n",
                                    field.name, value
                                ));
                                output.push_str("        }\n");
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    output.push_str("        Ok(())\n");
    output.push_str("    }\n");
    output.push_str("}");

    output
}

fn generate_mod_file(schema_files: &[SchemaFile]) -> String {
    let mut output = String::new();
    output.push_str("// This file is auto-generated. Do not edit manually.\n\n");

    for schema_file in schema_files {
        output.push_str(&format!("pub mod {};\n", schema_file.file_name));
    }

    output
}

fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == '_' || c == ' ' || c == '-')
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn to_snake_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                vec!['_', c.to_lowercase().next().unwrap()]
            } else {
                vec![c.to_lowercase().next().unwrap()]
            }
        })
        .collect()
}
