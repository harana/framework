# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Webhook(BaseModel):
    """
    Webhook
    
    Class: webhook
    ID fields: name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    secret: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    url: str
    class Config:
        from_attributes = True
        populate_by_name = True


class WebhookDelivery(BaseModel):
    """
    Webhook Delivery
    
    Class: webhook_delivery
    ID fields: webhook_id, event_type, created_at
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    event_type: str
    payload: str
    request_headers: Optional[str] = None
    response_body: Optional[str] = None
    response_code: Optional[int] = None
    response_headers: Optional[str] = None
    retry_count: int = Field(default=0)
    status: str = Field(default="pending")
    webhook_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


class WebhookEvent(BaseModel):
    """
    Webhook Event
    
    Class: webhook_event
    ID fields: name
    """

    description: Optional[str] = None
    is_active: bool = Field(default=True)
    payload_schema: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


