// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BentoGridItemEvent {
    Click,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BentoGridItem {
    pub bento_grid_id: String,
    pub col_span: String,
    pub description: Option<String>,
    pub href: Option<String>,
    pub image_dark: Option<String>,
    pub image_light: Option<String>,
    pub order: i64,
    pub row_span: String,
    pub subtitle: Option<String>,
    pub title: Option<String>,
}

