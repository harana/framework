# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class StripeCustomer(BaseModel):
    """
    stripe_customer
    
    ID fields: customer_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    customer_id: str
    description: Optional[str] = None
    email: Optional[str] = None
    metadata: Optional[str] = None
    name: Optional[str] = None
    phone: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_id: Optional[str] = Field(default=None, description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class StripePaymentIntent(BaseModel):
    """
    stripe_payment_intent
    
    ID fields: payment_intent_id
    """

    amount: int
    capture_method: str = Field(default="automatic")
    client_secret: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    currency: str
    customer_id: Optional[str] = None
    description: Optional[str] = None
    metadata: Optional[str] = None
    payment_intent_id: str
    payment_method: Optional[str] = None
    status: str = Field(default="requires_payment_method")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class StripeSubscription(BaseModel):
    """
    stripe_subscription
    
    ID fields: subscription_id
    """

    cancel_at_period_end: bool = Field(default=False)
    cancelled_at: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    current_period_end: Optional[datetime] = None
    current_period_start: Optional[datetime] = None
    customer_id: str
    metadata: Optional[str] = None
    price_id: str
    quantity: int = Field(default=1)
    status: str = Field(default="active")
    subscription_id: str
    trial_end: Optional[datetime] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class StripeProduct(BaseModel):
    """
    stripe_product
    
    ID fields: product_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    metadata: Optional[str] = None
    name: str
    product_id: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class StripePrice(BaseModel):
    """
    stripe_price
    
    ID fields: price_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    currency: str
    is_active: bool = Field(default=True)
    price_id: str
    product_id: str
    recurring_interval: str
    recurring_interval_count: int = Field(default=1)
    unit_amount: int
    class Config:
        from_attributes = True
        populate_by_name = True


class StripeRefund(BaseModel):
    """
    stripe_refund
    
    ID fields: refund_id
    """

    amount: int
    created_at: datetime = Field(default_factory=datetime.utcnow)
    metadata: Optional[str] = None
    payment_intent_id: str
    reason: str
    refund_id: str
    status: str = Field(default="pending")
    class Config:
        from_attributes = True
        populate_by_name = True


class StripeWebhookEvent(BaseModel):
    """
    stripe_webhook_event
    
    ID fields: event_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    event_id: str
    event_type: str
    payload: str
    processed_at: Optional[datetime] = None
    status: str = Field(default="pending")
    class Config:
        from_attributes = True
        populate_by_name = True


