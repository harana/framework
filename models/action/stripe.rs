// This file is auto-generated. Do not edit manually.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePaymentIntentInput {
    pub amount: i64,
    pub capture_method: String,
    #[serde(default)]
    pub confirm: bool,
    pub currency: String,
    pub customer_id: String,
    pub description: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub payment_method: String,
}

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
pub struct ConfirmPaymentIntentInput {
    pub payment_intent_id: String,
    pub payment_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConfirmPaymentIntentOutput {
    pub payment_intent_id: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CapturePaymentIntentInput {
    pub amount_to_capture: i64,
    pub payment_intent_id: String,
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
pub struct CancelPaymentIntentInput {
    pub cancellation_reason: String,
    pub payment_intent_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CancelPaymentIntentOutput {
    pub payment_intent_id: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateCustomerInput {
    pub description: String,
    pub email: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub name: String,
    pub payment_method: String,
    pub phone: String,
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
pub struct UpdateCustomerInput {
    pub customer_id: String,
    pub description: String,
    pub email: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub name: String,
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateCustomerOutput {
    pub customer_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteCustomerInput {
    pub customer_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeleteCustomerOutput {
    pub customer_id: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetCustomerInput {
    pub customer_id: String,
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
pub struct CreateSubscriptionInput {
    pub customer_id: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub price_id: String,
    pub quantity: i64,
    pub trial_period_days: i64,
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
pub struct CancelSubscriptionInput {
    #[serde(default)]
    pub cancel_at_period_end: bool,
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
pub struct CreateRefundInput {
    pub amount: i64,
    pub metadata: std::collections::HashMap<String, String>,
    pub payment_intent_id: String,
    pub reason: String,
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
pub struct CreatePriceInput {
    pub currency: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub product_id: String,
    pub recurring_interval: String,
    pub recurring_interval_count: i64,
    pub unit_amount: i64,
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
pub struct CreateProductInput {
    #[serde(default)]
    pub active: bool,
    pub description: String,
    pub images: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
    pub name: String,
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
pub struct CreateCheckoutSessionInput {
    pub cancel_url: String,
    pub customer_email: String,
    pub customer_id: String,
    pub line_items: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
    pub mode: String,
    pub success_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateCheckoutSessionOutput {
    pub session_id: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetrievePaymentIntentInput {
    pub payment_intent_id: String,
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
pub struct ListCustomersInput {
    pub ending_before: String,
    pub limit: i64,
    pub starting_after: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCustomersOutput {
    pub customers: Vec<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePaymentMethodInput {
    pub billing_details: String,
    pub card: String,
    pub type: String,
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
pub struct AttachPaymentMethodInput {
    pub customer_id: String,
    pub payment_method_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AttachPaymentMethodOutput {
    pub customer_id: String,
    pub payment_method_id: String,
    pub success: bool,
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

#[async_trait]
pub trait StripeAction {
    async fn create_payment_intent(&self, input: CreatePaymentIntentInput) -> Result<CreatePaymentIntentOutput, Box<dyn std::error::Error>>;
    async fn confirm_payment_intent(&self, input: ConfirmPaymentIntentInput) -> Result<ConfirmPaymentIntentOutput, Box<dyn std::error::Error>>;
    async fn capture_payment_intent(&self, input: CapturePaymentIntentInput) -> Result<CapturePaymentIntentOutput, Box<dyn std::error::Error>>;
    async fn cancel_payment_intent(&self, input: CancelPaymentIntentInput) -> Result<CancelPaymentIntentOutput, Box<dyn std::error::Error>>;
    async fn create_customer(&self, input: CreateCustomerInput) -> Result<CreateCustomerOutput, Box<dyn std::error::Error>>;
    async fn update_customer(&self, input: UpdateCustomerInput) -> Result<UpdateCustomerOutput, Box<dyn std::error::Error>>;
    async fn delete_customer(&self, input: DeleteCustomerInput) -> Result<DeleteCustomerOutput, Box<dyn std::error::Error>>;
    async fn get_customer(&self, input: GetCustomerInput) -> Result<GetCustomerOutput, Box<dyn std::error::Error>>;
    async fn create_subscription(&self, input: CreateSubscriptionInput) -> Result<CreateSubscriptionOutput, Box<dyn std::error::Error>>;
    async fn cancel_subscription(&self, input: CancelSubscriptionInput) -> Result<CancelSubscriptionOutput, Box<dyn std::error::Error>>;
    async fn create_refund(&self, input: CreateRefundInput) -> Result<CreateRefundOutput, Box<dyn std::error::Error>>;
    async fn create_price(&self, input: CreatePriceInput) -> Result<CreatePriceOutput, Box<dyn std::error::Error>>;
    async fn create_product(&self, input: CreateProductInput) -> Result<CreateProductOutput, Box<dyn std::error::Error>>;
    async fn create_checkout_session(&self, input: CreateCheckoutSessionInput) -> Result<CreateCheckoutSessionOutput, Box<dyn std::error::Error>>;
    async fn retrieve_payment_intent(&self, input: RetrievePaymentIntentInput) -> Result<RetrievePaymentIntentOutput, Box<dyn std::error::Error>>;
    async fn list_customers(&self, input: ListCustomersInput) -> Result<ListCustomersOutput, Box<dyn std::error::Error>>;
    async fn create_payment_method(&self, input: CreatePaymentMethodInput) -> Result<CreatePaymentMethodOutput, Box<dyn std::error::Error>>;
    async fn attach_payment_method(&self, input: AttachPaymentMethodInput) -> Result<AttachPaymentMethodOutput, Box<dyn std::error::Error>>;
}
