// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DropdownSection {
    pub dropdown_id: String,
    pub heading: Option<String>,
    pub order: i64,
}

