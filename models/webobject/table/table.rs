// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TableEvent {
    BulkSelectionToggle,
    PageChange,
    PageSizeChange,
    SelectAllClick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Table {
    #[serde(default)]
    pub is_bleed: bool,
    #[serde(default)]
    pub is_dense: bool,
    #[serde(default)]
    pub is_grid: bool,
    #[serde(default)]
    pub is_striped: bool,
}

