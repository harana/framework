use crate::models::*;
use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::path::Path;

/// Represents a directory tree node for generating __init__.py files
#[derive(Debug, Default)]
struct PkgTree {
    /// Module names (file-based modules) at this level, with their model class names
    modules: BTreeMap<String, Vec<String>>,
    /// Subdirectory package names -> subtree
    subdirs: BTreeMap<String, PkgTree>,
}

impl PkgTree {
    fn insert(&mut self, relative_dir: &str, file_name: &str, class_names: Vec<String>) {
        if relative_dir.is_empty() {
            self.modules.insert(file_name.to_string(), class_names);
        } else {
            let parts: Vec<&str> = relative_dir.splitn(2, '/').collect();
            let first = parts[0];
            let rest = if parts.len() > 1 { parts[1] } else { "" };
            self.subdirs
                .entry(first.to_string())
                .or_default()
                .insert(rest, file_name, class_names);
        }
    }
}

pub fn generate(schema_files: &[SchemaFile], output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir).context("Failed to create Python output directory")?;

    // Build a top-level package tree with category subdirectories
    let mut top_pkg_tree = PkgTree::default();

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
        top_pkg_tree.subdirs.entry(cat_dir_name.to_string()).or_default();

        // Build a package tree within this category
        let mut cat_pkg_tree = PkgTree::default();

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

            let file_path = target_dir.join(format!("{}.py", schema_file.file_name));
            let content = generate_file_content(schema_file)?;
            fs::write(&file_path, content)
                .with_context(|| format!("Failed to write Python file: {}", file_path.display()))?;

            let class_names: Vec<String> = schema_file.models.iter().map(|m| to_pascal_case(m.name())).collect();
            cat_pkg_tree.insert(&effective_relative_dir, &schema_file.file_name, class_names);
        }

        // Generate __init__.py files within the category directory
        generate_init_files(&cat_pkg_tree, &cat_output_dir)?;
    }

    // Generate top-level __init__.py (without recursion, since category subtrees are already generated)
    generate_top_level_init_file(&top_pkg_tree, output_dir)?;

    Ok(())
}

/// Writes a __init__.py at the given directory listing only the direct children (no recursion).
fn generate_top_level_init_file(tree: &PkgTree, dir: &Path) -> Result<()> {
    let mut output = String::from("# This file is auto-generated. Do not edit manually.\n\n");

    for subdir_name in tree.subdirs.keys() {
        output.push_str(&format!("from . import {}\n", subdir_name));
    }
    for (module_name, _) in &tree.modules {
        output.push_str(&format!("from .{} import *\n", module_name));
    }

    output.push_str("\n__all__ = [\n");
    for subdir_name in tree.subdirs.keys() {
        output.push_str(&format!("    \"{}\",\n", subdir_name));
    }
    for (_, class_names) in &tree.modules {
        for class_name in class_names {
            output.push_str(&format!("    \"{}\",\n", class_name));
        }
    }
    output.push_str("]\n");

    fs::write(dir.join("__init__.py"), &output)
        .with_context(|| format!("Failed to write __init__.py in {}", dir.display()))?;

    Ok(())
}

fn generate_init_files(tree: &PkgTree, dir: &Path) -> Result<()> {
    let mut output = String::from("# This file is auto-generated. Do not edit manually.\n\n");

    // Collect all module names deduplicating subdirs and file modules
    let mut all_subdir_names: BTreeSet<String> = BTreeSet::new();
    for subdir_name in tree.subdirs.keys() {
        all_subdir_names.insert(subdir_name.clone());
    }

    // Import from subpackages (only those not also a file module)
    for subdir_name in &all_subdir_names {
        output.push_str(&format!("from . import {}\n", subdir_name));
    }

    // Import from file modules
    for (module_name, _class_names) in &tree.modules {
        output.push_str(&format!("from .{} import *\n", module_name));
    }

    // __all__
    output.push_str("\n__all__ = [\n");
    for subdir_name in &all_subdir_names {
        if !tree.modules.contains_key(subdir_name) {
            output.push_str(&format!("    \"{}\",\n", subdir_name));
        }
    }
    for (_module_name, class_names) in &tree.modules {
        for class_name in class_names {
            output.push_str(&format!("    \"{}\",\n", class_name));
        }
    }
    output.push_str("]\n");

    fs::write(dir.join("__init__.py"), &output)
        .with_context(|| format!("Failed to write __init__.py in {}", dir.display()))?;

    // Recurse into subdirectories
    for (subdir_name, subtree) in &tree.subdirs {
        let subdir_path = dir.join(subdir_name);
        fs::create_dir_all(&subdir_path)?;
        generate_init_files(subtree, &subdir_path)?;
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
// Action file generation (ABCs + dataclasses for classes)
// ---------------------------------------------------------------------------

fn generate_action_file(schema_file: &SchemaFile) -> Result<String> {
    let mut output = String::new();

    output.push_str("# This file is auto-generated. Do not edit manually.\n\n");
    output.push_str("from abc import ABC, abstractmethod\n");
    output.push_str("from typing import Optional, List\n");
    output.push_str("from pydantic import BaseModel\n\n");

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

    // Generate dataclasses for input/output types
    for method in &methods {
        if !method.inputs.is_empty() {
            let class_name = to_pascal_case(&format!("{}_input", method.name));
            output.push_str(&generate_action_dataclass(&class_name, &method.inputs));
            output.push_str("\n\n");
        }
        if !method.outputs.is_empty() {
            let class_name = to_pascal_case(&format!("{}_output", method.name));
            output.push_str(&generate_action_dataclass(&class_name, &method.outputs));
            output.push_str("\n\n");
        }
    }

    // Generate dataclasses for class definitions
    for class in &classes {
        output.push_str(&generate_action_dataclass(&to_pascal_case(&class.name), &class.fields));
        output.push_str("\n\n");
    }

    // Generate abstract base class (trait equivalent)
    if !methods.is_empty() {
        let trait_name = to_pascal_case(&format!("{}_action", schema_file.file_name));
        output.push_str(&format!("class {}(ABC):\n\n", trait_name));

        for method in &methods {
            let method_name = to_snake_case(&method.name);

            let input_param = if method.inputs.is_empty() {
                String::new()
            } else {
                format!(", input: {}", to_pascal_case(&format!("{}_input", method.name)))
            };

            let output_type = if method.outputs.is_empty() {
                "None".to_string()
            } else {
                to_pascal_case(&format!("{}_output", method.name))
            };

            output.push_str("    @abstractmethod\n");
            output.push_str(&format!(
                "    async def {}(self{}) -> {}:\n",
                method_name, input_param, output_type
            ));
            output.push_str("        ...\n\n");
        }
    }

    Ok(output)
}

fn generate_action_dataclass(name: &str, fields: &[ActionField]) -> String {
    let mut output = String::new();

    output.push_str(&format!("class {}(BaseModel):\n\n", name));

    if fields.is_empty() {
        output.push_str("    pass\n");
        return output;
    }

    for field in fields {
        let field_name = to_snake_case(&field.name);
        let field_type = field.field_type.to_python_type(false);

        if let Some(default) = &field.default_value {
            if default == "true" {
                output.push_str(&format!("    {}: {} = True\n", field_name, field_type));
            } else if default == "false" {
                output.push_str(&format!("    {}: {} = False\n", field_name, field_type));
            } else if let Ok(_) = default.parse::<i64>() {
                output.push_str(&format!("    {}: {} = {}\n", field_name, field_type, default));
            } else if let Ok(_) = default.parse::<f64>() {
                output.push_str(&format!("    {}: {} = {}\n", field_name, field_type, default));
            } else {
                output.push_str(&format!("    {}: {} = \"{}\"\n", field_name, field_type, default));
            }
        } else {
            output.push_str(&format!("    {}: {}\n", field_name, field_type));
        }
    }

    output
}

// ---------------------------------------------------------------------------
// Struct file generation (config / event / object)
// ---------------------------------------------------------------------------

fn generate_struct_file(schema_file: &SchemaFile) -> Result<String> {
    let mut output = String::new();

    output.push_str("# This file is auto-generated. Do not edit manually.\n\n");
    output.push_str("from datetime import datetime\n");
    output.push_str("from typing import Optional, List\n");
    output.push_str("from enum import Enum\n");
    output.push_str("from pydantic import BaseModel, Field, field_validator\n\n");

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

            // Generate class
            output.push_str(&generate_class(s));
            output.push_str("\n\n");
        }
    }

    Ok(output)
}

// ---------------------------------------------------------------------------
// WebObject file generation
// ---------------------------------------------------------------------------

fn generate_webobject_file(schema_file: &SchemaFile) -> Result<String> {
    let mut output = String::new();

    output.push_str("# This file is auto-generated. Do not edit manually.\n\n");
    output.push_str("from typing import Optional, List\n");
    output.push_str("from enum import Enum\n");
    output.push_str("from pydantic import BaseModel, Field\n\n");

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

            // Generate event enum
            if !wo.events.is_empty() {
                let event_enum_name = to_pascal_case(&format!("{}_event", wo.name));
                output.push_str(&format!("class {}(str, Enum):\n", event_enum_name));
                for event in &wo.events {
                    output.push_str(&format!("    {} = \"{}\"\n", event.to_uppercase(), event));
                }
                output.push_str("\n\n");
            }

            // Generate class
            let class_name = to_pascal_case(&wo.name);
            output.push_str(&format!("class {}(BaseModel):\n\n", class_name));

            if wo.attributes.is_empty() {
                output.push_str("    pass\n");
            } else {
                for field in &wo.attributes {
                    let field_name = to_snake_case(&field.name);
                    let field_type = field.field_type.to_python_type(field.nullable);

                    if field.nullable {
                        output.push_str(&format!("    {}: {} = None\n", field_name, field_type));
                    } else if let Some(default) = &field.default_value {
                        if default == "true" {
                            output.push_str(&format!("    {}: {} = True\n", field_name, field_type));
                        } else if default == "false" {
                            output.push_str(&format!("    {}: {} = False\n", field_name, field_type));
                        } else if let Ok(_) = default.parse::<i64>() {
                            output.push_str(&format!("    {}: {} = {}\n", field_name, field_type, default));
                        } else {
                            output.push_str(&format!("    {}: {} = \"{}\"\n", field_name, field_type, default));
                        }
                    } else {
                        output.push_str(&format!("    {}: {}\n", field_name, field_type));
                    }
                }
            }

            output.push_str("\n\n");
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

    output.push_str(&format!("class {}(str, Enum):\n", enum_name));

    for variant in variants {
        let variant_upper = variant.to_uppercase();
        output.push_str(&format!("    {} = \"{}\"\n", variant_upper, variant));
    }

    output
}

fn generate_class(model: &StructModel) -> String {
    let class_name = to_pascal_case(&model.name);
    let mut output = String::new();

    output.push_str(&format!("class {}(BaseModel):\n\n", class_name));

    for field in &model.fields {
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

        if !validators.is_empty() || field.default_value.is_some() {
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

            output.push_str(&format!(
                "    {}: {} = Field({})\n",
                field_name,
                field_type,
                field_args.join(", ")
            ));
        } else {
            if field.nullable {
                output.push_str(&format!("    {}: {} = None\n", field_name, field_type));
            } else {
                output.push_str(&format!("    {}: {}\n", field_name, field_type));
            }
        }
    }

    // Add validators for special constraints
    let email_fields: Vec<&Field> = model
        .fields
        .iter()
        .filter(|f| f.constraints.iter().any(|c| c.kind == ConstraintKind::Email))
        .collect();

    let url_fields: Vec<&Field> = model
        .fields
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
