use serde::{Deserialize, Serialize};

/// The top-level schema category, determined by the first directory under `schema/`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SchemaCategory {
    Action,
    Config,
    Event,
    Object,
    WebObject,
}

impl SchemaCategory {
    pub fn from_dir(dir: &str) -> Option<Self> {
        let first = dir.split('/').next().unwrap_or("");
        match first {
            "action" => Some(Self::Action),
            "config" => Some(Self::Config),
            "event" => Some(Self::Event),
            "object" => Some(Self::Object),
            "webobject" => Some(Self::WebObject),
            _ => None,
        }
    }

    pub fn dir_name(&self) -> &'static str {
        match self {
            Self::Action => "action",
            Self::Config => "config",
            Self::Event => "event",
            Self::Object => "object",
            Self::WebObject => "webobject",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaFile {
    pub file_name: String,
    /// Relative dir from the schema root (e.g. "action/framework")
    pub relative_dir: String,
    /// Relative dir *within* the category (e.g. "framework" when category is Action)
    pub category_relative_dir: String,
    pub category: SchemaCategory,
    pub models: Vec<Model>,
}

/// A parsed model entry. The variant depends on the schema category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Model {
    /// An action method with inputs/outputs (generates a trait method)
    ActionMethod(ActionMethod),
    /// A class definition found in action schemas (generates a struct)
    ActionClass(ActionClass),
    /// A struct model from config/event/object schemas
    Struct(StructModel),
    /// A web object with attributes and events
    WebObject(WebObjectModel),
}

impl Model {
    pub fn name(&self) -> &str {
        match self {
            Model::ActionMethod(m) => &m.name,
            Model::ActionClass(m) => &m.name,
            Model::Struct(m) => &m.name,
            Model::WebObject(m) => &m.name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionMethod {
    pub name: String,
    pub inputs: Vec<ActionField>,
    pub outputs: Vec<ActionField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionField {
    pub name: String,
    pub field_type: FieldType,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionClass {
    pub name: String,
    pub fields: Vec<ActionField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructModel {
    pub name: String,
    pub id: Vec<String>,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebObjectModel {
    pub name: String,
    pub attributes: Vec<Field>,
    pub events: Vec<String>,
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
    List(Box<FieldType>),
    Map,
    Enum(Vec<String>),
}

impl FieldType {
    pub fn to_rust_type(&self, nullable: bool) -> String {
        let base_type = match self {
            FieldType::String => "String".to_string(),
            FieldType::Int => "i64".to_string(),
            FieldType::Decimal => "f64".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Datetime => "chrono::DateTime<chrono::Utc>".to_string(),
            FieldType::Id => "String".to_string(),
            FieldType::List(inner) => format!("Vec<{}>", inner.to_rust_type(false)),
            FieldType::Map => "std::collections::HashMap<String, String>".to_string(),
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
            base_type
        }
    }

    pub fn to_python_type(&self, nullable: bool) -> String {
        let base_type = match self {
            FieldType::String => "str".to_string(),
            FieldType::Int => "int".to_string(),
            FieldType::Decimal => "float".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::Datetime => "datetime".to_string(),
            FieldType::Id => "str".to_string(),
            FieldType::List(inner) => format!("List[{}]", inner.to_python_type(false)),
            FieldType::Map => "dict".to_string(),
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
            base_type
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
