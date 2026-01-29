// Harana Actions - Address Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};

// Address struct
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

// AddressValidationError struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressValidationError {
    pub field: String,
    pub code: String,
    pub message: String,
}

// AddressSuggestion struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressSuggestion {
    pub description: String,
    pub place_id: String,
    pub address: Address,
}

// autocomplete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteOutput {
    pub suggestions: Vec<AddressSuggestion>,
}

// normalize
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizeOutput {
    pub address: Address,
}

// parse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseOutput {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

// validate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateOutput {
    pub valid: bool,
    pub errors: Vec<AddressValidationError>,
}
