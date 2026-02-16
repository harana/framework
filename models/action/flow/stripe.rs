// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePaymentIntentOutput {
    pub amount: i64,
    pub client_secret: String,
    pub payment_intent_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConfirmPaymentIntentOutput {
    pub payment_intent_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CapturePaymentIntentOutput {
    pub amount_captured: i64,
    pub payment_intent_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelPaymentIntentOutput {
    pub payment_intent_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateCustomerOutput {
    pub created: i64,
    pub customer_id: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteCustomerOutput {
    pub customer_id: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCustomerOutput {
    pub created: i64,
    pub customer_id: String,
    pub email: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub name: String,
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSubscriptionOutput {
    pub current_period_end: i64,
    pub status: String,
    pub subscription_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelSubscriptionOutput {
    pub canceled_at: i64,
    pub status: String,
    pub subscription_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRefundOutput {
    pub amount: i64,
    pub refund_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePriceOutput {
    pub currency: String,
    pub price_id: String,
    pub unit_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateProductOutput {
    pub created: i64,
    pub name: String,
    pub product_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateCheckoutSessionOutput {
    pub session_id: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetrievePaymentIntentOutput {
    pub amount: i64,
    pub currency: String,
    pub customer_id: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub payment_intent_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCustomersOutput {
    pub customers: Vec<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePaymentMethodOutput {
    pub created: i64,
    pub payment_method_id: String,
    pub type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachPaymentMethodOutput {
    pub customer_id: String,
    pub payment_method_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripePaymentIntent {
    pub payment_intent_id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub customer_id: String,
    pub payment_method: String,
    pub client_secret: String,
    pub created: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeLineItem {
    pub price_id: String,
    pub quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeCustomer {
    pub customer_id: String,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeBillingDetails {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeAddress {
    pub line1: String,
    pub line2: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeCard {
    pub number: String,
    pub exp_month: i64,
    pub exp_year: i64,
    pub cvc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeSubscription {
    #[serde(default)]
    pub cancel_at_period_end: bool,
    pub cancelled_at: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub current_period_end: chrono::DateTime<chrono::Utc>,
    pub current_period_start: chrono::DateTime<chrono::Utc>,
    pub customer_id: String,
    pub metadata: String,
    pub price_id: String,
    pub quantity: i64,
    pub status: String,
    pub subscription_id: String,
    pub trial_end: chrono::DateTime<chrono::Utc>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeProduct {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    #[serde(default)]
    pub is_active: bool,
    pub metadata: String,
    pub name: String,
    pub product_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripePrice {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub currency: String,
    #[serde(default)]
    pub is_active: bool,
    pub price_id: String,
    pub product_id: String,
    pub recurring_interval: String,
    pub recurring_interval_count: i64,
    pub unit_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeRefund {
    pub amount: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: String,
    pub payment_intent_id: String,
    pub reason: String,
    pub refund_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeWebhookEvent {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub event_id: String,
    pub event_type: String,
    pub payload: String,
    pub processed_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[async_trait]
pub trait StripeAction {
    async fn create_payment_intent(&self, amount: i64, capture_method: String, confirm: bool, currency: String, customer_id: String, description: String, metadata: std::collections::HashMap<String, String>, payment_method: String) -> Result<CreatePaymentIntentOutput, Box<dyn std::error::Error>>;
    async fn confirm_payment_intent(&self, payment_intent_id: String, payment_method: String) -> Result<ConfirmPaymentIntentOutput, Box<dyn std::error::Error>>;
    async fn capture_payment_intent(&self, amount_to_capture: i64, payment_intent_id: String) -> Result<CapturePaymentIntentOutput, Box<dyn std::error::Error>>;
    async fn cancel_payment_intent(&self, cancellation_reason: String, payment_intent_id: String) -> Result<CancelPaymentIntentOutput, Box<dyn std::error::Error>>;
    async fn create_customer(&self, description: String, email: String, metadata: std::collections::HashMap<String, String>, name: String, payment_method: String, phone: String) -> Result<CreateCustomerOutput, Box<dyn std::error::Error>>;
    async fn update_customer(&self, customer_id: String, description: String, email: String, metadata: std::collections::HashMap<String, String>, name: String, phone: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn delete_customer(&self, customer_id: String) -> Result<DeleteCustomerOutput, Box<dyn std::error::Error>>;
    async fn get_customer(&self, customer_id: String) -> Result<GetCustomerOutput, Box<dyn std::error::Error>>;
    async fn create_subscription(&self, customer_id: String, metadata: std::collections::HashMap<String, String>, price_id: String, quantity: i64, trial_period_days: i64) -> Result<CreateSubscriptionOutput, Box<dyn std::error::Error>>;
    async fn cancel_subscription(&self, cancel_at_period_end: bool, subscription_id: String) -> Result<CancelSubscriptionOutput, Box<dyn std::error::Error>>;
    async fn create_refund(&self, amount: i64, metadata: std::collections::HashMap<String, String>, payment_intent_id: String, reason: String) -> Result<CreateRefundOutput, Box<dyn std::error::Error>>;
    async fn create_price(&self, currency: String, metadata: std::collections::HashMap<String, String>, product_id: String, recurring_interval: String, recurring_interval_count: i64, unit_amount: i64) -> Result<CreatePriceOutput, Box<dyn std::error::Error>>;
    async fn create_product(&self, active: bool, description: String, images: Vec<String>, metadata: std::collections::HashMap<String, String>, name: String) -> Result<CreateProductOutput, Box<dyn std::error::Error>>;
    async fn create_checkout_session(&self, cancel_url: String, customer_email: String, customer_id: String, line_items: Vec<String>, metadata: std::collections::HashMap<String, String>, mode: String, success_url: String) -> Result<CreateCheckoutSessionOutput, Box<dyn std::error::Error>>;
    async fn retrieve_payment_intent(&self, payment_intent_id: String) -> Result<RetrievePaymentIntentOutput, Box<dyn std::error::Error>>;
    async fn list_customers(&self, ending_before: String, limit: i64, starting_after: String) -> Result<ListCustomersOutput, Box<dyn std::error::Error>>;
    async fn create_payment_method(&self, billing_details: String, card: String, type: String) -> Result<CreatePaymentMethodOutput, Box<dyn std::error::Error>>;
    async fn attach_payment_method(&self, customer_id: String, payment_method_id: String) -> Result<AttachPaymentMethodOutput, Box<dyn std::error::Error>>;
}
