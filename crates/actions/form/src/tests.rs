#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;

    // Use a unique form ID for each test to avoid conflicts
    fn get_unique_form_id() -> String {
        format!("form_{}", uuid::Uuid::new_v4())
    }

    async fn setup_test_form(form_id: &str) {
        _register_form(
            form_id.to_string(),
            vec![
                ("name".to_string(), "string".to_string(), "Full Name".to_string(), true, vec![]),
                ("email".to_string(), "string".to_string(), "Email Address".to_string(), true, vec![]),
                ("message".to_string(), "string".to_string(), "Message".to_string(), true, vec![]),
                ("priority".to_string(), "string".to_string(), "Priority".to_string(), false, vec!["low".to_string(), "medium".to_string(), "high".to_string()]),
            ],
            "Contact Form".to_string(),
            "A simple contact form".to_string(),
        ).await;
    }

    #[tokio::test]
    async fn test_validate_valid_data() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        let data = HashMap::from([
            ("name".to_string(), json!("John Doe")),
            ("email".to_string(), json!("john@example.com")),
            ("message".to_string(), json!("Hello, world!")),
        ]);
        
        let result = validate(&form_id, data, None).await.unwrap();
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_validate_missing_required_field() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        let data = HashMap::from([
            ("name".to_string(), json!("John Doe")),
            ("email".to_string(), json!("john@example.com")),
            // Missing "message" field
        ]);
        
        let result = validate(&form_id, data, None).await.unwrap();
        assert!(!result.valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].get("field").unwrap(), &json!("message"));
        assert_eq!(result.errors[0].get("code").unwrap(), &json!("required"));
    }

    #[tokio::test]
    async fn test_validate_invalid_option() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        let data = HashMap::from([
            ("name".to_string(), json!("John Doe")),
            ("email".to_string(), json!("john@example.com")),
            ("message".to_string(), json!("Hello")),
            ("priority".to_string(), json!("urgent")), // Invalid option
        ]);
        
        let result = validate(&form_id, data, None).await.unwrap();
        assert!(!result.valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].get("field").unwrap(), &json!("priority"));
        assert_eq!(result.errors[0].get("code").unwrap(), &json!("invalid_option"));
    }

    #[tokio::test]
    async fn test_validate_strict_mode() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        let data = HashMap::from([
            ("name".to_string(), json!("John Doe")),
            ("email".to_string(), json!("john@example.com")),
            ("message".to_string(), json!("Hello")),
            ("unknown_field".to_string(), json!("value")),
        ]);
        
        let result = validate(&form_id, data.clone(), Some(true)).await.unwrap();
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.get("code").unwrap() == &json!("unknown_field")));
        
        // Non-strict mode should ignore unknown fields
        let result = validate(&form_id, data, Some(false)).await.unwrap();
        assert!(result.valid);
    }

    #[tokio::test]
    async fn test_submit_valid_data() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        let data = HashMap::from([
            ("name".to_string(), json!("John Doe")),
            ("email".to_string(), json!("john@example.com")),
            ("message".to_string(), json!("Hello, world!")),
        ]);
        
        let result = submit(data, &form_id, None).await.unwrap();
        assert!(result.success);
        assert!(!result.submission_id.is_empty());
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_submit_invalid_data() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        let data = HashMap::from([
            ("name".to_string(), json!("John Doe")),
            // Missing required fields
        ]);
        
        let result = submit(data, &form_id, Some(true)).await.unwrap();
        assert!(!result.success);
        assert!(result.submission_id.is_empty());
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_submit_without_validation() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        let data = HashMap::from([
            ("name".to_string(), json!("John Doe")),
            // Missing required fields, but validation is disabled
        ]);
        
        let result = submit(data, &form_id, Some(false)).await.unwrap();
        assert!(result.success);
        assert!(!result.submission_id.is_empty());
    }

    #[tokio::test]
    async fn test_get_form_definition() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        let result = get(&form_id).await.unwrap();
        assert_eq!(result.fields.len(), 4);
        assert_eq!(result.metadata.get("title").unwrap(), &json!("Contact Form"));
        assert_eq!(result.metadata.get("version").unwrap(), &json!("1.0"));
    }

    #[tokio::test]
    async fn test_get_nonexistent_form() {
        let result = get("nonexistent_form").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_submission() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        let data = HashMap::from([
            ("name".to_string(), json!("John Doe")),
            ("email".to_string(), json!("john@example.com")),
            ("message".to_string(), json!("Hello, world!")),
        ]);
        
        let submit_result = submit(data.clone(), &form_id, None).await.unwrap();
        
        let result = get_submission(&submit_result.submission_id).await.unwrap();
        assert_eq!(result.form_id, form_id);
        assert_eq!(result.status, "submitted");
        assert_eq!(result.data.get("name").unwrap(), &json!("John Doe"));
    }

    #[tokio::test]
    async fn test_list_submissions() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        // Submit multiple forms
        for i in 0..5 {
            let data = HashMap::from([
                ("name".to_string(), json!(format!("User {}", i))),
                ("email".to_string(), json!(format!("user{}@example.com", i))),
                ("message".to_string(), json!("Test message")),
            ]);
            submit(data, &form_id, None).await.unwrap();
        }
        
        let result = list_submissions(&form_id, None, None, None, None, None).await.unwrap();
        assert_eq!(result.total, 5);
        assert_eq!(result.submissions.len(), 5);
    }

    #[tokio::test]
    async fn test_list_submissions_pagination() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        // Submit multiple forms
        for i in 0..10 {
            let data = HashMap::from([
                ("name".to_string(), json!(format!("User {}", i))),
                ("email".to_string(), json!(format!("user{}@example.com", i))),
                ("message".to_string(), json!("Test message")),
            ]);
            submit(data, &form_id, None).await.unwrap();
        }
        
        let result = list_submissions(&form_id, Some(0), None, None, Some(5), None).await.unwrap();
        assert_eq!(result.total, 10);
        assert_eq!(result.submissions.len(), 5);
        
        let result = list_submissions(&form_id, Some(5), None, None, Some(5), None).await.unwrap();
        assert_eq!(result.total, 10);
        assert_eq!(result.submissions.len(), 5);
    }

    #[tokio::test]
    async fn test_update_submission() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        let data = HashMap::from([
            ("name".to_string(), json!("John Doe")),
            ("email".to_string(), json!("john@example.com")),
            ("message".to_string(), json!("Hello, world!")),
        ]);
        
        let submit_result = submit(data, &form_id, None).await.unwrap();
        
        let updated_data = HashMap::from([
            ("name".to_string(), json!("Jane Doe")),
            ("email".to_string(), json!("jane@example.com")),
            ("message".to_string(), json!("Updated message")),
        ]);
        
        let result = update_submission(&submit_result.submission_id, updated_data.clone(), None).await.unwrap();
        assert!(result.success);
        assert!(result.errors.is_empty());
        
        let submission = get_submission(&submit_result.submission_id).await.unwrap();
        assert_eq!(submission.data.get("name").unwrap(), &json!("Jane Doe"));
    }

    #[tokio::test]
    async fn test_update_submission_invalid_data() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        let data = HashMap::from([
            ("name".to_string(), json!("John Doe")),
            ("email".to_string(), json!("john@example.com")),
            ("message".to_string(), json!("Hello, world!")),
        ]);
        
        let submit_result = submit(data, &form_id, None).await.unwrap();
        
        let invalid_data = HashMap::from([
            ("name".to_string(), json!("Jane Doe")),
            // Missing required fields
        ]);
        
        let result = update_submission(&submit_result.submission_id, invalid_data, Some(true)).await.unwrap();
        assert!(!result.success);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_delete_submission() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        let data = HashMap::from([
            ("name".to_string(), json!("John Doe")),
            ("email".to_string(), json!("john@example.com")),
            ("message".to_string(), json!("Hello, world!")),
        ]);
        
        let submit_result = submit(data, &form_id, None).await.unwrap();
        
        let result = delete_submission(&submit_result.submission_id).await.unwrap();
        assert!(result.success);
        
        let get_result = get_submission(&submit_result.submission_id).await;
        assert!(get_result.is_err());
    }

    #[tokio::test]
    async fn test_delete_nonexistent_submission() {
        let result = delete_submission("nonexistent_id").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_export_submissions_csv() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        for i in 0..3 {
            let data = HashMap::from([
                ("name".to_string(), json!(format!("User {}", i))),
                ("email".to_string(), json!(format!("user{}@example.com", i))),
                ("message".to_string(), json!("Test message")),
            ]);
            submit(data, &form_id, None).await.unwrap();
        }
        
        let result = export_submissions(&form_id, Some("csv"), None, None).await.unwrap();
        assert_eq!(result.count, 3);
        assert!(result.filename.starts_with(&format!("form_{}_", form_id)));
        assert!(result.filename.ends_with(".csv"));
        assert!(!result.content.is_empty());
        
        let csv_str = String::from_utf8(result.content).unwrap();
        assert!(csv_str.contains("submission_id"));
        assert!(csv_str.contains("User 0"));
    }

    #[tokio::test]
    async fn test_export_submissions_json() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        for i in 0..3 {
            let data = HashMap::from([
                ("name".to_string(), json!(format!("User {}", i))),
                ("email".to_string(), json!(format!("user{}@example.com", i))),
                ("message".to_string(), json!("Test message")),
            ]);
            submit(data, &form_id, None).await.unwrap();
        }
        
        let result = export_submissions(&form_id, Some("json"), None, None).await.unwrap();
        assert_eq!(result.count, 3);
        assert!(result.filename.starts_with(&format!("form_{}_", form_id)));
        assert!(result.filename.ends_with(".json"));
        
        let json: serde_json::Value = serde_json::from_slice(&result.content).unwrap();
        assert!(json.is_array());
        assert_eq!(json.as_array().unwrap().len(), 3);
    }

    #[tokio::test]
    async fn test_export_submissions_invalid_format() {
        let form_id = get_unique_form_id();
        setup_test_form(&form_id).await;
        
        let result = export_submissions(&form_id, Some("xml"), None, None).await;
        assert!(result.is_err());
    }
}
