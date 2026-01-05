use serde::{Deserialize, Serialize};

use crate::models::data::Field;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Store {
    pub name: String,
    pub class: String,
    #[serde(default)]
    pub location: Option<String>,
    pub schema: Vec<Field>,
}

pub type StoreFile = Vec<Store>;
