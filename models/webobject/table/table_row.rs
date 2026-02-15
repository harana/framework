// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TableRowEvent {
    Click,
    Collapse,
    Deselect,
    Expand,
    Select,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TableRow {
    pub href: Option<String>,
    pub order: i64,
    pub table_id: String,
    pub target: String,
    pub title: Option<String>,
}

