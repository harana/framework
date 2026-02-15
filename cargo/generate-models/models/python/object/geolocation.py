# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GeolocationPoint(BaseModel):
    """
    geolocation_point
    
    ID fields: id
    """

    accuracy: Optional[float] = None
    altitude: Optional[float] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)
    label: Optional[str] = None
    latitude: float
    longitude: float
    metadata: Optional[str] = None
    class Config:
        from_attributes = True
        populate_by_name = True


class GeolocationRoute(BaseModel):
    """
    geolocation_route
    
    ID fields: from_point_id, to_point_id, mode
    """

    calculated_at: datetime = Field(default_factory=datetime.utcnow)
    distance: Optional[float] = None
    duration_seconds: Optional[int] = None
    from_point_id: str = Field(description="Reference: geolocation_point.id")
    mode: str = Field(default="driving")
    polyline: Optional[str] = None
    to_point_id: str = Field(description="Reference: geolocation_point.id")
    class Config:
        from_attributes = True
        populate_by_name = True


class GeolocationFence(BaseModel):
    """
    geolocation_fence
    
    ID fields: id
    """

    center_latitude: float
    center_longitude: float
    created_at: datetime = Field(default_factory=datetime.utcnow)
    is_active: bool = Field(default=True)
    radius_meters: float
    updated_at: datetime = Field(default_factory=datetime.utcnow)
    class Config:
        from_attributes = True
        populate_by_name = True


