// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BentoGridEvent {
    ItemClick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BentoGrid {
    pub background: String,
    pub description: Option<String>,
    pub layout: String,
    pub subtitle: Option<String>,
    pub title: Option<String>,
}

