// Harana Actions - Json Module
// This module provides json actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use output::*;
use serde::Serialize;
use serde_json::{Map, Value};

/// Compare Two JSON Objects
/// 
/// Compares two JSON objects and returns the differences.
pub async fn diff(
    target: &str,
    source: &str,
    include_unchanged: Option<bool>,
) -> Result<DiffOutput, String> {
    let source_val: Value = serde_json::from_str(source)
        .map_err(|e| format!("Invalid source JSON: {}", e))?;
    let target_val: Value = serde_json::from_str(target)
        .map_err(|e| format!("Invalid target JSON: {}", e))?;
    
    let include_unchanged = include_unchanged.unwrap_or(false);
    
    let mut added = HashMap::new();
    let mut removed = HashMap::new();
    let mut changed = HashMap::new();
    let mut unchanged = HashMap::new();
    
    fn compare_values(
        path: &str,
        source: &Value,
        target: &Value,
        added: &mut HashMap<String, Value>,
        removed: &mut HashMap<String, Value>,
        changed: &mut HashMap<String, Value>,
        unchanged: &mut HashMap<String, Value>,
    ) {
        match (source, target) {
            (Value::Object(s), Value::Object(t)) => {
                for (key, s_val) in s {
                    let new_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    
                    if let Some(t_val) = t.get(key) {
                        compare_values(&new_path, s_val, t_val, added, removed, changed, unchanged);
                    } else {
                        removed.insert(new_path, s_val.clone());
                    }
                }
                
                for (key, t_val) in t {
                    let new_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    
                    if !s.contains_key(key) {
                        added.insert(new_path, t_val.clone());
                    }
                }
            }
            _ => {
                if source != target {
                    changed.insert(path.to_string(), serde_json::json!({
                        "from": source,
                        "to": target
                    }));
                } else {
                    unchanged.insert(path.to_string(), source.clone());
                }
            }
        }
    }
    
    compare_values("", &source_val, &target_val, &mut added, &mut removed, &mut changed, &mut unchanged);
    
    Ok(DiffOutput {
        added,
        removed,
        changed,
        unchanged: if include_unchanged { Some(unchanged) } else { None },
    })
}

/// Query JSON With JSONPath
/// 
/// Queries JSON data using JSONPath syntax.
pub async fn jmespath_query(
    query: &str,
    data: &str,
) -> Result<JmespathQueryOutput, String> {
    let json: Value = serde_json::from_str(data)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    
    let result = jsonpath_rust::JsonPath::try_from(query)
        .map_err(|e| format!("Invalid JSONPath: {}", e))?
        .find(&json);
    
    Ok(JmespathQueryOutput { result })
}

/// Deep Merge JSON Objects
/// 
/// Merges multiple JSON objects into one.
pub async fn merge(
    objects: Vec<HashMap<String, Value>>,
    strategy: Option<&str>,
) -> Result<MergeOutput, String> {
    let strategy = strategy.unwrap_or("deep");
    
    fn deep_merge(base: &mut Value, other: Value) {
        match (base, other) {
            (Value::Object(base_map), Value::Object(other_map)) => {
                for (key, value) in other_map {
                    if let Some(base_value) = base_map.get_mut(&key) {
                        deep_merge(base_value, value);
                    } else {
                        base_map.insert(key, value);
                    }
                }
            }
            (base, other) => *base = other,
        }
    }
    
    let mut result: Map<String, Value> = Map::new();
    
    for obj in objects {
        let obj_value = Value::Object(obj.into_iter().collect());
        
        match strategy {
            "deep" => {
                let mut base = Value::Object(result.clone());
                deep_merge(&mut base, obj_value);
                if let Value::Object(merged) = base {
                    result = merged;
                }
            }
            "shallow" | _ => {
                if let Value::Object(map) = obj_value {
                    for (k, v) in map {
                        result.insert(k, v);
                    }
                }
            }
        }
    }
    
    Ok(MergeOutput {
        result: result.into_iter().collect(),
    })
}

/// Parse JSON String To Object
/// 
/// Parses a JSON string into a Value.
pub async fn parse(
    data: &str,
    _strict: Option<bool>,
) -> Result<ParseOutput, String> {
    let result: Value = serde_json::from_str(data)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    
    Ok(ParseOutput { result })
}

/// Convert Object To JSON String
/// 
/// Converts a JSON value to a formatted string.
pub async fn stringify(
    data: &str,
    indent: Option<i32>,
    sort_keys: Option<bool>,
) -> Result<StringifyOutput, String> {
    let value: Value = serde_json::from_str(data)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    
    let value = if sort_keys.unwrap_or(false) {
        sort_json_keys(value)
    } else {
        value
    };
    
    let json = match indent {
        Some(n) if n > 0 => {
            let indent_bytes = " ".repeat(n as usize).into_bytes();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent_bytes);
            let mut buf = Vec::new();
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            value.serialize(&mut ser)
                .map_err(|e| format!("Serialization error: {}", e))?;
            String::from_utf8(buf)
                .map_err(|e| format!("UTF-8 error: {}", e))?
        }
        _ => serde_json::to_string(&value)
            .map_err(|e| format!("Serialization error: {}", e))?,
    };
    
    Ok(StringifyOutput { json })
}

fn sort_json_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted: indexmap::IndexMap<String, Value> = indexmap::IndexMap::new();
            let mut keys: Vec<_> = map.keys().cloned().collect();
            keys.sort();
            for key in keys {
                if let Some(v) = map.get(&key) {
                    sorted.insert(key, sort_json_keys(v.clone()));
                }
            }
            Value::Object(sorted.into_iter().collect())
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(sort_json_keys).collect()),
        v => v,
    }
}

/// Validate JSON Against Schema
/// 
/// Validates JSON data against a JSON Schema.
pub async fn validate(
    data: &str,
    schema: HashMap<String, Value>,
) -> Result<ValidateOutput, String> {
    let data_val: Value = serde_json::from_str(data)
        .map_err(|e| format!("Invalid JSON data: {}", e))?;
    
    let mut errors = Vec::new();
    
    // Basic validation - check required fields if specified
    if let Some(Value::Array(required)) = schema.get("required") {
        if let Value::Object(obj) = &data_val {
            for req in required {
                if let Value::String(field) = req {
                    if !obj.contains_key(field) {
                        let mut error = HashMap::new();
                        error.insert("field".to_string(), Value::String(field.clone()));
                        error.insert("error".to_string(), Value::String("required field missing".to_string()));
                        errors.push(error);
                    }
                }
            }
        }
    }
    
    // Check type if specified
    if let Some(Value::String(expected_type)) = schema.get("type") {
        let actual_type = match &data_val {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(n) if n.is_i64() || n.is_u64() => "integer",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        };
        
        let type_matches = match expected_type.as_str() {
            "number" => actual_type == "number" || actual_type == "integer",
            t => t == actual_type,
        };
        
        if !type_matches {
            let mut error = HashMap::new();
            error.insert("field".to_string(), Value::String("root".to_string()));
            error.insert("error".to_string(), Value::String(format!(
                "expected type '{}', got '{}'", expected_type, actual_type
            )));
            errors.push(error);
        }
    }
    
    Ok(ValidateOutput {
        valid: errors.is_empty(),
        errors,
    })
}

#[cfg(test)]
mod tests;
