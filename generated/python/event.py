# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Event(BaseModel):
    """
    Event
    
    Class: event
    ID fields: type, source
    """

    attributes: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    source: str
    occured_at: datetime = Field(default_factory=datetime.utcnow)
    type: str
    class Config:
        from_attributes = True
        populate_by_name = True


class EventSubscription(BaseModel):
    """
    Event Subscription
    
    Class: event_subscription
    ID fields: id
    """

    callback_url: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    event_type: str
    filter: Optional[str] = None
    id: str
    is_active: bool = Field(default=True)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class EventLog(BaseModel):
    """
    Event Log
    
    Class: event_log
    ID fields: id
    """

    error_message: Optional[str] = None
    event_id: str
    id: str
    processed_at: datetime = Field(default_factory=datetime.utcnow)
    retry_count: int = Field(default=0)
    status: str = Field(default="received")
    subscription_id: Optional[str] = Field(default=None, description="Reference: EventSubscription.id")
    class Config:
        from_attributes = True
        populate_by_name = True


