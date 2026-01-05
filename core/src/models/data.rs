use serde::{Deserialize, Serialize};

use crate::models::field::Field;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub name: String,
    pub class: String,
    pub schema: Vec<Field>,
    #[serde(default)]
    pub unique: Option<Vec<String>>,
}

pub type DataFile = Vec<Data>;
