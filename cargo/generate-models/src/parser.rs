use crate::models::*;
use anyhow::{Context, Result, bail};
use serde_yaml::Value;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn parse_schema_directory(dir: &Path) -> Result<Vec<SchemaFile>> {
    let mut schema_files = Vec::new();

    let canonical_dir = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let ext = e.path().extension().and_then(|s| s.to_str());
            ext == Some("fml") || ext == Some("yml") || ext == Some("yaml")
        })
    {
        let file_path = entry.path();
        let file_name = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Compute relative directory from the schema root
        let canonical_file = file_path.canonicalize().unwrap_or_else(|_| file_path.to_path_buf());
        let relative_dir = canonical_file
            .parent()
            .and_then(|p| p.strip_prefix(&canonical_dir).ok())
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        // Determine category from the first directory component
        let category = match SchemaCategory::from_dir(&relative_dir) {
            Some(c) => c,
            None => {
                eprintln!("  ⚠️  Skipping {} (not in a known category directory)", file_name);
                continue;
            }
        };

        // Compute the relative dir within the category (strip the category prefix)
        let category_relative_dir = {
            let cat_dir = category.dir_name();
            let stripped = relative_dir
                .strip_prefix(cat_dir)
                .unwrap_or(&relative_dir)
                .trim_start_matches('/');
            stripped.to_string()
        };

        let display_path = if relative_dir.is_empty() {
            file_name.clone()
        } else {
            format!("{}/{}", relative_dir, file_name)
        };
        println!("  📄 Parsing {} (category: {:?})", display_path, category);

        let content =
            fs::read_to_string(file_path).with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        let models = match parse_schema_file(&content, &file_name, category) {
            Ok(models) => models,
            Err(e) => {
                eprintln!("  ⚠️  Skipping {} (parse error: {})", display_path, e);
                continue;
            }
        };

        if !models.is_empty() {
            schema_files.push(SchemaFile {
                file_name,
                relative_dir,
                category_relative_dir,
                category,
                models,
            });
        }
    }

    Ok(schema_files)
}

fn parse_schema_file(content: &str, file_name: &str, category: SchemaCategory) -> Result<Vec<Model>> {
    let yaml: Value =
        serde_yaml::from_str(content).with_context(|| format!("Failed to parse YAML in {}", file_name))?;

    let mut models = Vec::new();

    let entries = match &yaml {
        Value::Sequence(seq) => seq.clone(),
        Value::Mapping(_) => vec![yaml.clone()],
        _ => return Ok(models),
    };

    for entry in &entries {
        match category {
            SchemaCategory::Flow => {
                if let Some(parsed) = parse_flow_entry(entry)? {
                    models.push(parsed);
                }
            }
            SchemaCategory::Config => {
                if let Some(parsed) = parse_config_entry(entry)? {
                    models.push(Model::Struct(parsed));
                }
            }
            SchemaCategory::Event => {
                if let Some(parsed) = parse_event_entry(entry)? {
                    models.push(Model::Struct(parsed));
                }
            }
            SchemaCategory::WebObject => {
                if let Some(parsed) = parse_webobject_entry(entry)? {
                    models.push(Model::WebObject(parsed));
                }
            }
        }
    }

    Ok(models)
}

// ---------------------------------------------------------------------------
// Flow parsing (action methods + object classes)
// ---------------------------------------------------------------------------

fn parse_flow_entry(yaml: &Value) -> Result<Option<Model>> {
    let obj = match yaml.as_mapping() {
        Some(m) => m,
        None => return Ok(None),
    };

    // Check if this is an action method (has "action" key)
    if let Some(action_val) = obj.get(&Value::String("action".to_string())) {
        let name = action_val
            .as_str()
            .context("Flow 'action' field must be a string")?
            .to_string();

        let inputs = if let Some(inputs_val) = obj.get(&Value::String("inputs".to_string())) {
            parse_action_fields_from_mapping(inputs_val)?
        } else {
            Vec::new()
        };

        let outputs = if let Some(outputs_val) = obj.get(&Value::String("outputs".to_string())) {
            parse_action_fields_from_mapping(outputs_val)?
        } else {
            Vec::new()
        };

        return Ok(Some(Model::ActionMethod(ActionMethod { name, inputs, outputs })));
    }

    // Check if this is an object class definition (has "object" key with "class" or "schema")
    if let Some(object_val) = obj.get(&Value::String("object".to_string())) {
        let name = object_val
            .as_str()
            .context("Flow 'object' field must be a string")?
            .to_string();

        let fields_val = obj
            .get(&Value::String("class".to_string()))
            .or_else(|| obj.get(&Value::String("schema".to_string())));

        if let Some(fv) = fields_val {
            let fields = parse_action_fields_from_mapping(fv)?;
            return Ok(Some(Model::ActionClass(ActionClass { name, fields })));
        }
    }

    Ok(None)
}

fn parse_action_fields_from_mapping(yaml: &Value) -> Result<Vec<ActionField>> {
    let mut fields = Vec::new();

    match yaml {
        Value::Mapping(map) => {
            for (key, value) in map {
                let name = key.as_str().context("Field name must be a string")?.to_string();
                let def = value.as_str().unwrap_or("string");
                let (field_type, default_value) = parse_action_field_definition(def)?;
                fields.push(ActionField {
                    name,
                    field_type,
                    default_value,
                });
            }
        }
        Value::Sequence(seq) => {
            for item in seq {
                if let Some(map) = item.as_mapping() {
                    if map.len() == 1 {
                        let (key, value) = map.iter().next().unwrap();
                        let name = key.as_str().context("Field name must be a string")?.to_string();
                        let def = value.as_str().unwrap_or("string");
                        let (field_type, default_value) = parse_action_field_definition(def)?;
                        fields.push(ActionField {
                            name,
                            field_type,
                            default_value,
                        });
                    }
                }
            }
        }
        _ => {}
    }

    Ok(fields)
}

fn parse_action_field_definition(def: &str) -> Result<(FieldType, Option<String>)> {
    let parts = def.split_whitespace().collect::<Vec<_>>();
    if parts.is_empty() {
        bail!("Empty field definition");
    }

    let type_part = parts[0];
    let type_str = type_part.trim_end_matches('?');

    let field_type = if type_str.contains('|') {
        let variants = type_str.split('|').map(|s| s.trim().to_string()).collect();
        FieldType::Enum(variants)
    } else if type_str.starts_with("list[") && type_str.ends_with(']') {
        let inner_str = &type_str[5..type_str.len() - 1];
        let inner_type = parse_simple_type(inner_str);
        FieldType::List(Box::new(inner_type))
    } else if type_str.starts_with("map[") {
        FieldType::Map
    } else {
        parse_simple_type(type_str)
    };

    // Parse default value
    let mut default_value = None;
    let mut i = 1;
    while i < parts.len() {
        if parts[i] == "=" && i + 1 < parts.len() {
            default_value = Some(parts[i + 1].to_string());
            i += 2;
        } else {
            i += 1;
        }
    }

    Ok((field_type, default_value))
}

// ---------------------------------------------------------------------------
// Config parsing (uses "config:" key)
// ---------------------------------------------------------------------------

fn parse_config_entry(yaml: &Value) -> Result<Option<StructModel>> {
    let obj = match yaml.as_mapping() {
        Some(m) => m,
        None => return Ok(None),
    };

    let name = obj
        .get(&Value::String("config".to_string()))
        .and_then(|v| v.as_str())
        .context("Config entry missing 'config' field")?
        .to_string();

    let id = if let Some(id_field) = obj.get(&Value::String("id".to_string())) {
        id_field
            .as_sequence()
            .context("Config 'id' field must be an array")?
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect()
    } else {
        vec!["id".to_string()]
    };

    let fields = parse_schema_fields(obj)?;

    Ok(Some(StructModel { name, id, fields }))
}

// ---------------------------------------------------------------------------
// Event parsing (uses "event:" key)
// ---------------------------------------------------------------------------

fn parse_event_entry(yaml: &Value) -> Result<Option<StructModel>> {
    let obj = match yaml.as_mapping() {
        Some(m) => m,
        None => return Ok(None),
    };

    let name = obj
        .get(&Value::String("event".to_string()))
        .and_then(|v| v.as_str())
        .context("Event entry missing 'event' field")?
        .to_string();

    let id = if let Some(id_field) = obj.get(&Value::String("id".to_string())) {
        id_field
            .as_sequence()
            .context("Event 'id' field must be an array")?
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect()
    } else {
        vec!["id".to_string()]
    };

    let fields = parse_schema_fields(obj)?;

    Ok(Some(StructModel { name, id, fields }))
}

/// Shared helper to parse the "schema" block from a config/event entry.
fn parse_schema_fields(obj: &serde_yaml::Mapping) -> Result<Vec<Field>> {
    let schema_val = obj.get(&Value::String("schema".to_string()));

    let mut fields = Vec::new();
    if let Some(schema_val) = schema_val {
        match schema_val {
            Value::Sequence(seq) => {
                // Array of single-key mappings: [{field_name: "type ..."}]
                for field_yaml in seq {
                    if let Some(field) = parse_field(field_yaml)? {
                        fields.push(field);
                    }
                }
            }
            Value::Mapping(map) => {
                // Direct mapping: {field_name: "type ..."}
                for (key, value) in map {
                    let name = key.as_str().context("Field name must be a string")?.to_string();
                    let definition = match value.as_str() {
                        Some(s) => s.to_string(),
                        None => {
                            fields.push(Field {
                                name,
                                field_type: FieldType::String,
                                nullable: true,
                                default_value: None,
                                constraints: Vec::new(),
                                reference: None,
                            });
                            continue;
                        }
                    };
                    let (field_type, nullable, default_value, constraints, reference) =
                        parse_field_definition(&definition)?;
                    fields.push(Field {
                        name,
                        field_type,
                        nullable,
                        default_value,
                        constraints,
                        reference,
                    });
                }
            }
            _ => {}
        }
    }

    Ok(fields)
}

// ---------------------------------------------------------------------------
// WebObject parsing
// ---------------------------------------------------------------------------

fn parse_webobject_entry(yaml: &Value) -> Result<Option<WebObjectModel>> {
    let obj = match yaml.as_mapping() {
        Some(m) => m,
        None => return Ok(None),
    };

    let name = obj
        .get(&Value::String("webobject".to_string()))
        .and_then(|v| v.as_str())
        .context("WebObject missing 'webobject' field")?
        .to_string();

    let mut attributes = Vec::new();
    if let Some(attrs_val) = obj.get(&Value::String("attributes".to_string())) {
        if let Some(attrs_seq) = attrs_val.as_sequence() {
            for field_yaml in attrs_seq {
                if let Some(field) = parse_field(field_yaml)? {
                    attributes.push(field);
                }
            }
        }
    }

    let mut events = Vec::new();
    if let Some(events_val) = obj.get(&Value::String("events".to_string())) {
        if let Some(events_seq) = events_val.as_sequence() {
            for event_yaml in events_seq {
                if let Some(event_name) = event_yaml.as_str() {
                    events.push(event_name.to_string());
                }
            }
        }
    }

    Ok(Some(WebObjectModel {
        name,
        attributes,
        events,
    }))
}

// ---------------------------------------------------------------------------
// Shared field parsing
// ---------------------------------------------------------------------------

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

    // Field definition can be a string or other type; skip non-string definitions gracefully
    let definition = match field_def.as_str() {
        Some(s) => s.to_string(),
        None => {
            // Handle non-string definitions (e.g., nested objects) as opaque string types
            return Ok(Some(Field {
                name,
                field_type: FieldType::String,
                nullable: true,
                default_value: None,
                constraints: Vec::new(),
                reference: None,
            }));
        }
    };

    // Parse the field definition
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
        // Collect all enum variants from type_part and subsequent parts until we hit = or #
        let mut full_enum_str = type_str.to_string();
        let mut scan_idx = 1;
        while scan_idx < parts.len() {
            let p = parts[scan_idx];
            if p == "=" || p.starts_with('#') || p == "->" {
                break;
            }
            // Strip trailing ? from last variant
            full_enum_str.push_str(p);
            scan_idx += 1;
        }
        let variants = full_enum_str
            .split('|')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        FieldType::Enum(variants)
    } else if type_str.starts_with("list[") && type_str.ends_with(']') {
        let inner_str = &type_str[5..type_str.len() - 1];
        let inner_type = parse_simple_type(inner_str);
        FieldType::List(Box::new(inner_type))
    } else if type_str.starts_with("map[") || type_str == "map" {
        FieldType::Map
    } else {
        parse_simple_type(type_str)
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

fn parse_simple_type(type_str: &str) -> FieldType {
    match type_str {
        "string" => FieldType::String,
        "int" | "integer" => FieldType::Int,
        "decimal" | "float" => FieldType::Decimal,
        "bool" | "boolean" => FieldType::Bool,
        "datetime" | "date" => FieldType::Datetime,
        "id" => FieldType::Id,
        "map" => FieldType::Map,
        "bytes" => FieldType::String,
        "any" => FieldType::String,
        _ => FieldType::String, // default: treat unknown types as String
    }
}
