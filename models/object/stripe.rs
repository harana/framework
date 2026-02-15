// This file is auto-generated. Do not edit manually.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeCustomer {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub customer_id: String,
    pub description: Option<String>,
    pub email: Option<String>,
    pub metadata: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<String>,
}

impl StripeCustomer {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripePaymentIntent {
    pub amount: i64,
    pub capture_method: String,
    pub client_secret: Option<String>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub currency: String,
    pub customer_id: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<String>,
    pub payment_intent_id: String,
    pub payment_method: Option<String>,
    pub status: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl StripePaymentIntent {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeSubscription {
    #[serde(default)]
    pub cancel_at_period_end: bool,
    pub cancelled_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub current_period_end: Option<chrono::DateTime<chrono::Utc>>,
    pub current_period_start: Option<chrono::DateTime<chrono::Utc>>,
    pub customer_id: String,
    pub metadata: Option<String>,
    pub price_id: String,
    pub quantity: i64,
    pub status: String,
    pub subscription_id: String,
    pub trial_end: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl StripeSubscription {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeProduct {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_active: bool,
    pub metadata: Option<String>,
    pub name: String,
    pub product_id: String,
    #[serde(default = "chrono::Utc::now")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl StripeProduct {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl StripePrice {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeRefund {
    pub amount: i64,
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: Option<String>,
    pub payment_intent_id: String,
    pub reason: String,
    pub refund_id: String,
    pub status: String,
}

impl StripeRefund {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeWebhookEvent {
    #[serde(default = "chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub event_id: String,
    pub event_type: String,
    pub payload: String,
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
}

impl StripeWebhookEvent {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeLineItem {
    pub price_id: String,
    pub quantity: i64,
}

impl StripeLineItem {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeBillingDetails {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

impl StripeBillingDetails {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
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

impl StripeAddress {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StripeCard {
    pub number: String,
    pub exp_month: i64,
    pub exp_year: i64,
    pub cvc: String,
}

impl StripeCard {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

