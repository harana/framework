// Harana Actions - Transform Module
// This module provides transform actions and functionality.

#![warn(missing_docs)]

pub mod output;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use serde_json::Value;
use output::*;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use pulldown_cmark::{Parser, html};
use csv::{ReaderBuilder, WriterBuilder};

/// Base64 Decode Data
pub async fn base64_decode(
    data: &str,
) -> Result<Base64DecodeOutput, String> {
    let decoded_bytes = STANDARD.decode(data)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;
    let decoded = String::from_utf8(decoded_bytes)
        .map_err(|e| format!("Decoded data is not valid UTF-8: {}", e))?;
    Ok(Base64DecodeOutput { decoded })
}

/// Base64 Encode Data
pub async fn base64_encode(
    data: &str,
) -> Result<Base64EncodeOutput, String> {
    let encoded = STANDARD.encode(data.as_bytes());
    Ok(Base64EncodeOutput { encoded })
}

/// Convert CSV To JSON
pub async fn csv_to_json(
    data: &str,
    headers: Option<Vec<String>>,
    delimiter: Option<&str>,
) -> Result<CsvToJsonOutput, String> {
    let delimiter_char = delimiter
        .and_then(|d| d.chars().next())
        .unwrap_or(',');
    
    let has_headers = headers.is_none();
    let mut reader = ReaderBuilder::new()
        .delimiter(delimiter_char as u8)
        .has_headers(has_headers)
        .from_reader(data.as_bytes());
    
    // Get headers either from CSV or from provided option
    let header_names: Vec<String> = if let Some(h) = headers {
        h
    } else {
        reader.headers()
            .map_err(|e| format!("Failed to read CSV headers: {}", e))?
            .iter()
            .map(|s| s.to_string())
            .collect()
    };
    
    let mut json_records: Vec<HashMap<String, Value>> = Vec::new();
    
    for result in reader.records() {
        let record = result.map_err(|e| format!("Failed to read CSV record: {}", e))?;
        let mut row: HashMap<String, Value> = HashMap::new();
        
        for (i, field) in record.iter().enumerate() {
            let key = header_names.get(i)
                .cloned()
                .unwrap_or_else(|| format!("field_{}", i));
            
            // Try to parse as number or boolean, fallback to string
            let value = if let Ok(n) = field.parse::<i64>() {
                Value::Number(n.into())
            } else if let Ok(n) = field.parse::<f64>() {
                Value::Number(serde_json::Number::from_f64(n).unwrap_or_else(|| 0.into()))
            } else if field == "true" {
                Value::Bool(true)
            } else if field == "false" {
                Value::Bool(false)
            } else {
                Value::String(field.to_string())
            };
            
            row.insert(key, value);
        }
        
        json_records.push(row);
    }
    
    Ok(CsvToJsonOutput { json: json_records })
}

/// HTML Decode String
pub async fn html_decode(
    data: &str,
) -> Result<HtmlDecodeOutput, String> {
    let decoded = htmlize::unescape(data).to_string();
    Ok(HtmlDecodeOutput { decoded })
}

/// HTML Encode String
pub async fn html_encode(
    data: &str,
) -> Result<HtmlEncodeOutput, String> {
    let encoded = htmlize::escape_text(data).to_string();
    Ok(HtmlEncodeOutput { encoded })
}

/// Convert JSON To CSV
pub async fn json_to_csv(
    data: Vec<HashMap<String, Value>>,
    delimiter: Option<&str>,
    headers: Option<Vec<String>>,
) -> Result<JsonToCsvOutput, String> {
    if data.is_empty() {
        return Ok(JsonToCsvOutput { csv: String::new() });
    }
    
    let delimiter_char = delimiter
        .and_then(|d| d.chars().next())
        .unwrap_or(',');
    
    // Determine headers - use provided or collect all keys from data
    let header_names: Vec<String> = if let Some(h) = headers {
        h
    } else {
        let mut keys: Vec<String> = data.iter()
            .flat_map(|row| row.keys().cloned())
            .collect();
        keys.sort();
        keys.dedup();
        keys
    };
    
    let mut writer = WriterBuilder::new()
        .delimiter(delimiter_char as u8)
        .from_writer(Vec::new());
    
    // Write headers
    writer.write_record(&header_names)
        .map_err(|e| format!("Failed to write CSV headers: {}", e))?;
    
    // Write data rows
    for row in data {
        let record: Vec<String> = header_names.iter()
            .map(|key| {
                row.get(key)
                    .map(|v| match v {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null => String::new(),
                        _ => v.to_string(),
                    })
                    .unwrap_or_default()
            })
            .collect();
        
        writer.write_record(&record)
            .map_err(|e| format!("Failed to write CSV record: {}", e))?;
    }
    
    let csv = String::from_utf8(writer.into_inner()
        .map_err(|e| format!("Failed to finalize CSV: {}", e))?)
        .map_err(|e| format!("CSV is not valid UTF-8: {}", e))?;
    
    Ok(JsonToCsvOutput { csv })
}

/// Convert JSON To XML
pub async fn json_to_xml(
    data: HashMap<String, Value>,
    root_element: Option<&str>,
) -> Result<JsonToXmlOutput, String> {
    let root = root_element.unwrap_or("root");
    
    fn value_to_xml(value: &Value, indent: usize) -> String {
        let indent_str = "  ".repeat(indent);
        match value {
            Value::Null => "".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => htmlize::escape_text(s).to_string(),
            Value::Array(arr) => {
                arr.iter()
                    .map(|v| {
                        let content = value_to_xml(v, indent + 1);
                        format!("{}<item>{}</item>", indent_str, content)
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            Value::Object(obj) => {
                obj.iter()
                    .map(|(k, v)| {
                        let content = value_to_xml(v, indent + 1);
                        if matches!(v, Value::Array(_) | Value::Object(_)) {
                            format!("{}<{}>\n{}\n{}</{}>", indent_str, k, content, indent_str, k)
                        } else {
                            format!("{}<{}>{}</{}>", indent_str, k, content, k)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
    }
    
    let inner = data.iter()
        .map(|(k, v)| {
            let content = value_to_xml(v, 1);
            if matches!(v, Value::Array(_) | Value::Object(_)) {
                format!("  <{}>\n{}\n  </{}>", k, content, k)
            } else {
                format!("  <{}>{}</{}>", k, content, k)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    
    let xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<{}>\n{}\n</{}>", root, inner, root);
    
    Ok(JsonToXmlOutput { xml })
}

/// Convert JSON To YAML
pub async fn json_to_yaml(
    data: HashMap<String, Value>,
) -> Result<JsonToYamlOutput, String> {
    let yaml = serde_yaml::to_string(&data)
        .map_err(|e| format!("Failed to convert to YAML: {}", e))?;
    Ok(JsonToYamlOutput { yaml })
}

/// Convert Markdown To HTML
pub async fn markdown_to_html(
    data: &str,
    _options: Option<HashMap<String, Value>>,
) -> Result<MarkdownToHtmlOutput, String> {
    let parser = Parser::new(data);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Ok(MarkdownToHtmlOutput { html: html_output })
}

/// URL Decode String
pub async fn url_decode(
    data: &str,
) -> Result<UrlDecodeOutput, String> {
    let decoded = urlencoding::decode(data)
        .map_err(|e| format!("Failed to URL decode: {}", e))?
        .into_owned();
    Ok(UrlDecodeOutput { decoded })
}

/// URL Encode String
pub async fn url_encode(
    data: &str,
) -> Result<UrlEncodeOutput, String> {
    let encoded = urlencoding::encode(data).into_owned();
    Ok(UrlEncodeOutput { encoded })
}

/// Convert XML To JSON
pub async fn xml_to_json(
    data: &str,
) -> Result<XmlToJsonOutput, String> {
    use quick_xml::events::Event;
    use quick_xml::Reader;
    
    fn parse_element(reader: &mut Reader<&[u8]>) -> Result<Value, String> {
        let mut children: HashMap<String, Vec<Value>> = HashMap::new();
        let mut text_content = String::new();
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    let value = parse_element(reader)?;
                    children.entry(name).or_default().push(value);
                }
                Ok(Event::Empty(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    children.entry(name).or_default().push(Value::Null);
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape()
                        .map_err(|e| format!("Failed to unescape text: {}", e))?
                        .trim()
                        .to_string();
                    if !text.is_empty() {
                        text_content.push_str(&text);
                    }
                }
                Ok(Event::End(_)) => break,
                Ok(Event::Eof) => break,
                Ok(_) => {}
                Err(e) => return Err(format!("XML parse error: {}", e)),
            }
            buf.clear();
        }
        
        if children.is_empty() {
            // Leaf node - return text content or null
            if text_content.is_empty() {
                Ok(Value::Null)
            } else if let Ok(n) = text_content.parse::<i64>() {
                Ok(Value::Number(n.into()))
            } else if let Ok(n) = text_content.parse::<f64>() {
                Ok(Value::Number(serde_json::Number::from_f64(n).unwrap_or_else(|| 0.into())))
            } else if text_content == "true" {
                Ok(Value::Bool(true))
            } else if text_content == "false" {
                Ok(Value::Bool(false))
            } else {
                Ok(Value::String(text_content))
            }
        } else {
            // Has children - return object
            let mut obj: HashMap<String, Value> = HashMap::new();
            for (key, values) in children {
                if values.len() == 1 {
                    obj.insert(key, values.into_iter().next().unwrap());
                } else {
                    obj.insert(key, Value::Array(values));
                }
            }
            Ok(Value::Object(obj.into_iter().collect()))
        }
    }
    
    let mut reader = Reader::from_str(data);
    reader.config_mut().trim_text(true);
    
    let mut result: HashMap<String, Value> = HashMap::new();
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let value = parse_element(&mut reader)?;
                result.insert(name, value);
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(format!("XML parse error: {}", e)),
        }
        buf.clear();
    }
    
    Ok(XmlToJsonOutput { json: result })
}

/// Convert YAML To JSON
pub async fn yaml_to_json(
    data: &str,
) -> Result<YamlToJsonOutput, String> {
    let value: Value = serde_yaml::from_str(data)
        .map_err(|e| format!("Failed to parse YAML: {}", e))?;
    
    let json = match value {
        Value::Object(map) => map.into_iter().collect(),
        _ => {
            let mut map = HashMap::new();
            map.insert("value".to_string(), value);
            map
        }
    };
    
    Ok(YamlToJsonOutput { json })
}
