pub mod output;

use output::*;
use uuid::Uuid;

/// Generates one or more random UUID v4 values.
pub async fn generate_v4(
    uppercase: Option<bool>,
    count: Option<i32>,
) -> Result<GenerateV4Output, String> {
    let count = count.unwrap_or(1).max(1) as usize;
    let uppercase = uppercase.unwrap_or(false);
    
    let uuids: Vec<String> = (0..count)
        .map(|_| {
            let uuid = Uuid::new_v4();
            if uppercase {
                uuid.to_string().to_uppercase()
            } else {
                uuid.to_string()
            }
        })
        .collect();
    
    Ok(GenerateV4Output {
        uuid: uuids.first().cloned().unwrap_or_default(),
        uuids,
    })
}

/// Generates one or more time-ordered UUID v7 values.
pub async fn generate_v7(
    uppercase: Option<bool>,
    count: Option<i32>,
) -> Result<GenerateV7Output, String> {
    let count = count.unwrap_or(1).max(1) as usize;
    let uppercase = uppercase.unwrap_or(false);
    
    let uuids: Vec<String> = (0..count)
        .map(|_| {
            let uuid = Uuid::now_v7();
            if uppercase {
                uuid.to_string().to_uppercase()
            } else {
                uuid.to_string()
            }
        })
        .collect();
    
    Ok(GenerateV7Output {
        uuid: uuids.first().cloned().unwrap_or_default(),
        uuids,
    })
}

/// Parses a UUID string and extracts its components.
pub async fn parse(
    uuid: &str,
) -> Result<ParseOutput, String> {
    let parsed = Uuid::parse_str(uuid)
        .map_err(|e| format!("Invalid UUID: {}", e))?;
    
    let version = parsed.get_version_num() as i32;
    let variant = match parsed.get_variant() {
        uuid::Variant::NCS => "NCS",
        uuid::Variant::RFC4122 => "RFC4122",
        uuid::Variant::Microsoft => "Microsoft",
        uuid::Variant::Future => "Future",
        _ => "Unknown",
    }.to_string();
    
    // Extract timestamp for v7 UUIDs
    let timestamp = if version == 7 {
        let bytes = parsed.as_bytes();
        let ts_bytes: [u8; 8] = [
            0, 0,
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
        ];
        i64::from_be_bytes(ts_bytes) >> 4
    } else {
        0
    };
    
    Ok(ParseOutput {
        version,
        variant,
        timestamp: timestamp as i64,
    })
}

/// Validates whether a string is a valid UUID and optionally checks its version.
pub async fn validate(
    uuid: &str,
    version: Option<i32>,
) -> Result<ValidateOutput, String> {
    match Uuid::parse_str(uuid) {
        Ok(parsed) => {
            let actual_version = parsed.get_version_num() as i32;
            let version_match = version.map_or(true, |v| v == actual_version);
            
            Ok(ValidateOutput {
                valid: version_match,
                version: actual_version,
            })
        }
        Err(_) => Ok(ValidateOutput {
            valid: false,
            version: 0,
        }),
    }
}

#[cfg(test)]
mod tests;

