// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TableCellEvent {
    Edit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TableCell {
    pub alignment: String,
    pub column_order: i64,
    pub content: String,
    #[serde(default)]
    pub is_bold: bool,
    #[serde(default)]
    pub is_muted: bool,
    pub row_id: String,
}

