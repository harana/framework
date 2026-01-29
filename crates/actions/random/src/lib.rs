// Harana Actions - Random Module
// This module provides random actions and functionality.

#![warn(missing_docs)]

pub mod output;

use output::*;
use rand::prelude::*;
use serde_json::Value;

/// Generate Random Bytes
/// 
/// Generates a specified number of random bytes, returned as base64.
pub async fn bytes(
    length: i32,
) -> Result<BytesOutput, String> {
    if length <= 0 {
        return Err("Length must be positive".to_string());
    }
    
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..length as usize).map(|_| rng.gen()).collect();
    let base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
    
    Ok(BytesOutput { bytes, base64 })
}

/// Random Choice From List
/// 
/// Selects a random item from a list of values.
pub async fn choice(
    items: Vec<Value>,
) -> Result<ChoiceOutput, String> {
    if items.is_empty() {
        return Err("Items list cannot be empty".to_string());
    }
    
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..items.len());
    let item = items[index].clone();
    
    Ok(ChoiceOutput { item, index })
}

/// Generate Random Number
/// 
/// Generates a random number within a specified range.
pub async fn number(
    integer: Option<bool>,
    max: Option<f64>,
    min: Option<f64>,
) -> Result<NumberOutput, String> {
    let min_val = min.unwrap_or(0.0);
    let max_val = max.unwrap_or(1.0);
    
    if min_val >= max_val {
        return Err("Min must be less than max".to_string());
    }
    
    let mut rng = rand::thread_rng();
    let number = if integer.unwrap_or(false) {
        let min_int = min_val.ceil() as i64;
        let max_int = max_val.floor() as i64;
        if min_int > max_int {
            return Err("No integers in range".to_string());
        }
        rng.gen_range(min_int..=max_int) as f64
    } else {
        rng.gen_range(min_val..max_val)
    };
    
    Ok(NumberOutput { number })
}

/// Shuffle List Items
/// 
/// Randomly shuffles the order of items in a list.
pub async fn shuffle(
    items: Vec<Value>,
) -> Result<ShuffleOutput, String> {
    let mut shuffled = items;
    let mut rng = rand::thread_rng();
    shuffled.shuffle(&mut rng);
    
    Ok(ShuffleOutput { items: shuffled })
}

/// Generate Random String
/// 
/// Generates a random string of the specified length using the given charset.
pub async fn string(
    length: i32,
    charset: Option<&str>,
) -> Result<StringOutput, String> {
    if length <= 0 {
        return Err("Length must be positive".to_string());
    }
    
    let chars: Vec<char> = charset
        .unwrap_or("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
        .chars()
        .collect();
    
    if chars.is_empty() {
        return Err("Charset cannot be empty".to_string());
    }
    
    let mut rng = rand::thread_rng();
    let string: String = (0..length as usize)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect();
    
    Ok(StringOutput { string })
}

/// Generate Random UUID
/// 
/// Generates a random UUID of the specified version.
pub async fn uuid(
    version: Option<&str>,
) -> Result<UuidOutput, String> {
    let uuid = match version.unwrap_or("v4") {
        "v4" | "4" => uuid::Uuid::new_v4().to_string(),
        "v7" | "7" => uuid::Uuid::now_v7().to_string(),
        v => return Err(format!("Unsupported UUID version: {}. Use 'v4' or 'v7'", v)),
    };
    
    Ok(UuidOutput { uuid })
}

#[cfg(test)]
mod tests;
