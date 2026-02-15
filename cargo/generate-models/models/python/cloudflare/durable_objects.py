# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareDurableObjectNamespace(BaseModel):
    """
    cloudflare_durable_object_namespace
    
    ID fields: account_id, namespace_name
    """

    account_id: str
    class_name: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    namespace_id: Optional[str] = None
    namespace_name: str
    script_name: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectInstance(BaseModel):
    """
    cloudflare_durable_object_instance
    
    ID fields: namespace_id, object_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    has_storage: bool = Field(default=False)
    jurisdiction: Optional[str] = None
    namespace_id: str = Field(description="Reference: cf_durable_object_namespace.id")
    object_id: str
    object_name: Optional[str] = None
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectStorageEntry(BaseModel):
    """
    cloudflare_durable_object_storage_entry
    
    ID fields: instance_id, key
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    instance_id: str = Field(description="Reference: cf_durable_object_instance.id")
    key: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    value: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectAlarm(BaseModel):
    """
    cloudflare_durable_object_alarm
    
    ID fields: instance_id
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    instance_id: str = Field(description="Reference: cf_durable_object_instance.id")
    scheduled_time: datetime
    status: str = Field(default="scheduled")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareDurableObjectWebsocket(BaseModel):
    """
    cloudflare_durable_object_websocket
    
    ID fields: instance_id, websocket_id
    """

    accepted_at: datetime = Field(default_factory=datetime.utcnow)
    auto_response_request: Optional[str] = None
    auto_response_value: Optional[str] = None
    instance_id: str = Field(description="Reference: cf_durable_object_instance.id")
    is_hibernated: bool = Field(default=False)
    status: str = Field(default="open")
    tags: Optional[str] = None
    websocket_id: str
    class Config:
        from_attributes = True
        populate_by_name = True


