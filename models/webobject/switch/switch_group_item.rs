// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SwitchGroupItem {
    pub group_id: String,
    pub order: i64,
    pub switch_id: String,
}

