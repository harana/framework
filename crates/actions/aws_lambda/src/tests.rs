use super::*;

#[tokio::test]
async fn test_create_function() {
    clear_all_data();
    
    let result = create_function(
        "my-function".to_string(),
        "arn:aws:iam::123456789012:role/lambda-role".to_string(),
        FunctionCode {
            zip_file: Some(b"fake code".to_vec()),
            s3_bucket: None,
            s3_key: None,
            s3_object_version: None,
            image_uri: None,
        },
        Some("nodejs18.x".to_string()),
        Some("index.handler".to_string()),
        Some("My test function".to_string()),
        Some(30),
        Some(256),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    assert!(result.success);
    assert_eq!(result.function_name, "my-function");
    assert!(result.function_arn.contains("my-function"));
    assert_eq!(result.runtime, Some("nodejs18.x".to_string()));
    assert_eq!(result.handler, Some("index.handler".to_string()));
    assert_eq!(result.timeout, 30);
    assert_eq!(result.memory_size, 256);
    assert_eq!(result.state, "Active");
}

#[tokio::test]
async fn test_get_function() {
    clear_all_data();
    
    create_function(
        "get-test".to_string(),
        "arn:aws:iam::123456789012:role/lambda-role".to_string(),
        FunctionCode {
            zip_file: Some(b"code".to_vec()),
            s3_bucket: None,
            s3_key: None,
            s3_object_version: None,
            image_uri: None,
        },
        Some("python3.11".to_string()),
        Some("handler.main".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = get_function("get-test".to_string(), None, None).await;

    assert_eq!(result.configuration.function_name, "get-test");
    assert_eq!(result.configuration.runtime, Some("python3.11".to_string()));
    assert_eq!(result.configuration.state, "Active");
    assert!(result.code.is_some());
}

#[tokio::test]
async fn test_delete_function() {
    clear_all_data();
    
    create_function(
        "delete-test".to_string(),
        "arn:aws:iam::123456789012:role/lambda-role".to_string(),
        FunctionCode {
            zip_file: Some(b"code".to_vec()),
            s3_bucket: None,
            s3_key: None,
            s3_object_version: None,
            image_uri: None,
        },
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = delete_function("delete-test".to_string(), None, None).await;
    assert!(result.success);

    let get_result = get_function("delete-test".to_string(), None, None).await;
    assert_eq!(get_result.configuration.state, "Not Found");
}

#[tokio::test]
async fn test_list_functions() {
    clear_all_data();
    
    for i in 0..5 {
        create_function(
            format!("function-{}", i),
            "arn:aws:iam::123456789012:role/lambda-role".to_string(),
            FunctionCode {
                zip_file: Some(format!("code {}", i).into_bytes()),
                s3_bucket: None,
                s3_key: None,
                s3_object_version: None,
                image_uri: None,
            },
            Some("nodejs18.x".to_string()),
            Some("index.handler".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await;
    }

    let result = list_functions(Some(3), None, None, None, None).await;
    assert_eq!(result.functions.len(), 3);
}

#[tokio::test]
async fn test_update_function_code() {
    clear_all_data();
    
    create_function(
        "update-code-test".to_string(),
        "arn:aws:iam::123456789012:role/lambda-role".to_string(),
        FunctionCode {
            zip_file: Some(b"original code".to_vec()),
            s3_bucket: None,
            s3_key: None,
            s3_object_version: None,
            image_uri: None,
        },
        Some("nodejs18.x".to_string()),
        Some("index.handler".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = update_function_code(
        "update-code-test".to_string(),
        Some(b"new code here".to_vec()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    assert!(result.success);
    assert_eq!(result.code_size, 13); // "new code here" length
    assert_eq!(result.last_update_status, Some("Successful".to_string()));
}

#[tokio::test]
async fn test_update_function_configuration() {
    clear_all_data();
    
    create_function(
        "update-config-test".to_string(),
        "arn:aws:iam::123456789012:role/lambda-role".to_string(),
        FunctionCode {
            zip_file: Some(b"code".to_vec()),
            s3_bucket: None,
            s3_key: None,
            s3_object_version: None,
            image_uri: None,
        },
        Some("nodejs18.x".to_string()),
        Some("index.handler".to_string()),
        None,
        Some(3),
        Some(128),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = update_function_configuration(
        "update-config-test".to_string(),
        None,
        None,
        None,
        Some("Updated description".to_string()),
        Some(60),
        Some(512),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    assert!(result.success);
    assert_eq!(result.timeout, 60);
    assert_eq!(result.memory_size, 512);
    assert_eq!(result.description, Some("Updated description".to_string()));
}

#[tokio::test]
async fn test_invoke_request_response() {
    clear_all_data();
    
    create_function(
        "invoke-test".to_string(),
        "arn:aws:iam::123456789012:role/lambda-role".to_string(),
        FunctionCode {
            zip_file: Some(b"code".to_vec()),
            s3_bucket: None,
            s3_key: None,
            s3_object_version: None,
            image_uri: None,
        },
        Some("nodejs18.x".to_string()),
        Some("index.handler".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = invoke(
        "invoke-test".to_string(),
        Some(b"{\"key\": \"value\"}".to_vec()),
        Some(InvocationType::RequestResponse),
        None,
        None,
        None,
        None,
    ).await;

    assert_eq!(result.status_code, 200);
    assert!(result.payload.is_some());
    assert_eq!(result.executed_version, "$LATEST");
    assert!(result.function_error.is_none());
}

#[tokio::test]
async fn test_invoke_async() {
    clear_all_data();
    
    create_function(
        "invoke-async-test".to_string(),
        "arn:aws:iam::123456789012:role/lambda-role".to_string(),
        FunctionCode {
            zip_file: Some(b"code".to_vec()),
            s3_bucket: None,
            s3_key: None,
            s3_object_version: None,
            image_uri: None,
        },
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = invoke(
        "invoke-async-test".to_string(),
        Some(b"{}".to_vec()),
        Some(InvocationType::Event),
        None,
        None,
        None,
        None,
    ).await;

    assert_eq!(result.status_code, 202);
    assert!(result.payload.is_none()); // Async invocation has no immediate response
}

#[tokio::test]
async fn test_invoke_nonexistent() {
    clear_all_data();
    
    let result = invoke(
        "nonexistent".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    assert_eq!(result.status_code, 404);
    assert!(result.function_error.is_some());
}

#[tokio::test]
async fn test_invoke_with_logs() {
    clear_all_data();
    
    create_function(
        "invoke-logs-test".to_string(),
        "arn:aws:iam::123456789012:role/lambda-role".to_string(),
        FunctionCode {
            zip_file: Some(b"code".to_vec()),
            s3_bucket: None,
            s3_key: None,
            s3_object_version: None,
            image_uri: None,
        },
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = invoke(
        "invoke-logs-test".to_string(),
        None,
        Some(InvocationType::RequestResponse),
        Some(LogType::Tail),
        None,
        None,
        None,
    ).await;

    assert_eq!(result.status_code, 200);
    assert!(result.log_result.is_some());
}
