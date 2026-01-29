// Harana Actions - Excel Module
// This module provides excel actions and functionality.

#![warn(missing_docs)]

pub mod output;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use serde_json::Value;
use output::*;
use calamine::{Reader, Xlsx, open_workbook, Data, Range};
use rust_xlsxwriter::{Workbook, Format};

/// Helper function to parse cell value to JSON Value
fn cell_to_value(cell: &Data) -> Value {
    match cell {
        Data::Int(i) => Value::Number((*i).into()),
        Data::Float(f) => {
            if let Some(n) = serde_json::Number::from_f64(*f) {
                Value::Number(n)
            } else {
                Value::String(f.to_string())
            }
        }
        Data::String(s) => Value::String(s.clone()),
        Data::Bool(b) => Value::Bool(*b),
        Data::DateTime(dt) => Value::String(dt.to_string()),
        Data::DateTimeIso(s) => Value::String(s.clone()),
        Data::DurationIso(s) => Value::String(s.clone()),
        Data::Error(e) => Value::String(format!("Error: {:?}", e)),
        Data::Empty => Value::Null,
    }
}

/// Helper function to parse a range string like "A1:D10" into (start_row, start_col, end_row, end_col)
fn parse_range(range_str: &str) -> Option<(u32, u32, u32, u32)> {
    let parts: Vec<&str> = range_str.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    
    fn parse_cell(cell: &str) -> Option<(u32, u32)> {
        let cell = cell.trim().to_uppercase();
        let mut col_str = String::new();
        let mut row_str = String::new();
        
        for c in cell.chars() {
            if c.is_ascii_alphabetic() {
                col_str.push(c);
            } else if c.is_ascii_digit() {
                row_str.push(c);
            }
        }
        
        if col_str.is_empty() || row_str.is_empty() {
            return None;
        }
        
        // Convert column letters to 0-based index (A=0, B=1, etc.)
        let mut col: u32 = 0;
        for c in col_str.chars() {
            col = col * 26 + (c as u32 - 'A' as u32 + 1);
        }
        col -= 1;
        
        let row: u32 = row_str.parse().ok()?;
        let row = row.checked_sub(1)?; // Convert to 0-based
        
        Some((row, col))
    }
    
    let (start_row, start_col) = parse_cell(parts[0])?;
    let (end_row, end_col) = parse_cell(parts[1])?;
    
    Some((start_row, start_col, end_row, end_col))
}

/// Helper function to read data from a range
fn read_range_data(range: &Range<Data>, has_headers: bool, range_str: Option<&str>) -> Vec<HashMap<String, Value>> {
    let (start_row, start_col, end_row, end_col) = if let Some(r) = range_str {
        parse_range(r).unwrap_or((0, 0, range.height() as u32 - 1, range.width() as u32 - 1))
    } else {
        (0, 0, range.height() as u32 - 1, range.width() as u32 - 1)
    };
    
    let mut rows: Vec<HashMap<String, Value>> = Vec::new();
    let mut headers: Vec<String> = Vec::new();
    
    let data_start_row = if has_headers { start_row + 1 } else { start_row };
    
    // Get headers
    if has_headers && range.height() > 0 {
        for col in start_col..=end_col {
            if let Some(cell) = range.get((start_row as usize, col as usize)) {
                headers.push(match cell {
                    Data::String(s) => s.clone(),
                    _ => format!("column_{}", col),
                });
            } else {
                headers.push(format!("column_{}", col));
            }
        }
    } else {
        for col in start_col..=end_col {
            headers.push(format!("column_{}", col));
        }
    }
    
    // Read data rows
    for row_idx in data_start_row..=end_row {
        let mut row_data: HashMap<String, Value> = HashMap::new();
        for (col_offset, col_idx) in (start_col..=end_col).enumerate() {
            if let Some(cell) = range.get((row_idx as usize, col_idx as usize)) {
                let key = headers.get(col_offset).cloned().unwrap_or_else(|| format!("column_{}", col_idx));
                row_data.insert(key, cell_to_value(cell));
            }
        }
        if !row_data.is_empty() {
            rows.push(row_data);
        }
    }
    
    rows
}

/// List Sheets In Workbook
pub async fn get_sheets(
    file: &str,
) -> Result<GetSheetsOutput, String> {
    let workbook: Xlsx<_> = open_workbook(file)
        .map_err(|e| format!("Failed to open workbook: {}", e))?;
    
    let sheets: Vec<String> = workbook.sheet_names().to_vec();
    
    Ok(GetSheetsOutput { sheets })
}

/// Read Excel File To Data
pub async fn read(
    file: &str,
    has_headers: Option<bool>,
    sheet: Option<&str>,
    range: Option<&str>,
) -> Result<ReadOutput, String> {
    let has_headers = has_headers.unwrap_or(true);
    
    let mut workbook: Xlsx<_> = open_workbook(file)
        .map_err(|e| format!("Failed to open workbook: {}", e))?;
    
    // Get sheet name - use provided or first sheet
    let sheet_name = if let Some(s) = sheet {
        s.to_string()
    } else {
        workbook.sheet_names()
            .first()
            .cloned()
            .ok_or_else(|| "Workbook has no sheets".to_string())?
    };
    
    let worksheet = workbook.worksheet_range(&sheet_name)
        .map_err(|e| format!("Failed to read sheet '{}': {}", sheet_name, e))?;
    
    let rows = read_range_data(&worksheet, has_headers, range);
    
    Ok(ReadOutput { rows })
}

/// Read Specific Sheet
pub async fn read_sheet(
    sheet: &str,
    file: &str,
    range: Option<&str>,
    has_headers: Option<bool>,
) -> Result<ReadSheetOutput, String> {
    let has_headers = has_headers.unwrap_or(true);
    
    let mut workbook: Xlsx<_> = open_workbook(file)
        .map_err(|e| format!("Failed to open workbook: {}", e))?;
    
    let worksheet = workbook.worksheet_range(sheet)
        .map_err(|e| format!("Failed to read sheet '{}': {}", sheet, e))?;
    
    let rows = read_range_data(&worksheet, has_headers, range);
    
    Ok(ReadSheetOutput { rows })
}

/// Write Data To Excel File
pub async fn write(
    file: &str,
    data: Vec<HashMap<String, Value>>,
    headers: Option<Vec<String>>,
    include_headers: Option<bool>,
    sheet: Option<&str>,
) -> Result<WriteOutput, String> {
    let include_headers = include_headers.unwrap_or(true);
    let sheet_name = sheet.unwrap_or("Sheet1");
    
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_name(sheet_name)
        .map_err(|e| format!("Failed to set sheet name: {}", e))?;
    
    // Determine headers - use provided or collect from data
    let header_names: Vec<String> = if let Some(h) = headers {
        h
    } else if !data.is_empty() {
        let mut keys: Vec<String> = data.iter()
            .flat_map(|row| row.keys().cloned())
            .collect();
        keys.sort();
        keys.dedup();
        keys
    } else {
        Vec::new()
    };
    
    let header_format = Format::new().set_bold();
    let mut row_idx: u32 = 0;
    
    // Write headers
    if include_headers && !header_names.is_empty() {
        for (col_idx, header) in header_names.iter().enumerate() {
            worksheet.write_string_with_format(row_idx, col_idx as u16, header, &header_format)
                .map_err(|e| format!("Failed to write header: {}", e))?;
        }
        row_idx += 1;
    }
    
    // Write data rows
    for row in data {
        for (col_idx, header) in header_names.iter().enumerate() {
            if let Some(value) = row.get(header) {
                match value {
                    Value::String(s) => {
                        worksheet.write_string(row_idx, col_idx as u16, s)
                            .map_err(|e| format!("Failed to write string: {}", e))?;
                    }
                    Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            worksheet.write_number(row_idx, col_idx as u16, i as f64)
                                .map_err(|e| format!("Failed to write number: {}", e))?;
                        } else if let Some(f) = n.as_f64() {
                            worksheet.write_number(row_idx, col_idx as u16, f)
                                .map_err(|e| format!("Failed to write number: {}", e))?;
                        }
                    }
                    Value::Bool(b) => {
                        worksheet.write_boolean(row_idx, col_idx as u16, *b)
                            .map_err(|e| format!("Failed to write boolean: {}", e))?;
                    }
                    Value::Null => {
                        // Skip null values (leave cell empty)
                    }
                    _ => {
                        worksheet.write_string(row_idx, col_idx as u16, &value.to_string())
                            .map_err(|e| format!("Failed to write value: {}", e))?;
                    }
                }
            }
        }
        row_idx += 1;
    }
    
    workbook.save(file)
        .map_err(|e| format!("Failed to save workbook: {}", e))?;
    
    Ok(WriteOutput { path: file.to_string() })
}
