// Harana Actions - Form Module
// This module provides form actions and functionality.

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;
use parking_lot::RwLock;
use once_cell::sync::Lazy;
use chrono::Utc;

// In-memory storage for forms and submissions
static FORMS: Lazy<RwLock<HashMap<String, FormDefinition>>> = Lazy::new(|| RwLock::new(HashMap::new()));
static SUBMISSIONS: Lazy<RwLock<HashMap<String, FormSubmission>>> = Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Debug, Clone)]
struct FormDefinition {
    form_id: String,
    fields: Vec<FormField>,
    metadata: FormMetadata,
    validation_rules: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
struct FormField {
    name: String,
    field_type: String,
    label: String,
    required: bool,
    options: Vec<String>,
}

#[derive(Debug, Clone)]
struct FormMetadata {
    title: String,
    description: String,
    version: String,
}

#[derive(Debug, Clone)]
struct FormSubmission {
    submission_id: String,
    form_id: String,
    data: HashMap<String, Value>,
    status: String,
    submitted_at: String,
    updated_at: String,
}

/// Validate Form Data
pub async fn validate(
    form_id: &str,
    data: HashMap<String, Value>,
    strict: Option<bool>,
) -> Result<ValidateOutput, String> {
    let strict = strict.unwrap_or(true);
    let forms = FORMS.read();
    
    let form = forms.get(form_id)
        .ok_or_else(|| format!("Form '{}' not found", form_id))?;
    
    let mut errors = Vec::new();
    
    // Validate required fields
    for field in &form.fields {
        if field.required && !data.contains_key(&field.name) {
            errors.push(HashMap::from([
                ("field".to_string(), Value::String(field.name.clone())),
                ("message".to_string(), Value::String(format!("Field '{}' is required", field.label))),
                ("code".to_string(), Value::String("required".to_string())),
            ]));
        }
    }
    
    // Validate field types
    for (field_name, value) in &data {
        if let Some(field) = form.fields.iter().find(|f| &f.name == field_name) {
            let valid = match field.field_type.as_str() {
                "string" => value.is_string(),
                "number" => value.is_number(),
                "boolean" => value.is_boolean(),
                "array" => value.is_array(),
                "object" => value.is_object(),
                _ => true,
            };
            
            if !valid {
                errors.push(HashMap::from([
                    ("field".to_string(), Value::String(field_name.clone())),
                    ("message".to_string(), Value::String(format!("Field '{}' must be of type {}", field.label, field.field_type))),
                    ("code".to_string(), Value::String("invalid_type".to_string())),
                ]));
            }
            
            // Validate options (if specified)
            if !field.options.is_empty() && value.is_string() {
                let str_val = value.as_str().unwrap();
                if !field.options.contains(&str_val.to_string()) {
                    errors.push(HashMap::from([
                        ("field".to_string(), Value::String(field_name.clone())),
                        ("message".to_string(), Value::String(format!("Field '{}' must be one of: {}", field.label, field.options.join(", ")))),
                        ("code".to_string(), Value::String("invalid_option".to_string())),
                    ]));
                }
            }
        } else if strict {
            errors.push(HashMap::from([
                ("field".to_string(), Value::String(field_name.clone())),
                ("message".to_string(), Value::String(format!("Unknown field '{}'", field_name))),
                ("code".to_string(), Value::String("unknown_field".to_string())),
            ]));
        }
    }
    
    Ok(ValidateOutput {
        valid: errors.is_empty(),
        errors,
    })
}

/// Submit Form Data
pub async fn submit(
    data: HashMap<String, Value>,
    form_id: &str,
    validate_flag: Option<bool>,
) -> Result<SubmitOutput, String> {
    let validate_flag = validate_flag.unwrap_or(true);
    
    // Validate if requested
    let validation_result = if validate_flag {
        validate(form_id, data.clone(), Some(true)).await?
    } else {
        ValidateOutput {
            valid: true,
            errors: Vec::new(),
        }
    };
    
    if !validation_result.valid {
        return Ok(SubmitOutput {
            success: false,
            submission_id: String::new(),
            errors: validation_result.errors,
        });
    }
    
    // Create submission
    let submission_id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let submission = FormSubmission {
        submission_id: submission_id.clone(),
        form_id: form_id.to_string(),
        data,
        status: "submitted".to_string(),
        submitted_at: now.clone(),
        updated_at: now,
    };
    
    SUBMISSIONS.write().insert(submission_id.clone(), submission);
    
    Ok(SubmitOutput {
        success: true,
        submission_id,
        errors: Vec::new(),
    })
}

/// Get Form Definition
pub async fn get(
    form_id: &str,
) -> Result<GetOutput, String> {
    let forms = FORMS.read();
    
    let form = forms.get(form_id)
        .ok_or_else(|| format!("Form '{}' not found", form_id))?;
    
    let fields: Vec<HashMap<String, Value>> = form.fields.iter().map(|f| {
        HashMap::from([
            ("name".to_string(), Value::String(f.name.clone())),
            ("type".to_string(), Value::String(f.field_type.clone())),
            ("label".to_string(), Value::String(f.label.clone())),
            ("required".to_string(), Value::Bool(f.required)),
            ("options".to_string(), Value::Array(f.options.iter().map(|o| Value::String(o.clone())).collect())),
        ])
    }).collect();
    
    let metadata = HashMap::from([
        ("title".to_string(), Value::String(form.metadata.title.clone())),
        ("description".to_string(), Value::String(form.metadata.description.clone())),
        ("version".to_string(), Value::String(form.metadata.version.clone())),
    ]);
    
    Ok(GetOutput {
        fields,
        metadata,
        validation_rules: form.validation_rules.clone(),
    })
}

/// Get Form Submission
pub async fn get_submission(
    submission_id: &str,
) -> Result<GetSubmissionOutput, String> {
    let submissions = SUBMISSIONS.read();
    
    let submission = submissions.get(submission_id)
        .ok_or_else(|| format!("Submission '{}' not found", submission_id))?;
    
    Ok(GetSubmissionOutput {
        data: submission.data.clone(),
        form_id: submission.form_id.clone(),
        status: submission.status.clone(),
        submitted_at: submission.submitted_at.clone(),
    })
}

/// List Form Submissions
pub async fn list_submissions(
    form_id: &str,
    offset: Option<i32>,
    end_date: Option<&str>,
    start_date: Option<&str>,
    limit: Option<i32>,
    status: Option<&str>,
) -> Result<ListSubmissionsOutput, String> {
    let offset = offset.unwrap_or(0).max(0) as usize;
    let limit = limit.unwrap_or(100).max(1) as usize;
    
    let submissions = SUBMISSIONS.read();
    
    // Filter submissions
    let mut filtered: Vec<_> = submissions.values()
        .filter(|s| s.form_id == form_id)
        .filter(|s| status.map_or(true, |st| s.status == st))
        .filter(|s| start_date.map_or(true, |sd| s.submitted_at >= sd.to_string()))
        .filter(|s| end_date.map_or(true, |ed| s.submitted_at <= ed.to_string()))
        .collect();
    
    // Sort by submitted_at descending
    filtered.sort_by(|a, b| b.submitted_at.cmp(&a.submitted_at));
    
    let total = filtered.len() as i32;
    
    // Paginate
    let paginated: Vec<HashMap<String, Value>> = filtered.iter()
        .skip(offset)
        .take(limit)
        .map(|s| {
            HashMap::from([
                ("submission_id".to_string(), Value::String(s.submission_id.clone())),
                ("form_id".to_string(), Value::String(s.form_id.clone())),
                ("status".to_string(), Value::String(s.status.clone())),
                ("submitted_at".to_string(), Value::String(s.submitted_at.clone())),
            ])
        })
        .collect();
    
    Ok(ListSubmissionsOutput {
        submissions: paginated,
        total,
    })
}

/// Update Form Submission
pub async fn update_submission(
    submission_id: &str,
    data: HashMap<String, Value>,
    validate_flag: Option<bool>,
) -> Result<UpdateSubmissionOutput, String> {
    let validate_flag = validate_flag.unwrap_or(true);
    
    let mut submissions = SUBMISSIONS.write();
    
    let submission = submissions.get_mut(submission_id)
        .ok_or_else(|| format!("Submission '{}' not found", submission_id))?;
    
    // Validate if requested
    let validation_result = if validate_flag {
        validate(&submission.form_id, data.clone(), Some(true)).await?
    } else {
        ValidateOutput {
            valid: true,
            errors: Vec::new(),
        }
    };
    
    if !validation_result.valid {
        return Ok(UpdateSubmissionOutput {
            success: false,
            errors: validation_result.errors,
        });
    }
    
    // Update submission
    submission.data = data;
    submission.updated_at = Utc::now().to_rfc3339();
    
    Ok(UpdateSubmissionOutput {
        success: true,
        errors: Vec::new(),
    })
}

/// Delete Form Submission
pub async fn delete_submission(
    submission_id: &str,
) -> Result<DeleteSubmissionOutput, String> {
    let mut submissions = SUBMISSIONS.write();
    
    if submissions.remove(submission_id).is_some() {
        Ok(DeleteSubmissionOutput { success: true })
    } else {
        Err(format!("Submission '{}' not found", submission_id))
    }
}

/// Export Form Submissions
pub async fn export_submissions(
    form_id: &str,
    format: Option<&str>,
    end_date: Option<&str>,
    start_date: Option<&str>,
) -> Result<ExportSubmissionsOutput, String> {
    let format = format.unwrap_or("csv");
    
    let submissions = SUBMISSIONS.read();
    
    // Filter submissions
    let filtered: Vec<_> = submissions.values()
        .filter(|s| s.form_id == form_id)
        .filter(|s| start_date.map_or(true, |sd| s.submitted_at >= sd.to_string()))
        .filter(|s| end_date.map_or(true, |ed| s.submitted_at <= ed.to_string()))
        .collect();
    
    let count = filtered.len() as i32;
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("form_{}_{}.{}", form_id, timestamp, format);
    
    let content = match format {
        "json" => {
            let json: Vec<_> = filtered.iter().map(|s| {
                serde_json::json!({
                    "submission_id": s.submission_id,
                    "form_id": s.form_id,
                    "data": s.data,
                    "status": s.status,
                    "submitted_at": s.submitted_at,
                    "updated_at": s.updated_at,
                })
            }).collect();
            serde_json::to_vec_pretty(&json)
                .map_err(|e| format!("Failed to serialize JSON: {}", e))?
        }
        "csv" => {
            let mut wtr = csv::Writer::from_writer(vec![]);
            
            // Write header
            wtr.write_record(&["submission_id", "form_id", "status", "submitted_at", "updated_at", "data"])
                .map_err(|e| format!("Failed to write CSV header: {}", e))?;
            
            // Write rows
            for submission in &filtered {
                let data_json = serde_json::to_string(&submission.data)
                    .map_err(|e| format!("Failed to serialize data: {}", e))?;
                wtr.write_record(&[
                    &submission.submission_id,
                    &submission.form_id,
                    &submission.status,
                    &submission.submitted_at,
                    &submission.updated_at,
                    &data_json,
                ])
                .map_err(|e| format!("Failed to write CSV record: {}", e))?;
            }
            
            wtr.into_inner()
                .map_err(|e| format!("Failed to finalize CSV: {}", e))?
        }
        _ => return Err(format!("Unsupported export format: {}", format)),
    };
    
    Ok(ExportSubmissionsOutput {
        content,
        count,
        filename,
    })
}

// Helper function to register a form (for testing)
#[doc(hidden)]
pub async fn _register_form(
    form_id: String,
    fields: Vec<(String, String, String, bool, Vec<String>)>,
    title: String,
    description: String,
) {
    let form_fields: Vec<FormField> = fields.into_iter().map(|(name, field_type, label, required, options)| {
        FormField {
            name,
            field_type,
            label,
            required,
            options,
        }
    }).collect();
    
    let form = FormDefinition {
        form_id: form_id.clone(),
        fields: form_fields,
        metadata: FormMetadata {
            title,
            description,
            version: "1.0".to_string(),
        },
        validation_rules: HashMap::new(),
    };
    
    FORMS.write().insert(form_id, form);
}

// Helper function to clear all data (for testing)
#[doc(hidden)]
pub async fn _clear_all() {
    FORMS.write().clear();
    SUBMISSIONS.write().clear();
}

#[cfg(test)]
mod tests;
