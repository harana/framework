# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Register(BaseModel):
    """
    register
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Update(BaseModel):
    """
    update
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Unregister(BaseModel):
    """
    unregister
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Trigger(BaseModel):
    """
    trigger
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Get(BaseModel):
    """
    get
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Lists(BaseModel):
    """
    lists
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Test(BaseModel):
    """
    test
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GetDeliveries(BaseModel):
    """
    get_deliveries
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RetryDelivery(BaseModel):
    """
    retry_delivery
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class VerifySignature(BaseModel):
    """
    verify_signature
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RotateSecret(BaseModel):
    """
    rotate_secret
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Webhook(BaseModel):
    """
    webhook
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class WebhookDelivery(BaseModel):
    """
    webhook_delivery
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


