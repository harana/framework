# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CreatePaymentIntent(BaseModel):
    """
    create_payment_intent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ConfirmPaymentIntent(BaseModel):
    """
    confirm_payment_intent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CapturePaymentIntent(BaseModel):
    """
    capture_payment_intent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CancelPaymentIntent(BaseModel):
    """
    cancel_payment_intent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateCustomer(BaseModel):
    """
    create_customer
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UpdateCustomer(BaseModel):
    """
    update_customer
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeleteCustomer(BaseModel):
    """
    delete_customer
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetCustomer(BaseModel):
    """
    get_customer
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateSubscription(BaseModel):
    """
    create_subscription
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CancelSubscription(BaseModel):
    """
    cancel_subscription
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateRefund(BaseModel):
    """
    create_refund
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreatePrice(BaseModel):
    """
    create_price
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateProduct(BaseModel):
    """
    create_product
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreateCheckoutSession(BaseModel):
    """
    create_checkout_session
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RetrievePaymentIntent(BaseModel):
    """
    retrieve_payment_intent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ListCustomers(BaseModel):
    """
    list_customers
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CreatePaymentMethod(BaseModel):
    """
    create_payment_method
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class AttachPaymentMethod(BaseModel):
    """
    attach_payment_method
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StripePaymentIntent(BaseModel):
    """
    stripe_payment_intent
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StripeLineItem(BaseModel):
    """
    stripe_line_item
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StripeCustomer(BaseModel):
    """
    stripe_customer
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StripeBillingDetails(BaseModel):
    """
    stripe_billing_details
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StripeAddress(BaseModel):
    """
    stripe_address
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class StripeCard(BaseModel):
    """
    stripe_card
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


