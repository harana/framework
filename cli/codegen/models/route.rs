use serde::{Deserialize, Serialize};

use crate::models::field::Field;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
    Trace,
    Connect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteModifier {
    Authenticated,
    Public,
    RateLimited,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Json,
    Html,
    Text,
    Xml,
    FormUrlEncoded,
    Multipart,
    Binary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub name: String,
    pub path: String,
    pub method: HttpMethod,
    pub handler: String,
    #[serde(default)]
    pub modifiers: Vec<RouteModifier>,
    #[serde(default)]
    pub request_schema: Vec<Field>,
    #[serde(default)]
    pub response_schema: Vec<Field>,
    #[serde(default)]
    pub path_params: Vec<Field>,
    #[serde(default)]
    pub query_params: Vec<Field>,
    #[serde(default)]
    pub request_content_type: Option<ContentType>,
    #[serde(default)]
    pub response_content_type: Option<ContentType>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

pub type RouteFile = Vec<Route>;
