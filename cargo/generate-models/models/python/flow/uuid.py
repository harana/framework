# This file is auto-generated. Do not edit manually.

from datetime import datetime
from typing import Optional, List
from enum import Enum
from pydantic import BaseModel, Field, validator, field_validator

class GenerateV4(BaseModel):
    """
    generate_v4
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class GenerateV7(BaseModel):
    """
    generate_v7
    
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


class Parse(BaseModel):
    """
    parse
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


class Uuid(BaseModel):
    """
    uuid
    
    ID fields: id
    """

    class Config:
        from_attributes = True
        populate_by_name = True


