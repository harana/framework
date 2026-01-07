// Harana Actions - Http Client Module
// This module provides http client actions and functionality.

#![warn(missing_docs)]

pub mod output;

use std::collections::HashMap;
use serde_json::Value;
use output::*;

/// Send HTTP DELETE Request
pub async fn delete(
    url: &str,
    query_params: Option<HashMap<String, Value>>,
    timeout: Option<i32>,
    headers: Option<HashMap<String, Value>>,
) -> Result<DeleteOutput, String> {
    unimplemented!("delete")
}

/// Download File From URL
pub async fn download(
    url: &str,
    headers: Option<HashMap<String, Value>>,
    timeout: Option<i32>,
) -> Result<DownloadOutput, String> {
    unimplemented!("download")
}

/// Send HTTP GET Request
pub async fn get(
    url: &str,
    headers: Option<HashMap<String, Value>>,
    timeout: Option<i32>,
    query_params: Option<HashMap<String, Value>>,
) -> Result<GetOutput, String> {
    unimplemented!("get")
}

/// Send GraphQL Query
pub async fn graphql_query(
    url: &str,
    query: &str,
    headers: Option<HashMap<String, Value>>,
    variables: Option<HashMap<String, Value>>,
    timeout: Option<i32>,
) -> Result<GraphqlQueryOutput, String> {
    unimplemented!("graphql_query")
}

/// Send HTTP PATCH Request
pub async fn patch(
    url: &str,
    timeout: Option<i32>,
    body: Option<&str>,
    query_params: Option<HashMap<String, Value>>,
    content_type: Option<&str>,
    headers: Option<HashMap<String, Value>>,
) -> Result<PatchOutput, String> {
    unimplemented!("patch")
}

/// Send HTTP POST Request
pub async fn post(
    url: &str,
    timeout: Option<i32>,
    query_params: Option<HashMap<String, Value>>,
    content_type: Option<&str>,
    headers: Option<HashMap<String, Value>>,
    body: Option<&str>,
) -> Result<PostOutput, String> {
    unimplemented!("post")
}

/// Send HTTP PUT Request
pub async fn put(
    url: &str,
    body: Option<&str>,
    headers: Option<HashMap<String, Value>>,
    content_type: Option<&str>,
    query_params: Option<HashMap<String, Value>>,
    timeout: Option<i32>,
) -> Result<PutOutput, String> {
    unimplemented!("put")
}

/// Upload File To URL
pub async fn upload(
    file: &[u8],
    filename: &str,
    url: &str,
    timeout: Option<i32>,
    headers: Option<HashMap<String, Value>>,
) -> Result<UploadOutput, String> {
    unimplemented!("upload")
}
