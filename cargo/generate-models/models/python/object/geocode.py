# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GeocodeResult(BaseModel):
    """
    geocode_result
    
    ID fields: query, provider
    """

    city: Optional[str] = None
    confidence: Optional[float] = None
    country: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    formatted_address: Optional[str] = None
    latitude: float
    longitude: float
    postal_code: Optional[str] = None
    provider: str = Field(default="default")
    query: str
    region: Optional[str] = None
    street: Optional[str] = None
    type: str = Field(default="forward")
    class Config:
        from_attributes = True
        populate_by_name = True


class GeocodeCache(BaseModel):
    """
    geocode_cache
    
    ID fields: query_hash
    """

    created_at: datetime = Field(default_factory=datetime.utcnow)
    expires_at: Optional[datetime] = None
    latitude: float
    longitude: float
    query_hash: str
    response: str
    class Config:
        from_attributes = True
        populate_by_name = True


