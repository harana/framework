use crate::models::*;
use anyhow::{Context, Result, bail};
use serde_yaml::Value;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn parse_schema_directory(dir: &Path) -> Result<Vec<SchemaFile>> {
    let mut schema_files = Vec::new();

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("yml"))
    {
        let file_path = entry.path();
        let file_name = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        println!("  📄 Parsing {}", file_name);

        let content =
            fs::read_to_string(file_path).with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        let models = parse_schema_file(&content, &file_name)?;

        if !models.is_empty() {
            schema_files.push(SchemaFile { file_name, models });
        }
    }

    Ok(schema_files)
}

fn parse_schema_file(content: &str, file_name: &str) -> Result<Vec<Model>> {
    let yaml_models: Vec<Value> =
        serde_yaml::from_str(content).with_context(|| format!("Failed to parse YAML in {}", file_name))?;

    let mut models = Vec::new();

    for yaml_model in yaml_models {
        if let Some(model) = parse_model(&yaml_model)? {
            models.push(model);
        }
    }

    Ok(models)
}

fn parse_model(yaml: &Value) -> Result<Option<Model>> {
    let obj = match yaml.as_mapping() {
        Some(m) => m,
        None => return Ok(None),
    };

    let name = obj
        .get(&Value::String("name".to_string()))
        .and_then(|v| v.as_str())
        .context("Model missing 'name' field")?
        .to_string();

    let class = obj
        .get(&Value::String("class".to_string()))
        .and_then(|v| v.as_str())
        .context("Model missing 'class' field")?
        .to_string();

    // ID can be either a separate field or derived from schema fields named 'id'
    let id = if let Some(id_field) = obj.get(&Value::String("id".to_string())) {
        id_field
            .as_sequence()
            .context("Model 'id' field must be an array")?
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect()
    } else {
        // Try to find 'id' field in schema
        vec!["id".to_string()]
    };

    let schema = obj
        .get(&Value::String("schema".to_string()))
        .and_then(|v| v.as_sequence())
        .context("Model missing 'schema' field")?;

    let mut fields = Vec::new();
    for field_yaml in schema {
        if let Some(field) = parse_field(field_yaml)? {
            fields.push(field);
        }
    }

    Ok(Some(Model {
        name,
        class,
        id,
        schema: fields,
    }))
}

fn parse_field(yaml: &Value) -> Result<Option<Field>> {
    let obj = match yaml.as_mapping() {
        Some(m) => m,
        None => return Ok(None),
    };

    // Each field is a single key-value pair
    if obj.len() != 1 {
        return Ok(None);
    }

    let (field_name, field_def) = obj.iter().next().unwrap();
    let name = field_name.as_str().context("Field name must be a string")?.to_string();
    let definition = field_def
        .as_str()
        .context("Field definition must be a string")?
        .to_string();

    // Parse the field definition
    // Format: "type [= default] [#constraint1] [#constraint2] [-> Reference]"
    let (field_type, nullable, default_value, constraints, reference) = parse_field_definition(&definition)?;

    Ok(Some(Field {
        name,
        field_type,
        nullable,
        default_value,
        constraints,
        reference,
    }))
}

fn parse_field_definition(def: &str) -> Result<(FieldType, bool, Option<String>, Vec<Constraint>, Option<Reference>)> {
    let parts = def.split_whitespace().collect::<Vec<_>>();
    if parts.is_empty() {
        bail!("Empty field definition");
    }

    // Check for nullable (type ends with ?)
    let type_part = parts[0];
    let nullable = type_part.ends_with('?');
    let type_str = type_part.trim_end_matches('?');

    // Parse type (might be enum with |)
    let field_type = if type_str.contains('|') {
        let variants = type_str.split('|').map(|s| s.trim().to_string()).collect();
        FieldType::Enum(variants)
    } else {
        match type_str {
            "string" => FieldType::String,
            "int" => FieldType::Int,
            "decimal" => FieldType::Decimal,
            "bool" => FieldType::Bool,
            "datetime" => FieldType::Datetime,
            "id" => FieldType::Id,
            _ => FieldType::String, // default
        }
    };

    // Parse default value (after =)
    let mut default_value = None;
    let mut constraints = Vec::new();
    let mut reference = None;
    let mut i = 1;

    while i < parts.len() {
        if parts[i] == "=" && i + 1 < parts.len() {
            default_value = Some(parts[i + 1].to_string());
            i += 2;
        } else if parts[i].starts_with('#') {
            // Parse constraint
            let constraint_str = parts[i].trim_start_matches('#');

            if constraint_str == "required" {
                constraints.push(Constraint {
                    kind: ConstraintKind::Required,
                    value: None,
                });
            } else if constraint_str == "unique" {
                constraints.push(Constraint {
                    kind: ConstraintKind::Unique,
                    value: None,
                });
            } else if constraint_str == "email" {
                constraints.push(Constraint {
                    kind: ConstraintKind::Email,
                    value: None,
                });
            } else if constraint_str == "url" {
                constraints.push(Constraint {
                    kind: ConstraintKind::Url,
                    value: None,
                });
            } else if constraint_str == "lowercase" {
                constraints.push(Constraint {
                    kind: ConstraintKind::Lowercase,
                    value: None,
                });
            } else if constraint_str == "uppercase" {
                constraints.push(Constraint {
                    kind: ConstraintKind::Uppercase,
                    value: None,
                });
            } else if constraint_str.starts_with("min(") {
                let value = constraint_str.trim_start_matches("min(").trim_end_matches(')');
                constraints.push(Constraint {
                    kind: ConstraintKind::Min,
                    value: Some(value.to_string()),
                });
            } else if constraint_str.starts_with("max(") {
                let value = constraint_str.trim_start_matches("max(").trim_end_matches(')');
                constraints.push(Constraint {
                    kind: ConstraintKind::Max,
                    value: Some(value.to_string()),
                });
            }
            i += 1;
        } else if parts[i] == "->" && i + 1 < parts.len() {
            // Parse reference
            let ref_str = parts[i + 1];
            if let Some((model, field)) = ref_str.split_once('.') {
                reference = Some(Reference {
                    model: model.to_string(),
                    field: field.to_string(),
                });
            }
            i += 2;
        } else {
            i += 1;
        }
    }

    Ok((field_type, nullable, default_value, constraints, reference))
}
