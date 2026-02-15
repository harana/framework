// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SidebarSection {
    pub order: i64,
    pub parent_type: String,
    pub sidebar_id: String,
}

