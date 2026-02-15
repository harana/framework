// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvatarButtonEvent {
    Click,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AvatarButton {
    pub alt: Option<String>,
    pub href: Option<String>,
    pub initials: Option<String>,
    #[serde(default)]
    pub is_square: bool,
    pub size: String,
    pub src: Option<String>,
}

