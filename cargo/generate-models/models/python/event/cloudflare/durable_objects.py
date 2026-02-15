# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareDurableObjectCreated(BaseModel):
    """
    cloudflare_durable_object_created
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectFetchReceived(BaseModel):
    """
    cloudflare_durable_object_fetch_received
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectStorageUpdated(BaseModel):
    """
    cloudflare_durable_object_storage_updated
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectAlarmTriggered(BaseModel):
    """
    cloudflare_durable_object_alarm_triggered
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectAlarmSet(BaseModel):
    """
    cloudflare_durable_object_alarm_set
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectAlarmDeleted(BaseModel):
    """
    cloudflare_durable_object_alarm_deleted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectWebsocketAccepted(BaseModel):
    """
    cloudflare_durable_object_websocket_accepted
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectWebsocketMessageReceived(BaseModel):
    """
    cloudflare_durable_object_websocket_message_received
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectWebsocketClosed(BaseModel):
    """
    cloudflare_durable_object_websocket_closed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectWebsocketError(BaseModel):
    """
    cloudflare_durable_object_websocket_error
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectSqlExecuted(BaseModel):
    """
    cloudflare_durable_object_sql_executed
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


