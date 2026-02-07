pub mod output;

use output::*;
use regex::Regex;

/// US state abbreviations
const US_STATES: &[(&str, &str)] = &[
    ("AL", "Alabama"),
    ("AK", "Alaska"),
    ("AZ", "Arizona"),
    ("AR", "Arkansas"),
    ("CA", "California"),
    ("CO", "Colorado"),
    ("CT", "Connecticut"),
    ("DE", "Delaware"),
    ("FL", "Florida"),
    ("GA", "Georgia"),
    ("HI", "Hawaii"),
    ("ID", "Idaho"),
    ("IL", "Illinois"),
    ("IN", "Indiana"),
    ("IA", "Iowa"),
    ("KS", "Kansas"),
    ("KY", "Kentucky"),
    ("LA", "Louisiana"),
    ("ME", "Maine"),
    ("MD", "Maryland"),
    ("MA", "Massachusetts"),
    ("MI", "Michigan"),
    ("MN", "Minnesota"),
    ("MS", "Mississippi"),
    ("MO", "Missouri"),
    ("MT", "Montana"),
    ("NE", "Nebraska"),
    ("NV", "Nevada"),
    ("NH", "New Hampshire"),
    ("NJ", "New Jersey"),
    ("NM", "New Mexico"),
    ("NY", "New York"),
    ("NC", "North Carolina"),
    ("ND", "North Dakota"),
    ("OH", "Ohio"),
    ("OK", "Oklahoma"),
    ("OR", "Oregon"),
    ("PA", "Pennsylvania"),
    ("RI", "Rhode Island"),
    ("SC", "South Carolina"),
    ("SD", "South Dakota"),
    ("TN", "Tennessee"),
    ("TX", "Texas"),
    ("UT", "Utah"),
    ("VT", "Vermont"),
    ("VA", "Virginia"),
    ("WA", "Washington"),
    ("WV", "West Virginia"),
    ("WI", "Wisconsin"),
    ("WY", "Wyoming"),
    ("DC", "District of Columbia"),
];

/// Country postal code patterns
fn get_postal_code_pattern(country: &str) -> Option<&'static str> {
    match country.to_uppercase().as_str() {
        "US" | "USA" | "UNITED STATES" => Some(r"^\d{5}(-\d{4})?$"),
        "CA" | "CANADA" => Some(r"^[A-Za-z]\d[A-Za-z][ -]?\d[A-Za-z]\d$"),
        "UK" | "GB" | "UNITED KINGDOM" => Some(r"^[A-Za-z]{1,2}\d[A-Za-z\d]? ?\d[A-Za-z]{2}$"),
        "AU" | "AUSTRALIA" => Some(r"^\d{4}$"),
        "DE" | "GERMANY" => Some(r"^\d{5}$"),
        "FR" | "FRANCE" => Some(r"^\d{5}$"),
        "JP" | "JAPAN" => Some(r"^\d{3}-?\d{4}$"),
        _ => None,
    }
}

/// Normalize country name to standard code
fn normalize_country(country: &str) -> String {
    match country.to_uppercase().as_str() {
        "US" | "USA" | "UNITED STATES" | "UNITED STATES OF AMERICA" => "US".to_string(),
        "CA" | "CANADA" => "CA".to_string(),
        "UK" | "GB" | "UNITED KINGDOM" | "GREAT BRITAIN" => "GB".to_string(),
        "AU" | "AUSTRALIA" => "AU".to_string(),
        "DE" | "GERMANY" | "DEUTSCHLAND" => "DE".to_string(),
        "FR" | "FRANCE" => "FR".to_string(),
        "JP" | "JAPAN" => "JP".to_string(),
        _ => country.to_uppercase(),
    }
}

/// Normalize state name to abbreviation (US only)
fn normalize_state(state: &str) -> String {
    let state_upper = state.to_uppercase();

    // Check if already an abbreviation
    if US_STATES.iter().any(|(abbr, _)| *abbr == state_upper) {
        return state_upper;
    }

    // Find by full name
    for (abbr, name) in US_STATES {
        if name.to_uppercase() == state_upper {
            return abbr.to_string();
        }
    }

    state.to_string()
}

/// Get Address Autocomplete Suggestions.
pub async fn autocomplete(
    query: &str,
    country: Option<&str>,
    limit: Option<i32>,
    types: Option<Vec<String>>,
) -> Result<AutocompleteOutput, String> {
    let limit = limit.unwrap_or(5).min(10) as usize;
    let _types = types.unwrap_or_default();
    let country_filter = country.map(normalize_country);

    // In a real implementation, this would call an external API (Google Places, etc.)
    // For now, we provide a basic in-memory mock based on query matching
    let mut suggestions = Vec::new();

    let query_lower = query.to_lowercase();

    // Generate suggestions based on query (mock implementation)
    if query_lower.contains("123") || query_lower.contains("main") {
        let addr = Address {
            street: "123 Main Street".to_string(),
            city: "Springfield".to_string(),
            state: "IL".to_string(),
            postal_code: "62701".to_string(),
            country: country_filter.clone().unwrap_or_else(|| "US".to_string()),
        };
        suggestions.push(AddressSuggestion {
            description: format!("{}, {}, {} {}", addr.street, addr.city, addr.state, addr.postal_code),
            place_id: "place_001".to_string(),
            address: addr,
        });
    }

    if query_lower.contains("456") || query_lower.contains("oak") {
        let addr = Address {
            street: "456 Oak Avenue".to_string(),
            city: "Portland".to_string(),
            state: "OR".to_string(),
            postal_code: "97201".to_string(),
            country: country_filter.clone().unwrap_or_else(|| "US".to_string()),
        };
        suggestions.push(AddressSuggestion {
            description: format!("{}, {}, {} {}", addr.street, addr.city, addr.state, addr.postal_code),
            place_id: "place_002".to_string(),
            address: addr,
        });
    }

    if query_lower.contains("789") || query_lower.contains("elm") {
        let addr = Address {
            street: "789 Elm Boulevard".to_string(),
            city: "Austin".to_string(),
            state: "TX".to_string(),
            postal_code: "78701".to_string(),
            country: country_filter.clone().unwrap_or_else(|| "US".to_string()),
        };
        suggestions.push(AddressSuggestion {
            description: format!("{}, {}, {} {}", addr.street, addr.city, addr.state, addr.postal_code),
            place_id: "place_003".to_string(),
            address: addr,
        });
    }

    // If no specific matches, provide a generic suggestion based on query
    if suggestions.is_empty() && !query.trim().is_empty() {
        let addr = Address {
            street: query.to_string(),
            city: "Unknown".to_string(),
            state: "".to_string(),
            postal_code: "".to_string(),
            country: country_filter.unwrap_or_else(|| "US".to_string()),
        };
        suggestions.push(AddressSuggestion {
            description: query.to_string(),
            place_id: format!("place_{:08x}", query.len()),
            address: addr,
        });
    }

    suggestions.truncate(limit);

    Ok(AutocompleteOutput { suggestions })
}

/// Normalize Address Format.
pub async fn normalize(
    address: Address,
    country: Option<&str>,
    format: Option<&str>,
) -> Result<NormalizeOutput, String> {
    let _format = format.unwrap_or("standard");
    let country_hint = country.map(normalize_country);

    let mut normalized = Address {
        street: normalize_street(&address.street),
        city: title_case(address.city.trim()),
        state: if address.country.to_uppercase() == "US" || country_hint.as_ref().is_some_and(|c| c == "US") {
            normalize_state(&address.state)
        } else {
            address.state.trim().to_string()
        },
        postal_code: address.postal_code.trim().to_uppercase(),
        country: normalize_country(if !address.country.is_empty() {
            &address.country
        } else {
            country_hint.as_deref().unwrap_or("US")
        }),
    };

    // Apply format-specific transformations
    match _format {
        "uppercase" => {
            normalized.street = normalized.street.to_uppercase();
            normalized.city = normalized.city.to_uppercase();
            normalized.state = normalized.state.to_uppercase();
        }
        "lowercase" => {
            normalized.street = normalized.street.to_lowercase();
            normalized.city = normalized.city.to_lowercase();
            normalized.state = normalized.state.to_lowercase();
            normalized.country = normalized.country.to_lowercase();
        }
        _ => {} // "standard" format - no additional changes
    }

    Ok(NormalizeOutput { address: normalized })
}

/// Normalize street address
fn normalize_street(street: &str) -> String {
    let street = street.trim();

    // Common abbreviation expansions/normalizations
    // Using word boundaries to prevent partial matches
    let replacements: &[(&str, &str)] = &[
        // With periods (more specific, match first)
        (r"\bSt\.", "Street"),
        (r"\bAve\.", "Avenue"),
        (r"\bBlvd\.", "Boulevard"),
        (r"\bRd\.", "Road"),
        (r"\bDr\.", "Drive"),
        (r"\bLn\.", "Lane"),
        (r"\bCt\.", "Court"),
        (r"\bPl\.", "Place"),
        (r"\bCir\.", "Circle"),
        (r"\bApt\.", "Apartment"),
        (r"\bSte\.", "Suite"),
        // Without periods (at word boundary)
        (r"\bSt\b", "Street"),
        (r"\bAve\b", "Avenue"),
        (r"\bBlvd\b", "Boulevard"),
        (r"\bRd\b", "Road"),
        (r"\bDr\b", "Drive"),
        (r"\bLn\b", "Lane"),
        (r"\bCt\b", "Court"),
        (r"\bPl\b", "Place"),
        (r"\bCir\b", "Circle"),
        (r"\bApt\b", "Apartment"),
        (r"\bSte\b", "Suite"),
        // Special characters
        ("#", "Unit "),
    ];

    let mut result = street.to_string();
    for (pattern, replacement) in replacements {
        // Case-insensitive replacement with word boundaries
        let re = Regex::new(&format!("(?i){}", pattern)).unwrap();
        result = re.replace_all(&result, *replacement).to_string();
    }

    // Title case the result
    title_case(&result)
}

/// Convert string to title case
fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Parse Address String To Components.
pub async fn parse(address: &str, country: Option<&str>) -> Result<ParseOutput, String> {
    let country_hint = country.map(normalize_country).unwrap_or_else(|| "US".to_string());

    // Normalize the address string
    let address = address.replace('\n', ", ").replace("  ", " ");
    let parts: Vec<&str> = address.split(',').map(|s| s.trim()).collect();

    let mut result = ParseOutput {
        street: String::new(),
        city: String::new(),
        state: String::new(),
        postal_code: String::new(),
        country: country_hint.clone(),
    };

    if parts.is_empty() {
        return Ok(result);
    }

    // First part is typically the street
    result.street = parts[0].to_string();

    if parts.len() >= 2 {
        // Second part could be city or city + state
        let second = parts[1];

        // Try to find state abbreviation and postal code
        let state_postal_re = Regex::new(r"^(.+?)\s+([A-Z]{2})\s*(\d{5}(?:-\d{4})?)?$").unwrap();
        if let Some(caps) = state_postal_re.captures(second) {
            result.city = caps.get(1).map_or("", |m| m.as_str()).to_string();
            result.state = caps.get(2).map_or("", |m| m.as_str()).to_string();
            if let Some(postal) = caps.get(3) {
                result.postal_code = postal.as_str().to_string();
            }
        } else {
            result.city = second.to_string();
        }
    }

    if parts.len() >= 3 {
        // Third part could be state + postal or just state
        let third = parts[2];

        // Check for state and postal code
        let state_postal_re = Regex::new(r"^([A-Z]{2})\s*(\d{5}(?:-\d{4})?)?$").unwrap();
        if let Some(caps) = state_postal_re.captures(third) {
            result.state = caps.get(1).map_or("", |m| m.as_str()).to_string();
            if let Some(postal) = caps.get(2) {
                result.postal_code = postal.as_str().to_string();
            }
        } else {
            // Check if it's just a postal code
            let postal_re = Regex::new(r"^\d{5}(?:-\d{4})?$").unwrap();
            if postal_re.is_match(third) {
                result.postal_code = third.to_string();
            } else {
                // Might be city if we haven't set it
                if result.city.is_empty() {
                    result.city = third.to_string();
                } else {
                    result.state = third.to_string();
                }
            }
        }
    }

    if parts.len() >= 4 {
        // Fourth part could be postal code or country
        let fourth = parts[3];
        let postal_re = Regex::new(r"^\d{5}(?:-\d{4})?$").unwrap();
        if postal_re.is_match(fourth) {
            result.postal_code = fourth.to_string();
        } else {
            result.country = normalize_country(fourth);
        }
    }

    if parts.len() >= 5 {
        // Fifth part is likely country
        result.country = normalize_country(parts[4]);
    }

    // Normalize state
    if !result.state.is_empty() && result.country == "US" {
        result.state = normalize_state(&result.state);
    }

    Ok(result)
}

/// Validate Postal Address.
pub async fn validate(address: Address, country: Option<&str>) -> Result<ValidateOutput, String> {
    let mut errors = Vec::new();

    let effective_country = if !address.country.is_empty() {
        normalize_country(&address.country)
    } else {
        country.map(normalize_country).unwrap_or_else(|| "US".to_string())
    };

    // Validate street
    if address.street.trim().is_empty() {
        errors.push(AddressValidationError {
            field: "street".to_string(),
            code: "REQUIRED".to_string(),
            message: "Street address is required".to_string(),
        });
    } else if address.street.len() < 3 {
        errors.push(AddressValidationError {
            field: "street".to_string(),
            code: "TOO_SHORT".to_string(),
            message: "Street address is too short".to_string(),
        });
    }

    // Validate city
    if address.city.trim().is_empty() {
        errors.push(AddressValidationError {
            field: "city".to_string(),
            code: "REQUIRED".to_string(),
            message: "City is required".to_string(),
        });
    }

    // Validate state (for US)
    if effective_country == "US" {
        if address.state.trim().is_empty() {
            errors.push(AddressValidationError {
                field: "state".to_string(),
                code: "REQUIRED".to_string(),
                message: "State is required for US addresses".to_string(),
            });
        } else {
            let state_upper = address.state.to_uppercase();
            let is_valid_state = US_STATES
                .iter()
                .any(|(abbr, name)| *abbr == state_upper || name.to_uppercase() == state_upper);
            if !is_valid_state {
                errors.push(AddressValidationError {
                    field: "state".to_string(),
                    code: "INVALID".to_string(),
                    message: format!("'{}' is not a valid US state", address.state),
                });
            }
        }
    }

    // Validate postal code
    if address.postal_code.trim().is_empty() {
        errors.push(AddressValidationError {
            field: "postal_code".to_string(),
            code: "REQUIRED".to_string(),
            message: "Postal code is required".to_string(),
        });
    } else if let Some(pattern) = get_postal_code_pattern(&effective_country) {
        let re = Regex::new(pattern).unwrap();
        if !re.is_match(&address.postal_code) {
            errors.push(AddressValidationError {
                field: "postal_code".to_string(),
                code: "INVALID_FORMAT".to_string(),
                message: format!(
                    "Postal code '{}' is not valid for {}",
                    address.postal_code, effective_country
                ),
            });
        }
    }

    Ok(ValidateOutput {
        valid: errors.is_empty(),
        errors,
    })
}

#[cfg(test)]
mod tests;
