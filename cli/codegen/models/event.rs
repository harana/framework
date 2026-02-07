use serde::{Deserialize, Serialize};

use crate::models::field::Field;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventModifier {
    Server,
    Push,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub event: String,
    pub modifier: Option<EventModifier>,
    pub schema: Vec<Field>,
}

pub type EventFile = Vec<Event>;
