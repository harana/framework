use super::*;

#[tokio::test]
async fn test_create_payment_intent() {
    clear_all_data();
    
    let result = create_payment_intent(
        5000,
        "usd".to_string(),
        None,
        None,
        None,
        Some("Test payment".to_string()),
        None,
        None,
    ).await;

    assert!(result.payment_intent_id.starts_with("pi_"));
    assert_eq!(result.amount, 5000);
    assert_eq!(result.currency, "usd");
    assert_eq!(result.status, "requires_payment_method");
    assert!(!result.client_secret.is_empty());
}

#[tokio::test]
async fn test_confirm_payment_intent() {
    clear_all_data();
    
    let pi = create_payment_intent(
        5000,
        "usd".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = confirm_payment_intent(
        pi.payment_intent_id.clone(),
        Some("pm_card_visa".to_string()),
    ).await;

    assert_eq!(result.payment_intent_id, pi.payment_intent_id);
    assert_eq!(result.status, "succeeded");
    assert!(result.success);
}

#[tokio::test]
async fn test_capture_payment_intent() {
    clear_all_data();
    
    // Create with manual capture
    let pi = create_payment_intent(
        5000,
        "usd".to_string(),
        Some(CaptureMethod::Manual),
        Some(true),
        None,
        None,
        None,
        Some("pm_card_visa".to_string()),
    ).await;

    assert_eq!(pi.status, "requires_capture");

    let result = capture_payment_intent(
        pi.payment_intent_id.clone(),
        Some(3000),
    ).await;

    assert_eq!(result.payment_intent_id, pi.payment_intent_id);
    assert_eq!(result.amount_captured, 3000);
    assert_eq!(result.status, "succeeded");
}

#[tokio::test]
async fn test_cancel_payment_intent() {
    clear_all_data();
    
    let pi = create_payment_intent(
        5000,
        "usd".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
    ).await;

    let result = cancel_payment_intent(
        pi.payment_intent_id.clone(),
        Some(CancellationReason::RequestedByCustomer),
    ).await;

    assert_eq!(result.payment_intent_id, pi.payment_intent_id);
    assert_eq!(result.status, "canceled");
    assert!(result.success);
}

#[tokio::test]
async fn test_customer_lifecycle() {
    clear_all_data();
    
    // Create
    let create_result = create_customer(
        Some("test@example.com".to_string()),
        Some("John Doe".to_string()),
        Some("+1234567890".to_string()),
        None,
        None,
        None,
    ).await;

    assert!(create_result.customer_id.starts_with("cus_"));
    assert_eq!(create_result.email, Some("test@example.com".to_string()));

    // Get
    let get_result = get_customer(create_result.customer_id.clone()).await;
    assert_eq!(get_result.name, Some("John Doe".to_string()));
    assert_eq!(get_result.phone, Some("+1234567890".to_string()));

    // Update
    let update_result = update_customer(
        create_result.customer_id.clone(),
        Some("updated@example.com".to_string()),
        None,
        None,
        None,
        None,
    ).await;
    assert!(update_result.success);

    // Verify update
    let get_result2 = get_customer(create_result.customer_id.clone()).await;
    assert_eq!(get_result2.email, Some("updated@example.com".to_string()));

    // Delete
    let delete_result = delete_customer(create_result.customer_id.clone()).await;
    assert!(delete_result.deleted);
}

#[tokio::test]
async fn test_list_customers() {
    clear_all_data();
    
    // Create multiple customers
    for i in 0..5 {
        create_customer(
            Some(format!("test{}@example.com", i)),
            Some(format!("Customer {}", i)),
            None,
            None,
            None,
            None,
        ).await;
    }

    let result = list_customers(Some(3), None, None).await;
    assert_eq!(result.customers.len(), 3);
    assert!(result.has_more);
}

#[tokio::test]
async fn test_subscription_lifecycle() {
    clear_all_data();
    
    // Create customer first
    let customer = create_customer(
        Some("sub@example.com".to_string()),
        None,
        None,
        None,
        None,
        None,
    ).await;

    // Create product and price
    let product = create_product(
        "Premium Plan".to_string(),
        Some("Monthly premium subscription".to_string()),
        None,
        None,
        None,
    ).await;

    let price = create_price(
        product.product_id,
        1999,
        "usd".to_string(),
        Some(RecurringInterval::Month),
        None,
        None,
    ).await;

    // Create subscription
    let sub = create_subscription(
        customer.customer_id,
        price.price_id,
        Some(1),
        None,
        None,
    ).await;

    assert!(sub.subscription_id.starts_with("sub_"));
    assert_eq!(sub.status, "active");
    assert!(sub.current_period_end > 0);

    // Cancel subscription
    let cancel_result = cancel_subscription(sub.subscription_id.clone(), None).await;
    assert_eq!(cancel_result.status, "canceled");
    assert!(cancel_result.canceled_at.is_some());
}

#[tokio::test]
async fn test_create_refund() {
    clear_all_data();
    
    // Create and confirm a payment
    let pi = create_payment_intent(
        10000,
        "usd".to_string(),
        None,
        Some(true),
        None,
        None,
        None,
        Some("pm_card_visa".to_string()),
    ).await;

    // Create partial refund
    let refund = create_refund(
        pi.payment_intent_id,
        Some(5000),
        Some(RefundReason::RequestedByCustomer),
        None,
    ).await;

    assert!(refund.refund_id.starts_with("re_"));
    assert_eq!(refund.amount, 5000);
    assert_eq!(refund.status, "succeeded");
}

#[tokio::test]
async fn test_checkout_session() {
    clear_all_data();
    
    let line_items = vec![
        StripeLineItem {
            price_id: "price_123".to_string(),
            quantity: 2,
        },
    ];

    let session = create_checkout_session(
        CheckoutMode::Payment,
        "https://example.com/success".to_string(),
        "https://example.com/cancel".to_string(),
        line_items,
        None,
        Some("checkout@example.com".to_string()),
        None,
    ).await;

    assert!(session.session_id.starts_with("cs_"));
    assert!(session.url.contains(&session.session_id));
}

#[tokio::test]
async fn test_payment_method() {
    clear_all_data();
    
    // Create customer
    let customer = create_customer(
        Some("pm@example.com".to_string()),
        None,
        None,
        None,
        None,
        None,
    ).await;

    // Create payment method
    let card = StripeCard {
        number: "4242424242424242".to_string(),
        exp_month: 12,
        exp_year: 2025,
        cvc: "123".to_string(),
    };

    let pm = create_payment_method(
        PaymentMethodType::Card,
        Some(card),
        None,
    ).await;

    assert!(pm.payment_method_id.starts_with("pm_"));
    assert_eq!(pm.r#type, "card");

    // Attach to customer
    let attach_result = attach_payment_method(
        pm.payment_method_id,
        customer.customer_id,
    ).await;

    assert!(attach_result.success);
}

#[tokio::test]
async fn test_retrieve_payment_intent() {
    clear_all_data();
    
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("order_id".to_string(), "12345".to_string());

    let pi = create_payment_intent(
        7500,
        "eur".to_string(),
        None,
        None,
        None,
        None,
        Some(metadata.clone()),
        None,
    ).await;

    let retrieved = retrieve_payment_intent(pi.payment_intent_id.clone()).await;

    assert_eq!(retrieved.payment_intent_id, pi.payment_intent_id);
    assert_eq!(retrieved.amount, 7500);
    assert_eq!(retrieved.currency, "eur");
    assert_eq!(retrieved.metadata.get("order_id"), Some(&"12345".to_string()));
}

#[tokio::test]
async fn test_retrieve_nonexistent_payment_intent() {
    clear_all_data();
    
    let retrieved = retrieve_payment_intent("pi_nonexistent".to_string()).await;
    assert_eq!(retrieved.status, "not_found");
}
