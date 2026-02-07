pub mod output;

use output::*;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::io::Cursor;

/// Parses XML data into a JSON-like object structure.
pub async fn parse(
    data: &str,
    preserve_attributes: Option<bool>,
    preserve_namespaces: Option<bool>,
) -> Result<ParseOutput, String> {
    let preserve_attributes = preserve_attributes.unwrap_or(true);
    let _preserve_namespaces = preserve_namespaces.unwrap_or(false);
    
    let mut reader = Reader::from_str(data);
    reader.config_mut().trim_text(true);
    
    let result = parse_element(&mut reader, preserve_attributes)?;
    
    Ok(ParseOutput { result })
}

fn parse_element(reader: &mut Reader<&[u8]>, preserve_attributes: bool) -> Result<Value, String> {
    let mut root: Option<(String, Value)> = None;
    let mut stack: Vec<(String, Map<String, Value>, Vec<Value>)> = Vec::new();
    
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut attrs = Map::new();
                
                if preserve_attributes {
                    for attr in e.attributes().filter_map(|a| a.ok()) {
                        let key = format!("@{}", String::from_utf8_lossy(attr.key.as_ref()));
                        let value = String::from_utf8_lossy(&attr.value).to_string();
                        attrs.insert(key, Value::String(value));
                    }
                }
                
                stack.push((name, attrs, Vec::new()));
            }
            Ok(Event::End(_)) => {
                if let Some((name, attrs, children)) = stack.pop() {
                    let element = build_element(attrs, children);
                    
                    if stack.is_empty() {
                        root = Some((name, element));
                    } else {
                        let parent = stack.last_mut().unwrap();
                        parent.2.push(json!({ name: element }));
                    }
                }
            }
            Ok(Event::Empty(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut attrs = Map::new();
                
                if preserve_attributes {
                    for attr in e.attributes().filter_map(|a| a.ok()) {
                        let key = format!("@{}", String::from_utf8_lossy(attr.key.as_ref()));
                        let value = String::from_utf8_lossy(&attr.value).to_string();
                        attrs.insert(key, Value::String(value));
                    }
                }
                
                let element = if attrs.is_empty() {
                    Value::Null
                } else {
                    Value::Object(attrs)
                };
                
                if stack.is_empty() {
                    root = Some((name, element));
                } else {
                    let parent = stack.last_mut().unwrap();
                    parent.2.push(json!({ name: element }));
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().map_err(|e| format!("Failed to unescape text: {}", e))?;
                let text = text.trim();
                if !text.is_empty() && !stack.is_empty() {
                    let parent = stack.last_mut().unwrap();
                    parent.2.push(Value::String(text.to_string()));
                }
            }
            Ok(Event::CData(e)) => {
                let text = String::from_utf8_lossy(e.as_ref()).to_string();
                if !stack.is_empty() {
                    let parent = stack.last_mut().unwrap();
                    parent.2.push(Value::String(text));
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {} // Ignore other events like comments, processing instructions, etc.
            Err(e) => return Err(format!("XML parse error: {}", e)),
        }
    }
    
    match root {
        Some((name, value)) => Ok(json!({ name: value })),
        None => Ok(Value::Null),
    }
}

fn build_element(attrs: Map<String, Value>, children: Vec<Value>) -> Value {
    // If there's only text content
    if attrs.is_empty() && children.len() == 1 {
        if let Some(Value::String(text)) = children.first() {
            return Value::String(text.clone());
        }
    }
    
    // If there are no children and no attributes
    if attrs.is_empty() && children.is_empty() {
        return Value::Null;
    }
    
    let mut result = attrs;
    
    // Group children by their element name
    let mut child_groups: HashMap<String, Vec<Value>> = HashMap::new();
    let mut text_content: Option<String> = None;
    
    for child in children {
        match child {
            Value::Object(obj) => {
                for (key, value) in obj {
                    child_groups.entry(key).or_default().push(value);
                }
            }
            Value::String(s) => {
                text_content = Some(s);
            }
            _ => {}
        }
    }
    
    // Add text content if present
    if let Some(text) = text_content {
        if result.is_empty() && child_groups.is_empty() {
            return Value::String(text);
        }
        result.insert("#text".to_string(), Value::String(text));
    }
    
    // Add child elements
    for (key, values) in child_groups {
        if values.len() == 1 {
            result.insert(key, values.into_iter().next().unwrap());
        } else {
            result.insert(key, Value::Array(values));
        }
    }
    
    Value::Object(result)
}

/// Generates XML from a JSON-like object structure.
pub async fn generate(
    data: Value,
    indent: Option<bool>,
    root_element: Option<&str>,
    declaration: Option<bool>,
) -> Result<GenerateOutput, String> {
    let indent = indent.unwrap_or(true);
    let declaration = declaration.unwrap_or(true);
    let root_element = root_element.unwrap_or("root");
    
    let mut writer = if indent {
        Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2)
    } else {
        Writer::new(Cursor::new(Vec::new()))
    };
    
    // Write XML declaration
    if declaration {
        writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
            .map_err(|e| format!("Failed to write declaration: {}", e))?;
        if indent {
            writer
                .write_event(Event::Text(BytesText::from_escaped("\n")))
                .map_err(|e| format!("Failed to write newline: {}", e))?;
        }
    }
    
    // Determine root element name and content
    let (element_name, content) = match &data {
        Value::Object(obj) if obj.len() == 1 => {
            let (key, value) = obj.iter().next().unwrap();
            (key.as_str(), value.clone())
        }
        _ => (root_element, data),
    };
    
    write_element(&mut writer, element_name, &content)?;
    
    let result = writer.into_inner().into_inner();
    let xml = String::from_utf8(result).map_err(|e| format!("Failed to convert to string: {}", e))?;
    
    Ok(GenerateOutput { xml })
}

fn write_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    name: &str,
    value: &Value,
) -> Result<(), String> {
    match value {
        Value::Null => {
            let elem = BytesStart::new(name);
            writer
                .write_event(Event::Empty(elem))
                .map_err(|e| format!("Failed to write empty element: {}", e))?;
        }
        Value::String(s) => {
            let elem = BytesStart::new(name);
            writer
                .write_event(Event::Start(elem))
                .map_err(|e| format!("Failed to write start: {}", e))?;
            writer
                .write_event(Event::Text(BytesText::new(s)))
                .map_err(|e| format!("Failed to write text: {}", e))?;
            writer
                .write_event(Event::End(BytesEnd::new(name)))
                .map_err(|e| format!("Failed to write end: {}", e))?;
        }
        Value::Number(n) => {
            let elem = BytesStart::new(name);
            writer
                .write_event(Event::Start(elem))
                .map_err(|e| format!("Failed to write start: {}", e))?;
            writer
                .write_event(Event::Text(BytesText::new(&n.to_string())))
                .map_err(|e| format!("Failed to write text: {}", e))?;
            writer
                .write_event(Event::End(BytesEnd::new(name)))
                .map_err(|e| format!("Failed to write end: {}", e))?;
        }
        Value::Bool(b) => {
            let elem = BytesStart::new(name);
            writer
                .write_event(Event::Start(elem))
                .map_err(|e| format!("Failed to write start: {}", e))?;
            writer
                .write_event(Event::Text(BytesText::new(&b.to_string())))
                .map_err(|e| format!("Failed to write text: {}", e))?;
            writer
                .write_event(Event::End(BytesEnd::new(name)))
                .map_err(|e| format!("Failed to write end: {}", e))?;
        }
        Value::Array(arr) => {
            for item in arr {
                write_element(writer, name, item)?;
            }
        }
        Value::Object(obj) => {
            let mut elem = BytesStart::new(name);
            let mut children = Vec::new();
            let mut text_content: Option<String> = None;
            
            // Separate attributes from child elements
            for (key, val) in obj {
                if key.starts_with('@') {
                    // Attribute
                    let attr_name = &key[1..];
                    if let Value::String(attr_val) = val {
                        elem.push_attribute((attr_name, attr_val.as_str()));
                    }
                } else if key == "#text" {
                    // Text content
                    if let Value::String(text) = val {
                        text_content = Some(text.clone());
                    }
                } else {
                    // Child element
                    children.push((key.clone(), val.clone()));
                }
            }
            
            if children.is_empty() && text_content.is_none() {
                writer
                    .write_event(Event::Empty(elem))
                    .map_err(|e| format!("Failed to write empty element: {}", e))?;
            } else {
                writer
                    .write_event(Event::Start(elem))
                    .map_err(|e| format!("Failed to write start: {}", e))?;
                
                if let Some(text) = text_content {
                    writer
                        .write_event(Event::Text(BytesText::new(&text)))
                        .map_err(|e| format!("Failed to write text: {}", e))?;
                }
                
                for (child_name, child_val) in children {
                    write_element(writer, &child_name, &child_val)?;
                }
                
                writer
                    .write_event(Event::End(BytesEnd::new(name)))
                    .map_err(|e| format!("Failed to write end: {}", e))?;
            }
        }
    }
    
    Ok(())
}

/// Validates XML for well-formedness (basic structure validation).
/// Note: Full XSD validation requires additional dependencies.
pub async fn validate(
    data: &str,
    _schema: &str,
) -> Result<ValidateOutput, String> {
    let mut errors = Vec::new();
    let mut reader = Reader::from_str(data);
    reader.config_mut().trim_text(true);
    
    let mut line = 1;
    let mut col = 1;
    let mut last_pos: usize = 0;
    
    loop {
        let current_pos = reader.buffer_position() as usize;
        // Count newlines to track line numbers
        if current_pos > last_pos && current_pos <= data.len() {
            for c in data[last_pos..current_pos].chars() {
                if c == '\n' {
                    line += 1;
                    col = 1;
                } else {
                    col += 1;
                }
            }
        }
        last_pos = current_pos;
        
        match reader.read_event() {
            Ok(Event::Eof) => break,
            Ok(_) => {} // Valid event, continue
            Err(e) => {
                errors.push(XmlValidationError {
                    line,
                    column: col,
                    message: e.to_string(),
                });
                break;
            }
        }
    }
    
    let valid = errors.is_empty();
    Ok(ValidateOutput { errors, valid })
}

/// Performs a simple XPath-like query on XML data.
/// Supports basic path queries like "/root/element" or "//element".
pub async fn xpath_query(
    data: &str,
    query: &str,
    _namespaces: Option<HashMap<String, String>>,
) -> Result<XpathQueryOutput, String> {
    // Parse the XML first
    let parsed = parse(data, Some(true), Some(false)).await?;
    
    // Parse the query path
    let path_parts: Vec<&str> = query
        .trim_start_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();
    
    let is_recursive = query.starts_with("//");
    
    let mut results = Vec::new();
    
    if is_recursive {
        // Search recursively for all matching elements
        find_recursive(&parsed.result, &path_parts, &mut results);
    } else {
        // Follow the exact path
        find_path(&parsed.result, &path_parts, &mut results);
    }
    
    Ok(XpathQueryOutput { results })
}

fn find_path(value: &Value, path: &[&str], results: &mut Vec<Value>) {
    if path.is_empty() {
        results.push(value.clone());
        return;
    }
    
    let current = path[0];
    let remaining = &path[1..];
    
    if let Value::Object(obj) = value {
        if let Some(child) = obj.get(current) {
            if remaining.is_empty() {
                results.push(child.clone());
            } else {
                find_path(child, remaining, results);
            }
        }
    }
}

fn find_recursive(value: &Value, path: &[&str], results: &mut Vec<Value>) {
    if path.is_empty() {
        return;
    }
    
    let target = path[0];
    
    match value {
        Value::Object(obj) => {
            for (key, child) in obj {
                if key == target {
                    if path.len() == 1 {
                        results.push(child.clone());
                    } else {
                        find_recursive(child, &path[1..], results);
                    }
                }
                // Always recurse into children
                find_recursive(child, path, results);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                find_recursive(item, path, results);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests;
