// Harana Actions - Stripe Module Output Types
// Auto-generated output structs for action methods.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// attach_payment_method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachPaymentMethodOutput {
    pub success: bool,
    pub customer_id: String,
    pub payment_method_id: String
}

// cancel_payment_intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelPaymentIntentOutput {
    pub status: String,
    pub payment_intent_id: String,
    pub success: bool
}

// cancel_subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelSubscriptionOutput {
    pub status: String,
    pub canceled_at: i32,
    pub subscription_id: String
}

// capture_payment_intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturePaymentIntentOutput {
    pub payment_intent_id: String,
    pub status: String,
    pub amount_captured: i32
}

// confirm_payment_intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmPaymentIntentOutput {
    pub success: bool,
    pub payment_intent_id: String,
    pub status: String
}

// create_checkout_session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckoutSessionOutput {
    pub url: String,
    pub session_id: String
}

// create_customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerOutput {
    pub email: String,
    pub created: i32,
    pub customer_id: String
}

// create_payment_intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentIntentOutput {
    pub amount: i32,
    pub payment_intent_id: String,
    pub status: String,
    pub client_secret: String
}

// create_payment_method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentMethodOutput {
    pub created: i32,
    pub payment_method_id: String,
    pub r#type: String
}

// create_price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePriceOutput {
    pub currency: String,
    pub price_id: String,
    pub unit_amount: i32
}

// create_product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductOutput {
    pub product_id: String,
    pub created: i32,
    pub name: String
}

// create_refund
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRefundOutput {
    pub status: String,
    pub amount: i32,
    pub refund_id: String
}

// create_subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionOutput {
    pub current_period_end: i32,
    pub subscription_id: String,
    pub status: String
}

// delete_customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCustomerOutput {
    pub customer_id: String,
    pub deleted: bool
}

// get_customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomerOutput {
    pub name: String,
    pub email: String,
    pub customer_id: String,
    pub created: i32,
    pub phone: String,
    pub metadata: HashMap<String, Value>
}

// list_customers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCustomersOutput {
    pub customers: Vec<HashMap<String, Value>>,
    pub has_more: bool
}

// retrieve_payment_intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievePaymentIntentOutput {
    pub metadata: HashMap<String, Value>,
    pub status: String,
    pub customer_id: String,
    pub payment_intent_id: String,
    pub amount: i32,
    pub currency: String
}

// update_customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerOutput {
    pub customer_id: String,
    pub success: bool
}
