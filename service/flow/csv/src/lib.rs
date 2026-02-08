pub mod output;

use output::*;
use std::collections::HashMap;
use std::io::Cursor;

/// Parses CSV data into a vector of rows (each row is a HashMap of column name to value).
pub async fn parse(
    data: &str,
    delimiter: Option<&str>,
    has_headers: Option<bool>,
    headers: Option<Vec<String>>,
    skip_empty_lines: Option<bool>,
) -> Result<ParseOutput, String> {
    let delimiter = delimiter.unwrap_or(",");
    let has_headers = has_headers.unwrap_or(true);
    let skip_empty_lines = skip_empty_lines.unwrap_or(true);
    
    let delimiter_byte = delimiter.as_bytes().first().copied().unwrap_or(b',');
    
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter_byte)
        .has_headers(has_headers)
        .flexible(true)
        .from_reader(Cursor::new(data));
    
    // Determine headers
    let column_headers: Vec<String> = if let Some(custom_headers) = headers {
        custom_headers
    } else if has_headers {
        reader
            .headers()
            .map_err(|e| format!("Failed to read headers: {}", e))?
            .iter()
            .map(|s| s.to_string())
            .collect()
    } else {
        // Generate default headers (col_0, col_1, etc.)
        vec![]
    };
    
    let mut rows: Vec<HashMap<String, String>> = Vec::new();
    
    for (idx, result) in reader.records().enumerate() {
        let record = result.map_err(|e| format!("Failed to read row {}: {}", idx + 1, e))?;
        
        // Skip empty lines if configured
        if skip_empty_lines && record.iter().all(|field| field.trim().is_empty()) {
            continue;
        }
        
        let mut row: HashMap<String, String> = HashMap::new();
        
        for (i, field) in record.iter().enumerate() {
            let key = if i < column_headers.len() {
                column_headers[i].clone()
            } else {
                format!("col_{}", i)
            };
            row.insert(key, field.to_string());
        }
        
        rows.push(row);
    }
    
    Ok(ParseOutput { rows })
}

/// Generates CSV data from a vector of rows (each row is a HashMap of column name to value).
pub async fn generate(
    data: Vec<HashMap<String, String>>,
    delimiter: Option<&str>,
    headers: Option<Vec<String>>,
    include_headers: Option<bool>,
) -> Result<GenerateOutput, String> {
    let delimiter = delimiter.unwrap_or(",");
    let include_headers = include_headers.unwrap_or(true);
    let delimiter_byte = delimiter.as_bytes().first().copied().unwrap_or(b',');
    
    // Determine column order
    let column_headers: Vec<String> = if let Some(custom_headers) = headers {
        custom_headers
    } else {
        // Collect all unique keys from all rows, maintaining some order
        let mut all_keys: Vec<String> = Vec::new();
        for row in &data {
            for key in row.keys() {
                if !all_keys.contains(key) {
                    all_keys.push(key.clone());
                }
            }
        }
        all_keys.sort();
        all_keys
    };
    
    let mut writer = csv::WriterBuilder::new()
        .delimiter(delimiter_byte)
        .from_writer(Vec::new());
    
    // Write headers if configured
    if include_headers && !column_headers.is_empty() {
        writer
            .write_record(&column_headers)
            .map_err(|e| format!("Failed to write headers: {}", e))?;
    }
    
    // Write data rows
    for row in &data {
        let record: Vec<String> = column_headers
            .iter()
            .map(|header| row.get(header).cloned().unwrap_or_default())
            .collect();
        writer
            .write_record(&record)
            .map_err(|e| format!("Failed to write row: {}", e))?;
    }
    
    let csv_bytes = writer
        .into_inner()
        .map_err(|e| format!("Failed to finalize CSV: {}", e))?;
    
    let csv = String::from_utf8(csv_bytes)
        .map_err(|e| format!("Failed to convert CSV to string: {}", e))?;
    
    Ok(GenerateOutput { csv })
}

/// Transforms CSV data by applying operations to columns and rows.
pub async fn transform(
    data: &str,
    operations: Vec<CsvTransformOperation>,
    delimiter: Option<&str>,
) -> Result<TransformOutput, String> {
    // First parse the CSV
    let parsed = parse(data, delimiter, Some(true), None, Some(true)).await?;
    let mut rows = parsed.rows;
    
    // Apply each operation
    for op in operations {
        rows = apply_transform_operation(rows, &op)?;
    }
    
    // Convert back to CSV
    let result = generate(rows, delimiter, None, Some(true)).await?;
    
    Ok(TransformOutput { csv: result.csv })
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CsvTransformOperation {
    pub operation: String,
    pub column: Option<String>,
    pub parameters: Option<HashMap<String, String>>,
}

fn apply_transform_operation(
    mut rows: Vec<HashMap<String, String>>,
    op: &CsvTransformOperation,
) -> Result<Vec<HashMap<String, String>>, String> {
    match op.operation.as_str() {
        "uppercase" => {
            if let Some(col) = &op.column {
                for row in &mut rows {
                    if let Some(value) = row.get_mut(col) {
                        *value = value.to_uppercase();
                    }
                }
            }
        }
        "lowercase" => {
            if let Some(col) = &op.column {
                for row in &mut rows {
                    if let Some(value) = row.get_mut(col) {
                        *value = value.to_lowercase();
                    }
                }
            }
        }
        "trim" => {
            if let Some(col) = &op.column {
                for row in &mut rows {
                    if let Some(value) = row.get_mut(col) {
                        *value = value.trim().to_string();
                    }
                }
            } else {
                // Trim all columns
                for row in &mut rows {
                    for value in row.values_mut() {
                        *value = value.trim().to_string();
                    }
                }
            }
        }
        "rename" => {
            if let (Some(col), Some(params)) = (&op.column, &op.parameters) {
                if let Some(new_name) = params.get("new_name") {
                    for row in &mut rows {
                        if let Some(value) = row.remove(col) {
                            row.insert(new_name.clone(), value);
                        }
                    }
                }
            }
        }
        "remove" => {
            if let Some(col) = &op.column {
                for row in &mut rows {
                    row.remove(col);
                }
            }
        }
        "replace" => {
            if let (Some(col), Some(params)) = (&op.column, &op.parameters) {
                let pattern = params.get("pattern").map(|s| s.as_str()).unwrap_or("");
                let replacement = params.get("replacement").map(|s| s.as_str()).unwrap_or("");
                for row in &mut rows {
                    if let Some(value) = row.get_mut(col) {
                        *value = value.replace(pattern, replacement);
                    }
                }
            }
        }
        "filter" => {
            if let (Some(col), Some(params)) = (&op.column, &op.parameters) {
                let filter_value = params.get("value").map(|s| s.as_str()).unwrap_or("");
                let filter_op = params.get("operator").map(|s| s.as_str()).unwrap_or("equals");
                
                rows.retain(|row| {
                    if let Some(value) = row.get(col) {
                        match filter_op {
                            "equals" => value == filter_value,
                            "not_equals" => value != filter_value,
                            "contains" => value.contains(filter_value),
                            "starts_with" => value.starts_with(filter_value),
                            "ends_with" => value.ends_with(filter_value),
                            _ => true,
                        }
                    } else {
                        false
                    }
                });
            }
        }
        _ => {
            return Err(format!("Unknown operation: {}", op.operation));
        }
    }
    
    Ok(rows)
}

/// Validates CSV data against a schema.
pub async fn validate(
    data: &str,
    schema: CsvSchema,
    delimiter: Option<&str>,
) -> Result<ValidateOutput, String> {
    let parsed = parse(data, delimiter, Some(true), None, Some(true)).await?;
    let mut errors: Vec<CsvValidationError> = Vec::new();
    
    for (row_idx, row) in parsed.rows.iter().enumerate() {
        let row_num = (row_idx + 1) as i32;
        
        for column_schema in &schema.columns {
            let value = row.get(&column_schema.name);
            
            // Check required
            if column_schema.required.unwrap_or(false) {
                match value {
                    None => {
                        errors.push(CsvValidationError {
                            row: row_num,
                            column: column_schema.name.clone(),
                            message: "Required column is missing".to_string(),
                        });
                        continue;
                    }
                    Some(v) if v.trim().is_empty() => {
                        errors.push(CsvValidationError {
                            row: row_num,
                            column: column_schema.name.clone(),
                            message: "Required column is empty".to_string(),
                        });
                        continue;
                    }
                    _ => {}
                }
            }
            
            if let Some(val) = value {
                // Check type
                if let Some(col_type) = &column_schema.column_type {
                    let type_valid = match col_type.as_str() {
                        "string" => true, // All values are strings
                        "integer" => val.parse::<i64>().is_ok(),
                        "float" | "number" => val.parse::<f64>().is_ok(),
                        "boolean" => matches!(val.to_lowercase().as_str(), "true" | "false" | "1" | "0" | "yes" | "no"),
                        "email" => val.contains('@') && val.contains('.'),
                        _ => true,
                    };
                    
                    if !type_valid {
                        errors.push(CsvValidationError {
                            row: row_num,
                            column: column_schema.name.clone(),
                            message: format!("Value '{}' is not a valid {}", val, col_type),
                        });
                    }
                }
                
                // Check pattern
                if let Some(pattern) = &column_schema.pattern {
                    if let Ok(re) = regex::Regex::new(pattern) {
                        if !re.is_match(val) {
                            errors.push(CsvValidationError {
                                row: row_num,
                                column: column_schema.name.clone(),
                                message: format!("Value '{}' does not match pattern '{}'", val, pattern),
                            });
                        }
                    }
                }
            }
        }
        
        // Check for extra columns if not allowed
        if !schema.allow_extra_columns.unwrap_or(true) {
            let schema_columns: Vec<&str> = schema.columns.iter().map(|c| c.name.as_str()).collect();
            for key in row.keys() {
                if !schema_columns.contains(&key.as_str()) {
                    errors.push(CsvValidationError {
                        row: row_num,
                        column: key.clone(),
                        message: "Unexpected column not in schema".to_string(),
                    });
                }
            }
        }
    }
    
    let valid = errors.is_empty();
    Ok(ValidateOutput { errors, valid })
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CsvSchema {
    pub columns: Vec<CsvColumnSchema>,
    pub allow_extra_columns: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CsvColumnSchema {
    pub name: String,
    #[serde(rename = "type")]
    pub column_type: Option<String>,
    pub required: Option<bool>,
    pub pattern: Option<String>,
}

#[cfg(test)]
mod tests;
