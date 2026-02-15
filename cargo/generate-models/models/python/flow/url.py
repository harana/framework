# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class Parse(BaseModel):
    """
    parse
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Build(BaseModel):
    """
    build
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Encode(BaseModel):
    """
    encode
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Decode(BaseModel):
    """
    decode
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Shorten(BaseModel):
    """
    shorten
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Expand(BaseModel):
    """
    expand
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Validate(BaseModel):
    """
    validate
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Url(BaseModel):
    """
    url
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


