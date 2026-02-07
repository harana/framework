#[cfg(test)]
mod tests {
    use crate::*;
    use wiremock::matchers::{body_string_contains, header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_request() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(200).set_body_string("Hello, World!"))
            .mount(&mock_server)
            .await;
        
        let result = get(&format!("{}/test", mock_server.uri()), None, None, None)
            .await
            .unwrap();
        
        assert_eq!(result.status_code, 200);
        assert_eq!(result.body, "Hello, World!");
    }

    #[tokio::test]
    async fn test_get_with_query_params() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/search"))
            .and(query_param("q", "rust"))
            .respond_with(ResponseTemplate::new(200).set_body_string("search results"))
            .mount(&mock_server)
            .await;
        
        let mut params = std::collections::HashMap::new();
        params.insert("q".to_string(), "rust".to_string());
        
        let result = get(
            &format!("{}/search", mock_server.uri()),
            None,
            None,
            Some(params),
        )
        .await
        .unwrap();
        
        assert_eq!(result.status_code, 200);
        assert_eq!(result.body, "search results");
    }

    #[tokio::test]
    async fn test_get_with_headers() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/auth"))
            .and(header("Authorization", "Bearer token123"))
            .respond_with(ResponseTemplate::new(200).set_body_string("authenticated"))
            .mount(&mock_server)
            .await;
        
        let mut headers = std::collections::HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        
        let result = get(
            &format!("{}/auth", mock_server.uri()),
            Some(headers),
            None,
            None,
        )
        .await
        .unwrap();
        
        assert_eq!(result.status_code, 200);
        assert_eq!(result.body, "authenticated");
    }

    #[tokio::test]
    async fn test_post_request() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("POST"))
            .and(path("/data"))
            .and(header("content-type", "application/json"))
            .and(body_string_contains("test"))
            .respond_with(ResponseTemplate::new(201).set_body_string(r#"{"id": 1}"#))
            .mount(&mock_server)
            .await;
        
        let result = post(
            &format!("{}/data", mock_server.uri()),
            Some(r#"{"name": "test"}"#),
            Some("application/json"),
            None,
            None,
            None,
        )
        .await
        .unwrap();
        
        assert_eq!(result.status_code, 201);
        assert!(result.body.contains("id"));
    }

    #[tokio::test]
    async fn test_put_request() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("PUT"))
            .and(path("/data/1"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"updated": true}"#))
            .mount(&mock_server)
            .await;
        
        let result = put(
            &format!("{}/data/1", mock_server.uri()),
            Some(r#"{"name": "updated"}"#),
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
        
        assert_eq!(result.status_code, 200);
        assert!(result.body.contains("updated"));
    }

    #[tokio::test]
    async fn test_patch_request() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("PATCH"))
            .and(path("/data/1"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"patched": true}"#))
            .mount(&mock_server)
            .await;
        
        let result = patch(
            &format!("{}/data/1", mock_server.uri()),
            Some(r#"{"field": "value"}"#),
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
        
        assert_eq!(result.status_code, 200);
        assert!(result.body.contains("patched"));
    }

    #[tokio::test]
    async fn test_delete_request() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("DELETE"))
            .and(path("/data/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;
        
        let result = delete(&format!("{}/data/1", mock_server.uri()), None, None, None)
            .await
            .unwrap();
        
        assert_eq!(result.status_code, 204);
    }

    #[tokio::test]
    async fn test_download() {
        let mock_server = MockServer::start().await;
        let file_content = vec![0x89, 0x50, 0x4E, 0x47]; // PNG magic bytes
        
        Mock::given(method("GET"))
            .and(path("/file.png"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(file_content.clone())
                    .insert_header("content-type", "image/png"),
            )
            .mount(&mock_server)
            .await;
        
        let result = download(&format!("{}/file.png", mock_server.uri()), None, None)
            .await
            .unwrap();
        
        assert_eq!(result.content, file_content);
        assert_eq!(result.content_type, "image/png");
        assert_eq!(result.size, 4);
    }

    #[tokio::test]
    async fn test_upload() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("POST"))
            .and(path("/upload"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"uploaded": true}"#))
            .mount(&mock_server)
            .await;
        
        let file_content = b"Hello, this is test content";
        
        let result = upload(
            file_content,
            "test.txt",
            &format!("{}/upload", mock_server.uri()),
            None,
            None,
        )
        .await
        .unwrap();
        
        assert_eq!(result.status_code, 200);
        assert!(result.body.contains("uploaded"));
    }

    #[tokio::test]
    async fn test_graphql_query() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("POST"))
            .and(path("/graphql"))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{"data": {"user": {"id": "1", "name": "Test User"}}}"#,
            ))
            .mount(&mock_server)
            .await;
        
        let result = graphql_query(
            &format!("{}/graphql", mock_server.uri()),
            "query { user { id name } }",
            None,
            None,
            None,
        )
        .await
        .unwrap();
        
        assert!(result.data.is_some());
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_graphql_with_variables() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{"data": {"user": {"id": "123", "name": "John"}}}"#,
            ))
            .mount(&mock_server)
            .await;
        
        let mut variables = std::collections::HashMap::new();
        variables.insert("id".to_string(), serde_json::json!("123"));
        
        let result = graphql_query(
            &format!("{}/graphql", mock_server.uri()),
            "query GetUser($id: ID!) { user(id: $id) { id name } }",
            None,
            Some(variables),
            None,
        )
        .await
        .unwrap();
        
        assert!(result.data.is_some());
    }

    #[tokio::test]
    async fn test_graphql_with_errors() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{"data": null, "errors": [{"message": "Not found", "path": ["user"], "locations": [{"line": 1, "column": 10}]}]}"#,
            ))
            .mount(&mock_server)
            .await;
        
        let result = graphql_query(
            &format!("{}/graphql", mock_server.uri()),
            "query { user { id } }",
            None,
            None,
            None,
        )
        .await
        .unwrap();
        
        assert!(!result.errors.is_empty());
        assert_eq!(result.errors[0].message, "Not found");
        assert_eq!(result.errors[0].path, vec!["user"]);
        assert_eq!(result.errors[0].locations[0].line, 1);
        assert_eq!(result.errors[0].locations[0].column, 10);
    }

    #[tokio::test]
    async fn test_get_404_response() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/notfound"))
            .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
            .mount(&mock_server)
            .await;
        
        let result = get(&format!("{}/notfound", mock_server.uri()), None, None, None)
            .await
            .unwrap();
        
        assert_eq!(result.status_code, 404);
        assert_eq!(result.body, "Not Found");
    }
}
