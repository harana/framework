// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SendBulkTemplatedEmailOutput {
    pub message_ids: Vec<String>,
    pub status: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListIdentitiesOutput {
    pub identities: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetTemplateOutput {
    pub html_part: String,
    pub subject_part: String,
    pub template_name: String,
    pub text_part: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListTemplatesOutput {
    pub next_token: String,
    pub templates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetSendQuotaOutput {
    pub max_24_hour_send: f64,
    pub max_send_rate: f64,
    pub sent_last_24_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListConfigurationSetsOutput {
    pub configuration_sets: Vec<String>,
    pub next_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSesIdentity {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub dkim_enabled: bool,
    #[serde(default)]
    pub feedback_forwarding_enabled: bool,
    pub identity: String,
    pub identity_type: String,
    pub region: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub verification_status: String,
    pub verification_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSesTemplate {
    pub account_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub html_part: String,
    pub region: String,
    pub subject_part: String,
    pub template_name: String,
    pub text_part: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSesConfigurationSet {
    pub account_id: String,
    pub configuration_set_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub region: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwsSesSendLog {
    pub bcc_addresses: String,
    pub cc_addresses: String,
    pub configuration_set_name: String,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub from_address: String,
    pub identity_id: String,
    pub message_id: String,
    pub status: String,
    pub subject: String,
    pub template_name: String,
    pub to_addresses: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait SesAction {
    async fn send_email(&self, bcc_addresses: Vec<String>, cc_addresses: Vec<String>, configuration_set_name: String, from_address: String, html_body: String, region: String, reply_to_addresses: Vec<String>, return_path: String, subject: String, tags: String, text_body: String, to_addresses: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn send_raw_email(&self, configuration_set_name: String, destinations: Vec<String>, from_arn: String, raw_message: String, region: String, return_path_arn: String, source: String, source_arn: String, tags: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn send_templated_email(&self, bcc_addresses: Vec<String>, cc_addresses: Vec<String>, configuration_set_name: String, from_address: String, region: String, reply_to_addresses: Vec<String>, return_path: String, tags: String, template_data: String, template_name: String, to_addresses: Vec<String>) -> Result<String, Box<dyn std::error::Error>>;
    async fn send_bulk_templated_email(&self, configuration_set_name: String, default_tags: String, default_template_data: String, destinations: Vec<String>, from_address: String, region: String, reply_to_addresses: Vec<String>, return_path: String, template_name: String) -> Result<SendBulkTemplatedEmailOutput, Box<dyn std::error::Error>>;
    async fn verify_email_identity(&self, email_address: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn verify_domain_identity(&self, domain: String, region: String, verification_token: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_identity(&self, identity: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_identities(&self, identity_type: String, max_items: i64, region: String) -> Result<ListIdentitiesOutput, Box<dyn std::error::Error>>;
    async fn get_identity_verification_attributes(&self, identities: Vec<String>, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn set_identity_notification_topic(&self, identity: String, notification_type: String, region: String, sns_topic: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_template(&self, html_part: String, region: String, subject_part: String, template_name: String, text_part: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn update_template(&self, html_part: String, region: String, subject_part: String, template_name: String, text_part: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_template(&self, region: String, template_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_template(&self, region: String, template_name: String) -> Result<GetTemplateOutput, Box<dyn std::error::Error>>;
    async fn list_templates(&self, max_items: i64, region: String) -> Result<ListTemplatesOutput, Box<dyn std::error::Error>>;
    async fn get_send_quota(&self, region: String) -> Result<GetSendQuotaOutput, Box<dyn std::error::Error>>;
    async fn get_send_statistics(&self, region: String) -> Result<Vec<String>, Box<dyn std::error::Error>>;
    async fn put_configuration_set(&self, configuration_set_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_configuration_set(&self, configuration_set_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn list_configuration_sets(&self, max_items: i64, region: String) -> Result<ListConfigurationSetsOutput, Box<dyn std::error::Error>>;
    async fn put_identity_policy(&self, identity: String, policy: String, policy_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_identity_policies(&self, identity: String, policy_names: Vec<String>, region: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_identity_policy(&self, identity: String, policy_name: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn set_identity_feedback_forwarding(&self, forwarding_enabled: bool, identity: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn set_identity_dkim_enabled(&self, dkim_enabled: bool, identity: String, region: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_identity_dkim_attributes(&self, identities: Vec<String>, region: String) -> Result<String, Box<dyn std::error::Error>>;
}
