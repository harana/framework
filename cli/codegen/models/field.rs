use serde::{Deserialize, Serialize};

use crate::models::validator::Validator;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Bool,
    Date,
    Datetime,
    Decimal,
    Float,
    Int,
    String,
    Text,
    Time,
    Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub optional: bool,
    pub array: bool,
    pub default: Option<String>,
    pub enum_values: Option<Vec<String>>,
    pub validators: Vec<Validator>,
}
