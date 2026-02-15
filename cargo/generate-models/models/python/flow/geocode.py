# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Distance(BaseModel):
    """
    distance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Forward(BaseModel):
    """
    forward
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Reverse(BaseModel):
    """
    reverse
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Route(BaseModel):
    """
    route
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Timezone(BaseModel):
    """
    timezone
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GeoCoordinate(BaseModel):
    """
    geo_coordinate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GeocodingResult(BaseModel):
    """
    geocoding_result
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GeoAddress(BaseModel):
    """
    geo_address
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class RouteStep(BaseModel):
    """
    route_step
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


