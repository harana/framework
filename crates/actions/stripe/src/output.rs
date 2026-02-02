use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// === Action Outputs ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentIntentOutput {
    pub payment_intent_id: String,
    pub amount: i64,
    pub currency: String,
    pub client_secret: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmPaymentIntentOutput {
    pub payment_intent_id: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturePaymentIntentOutput {
    pub payment_intent_id: String,
    pub amount_captured: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelPaymentIntentOutput {
    pub payment_intent_id: String,
    pub status: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerOutput {
    pub customer_id: String,
    pub email: Option<String>,
    pub created: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerOutput {
    pub customer_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCustomerOutput {
    pub customer_id: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomerOutput {
    pub customer_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub created: i64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionOutput {
    pub subscription_id: String,
    pub status: String,
    pub current_period_end: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelSubscriptionOutput {
    pub subscription_id: String,
    pub status: String,
    pub canceled_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRefundOutput {
    pub refund_id: String,
    pub amount: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePriceOutput {
    pub price_id: String,
    pub unit_amount: i64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductOutput {
    pub product_id: String,
    pub name: String,
    pub created: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckoutSessionOutput {
    pub session_id: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievePaymentIntentOutput {
    pub payment_intent_id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub customer_id: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCustomersOutput {
    pub customers: Vec<StripeCustomer>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentMethodOutput {
    pub payment_method_id: String,
    pub r#type: String,
    pub created: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachPaymentMethodOutput {
    pub payment_method_id: String,
    pub customer_id: String,
    pub success: bool,
}

// === Class Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripePaymentIntent {
    pub payment_intent_id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub customer_id: Option<String>,
    pub payment_method: Option<String>,
    pub client_secret: String,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeLineItem {
    pub price_id: String,
    pub quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeCustomer {
    pub customer_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub created: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeBillingDetails {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<StripeAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeAddress {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeCard {
    pub number: String,
    pub exp_month: i32,
    pub exp_year: i32,
    pub cvc: String,
}

// === Internal Storage Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredPaymentIntent {
    pub payment_intent_id: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub capture_method: CaptureMethod,
    pub customer_id: Option<String>,
    pub payment_method: Option<String>,
    pub description: Option<String>,
    pub client_secret: String,
    pub metadata: HashMap<String, String>,
    pub created: DateTime<Utc>,
    pub amount_captured: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredCustomer {
    pub customer_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub payment_method: Option<String>,
    pub metadata: HashMap<String, String>,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSubscription {
    pub subscription_id: String,
    pub customer_id: String,
    pub price_id: String,
    pub quantity: i64,
    pub status: String,
    pub trial_period_days: Option<i64>,
    pub current_period_end: i64,
    pub canceled_at: Option<i64>,
    pub metadata: HashMap<String, String>,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredRefund {
    pub refund_id: String,
    pub payment_intent_id: String,
    pub amount: i64,
    pub reason: Option<RefundReason>,
    pub status: String,
    pub metadata: HashMap<String, String>,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredPrice {
    pub price_id: String,
    pub product_id: String,
    pub unit_amount: i64,
    pub currency: String,
    pub recurring_interval: Option<RecurringInterval>,
    pub recurring_interval_count: i64,
    pub metadata: HashMap<String, String>,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredProduct {
    pub product_id: String,
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
    pub images: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredCheckoutSession {
    pub session_id: String,
    pub url: String,
    pub mode: CheckoutMode,
    pub customer_id: Option<String>,
    pub customer_email: Option<String>,
    pub line_items: Vec<StripeLineItem>,
    pub success_url: String,
    pub cancel_url: String,
    pub metadata: HashMap<String, String>,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredPaymentMethod {
    pub payment_method_id: String,
    pub r#type: PaymentMethodType,
    pub customer_id: Option<String>,
    pub billing_details: Option<StripeBillingDetails>,
    pub card: Option<StripeCard>,
    pub created: DateTime<Utc>,
}

// === Enums ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptureMethod {
    Automatic,
    Manual,
}

impl Default for CaptureMethod {
    fn default() -> Self {
        CaptureMethod::Automatic
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CancellationReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
    Abandoned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurringInterval {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckoutMode {
    Payment,
    Subscription,
    Setup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethodType {
    Card,
    BankAccount,
    UsBankAccount,
}
