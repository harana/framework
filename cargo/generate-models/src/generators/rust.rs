use crate::models::*;
use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::path::Path;

/// Represents a directory tree node for generating mod.rs files
#[derive(Debug, Default)]
struct ModTree {
    /// Module names (file-based modules) at this level
    modules: BTreeSet<String>,
    /// Subdirectory module names -> subtree
    subdirs: BTreeMap<String, ModTree>,
}

impl ModTree {
    fn insert(&mut self, relative_dir: &str, file_name: &str) {
        if relative_dir.is_empty() {
            self.modules.insert(file_name.to_string());
        } else {
            let parts: Vec<&str> = relative_dir.splitn(2, '/').collect();
            let first = parts[0];
            let rest = if parts.len() > 1 { parts[1] } else { "" };
            self.subdirs
                .entry(first.to_string())
                .or_default()
                .insert(rest, file_name);
        }
    }
}

pub fn generate(schema_files: &[SchemaFile], output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir).context("Failed to create Rust output directory")?;

    // Build a top-level mod tree with category subdirectories
    let mut top_mod_tree = ModTree::default();

    // Group schema files by category
    let mut by_category: BTreeMap<SchemaCategory, Vec<&SchemaFile>> = BTreeMap::new();
    for sf in schema_files {
        by_category.entry(sf.category).or_default().push(sf);
    }

    for (category, files) in &by_category {
        let cat_dir_name = category.dir_name();
        let cat_output_dir = output_dir.join(cat_dir_name);
        fs::create_dir_all(&cat_output_dir)?;

        // Register this category as a subdir of top-level
        top_mod_tree.subdirs.entry(cat_dir_name.to_string()).or_default();

        // Build a mod tree within this category
        let mut cat_mod_tree = ModTree::default();

        // Collect dir names per relative dir level for collision detection
        let mut dirs_per_level: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for schema_file in files {
            if !schema_file.category_relative_dir.is_empty() {
                let parent = std::path::Path::new(&schema_file.category_relative_dir)
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                let child_dir = std::path::Path::new(&schema_file.category_relative_dir)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string());
                if let Some(dir_name) = child_dir {
                    dirs_per_level.entry(parent).or_default().insert(dir_name);
                }
            }
        }

        // Generate a file for each schema file
        for schema_file in files {
            let parent_dir_key = schema_file.category_relative_dir.clone();

            // Check if this file name collides with a subdirectory at the same level
            let collides_with_subdir = dirs_per_level
                .get(&parent_dir_key)
                .map(|dirs| dirs.contains(&schema_file.file_name))
                .unwrap_or(false);

            let (target_dir, effective_relative_dir) = if collides_with_subdir {
                let new_rel = if schema_file.category_relative_dir.is_empty() {
                    schema_file.file_name.clone()
                } else {
                    format!("{}/{}", schema_file.category_relative_dir, schema_file.file_name)
                };
                (cat_output_dir.join(&new_rel), new_rel)
            } else {
                let dir = if schema_file.category_relative_dir.is_empty() {
                    cat_output_dir.clone()
                } else {
                    cat_output_dir.join(&schema_file.category_relative_dir)
                };
                (dir, schema_file.category_relative_dir.clone())
            };

            fs::create_dir_all(&target_dir)
                .with_context(|| format!("Failed to create directory: {}", target_dir.display()))?;

            let file_path = target_dir.join(format!("{}.rs", schema_file.file_name));
            let content = generate_file_content(schema_file)?;
            fs::write(&file_path, content)
                .with_context(|| format!("Failed to write Rust file: {}", file_path.display()))?;

            cat_mod_tree.insert(&effective_relative_dir, &schema_file.file_name);
        }

        // Generate mod.rs files within the category directory
        generate_mod_files(&cat_mod_tree, &cat_output_dir)?;
    }

    // Generate top-level mod.rs (without recursion, since category subtrees are already generated)
    generate_top_level_mod_file(&top_mod_tree, output_dir)?;

    Ok(())
}

/// Writes a mod.rs at the given directory listing only the direct children (no recursion).
fn generate_top_level_mod_file(tree: &ModTree, dir: &Path) -> Result<()> {
    let mut mod_content = String::from("// This file is auto-generated. Do not edit manually.\n\n");

    let mut all_modules = BTreeSet::new();
    for subdir_name in tree.subdirs.keys() {
        all_modules.insert(subdir_name.clone());
    }
    for module_name in &tree.modules {
        all_modules.insert(module_name.clone());
    }

    for module_name in &all_modules {
        mod_content.push_str(&format!("pub mod {};\n", module_name));
    }

    fs::write(dir.join("mod.rs"), &mod_content)
        .with_context(|| format!("Failed to write mod.rs in {}", dir.display()))?;

    Ok(())
}

fn generate_mod_files(tree: &ModTree, dir: &Path) -> Result<()> {
    let mut mod_content = String::from("// This file is auto-generated. Do not edit manually.\n\n");

    // Collect all module names (both subdirs and files), deduplicating
    let mut all_modules = BTreeSet::new();
    for subdir_name in tree.subdirs.keys() {
        all_modules.insert(subdir_name.clone());
    }
    for module_name in &tree.modules {
        all_modules.insert(module_name.clone());
    }

    for module_name in &all_modules {
        mod_content.push_str(&format!("pub mod {};\n", module_name));
    }

    fs::write(dir.join("mod.rs"), &mod_content)
        .with_context(|| format!("Failed to write mod.rs in {}", dir.display()))?;

    // Recurse into subdirectories
    for (subdir_name, subtree) in &tree.subdirs {
        let subdir_path = dir.join(subdir_name);
        fs::create_dir_all(&subdir_path)?;
        generate_mod_files(subtree, &subdir_path)?;
    }

    Ok(())
}

fn generate_file_content(schema_file: &SchemaFile) -> Result<String> {
    match schema_file.category {
        SchemaCategory::Action => generate_action_file(schema_file),
        SchemaCategory::Config | SchemaCategory::Event | SchemaCategory::Object => generate_struct_file(schema_file),
        SchemaCategory::WebObject => generate_webobject_file(schema_file),
    }
}

// ---------------------------------------------------------------------------
// Action file generation (traits + structs for classes)
// ---------------------------------------------------------------------------

fn generate_action_file(schema_file: &SchemaFile) -> Result<String> {
    let mut output = String::new();

    output.push_str("// This file is auto-generated. Do not edit manually.\n\n");
    output.push_str("use async_trait::async_trait;\n");
    output.push_str("use serde::{Deserialize, Serialize};\n\n");

    // Separate methods and classes
    let mut methods: Vec<&ActionMethod> = Vec::new();
    let mut classes: Vec<&ActionClass> = Vec::new();

    for model in &schema_file.models {
        match model {
            Model::ActionMethod(m) => methods.push(m),
            Model::ActionClass(c) => classes.push(c),
            _ => {}
        }
    }

    // Generate structs for input/output types
    for method in &methods {
        // Input struct
        if !method.inputs.is_empty() {
            let struct_name = to_pascal_case(&format!("{}_input", method.name));
            output.push_str(&generate_action_struct(&struct_name, &method.inputs));
            output.push_str("\n\n");
        }

        // Output struct
        if !method.outputs.is_empty() {
            let struct_name = to_pascal_case(&format!("{}_output", method.name));
            output.push_str(&generate_action_struct(&struct_name, &method.outputs));
            output.push_str("\n\n");
        }
    }

    // Generate structs for class definitions
    for class in &classes {
        output.push_str(&generate_action_struct(&to_pascal_case(&class.name), &class.fields));
        output.push_str("\n\n");
    }

    // Generate trait
    if !methods.is_empty() {
        let trait_name = to_pascal_case(&format!("{}_action", schema_file.file_name));
        output.push_str("#[async_trait]\n");
        output.push_str(&format!("pub trait {} {{\n", trait_name));

        for method in &methods {
            let method_name = to_snake_case(&method.name);

            let input_type = if method.inputs.is_empty() {
                String::new()
            } else {
                format!("input: {}", to_pascal_case(&format!("{}_input", method.name)))
            };

            let output_type = if method.outputs.is_empty() {
                "()".to_string()
            } else {
                to_pascal_case(&format!("{}_output", method.name))
            };

            output.push_str(&format!(
                "    async fn {}(&self{}) -> Result<{}, Box<dyn std::error::Error>>;\n",
                method_name,
                if input_type.is_empty() {
                    String::new()
                } else {
                    format!(", {}", input_type)
                },
                output_type,
            ));
        }

        output.push_str("}\n");
    }

    Ok(output)
}

fn generate_action_struct(name: &str, fields: &[ActionField]) -> String {
    let mut output = String::new();

    // Collect enums
    let mut enums = HashSet::new();
    for field in fields {
        if let FieldType::Enum(variants) = &field.field_type {
            enums.insert((field.name.clone(), variants.clone()));
        }
    }

    // Generate enums
    for (field_name, variants) in &enums {
        output.push_str(&generate_enum(field_name, variants));
        output.push_str("\n\n");
    }

    output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    output.push_str("#[serde(rename_all = \"snake_case\")]\n");
    output.push_str(&format!("pub struct {} {{\n", name));

    for field in fields {
        let field_name = to_snake_case(&field.name);
        let field_type = field.field_type.to_rust_type(false);

        if let Some(default) = &field.default_value {
            if default == "now()" {
                output.push_str("    #[serde(default = \"chrono::Utc::now\")]\n");
            } else if default == "true" || default == "false" {
                output.push_str("    #[serde(default)]\n");
            }
        }

        output.push_str(&format!("    pub {}: {},\n", field_name, field_type));
    }

    output.push_str("}");
    output
}

// ---------------------------------------------------------------------------
// Struct file generation (config / event / object)
// ---------------------------------------------------------------------------

fn generate_struct_file(schema_file: &SchemaFile) -> Result<String> {
    let mut output = String::new();

    output.push_str("// This file is auto-generated. Do not edit manually.\n\n");
    output.push_str("use serde::{Deserialize, Serialize};\n");
    output.push_str("use chrono::{DateTime, Utc};\n\n");

    for model in &schema_file.models {
        if let Model::Struct(s) = model {
            // Collect all enums
            let mut enums = HashSet::new();
            for field in &s.fields {
                if let FieldType::Enum(variants) = &field.field_type {
                    enums.insert((field.name.clone(), variants.clone()));
                }
            }

            // Generate enums
            for (field_name, variants) in &enums {
                output.push_str(&generate_enum(field_name, variants));
                output.push_str("\n\n");
            }

            // Generate struct
            output.push_str(&generate_struct(s));
            output.push_str("\n\n");
        }
    }

    Ok(output)
}

fn generate_struct(model: &StructModel) -> String {
    let struct_name = to_pascal_case(&model.name);
    let mut output = String::new();

    output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    output.push_str("#[serde(rename_all = \"snake_case\")]\n");
    output.push_str(&format!("pub struct {} {{\n", struct_name));

    for field in &model.fields {
        let field_name = &field.name;
        let field_type = field.field_type.to_rust_type(field.nullable);

        if let Some(default) = &field.default_value {
            if default == "now()" {
                output.push_str("    #[serde(default = \"chrono::Utc::now\")]\n");
            } else if default == "true" || default == "false" {
                output.push_str("    #[serde(default)]\n");
            }
        }

        output.push_str(&format!("    pub {}: {},\n", to_snake_case(field_name), field_type));
    }

    output.push_str("}\n");

    // Add implementation with validation methods
    output.push_str(&format!("\nimpl {} {{\n", struct_name));
    output.push_str("    pub fn validate(&self) -> Result<(), String> {\n");

    for field in &model.fields {
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
                                output.push_str(&format!(
                                    "            return Err(\"Field '{}' is below min value of {}\".to_string());\n",
                                    field.name, value
                                ));
                                output.push_str("        }\n");
                            }
                            _ => {}
                        }
                    }
                }
                ConstraintKind::Max => {
                    if let Some(value) = &constraint.value {
                        match field.field_type {
                            FieldType::String => {
                                if field.nullable {
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
                                } else {
                                    output.push_str(&format!(
                                        "        if self.{}.len() > {} {{\n",
                                        field_name_snake, value
                                    ));
                                    output.push_str(&format!(
                                        "            return Err(\"Field '{}' exceeds max length of {}\".to_string());\n",
                                        field.name, value
                                    ));
                                    output.push_str("        }\n");
                                }
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

// ---------------------------------------------------------------------------
// WebObject file generation
// ---------------------------------------------------------------------------

fn generate_webobject_file(schema_file: &SchemaFile) -> Result<String> {
    let mut output = String::new();

    output.push_str("// This file is auto-generated. Do not edit manually.\n\n");
    output.push_str("use serde::{Deserialize, Serialize};\n\n");

    for model in &schema_file.models {
        if let Model::WebObject(wo) = model {
            // Collect all enums
            let mut enums = HashSet::new();
            for field in &wo.attributes {
                if let FieldType::Enum(variants) = &field.field_type {
                    enums.insert((field.name.clone(), variants.clone()));
                }
            }

            // Generate enums
            for (field_name, variants) in &enums {
                output.push_str(&generate_enum(field_name, variants));
                output.push_str("\n\n");
            }

            // Generate event enum if there are events
            if !wo.events.is_empty() {
                let event_enum_name = to_pascal_case(&format!("{}_event", wo.name));
                output.push_str("#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]\n");
                output.push_str("#[serde(rename_all = \"snake_case\")]\n");
                output.push_str(&format!("pub enum {} {{\n", event_enum_name));
                for event in &wo.events {
                    output.push_str(&format!("    {},\n", to_pascal_case(event)));
                }
                output.push_str("}\n\n");
            }

            // Generate struct
            let struct_name = to_pascal_case(&wo.name);
            output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
            output.push_str("#[serde(rename_all = \"snake_case\")]\n");
            output.push_str(&format!("pub struct {} {{\n", struct_name));

            for field in &wo.attributes {
                let field_name = &field.name;
                let field_type = field.field_type.to_rust_type(field.nullable);

                if let Some(default) = &field.default_value {
                    if default == "true" || default == "false" {
                        output.push_str("    #[serde(default)]\n");
                    }
                }

                output.push_str(&format!("    pub {}: {},\n", to_snake_case(field_name), field_type));
            }

            output.push_str("}\n\n");
        }
    }

    Ok(output)
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn generate_enum(field_name: &str, variants: &[String]) -> String {
    let enum_name = to_pascal_case(&format!("{}_type", field_name));
    let mut output = String::new();

    output.push_str("#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]\n");
    output.push_str("#[serde(rename_all = \"lowercase\")]\n");
    output.push_str(&format!("pub enum {} {{\n", enum_name));

    for variant in variants {
        output.push_str(&format!("    {},\n", to_pascal_case(variant)));
    }

    output.push_str("}");
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
