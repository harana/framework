// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateInput {
    pub data: String,
    pub engine: String,
    pub template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegexMatchInput {
    pub flags: String,
    pub pattern: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegexMatchOutput {
    pub matches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegexReplaceInput {
    pub flags: String,
    pub pattern: String,
    pub replacement: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegexReplaceOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SplitInput {
    pub delimiter: String,
    pub limit: i64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SplitOutput {
    pub parts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JoinInput {
    pub items: Vec<String>,
    pub separator: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JoinOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TrimInput {
    pub characters: String,
    pub mode: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TrimOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CaseConvertInput {
    pub format: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CaseConvertOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TruncateInput {
    pub length: i64,
    pub suffix: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TruncateOutput {
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlugifyInput {
    #[serde(default)]
    pub lowercase: bool,
    pub separator: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SlugifyOutput {
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TextString {
    pub content: String,
    pub length: i64,
    pub encoding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateData {
    pub values: std::collections::HashMap<String, String>,
}

#[async_trait]
pub trait TextAction {
    async fn template(&self, input: TemplateInput) -> Result<TemplateOutput, Box<dyn std::error::Error>>;
    async fn regex_match(&self, input: RegexMatchInput) -> Result<RegexMatchOutput, Box<dyn std::error::Error>>;
    async fn regex_replace(&self, input: RegexReplaceInput) -> Result<RegexReplaceOutput, Box<dyn std::error::Error>>;
    async fn split(&self, input: SplitInput) -> Result<SplitOutput, Box<dyn std::error::Error>>;
    async fn join(&self, input: JoinInput) -> Result<JoinOutput, Box<dyn std::error::Error>>;
    async fn trim(&self, input: TrimInput) -> Result<TrimOutput, Box<dyn std::error::Error>>;
    async fn case_convert(&self, input: CaseConvertInput) -> Result<CaseConvertOutput, Box<dyn std::error::Error>>;
    async fn truncate(&self, input: TruncateInput) -> Result<TruncateOutput, Box<dyn std::error::Error>>;
    async fn slugify(&self, input: SlugifyInput) -> Result<SlugifyOutput, Box<dyn std::error::Error>>;
}
