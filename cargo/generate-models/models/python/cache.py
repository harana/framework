# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class CacheEntry(BaseModel):
    """
    cache_entry
    
    ID fields: namespace, key
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    expires_at: Optional[datetime] = None
    key: str
    namespace: str = Field(default="default")
    ttl_seconds: Optional[int] = None
    value: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CacheConfig(BaseModel):
    """
    cache_config
    
    ID fields: id
    """

    default_ttl_seconds: int = Field(default=3600)
    is_enabled: bool = Field(default=True)
    max_entries: int = Field(default=10000)
    strategy: str = Field(default="lru")
    class Config:
        from_attributes = True
        populate_by_name = True


class CacheValues(BaseModel):
    """
    cache_values
    
    ID fields: id
    """

    values: str
    class Config:
        from_attributes = True
        populate_by_name = True


class CacheEntries(BaseModel):
    """
    cache_entries
    
    ID fields: id
    """

    entries: str
    class Config:
        from_attributes = True
        populate_by_name = True


