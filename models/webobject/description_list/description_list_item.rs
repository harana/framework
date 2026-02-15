// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DescriptionListItemEvent {
    Click,
    Collapse,
    Expand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DescriptionListItem {
    pub details: String,
    pub list_id: String,
    pub order: i64,
    pub term: String,
}

