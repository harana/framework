# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Notification(BaseModel):
    """
    notification
    
    ID fields: user_id, created_at
    """

    action_url: Optional[str] = None
    body: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    is_read: bool = Field(default=False)
    read_at: Optional[datetime] = None
    title: str
    type: str = Field(default="info")
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class NotificationPreference(BaseModel):
    """
    notification_preference
    
    ID fields: user_id, notification_type, channel
    """

    channel: str = Field(default="email")
    is_enabled: bool = Field(default=True)
    notification_type: str
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class PushSubscription(BaseModel):
    """
    push_subscription
    
    ID fields: user_id, device_token
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    device_token: str
    is_active: bool = Field(default=True)
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


