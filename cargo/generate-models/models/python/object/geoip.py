# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GeoipLookup(BaseModel):
    """
    geoip_lookup
    
    ID fields: ip_address
    """

    city: Optional[str] = None
    country: Optional[str] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    ip_address: str
    latitude: Optional[float] = None
    longitude: Optional[float] = None
    region: Optional[str] = None
    timezone: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class GeoipCache(BaseModel):
    """
    geoip_cache
    
    ID fields: ip_address
    """

    city: Optional[str] = None
    country: Optional[str] = None
    expires_at: Optional[datetime] = None
    ip_address: str
    latitude: Optional[float] = None
    longitude: Optional[float] = None
    looked_up_at: datetime = Field(default_factory=datetime.utcnow)
    region: Optional[str] = None
    timezone: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


