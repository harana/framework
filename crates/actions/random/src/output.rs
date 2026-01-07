// Harana Actions - Random Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// bytes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytesOutput {
    pub bytes: Vec<u8>
}

// choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceOutput {
    pub item: String
}

// number
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberOutput {
    pub number: f64
}

// shuffle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShuffleOutput {
    pub items: Vec<Value>
}

// string
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringOutput {
    pub string: String
}

// uuid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UuidOutput {
    pub uuid: String
}
