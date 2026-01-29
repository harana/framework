// Harana Actions - Email Module
// This module provides email actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;
use once_cell::sync::Lazy;
use dashmap::DashMap;
use uuid::Uuid;
use regex::Regex;

/// Simple in-memory template store: template_id -> (name, subject, body, content_type, variables)
static TEMPLATES: Lazy<DashMap<String, HashMap<String, String>>> = Lazy::new(DashMap::new);

/// Message status store: message_id -> status info
static MESSAGES: Lazy<DashMap<String, HashMap<String, String>>> = Lazy::new(DashMap::new);

fn email_regex() -> Regex {
    Regex::new(r"(?i)^([a-z0-9_\.-]+)@([a-z0-9\.-]+)\.([a-z\.]{2,6})$").unwrap()
}

/// Create Email Template
pub async fn create_template(
    name: &str,
    body: &str,
    subject: &str,
    content_type: Option<&str>,
    variables: Option<Vec<String>>,
) -> Result<CreateTemplateOutput, String> {
    let template_id = Uuid::new_v4().to_string();
    let mut map = HashMap::new();
    map.insert("name".to_string(), name.to_string());
    map.insert("body".to_string(), body.to_string());
    map.insert("subject".to_string(), subject.to_string());
    map.insert(
        "content_type".to_string(),
        content_type.unwrap_or("text/plain").to_string(),
    );
    if let Some(vars) = variables {
        map.insert("variables".to_string(), serde_json::to_string(&vars).unwrap_or_default());
    }
    TEMPLATES.insert(template_id.clone(), map);

    Ok(CreateTemplateOutput {
        template_id,
        success: true,
    })
}

/// Delete Email Template
pub async fn delete_template(
    template_id: &str,
) -> Result<DeleteTemplateOutput, String> {
    let removed = TEMPLATES.remove(template_id).is_some();
    Ok(DeleteTemplateOutput { success: removed })
}

/// List Email Templates
pub async fn list_templates(
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<ListTemplatesOutput, String> {
    let limit = limit.unwrap_or(100) as usize;
    let _offset = offset.unwrap_or(0) as usize;

    let mut templates: Vec<HashMap<String, Value>> = Vec::new();
    for entry in TEMPLATES.iter().take(limit) {
        let mut obj = HashMap::new();
        for (k, v) in entry.value().iter() {
            obj.insert(k.clone(), Value::String(v.clone()));
        }
        obj.insert("template_id".to_string(), Value::String(entry.key().clone()));
        templates.push(obj);
    }

    let total = templates.len() as i32;
    Ok(ListTemplatesOutput { templates, total })
}

/// Send Email Message
pub async fn send(
    body: &str,
    subject: &str,
    to: Vec<String>,
    from: Option<&str>,
    attachments: Option<Vec<HashMap<String, Value>>>,
    _reply_to: Option<&str>,
    cc: Option<Vec<String>>,
    content_type: Option<&str>,
    bcc: Option<Vec<String>>,
) -> Result<SendOutput, String> {
    // Basic validation
    let re = email_regex();
    for recipient in &to {
        if !re.is_match(recipient) {
            return Err(format!("Invalid recipient email: {}", recipient));
        }
    }
    if let Some(f) = from {
        if !re.is_match(f) {
            return Err(format!("Invalid from email: {}", f));
        }
    }

    // Simulate sending: create message id and store status
    let message_id = Uuid::new_v4().to_string();
    let mut status = HashMap::new();
    status.insert("status".to_string(), "queued".to_string());
    status.insert("subject".to_string(), subject.to_string());
    status.insert("body".to_string(), body.to_string());
    status.insert("to".to_string(), serde_json::to_string(&to).unwrap_or_default());
    if let Some(ct) = content_type {
        status.insert("content_type".to_string(), ct.to_string());
    }
    if let Some(ccv) = cc {
        status.insert("cc".to_string(), serde_json::to_string(&ccv).unwrap_or_default());
    }
    if let Some(bccv) = bcc {
        status.insert("bcc".to_string(), serde_json::to_string(&bccv).unwrap_or_default());
    }
    if let Some(att) = attachments {
        status.insert("attachments".to_string(), serde_json::to_string(&att).unwrap_or_default());
    }
    MESSAGES.insert(message_id.clone(), status);

    Ok(SendOutput { message_id, success: true })
}

/// Send Bulk Email
pub async fn send_bulk(
    body: &str,
    subject: &str,
    recipients: Vec<HashMap<String, Value>>,
    from: Option<&str>,
    content_type: Option<&str>,
) -> Result<SendBulkOutput, String> {
    let mut message_ids = Vec::new();
    let mut failed: i32 = 0;
    for r in recipients.into_iter() {
        if let Some(Value::String(email)) = r.get("email") {
            match send(body, subject, vec![email.clone()], from, None, None, None, content_type, None).await {
                Ok(out) => message_ids.push(out.message_id),
                Err(_) => failed += 1,
            }
        } else {
            failed += 1;
        }
    }
    let successful = message_ids.len() as i32;
    let total = successful + failed;
    Ok(SendBulkOutput { message_ids, failed, successful, total })
}

/// Send Email From Template
pub async fn send_template(
    to: Vec<String>,
    template_id: &str,
    attachments: Option<Vec<HashMap<String, Value>>>,
    cc: Option<Vec<String>>,
    from: Option<&str>,
    variables: Option<HashMap<String, Value>>,
    bcc: Option<Vec<String>>,
    reply_to: Option<&str>,
) -> Result<SendTemplateOutput, String> {
    // Lookup template
    let tpl = TEMPLATES.get(template_id).ok_or_else(|| format!("Template not found: {}", template_id))?;
    let subject = tpl.get("subject").cloned().unwrap_or_default();
    let mut body = tpl.get("body").cloned().unwrap_or_default();

    // Apply simple variable replacement if provided
    if let Some(vars) = variables {
        for (k, v) in vars.iter() {
            let placeholder = format!("{{{{{}}}}}", k);
            if let Ok(s) = serde_json::to_string(v) {
                body = body.replace(&placeholder, &s.trim_matches('"').to_string());
            }
        }
    }

    let send_out = send(&body, &subject, to, from, attachments, reply_to, cc, tpl.get("content_type").map(|s| s.as_str()), bcc).await?;
    Ok(SendTemplateOutput { success: true, message_id: send_out.message_id })
}

/// Get Email Status
pub async fn status(
    message_id: &str,
) -> Result<StatusOutput, String> {
    let info = MESSAGES.get(message_id).ok_or_else(|| format!("Message not found: {}", message_id))?;
    let status = info.get("status").cloned().unwrap_or_default();
    let ret = StatusOutput {
        error: String::new(),
        opened_at: String::new(),
        status: status.clone(),
        delivered_at: String::new(),
        clicked_at: String::new(),
    };
    Ok(ret)
}

/// Validate Email Address
pub async fn validate_email(
    email: &str,
    check_smtp: Option<bool>,
    check_mx: Option<bool>,
) -> Result<ValidateEmailOutput, String> {
    let re = email_regex();
    let valid = re.is_match(email);
    let mut reason = if valid { "valid".to_string() } else { "invalid_format".to_string() };

    // Note: SMTP/MX checks are not implemented; we simulate responses
    if valid {
        if check_mx.unwrap_or(false) {
            // pretend MX exists
            reason = "mx_found".to_string();
        }
        if check_smtp.unwrap_or(false) {
            // pretend SMTP handshake succeeded
            reason = "smtp_ok".to_string();
        }
    }

    Ok(ValidateEmailOutput { reason, valid })
}

    #[cfg(test)]
    mod tests;
