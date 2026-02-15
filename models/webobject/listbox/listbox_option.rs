// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListboxOption {
    pub avatar_initials: Option<String>,
    pub avatar_src: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub label: String,
    pub listbox_id: String,
    pub order: i64,
    pub value: String,
}

