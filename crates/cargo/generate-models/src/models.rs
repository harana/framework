use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaFile {
    pub file_name: String,
    pub models: Vec<Model>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub class: String,
    pub id: Vec<String>,
    pub schema: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub constraints: Vec<Constraint>,
    pub reference: Option<Reference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    String,
    Int,
    Decimal,
    Bool,
    Datetime,
    Id,
    Enum(Vec<String>),
}

impl FieldType {
    pub fn to_rust_type(&self, nullable: bool) -> String {
        let base_type = match self {
            FieldType::String => "String",
            FieldType::Int => "i64",
            FieldType::Decimal => "f64",
            FieldType::Bool => "bool",
            FieldType::Datetime => "chrono::DateTime<chrono::Utc>",
            FieldType::Id => "String",
            FieldType::Enum(variants) => {
                // Will be generated as a separate enum
                return if nullable {
                    format!("Option<{}>", Self::enum_name_from_variants(variants))
                } else {
                    Self::enum_name_from_variants(variants)
                };
            }
        };

        if nullable {
            format!("Option<{}>", base_type)
        } else {
            base_type.to_string()
        }
    }

    pub fn to_python_type(&self, nullable: bool) -> String {
        let base_type = match self {
            FieldType::String => "str",
            FieldType::Int => "int",
            FieldType::Decimal => "float",
            FieldType::Bool => "bool",
            FieldType::Datetime => "datetime",
            FieldType::Id => "str",
            FieldType::Enum(variants) => {
                return if nullable {
                    format!("Optional[{}]", Self::enum_name_from_variants(variants))
                } else {
                    Self::enum_name_from_variants(variants)
                };
            }
        };

        if nullable {
            format!("Optional[{}]", base_type)
        } else {
            base_type.to_string()
        }
    }

    fn enum_name_from_variants(variants: &[String]) -> String {
        // Generate a reasonable enum name from variants
        // For now, just capitalize first variant and add "Type"
        if let Some(first) = variants.first() {
            format!(
                "{}Type",
                first.chars().next().unwrap().to_uppercase().collect::<String>() + &first[1..]
            )
        } else {
            "CustomType".to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub kind: ConstraintKind,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConstraintKind {
    Required,
    Unique,
    Min,
    Max,
    MinLength,
    MaxLength,
    Email,
    Url,
    Lowercase,
    Uppercase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub model: String,
    pub field: String,
}
