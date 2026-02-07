#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_parse_basic() {
        let csv_data = "name,age,city\nAlice,30,New York\nBob,25,Los Angeles";
        
        let result = parse(csv_data, None, None, None, None).await.unwrap();
        
        assert_eq!(result.rows.len(), 2);
        assert_eq!(result.rows[0].get("name").unwrap(), "Alice");
        assert_eq!(result.rows[0].get("age").unwrap(), "30");
        assert_eq!(result.rows[0].get("city").unwrap(), "New York");
        assert_eq!(result.rows[1].get("name").unwrap(), "Bob");
    }

    #[tokio::test]
    async fn test_parse_custom_delimiter() {
        let csv_data = "name;age;city\nAlice;30;New York";
        
        let result = parse(csv_data, Some(";"), None, None, None).await.unwrap();
        
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0].get("name").unwrap(), "Alice");
        assert_eq!(result.rows[0].get("age").unwrap(), "30");
    }

    #[tokio::test]
    async fn test_parse_no_headers() {
        let csv_data = "Alice,30,New York\nBob,25,Los Angeles";
        
        let result = parse(
            csv_data,
            None,
            Some(false),
            Some(vec!["name".to_string(), "age".to_string(), "city".to_string()]),
            None,
        )
        .await
        .unwrap();
        
        assert_eq!(result.rows.len(), 2);
        assert_eq!(result.rows[0].get("name").unwrap(), "Alice");
    }

    #[tokio::test]
    async fn test_parse_skip_empty_lines() {
        let csv_data = "name,age\nAlice,30\n\nBob,25";
        
        let result = parse(csv_data, None, None, None, Some(true)).await.unwrap();
        
        assert_eq!(result.rows.len(), 2);
    }

    #[tokio::test]
    async fn test_generate_basic() {
        let mut row1 = std::collections::HashMap::new();
        row1.insert("name".to_string(), "Alice".to_string());
        row1.insert("age".to_string(), "30".to_string());
        
        let mut row2 = std::collections::HashMap::new();
        row2.insert("name".to_string(), "Bob".to_string());
        row2.insert("age".to_string(), "25".to_string());
        
        let result = generate(
            vec![row1, row2],
            None,
            Some(vec!["name".to_string(), "age".to_string()]),
            Some(true),
        )
        .await
        .unwrap();
        
        assert!(result.csv.contains("name,age"));
        assert!(result.csv.contains("Alice,30"));
        assert!(result.csv.contains("Bob,25"));
    }

    #[tokio::test]
    async fn test_generate_custom_delimiter() {
        let mut row = std::collections::HashMap::new();
        row.insert("name".to_string(), "Alice".to_string());
        row.insert("age".to_string(), "30".to_string());
        
        let result = generate(
            vec![row],
            Some(";"),
            Some(vec!["name".to_string(), "age".to_string()]),
            Some(true),
        )
        .await
        .unwrap();
        
        assert!(result.csv.contains("name;age"));
        assert!(result.csv.contains("Alice;30"));
    }

    #[tokio::test]
    async fn test_generate_no_headers() {
        let mut row = std::collections::HashMap::new();
        row.insert("name".to_string(), "Alice".to_string());
        row.insert("age".to_string(), "30".to_string());
        
        let result = generate(
            vec![row],
            None,
            Some(vec!["name".to_string(), "age".to_string()]),
            Some(false),
        )
        .await
        .unwrap();
        
        assert!(!result.csv.contains("name,age\n"));
        assert!(result.csv.contains("Alice,30"));
    }

    #[tokio::test]
    async fn test_transform_uppercase() {
        let csv_data = "name,city\nalice,new york\nbob,los angeles";
        
        let ops = vec![CsvTransformOperation {
            operation: "uppercase".to_string(),
            column: Some("name".to_string()),
            parameters: None,
        }];
        
        let result = transform(csv_data, ops, None).await.unwrap();
        
        assert!(result.csv.contains("ALICE"));
        assert!(result.csv.contains("BOB"));
    }

    #[tokio::test]
    async fn test_transform_lowercase() {
        let csv_data = "name,city\nALICE,NEW YORK";
        
        let ops = vec![CsvTransformOperation {
            operation: "lowercase".to_string(),
            column: Some("name".to_string()),
            parameters: None,
        }];
        
        let result = transform(csv_data, ops, None).await.unwrap();
        
        assert!(result.csv.contains("alice"));
    }

    #[tokio::test]
    async fn test_transform_trim() {
        let csv_data = "name,city\n  alice  ,  new york  ";
        
        let ops = vec![CsvTransformOperation {
            operation: "trim".to_string(),
            column: None, // Trim all columns
            parameters: None,
        }];
        
        let result = transform(csv_data, ops, None).await.unwrap();
        
        // Parse the result to verify trimming worked
        let parsed = parse(&result.csv, None, None, None, None).await.unwrap();
        assert_eq!(parsed.rows[0].get("name").unwrap(), "alice");
        assert_eq!(parsed.rows[0].get("city").unwrap(), "new york");
    }

    #[tokio::test]
    async fn test_transform_remove_column() {
        let csv_data = "name,age,city\nalice,30,new york";
        
        let ops = vec![CsvTransformOperation {
            operation: "remove".to_string(),
            column: Some("age".to_string()),
            parameters: None,
        }];
        
        let result = transform(csv_data, ops, None).await.unwrap();
        
        assert!(!result.csv.contains("30"));
    }

    #[tokio::test]
    async fn test_transform_replace() {
        let csv_data = "name,city\nalice,new york";
        
        let mut params = std::collections::HashMap::new();
        params.insert("pattern".to_string(), "new".to_string());
        params.insert("replacement".to_string(), "old".to_string());
        
        let ops = vec![CsvTransformOperation {
            operation: "replace".to_string(),
            column: Some("city".to_string()),
            parameters: Some(params),
        }];
        
        let result = transform(csv_data, ops, None).await.unwrap();
        
        assert!(result.csv.contains("old york"));
    }

    #[tokio::test]
    async fn test_transform_filter() {
        let csv_data = "name,age\nalice,30\nbob,25\ncharlie,30";
        
        let mut params = std::collections::HashMap::new();
        params.insert("value".to_string(), "30".to_string());
        params.insert("operator".to_string(), "equals".to_string());
        
        let ops = vec![CsvTransformOperation {
            operation: "filter".to_string(),
            column: Some("age".to_string()),
            parameters: Some(params),
        }];
        
        let result = transform(csv_data, ops, None).await.unwrap();
        
        // Should only have alice and charlie
        let parsed = parse(&result.csv, None, None, None, None).await.unwrap();
        assert_eq!(parsed.rows.len(), 2);
        assert!(!result.csv.contains("bob"));
    }

    #[tokio::test]
    async fn test_validate_required() {
        let csv_data = "name,age\nalice,30\nbob,";
        
        let schema = CsvSchema {
            columns: vec![
                CsvColumnSchema {
                    name: "name".to_string(),
                    column_type: Some("string".to_string()),
                    required: Some(true),
                    pattern: None,
                },
                CsvColumnSchema {
                    name: "age".to_string(),
                    column_type: Some("integer".to_string()),
                    required: Some(true),
                    pattern: None,
                },
            ],
            allow_extra_columns: Some(true),
        };
        
        let result = validate(csv_data, schema, None).await.unwrap();
        
        assert!(!result.valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].row, 2);
        assert_eq!(result.errors[0].column, "age");
    }

    #[tokio::test]
    async fn test_validate_type_integer() {
        let csv_data = "name,age\nalice,thirty";
        
        let schema = CsvSchema {
            columns: vec![
                CsvColumnSchema {
                    name: "name".to_string(),
                    column_type: Some("string".to_string()),
                    required: None,
                    pattern: None,
                },
                CsvColumnSchema {
                    name: "age".to_string(),
                    column_type: Some("integer".to_string()),
                    required: None,
                    pattern: None,
                },
            ],
            allow_extra_columns: Some(true),
        };
        
        let result = validate(csv_data, schema, None).await.unwrap();
        
        assert!(!result.valid);
        assert!(result.errors[0].message.contains("not a valid integer"));
    }

    #[tokio::test]
    async fn test_validate_pattern() {
        let csv_data = "email\ninvalid-email\nvalid@example.com";
        
        let schema = CsvSchema {
            columns: vec![CsvColumnSchema {
                name: "email".to_string(),
                column_type: None,
                required: None,
                pattern: Some(r"^[\w\.-]+@[\w\.-]+\.\w+$".to_string()),
            }],
            allow_extra_columns: Some(true),
        };
        
        let result = validate(csv_data, schema, None).await.unwrap();
        
        assert!(!result.valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].row, 1);
    }

    #[tokio::test]
    async fn test_validate_extra_columns_not_allowed() {
        let csv_data = "name,age,extra\nalice,30,data";
        
        let schema = CsvSchema {
            columns: vec![
                CsvColumnSchema {
                    name: "name".to_string(),
                    column_type: None,
                    required: None,
                    pattern: None,
                },
                CsvColumnSchema {
                    name: "age".to_string(),
                    column_type: None,
                    required: None,
                    pattern: None,
                },
            ],
            allow_extra_columns: Some(false),
        };
        
        let result = validate(csv_data, schema, None).await.unwrap();
        
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.column == "extra"));
    }

    #[tokio::test]
    async fn test_validate_valid_csv() {
        let csv_data = "name,age\nalice,30\nbob,25";
        
        let schema = CsvSchema {
            columns: vec![
                CsvColumnSchema {
                    name: "name".to_string(),
                    column_type: Some("string".to_string()),
                    required: Some(true),
                    pattern: None,
                },
                CsvColumnSchema {
                    name: "age".to_string(),
                    column_type: Some("integer".to_string()),
                    required: Some(true),
                    pattern: None,
                },
            ],
            allow_extra_columns: Some(true),
        };
        
        let result = validate(csv_data, schema, None).await.unwrap();
        
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_roundtrip_parse_generate() {
        let original_csv = "name,age\nAlice,30\nBob,25\n";
        
        // Parse the CSV
        let parsed = parse(original_csv, None, None, None, None).await.unwrap();
        
        // Generate it back
        let generated = generate(
            parsed.rows,
            None,
            Some(vec!["name".to_string(), "age".to_string()]),
            Some(true),
        )
        .await
        .unwrap();
        
        // Parse again and compare
        let reparsed = parse(&generated.csv, None, None, None, None)
            .await
            .unwrap();
        
        assert_eq!(reparsed.rows.len(), 2);
        assert_eq!(reparsed.rows[0].get("name").unwrap(), "Alice");
        assert_eq!(reparsed.rows[1].get("name").unwrap(), "Bob");
    }
}
