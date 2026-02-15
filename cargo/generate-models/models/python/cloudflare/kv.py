# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CloudflareKvNamespace(BaseModel):
    """
    cloudflare_kv_namespace
    
    ID fields: account_id, namespace_name
    """

    account_id: str
    created_at: datetime = Field(default_factory=datetime.utcnow)
    namespace_id: Optional[str] = None
    namespace_name: str
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


class CloudflareKvEntry(BaseModel):
    """
    cloudflare_kv_entry
    
    ID fields: namespace_id, key
    """

    cache_ttl: Optional[int] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    expiration: Optional[int] = None
    expiration_ttl: Optional[int] = None
    key: str
    metadata: Optional[str] = None
    namespace_id: str = Field(description="Reference: cf_kv_namespace.id")
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    value: Optional[str] = None
    value_type: str = Field(default="text")
    class Config:
        from_attributes = True
        populate_by_name = True


