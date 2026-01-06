use crate::models::*;
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn generate(schema_files: &[SchemaFile], output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir).context("Failed to create Python output directory")?;

    // Generate a file for each schema file
    for schema_file in schema_files {
        let file_path = output_dir.join(format!("{}.py", schema_file.file_name));
        let content = generate_file_content(schema_file)?;
        fs::write(&file_path, content)
            .with_context(|| format!("Failed to write Python file: {}", file_path.display()))?;
    }

    // Generate __init__.py
    let init_content = generate_init_file(schema_files);
    fs::write(output_dir.join("__init__.py"), init_content).context("Failed to write __init__.py")?;

    Ok(())
}

fn generate_file_content(schema_file: &SchemaFile) -> Result<String> {
    let mut output = String::new();

    // Header
    output.push_str("# This file is auto-generated. Do not edit manually.\n\n");
    output.push_str("from datetime import datetime\n");
    output.push_str("from typing import Optional, List\n");
    output.push_str("from enum import Enum\n");
    output.push_str("from pydantic import BaseModel, Field, validator, field_validator\n\n");

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

    // Generate classes
    for model in &schema_file.models {
        output.push_str(&generate_class(model));
        output.push_str("\n\n");
    }

    Ok(output)
}

fn generate_enum(field_name: &str, variants: &[String]) -> String {
    let enum_name = to_pascal_case(&format!("{}_type", field_name));
    let mut output = String::new();

    output.push_str(&format!("class {}(str, Enum):\n", enum_name));
    output.push_str(&format!("    \"\"\"Enum for {} field\"\"\"\n", field_name));

    for variant in variants {
        let variant_upper = variant.to_uppercase();
        output.push_str(&format!("    {} = \"{}\"\n", variant_upper, variant));
    }

    output
}

fn generate_class(model: &Model) -> String {
    let class_name = to_pascal_case(&model.name);
    let mut output = String::new();

    // Class definition
    output.push_str(&format!("class {}(BaseModel):\n", class_name));
    output.push_str(&format!("    \"\"\"\n"));
    output.push_str(&format!("    {}\n", model.name));
    output.push_str(&format!("    \n"));
    output.push_str(&format!("    Class: {}\n", model.class));
    output.push_str(&format!("    ID fields: {}\n", model.id.join(", ")));
    output.push_str(&format!("    \"\"\"\n\n"));

    // Fields
    for field in &model.schema {
        let field_name = to_snake_case(&field.name);
        let field_type = field.field_type.to_python_type(field.nullable);

        // Build field definition with validators
        let mut validators = Vec::new();

        for constraint in &field.constraints {
            match constraint.kind {
                ConstraintKind::Min => {
                    if let Some(value) = &constraint.value {
                        validators.push(format!("ge={}", value));
                    }
                }
                ConstraintKind::Max => {
                    if let Some(value) = &constraint.value {
                        match field.field_type {
                            FieldType::String => {
                                validators.push(format!("max_length={}", value));
                            }
                            FieldType::Int | FieldType::Decimal => {
                                validators.push(format!("le={}", value));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        // Add description
        let mut description_parts = Vec::new();
        if !field.constraints.is_empty() {
            let constraints_str = field
                .constraints
                .iter()
                .map(|c| format!("{:?}", c.kind))
                .collect::<Vec<_>>()
                .join(", ");
            description_parts.push(format!("Constraints: {}", constraints_str));
        }
        if let Some(ref_info) = &field.reference {
            description_parts.push(format!("Reference: {}.{}", ref_info.model, ref_info.field));
        }

        let description = if !description_parts.is_empty() {
            format!("description=\"{}\"", description_parts.join("; "))
        } else {
            String::new()
        };

        // Generate field with Field() if there are validators or description
        if !validators.is_empty() || !description.is_empty() || field.default_value.is_some() {
            let mut field_args = Vec::new();

            if let Some(default) = &field.default_value {
                if default == "now()" {
                    field_args.push("default_factory=datetime.utcnow".to_string());
                } else if default == "true" {
                    field_args.push("default=True".to_string());
                } else if default == "false" {
                    field_args.push("default=False".to_string());
                } else if let Ok(_) = default.parse::<i64>() {
                    field_args.push(format!("default={}", default));
                } else {
                    field_args.push(format!("default=\"{}\"", default));
                }
            } else if field.nullable {
                field_args.push("default=None".to_string());
            }

            field_args.extend(validators);

            if !description.is_empty() {
                field_args.push(description);
            }

            output.push_str(&format!(
                "    {}: {} = Field({})\n",
                field_name,
                field_type,
                field_args.join(", ")
            ));
        } else {
            // Simple field without Field()
            if field.nullable {
                output.push_str(&format!("    {}: {} = None\n", field_name, field_type));
            } else {
                output.push_str(&format!("    {}: {}\n", field_name, field_type));
            }
        }
    }

    // Add validators for special constraints
    let email_fields: Vec<&Field> = model
        .schema
        .iter()
        .filter(|f| f.constraints.iter().any(|c| c.kind == ConstraintKind::Email))
        .collect();

    let url_fields: Vec<&Field> = model
        .schema
        .iter()
        .filter(|f| f.constraints.iter().any(|c| c.kind == ConstraintKind::Url))
        .collect();

    if !email_fields.is_empty() {
        output.push_str("\n");
        for field in email_fields {
            let field_name = to_snake_case(&field.name);
            output.push_str(&format!("    @field_validator('{}')\n", field_name));
            output.push_str("    @classmethod\n");
            output.push_str(&format!("    def validate_{}(cls, v):\n", field_name));
            output.push_str("        if v is not None and '@' not in v:\n");
            output.push_str("            raise ValueError('Invalid email address')\n");
            output.push_str("        return v\n\n");
        }
    }

    if !url_fields.is_empty() {
        for field in url_fields {
            let field_name = to_snake_case(&field.name);
            output.push_str(&format!("    @field_validator('{}')\n", field_name));
            output.push_str("    @classmethod\n");
            output.push_str(&format!("    def validate_{}(cls, v):\n", field_name));
            output
                .push_str("        if v is not None and not (v.startswith('http://') or v.startswith('https://')):\n");
            output.push_str("            raise ValueError('Invalid URL')\n");
            output.push_str("        return v\n\n");
        }
    }

    // Config
    output.push_str("    class Config:\n");
    output.push_str("        from_attributes = True\n");
    output.push_str("        populate_by_name = True\n");

    output
}

fn generate_init_file(schema_files: &[SchemaFile]) -> String {
    let mut output = String::new();
    output.push_str("# This file is auto-generated. Do not edit manually.\n\n");

    for schema_file in schema_files {
        output.push_str(&format!("from .{} import *\n", schema_file.file_name));
    }

    output.push_str("\n__all__ = [\n");

    for schema_file in schema_files {
        for model in &schema_file.models {
            let class_name = to_pascal_case(&model.name);
            output.push_str(&format!("    \"{}\",\n", class_name));
        }
    }

    output.push_str("]\n");

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
