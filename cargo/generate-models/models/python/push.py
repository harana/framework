# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class PushDevice(BaseModel):
    """
    push_device
    
    ID fields: user_id, device_token
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    device_token: str
    is_active: bool = Field(default=True)
    platform: str = Field(default="fcm")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    user_agent: Optional[str] = None
    user_id: str = Field(description="Reference: user.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class PushTopic(BaseModel):
    """
    push_topic
    
    ID fields: name
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    description: Optional[str] = None
    is_active: bool = Field(default=True)
    name: str
    platform: str = Field(default="fcm")
    class Config:
        from_attributes = True
        populate_by_name = True


class PushTopicSubscription(BaseModel):
    """
    push_topic_subscription
    
    ID fields: topic_id, device_id
    """

    device_id: str = Field(description="Reference: push_device.id")
    subscribed_at: datetime = Field(default_factory=datetime.utcnow)
    topic_id: str = Field(description="Reference: push_topic.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class PushNotificationLog(BaseModel):
    """
    push_notification_log
    
    ID fields: device_id, sent_at
    """

    body: str
    device_id: Optional[str] = Field(default=None, description="Reference: push_device.id")
    error_message: Optional[str] = None
    message_id: Optional[str] = None
    platform: str = Field(default="fcm")
    sent_at: datetime = Field(default_factory=datetime.utcnow)
    status: str = Field(default="pending")
    title: str
    topic_id: Optional[str] = Field(default=None, description="Reference: push_topic.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class PushNotification(BaseModel):
    """
    push_notification
    
    ID fields: id
    """

    title: str
    body: str
    platform: str
    token: str
    data: str
    badge: int
    sound: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PushAction(BaseModel):
    """
    push_action
    
    ID fields: id
    """

    method: str
    title: str
    icon: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PushSubscription(BaseModel):
    """
    push_subscription
    
    ID fields: id
    """

    endpoint: str
    keys: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PushSubscriptionKeys(BaseModel):
    """
    push_subscription_keys
    
    ID fields: id
    """

    p256dh: str
    auth: str
    class Config:
        from_attributes = True
        populate_by_name = True


class PushFailure(BaseModel):
    """
    push_failure
    
    ID fields: id
    """

    token: str
    error: str
    class Config:
        from_attributes = True
        populate_by_name = True


