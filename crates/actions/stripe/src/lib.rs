pub mod output;

#[cfg(test)]
mod tests;

use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

use output::*;

// In-memory storage for mock Stripe data
static PAYMENT_INTENTS: Lazy<DashMap<String, StoredPaymentIntent>> = Lazy::new(DashMap::new);
static CUSTOMERS: Lazy<DashMap<String, StoredCustomer>> = Lazy::new(DashMap::new);
static SUBSCRIPTIONS: Lazy<DashMap<String, StoredSubscription>> = Lazy::new(DashMap::new);
static REFUNDS: Lazy<DashMap<String, StoredRefund>> = Lazy::new(DashMap::new);
static PRICES: Lazy<DashMap<String, StoredPrice>> = Lazy::new(DashMap::new);
static PRODUCTS: Lazy<DashMap<String, StoredProduct>> = Lazy::new(DashMap::new);
static CHECKOUT_SESSIONS: Lazy<DashMap<String, StoredCheckoutSession>> = Lazy::new(DashMap::new);
static PAYMENT_METHODS: Lazy<DashMap<String, StoredPaymentMethod>> = Lazy::new(DashMap::new);

fn generate_id(prefix: &str) -> String {
    format!("{}_{}", prefix, Uuid::new_v4().to_string().replace("-", "")[..24].to_string())
}

fn generate_client_secret(payment_intent_id: &str) -> String {
    format!("{}_secret_{}", payment_intent_id, Uuid::new_v4().to_string().replace("-", "")[..24].to_string())
}

/// Create a new payment intent
pub async fn create_payment_intent(
    amount: i64,
    currency: String,
    capture_method: Option<CaptureMethod>,
    confirm: Option<bool>,
    customer_id: Option<String>,
    description: Option<String>,
    metadata: Option<HashMap<String, String>>,
    payment_method: Option<String>,
) -> CreatePaymentIntentOutput {
    let payment_intent_id = generate_id("pi");
    let client_secret = generate_client_secret(&payment_intent_id);
    let capture_method = capture_method.unwrap_or(CaptureMethod::Automatic);
    let confirm = confirm.unwrap_or(false);
    
    let status = if confirm && payment_method.is_some() {
        if capture_method == CaptureMethod::Automatic {
            "succeeded".to_string()
        } else {
            "requires_capture".to_string()
        }
    } else if payment_method.is_some() {
        "requires_confirmation".to_string()
    } else {
        "requires_payment_method".to_string()
    };

    let stored = StoredPaymentIntent {
        payment_intent_id: payment_intent_id.clone(),
        amount,
        currency: currency.clone(),
        status: status.clone(),
        capture_method,
        customer_id,
        payment_method,
        description,
        client_secret: client_secret.clone(),
        metadata: metadata.unwrap_or_default(),
        created: Utc::now(),
        amount_captured: 0,
    };

    PAYMENT_INTENTS.insert(payment_intent_id.clone(), stored);

    CreatePaymentIntentOutput {
        payment_intent_id,
        amount,
        currency,
        client_secret,
        status,
    }
}

/// Confirm a payment intent
pub async fn confirm_payment_intent(
    payment_intent_id: String,
    payment_method: Option<String>,
) -> ConfirmPaymentIntentOutput {
    let mut success = false;
    let mut status = "failed".to_string();

    if let Some(mut entry) = PAYMENT_INTENTS.get_mut(&payment_intent_id) {
        if let Some(pm) = payment_method {
            entry.payment_method = Some(pm);
        }
        
        if entry.payment_method.is_some() {
            if entry.capture_method == CaptureMethod::Automatic {
                entry.status = "succeeded".to_string();
                entry.amount_captured = entry.amount;
            } else {
                entry.status = "requires_capture".to_string();
            }
            success = true;
        } else {
            entry.status = "requires_payment_method".to_string();
        }
        status = entry.status.clone();
    }

    ConfirmPaymentIntentOutput {
        payment_intent_id,
        status,
        success,
    }
}

/// Capture a payment intent
pub async fn capture_payment_intent(
    payment_intent_id: String,
    amount_to_capture: Option<i64>,
) -> CapturePaymentIntentOutput {
    let mut amount_captured = 0;
    let mut status = "failed".to_string();

    if let Some(mut entry) = PAYMENT_INTENTS.get_mut(&payment_intent_id) {
        if entry.status == "requires_capture" {
            let capture_amount = amount_to_capture.unwrap_or(entry.amount);
            entry.amount_captured = capture_amount.min(entry.amount);
            entry.status = "succeeded".to_string();
            amount_captured = entry.amount_captured;
            status = entry.status.clone();
        }
    }

    CapturePaymentIntentOutput {
        payment_intent_id,
        amount_captured,
        status,
    }
}

/// Cancel a payment intent
pub async fn cancel_payment_intent(
    payment_intent_id: String,
    _cancellation_reason: Option<CancellationReason>,
) -> CancelPaymentIntentOutput {
    let mut success = false;
    let mut status = "failed".to_string();

    if let Some(mut entry) = PAYMENT_INTENTS.get_mut(&payment_intent_id) {
        if entry.status != "succeeded" && entry.status != "canceled" {
            entry.status = "canceled".to_string();
            success = true;
        }
        status = entry.status.clone();
    }

    CancelPaymentIntentOutput {
        payment_intent_id,
        status,
        success,
    }
}

/// Create a new customer
pub async fn create_customer(
    email: Option<String>,
    name: Option<String>,
    phone: Option<String>,
    description: Option<String>,
    payment_method: Option<String>,
    metadata: Option<HashMap<String, String>>,
) -> CreateCustomerOutput {
    let customer_id = generate_id("cus");
    let now = Utc::now();

    let stored = StoredCustomer {
        customer_id: customer_id.clone(),
        email: email.clone(),
        name,
        phone,
        description,
        payment_method,
        metadata: metadata.unwrap_or_default(),
        created: now,
    };

    CUSTOMERS.insert(customer_id.clone(), stored);

    CreateCustomerOutput {
        customer_id,
        email,
        created: now.timestamp(),
    }
}

/// Update an existing customer
pub async fn update_customer(
    customer_id: String,
    email: Option<String>,
    name: Option<String>,
    phone: Option<String>,
    description: Option<String>,
    metadata: Option<HashMap<String, String>>,
) -> UpdateCustomerOutput {
    let mut success = false;

    if let Some(mut entry) = CUSTOMERS.get_mut(&customer_id) {
        if let Some(e) = email {
            entry.email = Some(e);
        }
        if let Some(n) = name {
            entry.name = Some(n);
        }
        if let Some(p) = phone {
            entry.phone = Some(p);
        }
        if let Some(d) = description {
            entry.description = Some(d);
        }
        if let Some(m) = metadata {
            entry.metadata.extend(m);
        }
        success = true;
    }

    UpdateCustomerOutput {
        customer_id,
        success,
    }
}

/// Delete a customer
pub async fn delete_customer(customer_id: String) -> DeleteCustomerOutput {
    let deleted = CUSTOMERS.remove(&customer_id).is_some();

    DeleteCustomerOutput {
        customer_id,
        deleted,
    }
}

/// Get a customer by ID
pub async fn get_customer(customer_id: String) -> GetCustomerOutput {
    if let Some(entry) = CUSTOMERS.get(&customer_id) {
        GetCustomerOutput {
            customer_id: entry.customer_id.clone(),
            email: entry.email.clone(),
            name: entry.name.clone(),
            phone: entry.phone.clone(),
            created: entry.created.timestamp(),
            metadata: entry.metadata.clone(),
        }
    } else {
        GetCustomerOutput {
            customer_id,
            email: None,
            name: None,
            phone: None,
            created: 0,
            metadata: HashMap::new(),
        }
    }
}

/// Create a new subscription
pub async fn create_subscription(
    customer_id: String,
    price_id: String,
    quantity: Option<i64>,
    trial_period_days: Option<i64>,
    metadata: Option<HashMap<String, String>>,
) -> CreateSubscriptionOutput {
    let subscription_id = generate_id("sub");
    let now = Utc::now();
    let quantity = quantity.unwrap_or(1);
    
    // Calculate period end (30 days from now by default, or after trial)
    let period_days = trial_period_days.unwrap_or(0) + 30;
    let current_period_end = now.timestamp() + (period_days * 24 * 60 * 60);

    let status = if trial_period_days.is_some() {
        "trialing".to_string()
    } else {
        "active".to_string()
    };

    let stored = StoredSubscription {
        subscription_id: subscription_id.clone(),
        customer_id,
        price_id,
        quantity,
        status: status.clone(),
        trial_period_days,
        current_period_end,
        canceled_at: None,
        metadata: metadata.unwrap_or_default(),
        created: now,
    };

    SUBSCRIPTIONS.insert(subscription_id.clone(), stored);

    CreateSubscriptionOutput {
        subscription_id,
        status,
        current_period_end,
    }
}

/// Cancel a subscription
pub async fn cancel_subscription(
    subscription_id: String,
    cancel_at_period_end: Option<bool>,
) -> CancelSubscriptionOutput {
    let cancel_at_period_end = cancel_at_period_end.unwrap_or(false);
    let mut status = "canceled".to_string();
    let mut canceled_at = None;

    if let Some(mut entry) = SUBSCRIPTIONS.get_mut(&subscription_id) {
        if cancel_at_period_end {
            entry.status = "active".to_string(); // Still active until period end
            status = "active".to_string();
        } else {
            entry.status = "canceled".to_string();
            entry.canceled_at = Some(Utc::now().timestamp());
            canceled_at = entry.canceled_at;
        }
    }

    CancelSubscriptionOutput {
        subscription_id,
        status,
        canceled_at,
    }
}

/// Create a refund
pub async fn create_refund(
    payment_intent_id: String,
    amount: Option<i64>,
    reason: Option<RefundReason>,
    metadata: Option<HashMap<String, String>>,
) -> CreateRefundOutput {
    let refund_id = generate_id("re");
    
    // Get the original payment intent amount if not specified
    let refund_amount = amount.unwrap_or_else(|| {
        PAYMENT_INTENTS.get(&payment_intent_id)
            .map(|pi| pi.amount)
            .unwrap_or(0)
    });

    let stored = StoredRefund {
        refund_id: refund_id.clone(),
        payment_intent_id,
        amount: refund_amount,
        reason,
        status: "succeeded".to_string(),
        metadata: metadata.unwrap_or_default(),
        created: Utc::now(),
    };

    REFUNDS.insert(refund_id.clone(), stored);

    CreateRefundOutput {
        refund_id,
        amount: refund_amount,
        status: "succeeded".to_string(),
    }
}

/// Create a price
pub async fn create_price(
    product_id: String,
    unit_amount: i64,
    currency: String,
    recurring_interval: Option<RecurringInterval>,
    recurring_interval_count: Option<i64>,
    metadata: Option<HashMap<String, String>>,
) -> CreatePriceOutput {
    let price_id = generate_id("price");

    let stored = StoredPrice {
        price_id: price_id.clone(),
        product_id,
        unit_amount,
        currency: currency.clone(),
        recurring_interval,
        recurring_interval_count: recurring_interval_count.unwrap_or(1),
        metadata: metadata.unwrap_or_default(),
        created: Utc::now(),
    };

    PRICES.insert(price_id.clone(), stored);

    CreatePriceOutput {
        price_id,
        unit_amount,
        currency,
    }
}

/// Create a product
pub async fn create_product(
    name: String,
    description: Option<String>,
    active: Option<bool>,
    images: Option<Vec<String>>,
    metadata: Option<HashMap<String, String>>,
) -> CreateProductOutput {
    let product_id = generate_id("prod");
    let now = Utc::now();

    let stored = StoredProduct {
        product_id: product_id.clone(),
        name: name.clone(),
        description,
        active: active.unwrap_or(true),
        images: images.unwrap_or_default(),
        metadata: metadata.unwrap_or_default(),
        created: now,
    };

    PRODUCTS.insert(product_id.clone(), stored);

    CreateProductOutput {
        product_id,
        name,
        created: now.timestamp(),
    }
}

/// Create a checkout session
pub async fn create_checkout_session(
    mode: CheckoutMode,
    success_url: String,
    cancel_url: String,
    line_items: Vec<StripeLineItem>,
    customer_id: Option<String>,
    customer_email: Option<String>,
    metadata: Option<HashMap<String, String>>,
) -> CreateCheckoutSessionOutput {
    let session_id = generate_id("cs");
    let url = format!("https://checkout.stripe.com/c/pay/{}", session_id);

    let stored = StoredCheckoutSession {
        session_id: session_id.clone(),
        url: url.clone(),
        mode,
        customer_id,
        customer_email,
        line_items,
        success_url,
        cancel_url,
        metadata: metadata.unwrap_or_default(),
        created: Utc::now(),
    };

    CHECKOUT_SESSIONS.insert(session_id.clone(), stored);

    CreateCheckoutSessionOutput {
        session_id,
        url,
    }
}

/// Retrieve a payment intent
pub async fn retrieve_payment_intent(payment_intent_id: String) -> RetrievePaymentIntentOutput {
    if let Some(entry) = PAYMENT_INTENTS.get(&payment_intent_id) {
        RetrievePaymentIntentOutput {
            payment_intent_id: entry.payment_intent_id.clone(),
            amount: entry.amount,
            currency: entry.currency.clone(),
            status: entry.status.clone(),
            customer_id: entry.customer_id.clone(),
            metadata: entry.metadata.clone(),
        }
    } else {
        RetrievePaymentIntentOutput {
            payment_intent_id,
            amount: 0,
            currency: String::new(),
            status: "not_found".to_string(),
            customer_id: None,
            metadata: HashMap::new(),
        }
    }
}

/// List customers with pagination
pub async fn list_customers(
    limit: Option<i64>,
    starting_after: Option<String>,
    ending_before: Option<String>,
) -> ListCustomersOutput {
    let limit = limit.unwrap_or(10) as usize;
    let mut customers: Vec<StripeCustomer> = CUSTOMERS
        .iter()
        .map(|entry| {
            let c = entry.value();
            StripeCustomer {
                customer_id: c.customer_id.clone(),
                email: c.email.clone(),
                name: c.name.clone(),
                phone: c.phone.clone(),
                created: c.created,
                metadata: c.metadata.clone(),
            }
        })
        .collect();

    // Sort by created date (newest first)
    customers.sort_by(|a, b| b.created.cmp(&a.created));

    // Apply pagination
    let start_idx = if let Some(after) = starting_after {
        customers.iter().position(|c| c.customer_id == after).map(|i| i + 1).unwrap_or(0)
    } else if let Some(before) = ending_before {
        customers.iter().position(|c| c.customer_id == before).unwrap_or(customers.len()).saturating_sub(limit)
    } else {
        0
    };

    let end_idx = (start_idx + limit).min(customers.len());
    let has_more = end_idx < customers.len();
    let result = customers[start_idx..end_idx].to_vec();

    ListCustomersOutput {
        customers: result,
        has_more,
    }
}

/// Create a payment method
pub async fn create_payment_method(
    r#type: PaymentMethodType,
    card: Option<StripeCard>,
    billing_details: Option<StripeBillingDetails>,
) -> CreatePaymentMethodOutput {
    let payment_method_id = generate_id("pm");
    let now = Utc::now();

    let stored = StoredPaymentMethod {
        payment_method_id: payment_method_id.clone(),
        r#type,
        customer_id: None,
        billing_details,
        card,
        created: now,
    };

    PAYMENT_METHODS.insert(payment_method_id.clone(), stored);

    let type_str = match r#type {
        PaymentMethodType::Card => "card",
        PaymentMethodType::BankAccount => "bank_account",
        PaymentMethodType::UsBankAccount => "us_bank_account",
    };

    CreatePaymentMethodOutput {
        payment_method_id,
        r#type: type_str.to_string(),
        created: now.timestamp(),
    }
}

/// Attach a payment method to a customer
pub async fn attach_payment_method(
    payment_method_id: String,
    customer_id: String,
) -> AttachPaymentMethodOutput {
    let mut success = false;

    if let Some(mut entry) = PAYMENT_METHODS.get_mut(&payment_method_id) {
        if CUSTOMERS.contains_key(&customer_id) {
            entry.customer_id = Some(customer_id.clone());
            success = true;
        }
    }

    AttachPaymentMethodOutput {
        payment_method_id,
        customer_id,
        success,
    }
}

// Utility functions for testing
#[cfg(test)]
pub fn clear_all_data() {
    PAYMENT_INTENTS.clear();
    CUSTOMERS.clear();
    SUBSCRIPTIONS.clear();
    REFUNDS.clear();
    PRICES.clear();
    PRODUCTS.clear();
    CHECKOUT_SESSIONS.clear();
    PAYMENT_METHODS.clear();
}
