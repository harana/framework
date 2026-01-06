use crate::parser::Action;

pub struct RustGenerator;

impl RustGenerator {
    /// Map YAML types to Rust types
    fn map_type(yaml_type: &str, required: bool) -> String {
        let base_type = if yaml_type.starts_with("list[") {
            // Extract inner type from list[type]
            let inner = &yaml_type[5..yaml_type.len() - 1];
            match inner {
                "object" => "Vec<HashMap<String, Value>>",
                "string" => "Vec<String>",
                "integer" => "Vec<i32>",
                "float" => "Vec<f64>",
                "boolean" => "Vec<bool>",
                _ => "Vec<Value>",
            }
        } else {
            match yaml_type {
                "string" => "&str",
                "integer" => "i32",
                "float" => "f64",
                "boolean" => "bool",
                "datetime" => "&str",
                "bytes" => "&[u8]",
                "object" => "HashMap<String, Value>",
                _ => "&str",
            }
        };

        if required {
            base_type.to_string()
        } else {
            format!("Option<{}>", base_type)
        }
    }

    /// Map YAML types to Rust output struct field types
    fn map_output_type(yaml_type: &str) -> String {
        if yaml_type.starts_with("list[") {
            // Extract inner type from list[type]
            let inner = &yaml_type[5..yaml_type.len() - 1];
            match inner {
                "object" => "Vec<HashMap<String, Value>>".to_string(),
                "string" => "Vec<String>".to_string(),
                "integer" => "Vec<i32>".to_string(),
                "float" => "Vec<f64>".to_string(),
                "boolean" => "Vec<bool>".to_string(),
                _ => "Vec<Value>".to_string(),
            }
        } else {
            match yaml_type {
                "string" => "String",
                "integer" => "i32",
                "float" => "f64",
                "boolean" => "bool",
                "datetime" => "String",
                "bytes" => "Vec<u8>",
                "object" => "HashMap<String, Value>",
                _ => "String",
            }
            .to_string()
        }
    }

    /// Convert snake_case to PascalCase
    pub fn to_pascal_case(s: &str) -> String {
        s.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect()
    }

    /// Generate function signature
    pub fn generate_function(action: &Action) -> String {
        let mut params = Vec::new();

        // Sort inputs: required first, then optional
        let mut sorted_inputs: Vec<_> = action.inputs.iter().collect();
        sorted_inputs.sort_by_key(|(_, field)| !field.required);

        for (name, field) in sorted_inputs {
            let rust_type = Self::map_type(&field.field_type, field.required);
            params.push(format!("    {}: {}", name, rust_type));
        }

        let output_struct = Self::to_pascal_case(&action.class) + "Output";
        let params_str = if params.is_empty() {
            String::new()
        } else {
            format!("\n{},\n", params.join(",\n"))
        };

        format!(
            r#"/// {}
pub async fn {}({}) -> Result<{}, String> {{
    unimplemented!("{}")
}}"#,
            action.name, action.class, params_str, output_struct, action.class
        )
    }

    /// Generate output struct
    pub fn generate_output_struct(action: &Action) -> String {
        let struct_name = Self::to_pascal_case(&action.class) + "Output";
        let mut fields = Vec::new();

        for (name, field) in &action.outputs {
            let rust_type = Self::map_output_type(&field.field_type);
            fields.push(format!("    pub {}: {}", name, rust_type));
        }

        let fields_str = fields.join(",\n");

        format!(
            r#"// {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {} {{
{}
}}"#,
            action.class, struct_name, fields_str
        )
    }

    /// Check if we need to import serde_json::Value
    pub fn needs_value_import(actions: &[Action]) -> bool {
        for action in actions {
            for field in action.inputs.values() {
                if field.field_type.contains("object") {
                    return true;
                }
            }
            for field in action.outputs.values() {
                if field.field_type.contains("object") {
                    return true;
                }
            }
        }
        false
    }

    /// Check if we need to import HashMap
    pub fn needs_hashmap_import(actions: &[Action]) -> bool {
        for action in actions {
            for field in action.inputs.values() {
                if field.field_type.contains("object") {
                    return true;
                }
            }
            for field in action.outputs.values() {
                if field.field_type.contains("object") {
                    return true;
                }
            }
        }
        false
    }
}
