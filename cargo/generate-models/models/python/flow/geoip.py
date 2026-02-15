# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Lookup(BaseModel):
    """
    lookup
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Distance(BaseModel):
    """
    distance
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Geocode(BaseModel):
    """
    geocode
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class ReverseGeocode(BaseModel):
    """
    reverse_geocode
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GeoLocation(BaseModel):
    """
    geo_location
    
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


