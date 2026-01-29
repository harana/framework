#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_render_template() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), json!("World"));
        
        let result = render_template(
            "Hello, {{name}}!",
            None,
            Some(data),
            None,
        ).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.rendered, "Hello, World!");
    }

    #[tokio::test]
    async fn test_render_template_complex() {
        let mut data = HashMap::new();
        data.insert("items".to_string(), json!(["apple", "banana", "cherry"]));
        
        let result = render_template(
            "{{#each items}}{{this}},{{/each}}",
            None,
            Some(data),
            None,
        ).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.rendered, "apple,banana,cherry,");
    }

    #[tokio::test]
    async fn test_json_response() {
        let data = json!({"message": "Hello", "count": 42});
        
        let result = json(data.clone(), None, None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 200);
        assert_eq!(result.response.content_type, "application/json");
    }

    #[tokio::test]
    async fn test_json_response_pretty() {
        let data = json!({"key": "value"});
        
        let result = json(data, Some(true), Some(201)).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 201);
    }

    #[tokio::test]
    async fn test_text_response() {
        let result = text("Hello, World!", None, None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 200);
        assert_eq!(result.response.content_type, "text/plain");
        assert_eq!(result.response.body, Some(Value::String("Hello, World!".to_string())));
    }

    #[tokio::test]
    async fn test_text_response_custom_content_type() {
        let result = text("data", Some("text/csv"), Some(200)).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.content_type, "text/csv");
    }

    #[tokio::test]
    async fn test_html_response() {
        let html_content = "<html><body><h1>Hello</h1></body></html>";
        let result = html(html_content, None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 200);
        assert_eq!(result.response.content_type, "text/html");
    }

    #[tokio::test]
    async fn test_xml_response() {
        let xml_content = "<?xml version=\"1.0\"?><root><item>test</item></root>";
        let result = xml(xml_content, None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 200);
        assert_eq!(result.response.content_type, "application/xml");
    }

    #[tokio::test]
    async fn test_binary_response() {
        let data = vec![0x00, 0x01, 0x02, 0x03];
        let result = binary("application/octet-stream", &data, Some("test.bin"), None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 200);
        assert!(result.response.headers.contains_key("Content-Disposition"));
    }

    #[tokio::test]
    async fn test_sse_event() {
        let data = json!({"message": "hello"});
        let result = sse_event(data, Some("message"), Some("1"), None).await.unwrap();
        
        assert!(result.success);
        assert!(result.formatted.contains("id: 1"));
        assert!(result.formatted.contains("event: message"));
        assert!(result.formatted.contains("data:"));
    }

    #[tokio::test]
    async fn test_sse_event_simple() {
        let data = json!("simple text");
        let result = sse_event(data, None, None, Some(5000)).await.unwrap();
        
        assert!(result.success);
        assert!(result.formatted.contains("retry: 5000"));
        assert!(result.formatted.contains("data: simple text"));
    }

    #[tokio::test]
    async fn test_sse_stream() {
        let events = vec![
            SseEvent {
                data: json!("event1"),
                event: Some("type1".to_string()),
                id: Some("1".to_string()),
                retry: None,
            },
            SseEvent {
                data: json!("event2"),
                event: Some("type2".to_string()),
                id: Some("2".to_string()),
                retry: None,
            },
        ];
        
        let result = sse_stream(events, None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.events.len(), 2);
    }

    #[tokio::test]
    async fn test_not_found() {
        let result = not_found(None, None, None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 404);
        
        let body = result.response.body.unwrap();
        assert_eq!(body["message"], "Not Found");
    }

    #[tokio::test]
    async fn test_not_found_custom_message() {
        let result = not_found(None, Some(true), Some("Resource not found")).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 404);
        
        let body = result.response.body.unwrap();
        assert_eq!(body["message"], "Resource not found");
    }

    #[tokio::test]
    async fn test_bad_request() {
        let errors = vec!["Field required".to_string(), "Invalid format".to_string()];
        let result = bad_request(None, Some(errors), Some("Validation failed")).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 400);
        
        let body = result.response.body.unwrap();
        assert_eq!(body["message"], "Validation failed");
        assert!(body["errors"].is_array());
    }

    #[tokio::test]
    async fn test_unauthorized() {
        let result = unauthorized(None, None, Some("Bearer")).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 401);
        assert_eq!(result.response.headers.get("WWW-Authenticate"), Some(&"Bearer".to_string()));
    }

    #[tokio::test]
    async fn test_forbidden() {
        let result = forbidden(None, Some("Access denied")).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 403);
        
        let body = result.response.body.unwrap();
        assert_eq!(body["message"], "Access denied");
    }

    #[tokio::test]
    async fn test_internal_error() {
        let result = internal_error(None, Some("Database connection failed"), Some(true), None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 500);
        
        let body = result.response.body.unwrap();
        assert_eq!(body["details"], "Database connection failed");
    }

    #[tokio::test]
    async fn test_internal_error_no_details() {
        let result = internal_error(None, Some("secret error"), Some(false), None).await.unwrap();
        
        assert!(result.success);
        let body = result.response.body.unwrap();
        assert!(body.get("details").is_none());
    }

    #[tokio::test]
    async fn test_bad_gateway() {
        let result = bad_gateway(None, Some("Upstream server error")).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 502);
    }

    #[tokio::test]
    async fn test_service_unavailable() {
        let result = service_unavailable(None, None, Some(120)).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 503);
        assert_eq!(result.response.headers.get("Retry-After"), Some(&"120".to_string()));
    }

    #[tokio::test]
    async fn test_custom_error() {
        let details = json!({"field": "email", "reason": "invalid format"});
        let result = error(Some(details), Some("VALIDATION_ERROR"), "Invalid input", 422).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 422);
        
        let body = result.response.body.unwrap();
        assert_eq!(body["error_code"], "VALIDATION_ERROR");
        assert_eq!(body["message"], "Invalid input");
    }

    #[tokio::test]
    async fn test_set_cookie() {
        let options = CookieOptions {
            path: Some("/app".to_string()),
            domain: Some("example.com".to_string()),
            max_age: Some(3600),
            http_only: true,
            secure: true,
            same_site: Some("Strict".to_string()),
            expires: None,
        };
        
        let result = set_cookie("session", "abc123", Some(options)).await.unwrap();
        
        assert!(result.success);
        assert!(result.header_value.contains("session=abc123"));
        assert!(result.header_value.contains("Path=/app"));
        assert!(result.header_value.contains("Domain=example.com"));
        assert!(result.header_value.contains("Max-Age=3600"));
        assert!(result.header_value.contains("HttpOnly"));
        assert!(result.header_value.contains("Secure"));
        assert!(result.header_value.contains("SameSite=Strict"));
    }

    #[tokio::test]
    async fn test_set_cookie_default() {
        let result = set_cookie("token", "xyz", None).await.unwrap();
        
        assert!(result.success);
        assert!(result.header_value.contains("token=xyz"));
        assert!(result.header_value.contains("Path=/"));
        assert!(result.header_value.contains("HttpOnly"));
    }

    #[tokio::test]
    async fn test_delete_cookie() {
        let result = delete_cookie("session", Some("example.com"), None).await.unwrap();
        
        assert!(result.success);
        assert!(result.header_value.contains("session="));
        assert!(result.header_value.contains("Max-Age=0"));
        assert!(result.header_value.contains("Expires=Thu, 01 Jan 1970"));
        assert!(result.header_value.contains("Domain=example.com"));
    }

    #[tokio::test]
    async fn test_empty_response() {
        let result = empty(None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 204);
        assert!(result.response.body.is_none());
    }

    #[tokio::test]
    async fn test_empty_response_custom_status() {
        let result = empty(Some(202)).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 202);
    }

    #[tokio::test]
    async fn test_redirect() {
        let result = redirect("https://example.com", None, None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 302);
        assert_eq!(result.response.headers.get("Location"), Some(&"https://example.com".to_string()));
    }

    #[tokio::test]
    async fn test_redirect_preserve_method() {
        let result = redirect("https://example.com/new", Some(true), None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 307);
    }

    #[tokio::test]
    async fn test_redirect_permanent() {
        let result = redirect("https://example.com/new", None, Some(301)).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 301);
    }

    #[tokio::test]
    async fn test_cors_preflight() {
        let result = cors_preflight(None, None, None, None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 204);
        assert!(result.response.headers.contains_key("Access-Control-Allow-Origin"));
        assert!(result.response.headers.contains_key("Access-Control-Allow-Methods"));
        assert!(result.response.headers.contains_key("Access-Control-Allow-Headers"));
        assert!(result.response.headers.contains_key("Access-Control-Max-Age"));
    }

    #[tokio::test]
    async fn test_cors_preflight_custom() {
        let result = cors_preflight(
            Some(vec!["Content-Type".to_string(), "Authorization".to_string()]),
            Some(vec!["GET".to_string(), "POST".to_string()]),
            Some(vec!["https://example.com".to_string()]),
            Some(3600),
        ).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.headers.get("Access-Control-Allow-Origin"), Some(&"https://example.com".to_string()));
        assert_eq!(result.response.headers.get("Access-Control-Allow-Methods"), Some(&"GET, POST".to_string()));
        assert_eq!(result.response.headers.get("Access-Control-Max-Age"), Some(&"3600".to_string()));
    }

    #[tokio::test]
    async fn test_graphql_success() {
        let data = json!({"user": {"id": 1, "name": "John"}});
        let result = graphql(Some(data), None, None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 200);
        assert_eq!(result.response.content_type, "application/json");
        
        let body = result.response.body.unwrap();
        assert!(body.get("data").is_some());
    }

    #[tokio::test]
    async fn test_graphql_with_errors() {
        let errors = vec![
            GraphQLError {
                message: "Field 'name' not found".to_string(),
                locations: Some(vec![GraphQLLocation { line: 1, column: 5 }]),
                path: Some(vec![json!("user"), json!("name")]),
                extensions: None,
            }
        ];
        
        let result = graphql(None, Some(errors), None).await.unwrap();
        
        assert!(result.success);
        let body = result.response.body.unwrap();
        assert!(body.get("errors").is_some());
    }

    #[tokio::test]
    async fn test_paginated() {
        let data = vec![json!({"id": 1}), json!({"id": 2}), json!({"id": 3})];
        let result = paginated(data, Some(true), Some(1), Some(10), 25).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.status_code, 200);
        
        let body = result.response.body.unwrap();
        assert!(body.get("data").is_some());
        assert!(body.get("metadata").is_some());
        
        let metadata = &body["metadata"];
        assert_eq!(metadata["page"], 1);
        assert_eq!(metadata["page_size"], 10);
        assert_eq!(metadata["total"], 25);
        assert_eq!(metadata["total_pages"], 3);
        assert_eq!(metadata["has_next"], true);
        assert_eq!(metadata["has_previous"], false);
    }

    #[tokio::test]
    async fn test_paginated_no_metadata() {
        let data = vec![json!({"id": 1})];
        let result = paginated(data, Some(false), None, None, 1).await.unwrap();
        
        assert!(result.success);
        let body = result.response.body.unwrap();
        assert!(body.get("metadata").is_none());
    }

    #[tokio::test]
    async fn test_csv_response() {
        let data = vec![
            {
                let mut row = HashMap::new();
                row.insert("name".to_string(), json!("Alice"));
                row.insert("age".to_string(), json!(30));
                row
            },
            {
                let mut row = HashMap::new();
                row.insert("name".to_string(), json!("Bob"));
                row.insert("age".to_string(), json!(25));
                row
            },
        ];
        
        let result = csv(data, None, Some("users.csv"), Some(true)).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.content_type, "text/csv");
        assert!(result.response.headers.get("Content-Disposition").unwrap().contains("users.csv"));
        
        let body = result.response.body.unwrap();
        let csv_content = body.as_str().unwrap();
        assert!(csv_content.contains("name"));
        assert!(csv_content.contains("age"));
        assert!(csv_content.contains("Alice"));
        assert!(csv_content.contains("Bob"));
    }

    #[tokio::test]
    async fn test_csv_empty() {
        let data: Vec<HashMap<String, Value>> = vec![];
        let result = csv(data, None, None, None).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.response.content_type, "text/csv");
    }

    #[tokio::test]
    async fn test_csv_custom_delimiter() {
        let data = vec![{
            let mut row = HashMap::new();
            row.insert("a".to_string(), json!("1"));
            row.insert("b".to_string(), json!("2"));
            row
        }];
        
        let result = csv(data, Some(";"), None, Some(true)).await.unwrap();
        
        assert!(result.success);
        let body = result.response.body.unwrap();
        let csv_content = body.as_str().unwrap();
        assert!(csv_content.contains(";"));
    }

    #[tokio::test]
    async fn test_set_header() {
        let result = set_header("X-Custom-Header", "custom-value").await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.name, "X-Custom-Header");
        assert_eq!(result.value, "custom-value");
    }
}
