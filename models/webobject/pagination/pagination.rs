// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaginationEvent {
    FirstPageReach,
    LastPageReach,
    NextClick,
    PageChange,
    PageNumberClick,
    PreviousClick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Pagination {
    pub aria_label: String,
    pub next_href: Option<String>,
    pub next_label: String,
    pub previous_href: Option<String>,
    pub previous_label: String,
}

