# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class SendPush(BaseModel):
    """
    send_push
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendSms(BaseModel):
    """
    send_sms
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendInApp(BaseModel):
    """
    send_in_app
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class SendBulk(BaseModel):
    """
    send_bulk
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Status(BaseModel):
    """
    status
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class MarkAsRead(BaseModel):
    """
    mark_as_read
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class List(BaseModel):
    """
    list
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Delete(BaseModel):
    """
    delete
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RegisterDevice(BaseModel):
    """
    register_device
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class UnregisterDevice(BaseModel):
    """
    unregister_device
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Notification(BaseModel):
    """
    notification
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class DeviceRegistration(BaseModel):
    """
    device_registration
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


