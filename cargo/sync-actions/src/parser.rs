use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionSchema {
    pub module_name: String, // e.g., "aws_iam"
    pub file_path: PathBuf,
    pub actions: Vec<Action>,
    pub classes: Vec<ClassDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub action: String, // The function/method identifier (e.g., "generate_text")
    pub inputs: HashMap<String, Field>,
    pub outputs: HashMap<String, Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDefinition {
    pub name: String,
    pub fields: HashMap<String, Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub field_type: String, // e.g., "string", "integer", "list[object]"
    pub required: bool,
    pub default_value: Option<String>,
}

impl Action {
    /// Parse field definition from YAML value
    /// Formats: "string", "string = default", "string #required"
    fn parse_field(value: &serde_yaml::Value) -> Result<Field> {
        let value_str = value.as_str().context("Field value must be a string")?.trim();

        let mut field_type = value_str.to_string();
        let mut required = false;
        let mut default_value = None;

        // Check for #required marker (must check before processing default value)
        if field_type.contains("#required") {
            required = true;
            field_type = field_type.replace("#required", "").trim().to_string();
        }

        // Check for default value (format: "type = value")
        if field_type.contains('=') {
            let parts: Vec<&str> = field_type.split('=').collect();
            let new_type = parts[0].trim().to_string();
            default_value = Some(parts[1].trim().to_string());
            field_type = new_type;
        }

        Ok(Field {
            field_type,
            required,
            default_value,
        })
    }
}

pub fn parse_action_schemas(schema_dir: &Path, module_filter: Option<&str>) -> Result<Vec<ActionSchema>> {
    let mut schemas = Vec::new();

    for entry in WalkDir::new(schema_dir).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // Only process FML/YAML files
        if !path.is_file() {
            continue;
        }

        let extension = path.extension().and_then(|s| s.to_str());
        if extension != Some("fml") && extension != Some("yml") && extension != Some("yaml") {
            continue;
        }

        // Get module name from filename (e.g., "aws_iam.fml" -> "aws_iam")
        let module_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .context("Invalid filename")?
            .to_string();

        // Apply module filter if specified
        if let Some(filter) = module_filter {
            if module_name != filter {
                continue;
            }
        }

        println!("  📄 Parsing {}.fml", module_name);

        // Read and parse YAML file
        let content = fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;

        let items_yaml: Vec<serde_yaml::Value> =
            serde_yaml::from_str(&content).with_context(|| format!("Failed to parse YAML from {}", path.display()))?;

        let mut actions = Vec::new();
        let mut classes = Vec::new();

        for item_yaml in items_yaml {
            let mapping = item_yaml.as_mapping().context("Item must be a mapping")?;

            let name = mapping
                .get("name")
                .and_then(|v| v.as_str())
                .context("Item missing 'name' field")?
                .to_string();

            // Check if this is an action (has "action" field with string value)
            // or a class definition (has "class" field with mapping value)
            if let Some(action_value) = mapping.get("action") {
                // This is an action definition
                let action_name = action_value
                    .as_str()
                    .context("'action' field must be a string")?
                    .to_string();

                // Parse inputs
                let mut inputs = HashMap::new();
                if let Some(inputs_yaml) = mapping.get("inputs") {
                    if let Some(inputs_map) = inputs_yaml.as_mapping() {
                        for (key, value) in inputs_map {
                            let field_name = key.as_str().context("Input key must be string")?.to_string();
                            let field = Action::parse_field(value)?;
                            inputs.insert(field_name, field);
                        }
                    }
                }

                // Parse outputs
                let mut outputs = HashMap::new();
                if let Some(outputs_yaml) = mapping.get("outputs") {
                    if let Some(outputs_map) = outputs_yaml.as_mapping() {
                        for (key, value) in outputs_map {
                            let field_name = key.as_str().context("Output key must be string")?.to_string();
                            let field = Action::parse_field(value)?;
                            outputs.insert(field_name, field);
                        }
                    }
                }

                actions.push(Action {
                    name,
                    action: action_name,
                    inputs,
                    outputs,
                });
            } else if let Some(class_value) = mapping.get("class") {
                // This is a class/struct definition
                if let Some(class_map) = class_value.as_mapping() {
                    let mut fields = HashMap::new();
                    for (key, value) in class_map {
                        let field_name = key.as_str().context("Class field key must be string")?.to_string();
                        let field = Action::parse_field(value)?;
                        fields.insert(field_name, field);
                    }

                    classes.push(ClassDefinition { name, fields });
                }
            }
        }

        schemas.push(ActionSchema {
            module_name,
            file_path: path.to_path_buf(),
            actions,
            classes,
        });
    }

    Ok(schemas)
}
