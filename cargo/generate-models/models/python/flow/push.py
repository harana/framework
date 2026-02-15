# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SendApns(BaseModel):
    """
    send_apns
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendWebPush(BaseModel):
    """
    send_web_push
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendFcm(BaseModel):
    """
    send_fcm
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendMulticastPush(BaseModel):
    """
    send_multicast_push
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendTopicPush(BaseModel):
    """
    send_topic_push
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SubscribeToTopic(BaseModel):
    """
    subscribe_to_topic
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UnsubscribeFromTopic(BaseModel):
    """
    unsubscribe_from_topic
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ValidatePushToken(BaseModel):
    """
    validate_push_token
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PushNotification(BaseModel):
    """
    push_notification
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PushAction(BaseModel):
    """
    push_action
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PushSubscription(BaseModel):
    """
    push_subscription
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PushSubscriptionKeys(BaseModel):
    """
    push_subscription_keys
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class PushFailure(BaseModel):
    """
    push_failure
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


