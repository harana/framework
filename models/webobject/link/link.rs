// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkEvent {
    Click,
    Focus,
    Hover,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Link {
    pub href: String,
    pub label: String,
    pub target: String,
}

